use candid::Principal;
use ic_cdk::api::{call, time};
use ic_scalable_canister::store::Data;
use ic_scalable_misc::{
    enums::{
        api_error_type::{ApiError, ApiErrorType},
        filter_type::FilterType,
        privacy_type::Privacy,
        sort_type::SortDirection,
    },
    helpers::{
        error_helper::api_error,
        paging_helper::get_paged_data,
        role_helper::{default_roles, get_group_roles, get_member_roles, has_permission},
        serialize_helper::serialize,
    },
    models::{
        identifier_model::Identifier,
        paged_response_models::PagedResponse,
        permissions_models::{PermissionActionType, PermissionType},
    },
};

use shared::event_models::{Event, EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent};

use std::{cell::RefCell, collections::HashMap, iter::FromIterator};

thread_local! {
    pub static DATA: RefCell<Data<Event>> = RefCell::new(Data::default());
}

pub struct Store;

impl Store {
    pub async fn add_event(
        caller: Principal,
        post_event: PostEvent,
        group_identifier: Principal,
        event_attendee_canister: Principal,
    ) -> Result<EventResponse, ApiError> {
        let new_event = Event {
            name: post_event.name,
            description: post_event.description,
            date: post_event.date,
            privacy: post_event.privacy,
            created_by: caller,
            owner: caller,
            website: post_event.website,
            location: post_event.location,
            image: post_event.image,
            banner_image: post_event.banner_image,
            tags: post_event.tags,
            attendee_count: HashMap::from_iter(vec![(event_attendee_canister, 1)]),
            is_canceled: (false, "".to_string()),
            is_deleted: false,
            updated_on: time(),
            created_on: time(),
            group_identifier,
        };
        match DATA.with(|data| Data::add_entry(data, new_event.clone(), Some("evt".to_string()))) {
            Err(err) => match err {
                ApiError::CanisterAtCapacity(message) => {
                    let _data = DATA.with(|v| v.borrow().clone());
                    match Data::spawn_sibling(_data, new_event).await {
                        Ok(_) => Err(ApiError::CanisterAtCapacity(message)),
                        Err(err) => Err(err),
                    }
                }
                _ => Err(err),
            },
            Ok((_identifier, event)) => {
                let add_attendee_result = Self::add_owner_as_attendee(
                    &caller,
                    &_identifier,
                    group_identifier,
                    &event_attendee_canister,
                )
                .await;

                match add_attendee_result {
                    Ok(_) => Ok(Self::map_to_event_response(_identifier, event)),
                    Err(_) => {
                        DATA.with(|data| Data::remove_entry(data, &_identifier));
                        return Err(api_error(
                            ApiErrorType::Unauthorized,
                            "ATTENDEE_ADD_FAILED",
                            "Storing the attendee failed",
                            DATA.with(|data| Data::get_name(data)).as_str(),
                            "add_event",
                            None,
                        ));
                    }
                }
            }
        }
    }

    pub fn edit_event(
        caller: Principal,
        identifier: Principal,
        update_event: UpdateEvent,
    ) -> Result<EventResponse, ApiError> {
        DATA.with(|data| match Data::get_entry(data, identifier) {
            Err(err) => Err(err),
            Ok((_identifier, _existing)) => {
                if _existing.owner == caller {
                    let updated_event = Event {
                        name: update_event.name,
                        description: update_event.description,
                        date: update_event.date,
                        privacy: update_event.privacy,
                        website: update_event.website,
                        location: update_event.location,
                        image: update_event.image,
                        banner_image: update_event.banner_image,
                        tags: update_event.tags,
                        updated_on: time(),
                        .._existing
                    };
                    match DATA.with(|data| Data::update_entry(data, _identifier, updated_event)) {
                        Err(err) => Err(err),
                        Ok((__identifier, event)) => {
                            Ok(Self::map_to_event_response(__identifier, event))
                        }
                    }
                } else {
                    return Err(api_error(
                        ApiErrorType::Unauthorized,
                        "NOT_AUTHORIZED",
                        "You are not authorized to perform this action",
                        Data::get_name(data).as_str(),
                        "edit_event",
                        None,
                    ));
                }
            }
        })
    }

    pub fn delete_event(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<(), ApiError> {
        match DATA.with(|data| Data::get_entry(data, identifier)) {
            Err(err) => Err(err),
            Ok((_identifier, mut _event)) => {
                if &_event.group_identifier != &group_identifier {
                    return Err(api_error(
                        ApiErrorType::NotFound,
                        "EVENT_NOT_FOUND",
                        "No event found for this group",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "delete_event",
                        None,
                    ));
                }
                _event.is_deleted = true;
                match DATA.with(|data| Data::update_entry(data, _identifier, _event)) {
                    Err(err) => Err(err),
                    Ok((_identifier, _event)) => Ok(()),
                }
            }
        }
    }

    pub fn cancel_event(
        identifier: Principal,
        reason: String,
        group_identifier: Principal,
    ) -> Result<(), ApiError> {
        match DATA.with(|data| Data::get_entry(data, identifier)) {
            Err(err) => Err(err),
            Ok((_identifier, mut _event)) => {
                if &_event.group_identifier != &group_identifier {
                    return Err(api_error(
                        ApiErrorType::NotFound,
                        "EVENT_NOT_FOUND",
                        "No event found for this group",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "cancel_event",
                        None,
                    ));
                }
                _event.is_canceled = (true, reason);
                match DATA.with(|data| Data::update_entry(data, _identifier, _event)) {
                    Err(err) => Err(err),
                    Ok((_identifier, _event)) => Ok(()),
                }
            }
        }
    }

    pub fn get_event(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<EventResponse, ApiError> {
        DATA.with(|data| match Data::get_entry(data, identifier) {
            Err(err) => Err(err),
            Ok((_identifier, event)) => {
                if &event.group_identifier != &group_identifier {
                    return Err(api_error(
                        ApiErrorType::NotFound,
                        "EVENT_NOT_FOUND",
                        "No event found for this group",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "get_event",
                        None,
                    ));
                }
                Ok(Self::map_to_event_response(_identifier, event))
            }
        })
    }

    pub fn get_event_privacy_and_owner(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<(Principal, Privacy), ApiError> {
        match Self::get_event(identifier, group_identifier) {
            Err(err) => Err(err),
            Ok(_response) => Ok((_response.owner, _response.privacy)),
        }
    }

    pub fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<EventFilter>,
        filter_type: FilterType,
        group_identifier: Principal,
    ) -> PagedResponse<EventResponse> {
        DATA.with(|data| {
            let entries = Data::get_entries(data);
            let events: Vec<EventResponse> = entries
                .into_iter()
                .filter(|(_, _event)| &_event.group_identifier == &group_identifier)
                .map(|(id, event)| Self::map_to_event_response(id, event))
                .collect();

            let filtered_events = Self::get_filtered_events(events, filters, filter_type);
            let ordered_events = Self::get_ordered_events(filtered_events, sort);

            get_paged_data(ordered_events, limit, page)
        })
    }

    pub fn invite_for_group_event() -> () {}

    pub fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
        let mut events_counts: Vec<(Principal, usize)> = vec![];

        DATA.with(|data| {
            for group_identifier in group_identifiers {
                let count = Data::get_entries(data)
                    .into_iter()
                    .filter(|(_, _event)| &_event.group_identifier == &group_identifier)
                    .count();

                events_counts.push((group_identifier, count));
            }
        });
        events_counts
    }

    pub fn get_chunked_data(
        filters: Vec<EventFilter>,
        filter_type: FilterType,
        chunk: usize,
        max_bytes_per_chunk: usize,
    ) -> (Vec<u8>, (usize, usize)) {
        let groups = DATA.with(|data| Data::get_entries(data));
        let mapped_groups: Vec<EventResponse> = groups
            .iter()
            .filter(|(_identifier, _group_data)| !_group_data.is_deleted)
            .map(|(_identifier, _group_data)| {
                Self::map_to_event_response(_identifier.clone(), _group_data.clone())
            })
            .collect();

        let filtered_events = Self::get_filtered_events(mapped_groups, filters, filter_type);
        if let Ok(bytes) = serialize(&filtered_events) {
            if bytes.len() >= max_bytes_per_chunk {
                let start = chunk * max_bytes_per_chunk;
                let end = (chunk + 1) * (max_bytes_per_chunk);

                let response = if end >= bytes.len() {
                    bytes[start..].to_vec()
                } else {
                    bytes[start..end].to_vec()
                };

                let mut max_chunks: f64 = 0.00;
                if max_bytes_per_chunk < bytes.len() {
                    max_chunks = (bytes.len() / max_bytes_per_chunk) as f64;
                }
                return (response, (chunk, max_chunks.ceil() as usize));
            }
            return (bytes, (0, 0));
        } else {
            return (vec![], (0, 0));
        }
    }

    fn map_to_event_response(identifier: Principal, event: Event) -> EventResponse {
        EventResponse {
            identifier,
            name: event.name,
            description: event.description,
            date: event.date,
            privacy: event.privacy,
            created_by: event.created_by,
            owner: event.owner,
            website: event.website,
            location: event.location,
            image: event.image,
            banner_image: event.banner_image,
            attendee_count: event
                .attendee_count
                .into_iter()
                .map(|(_, value)| value)
                .sum(),
            tags: event.tags,
            updated_on: event.updated_on,
            created_on: event.created_on,
            is_canceled: event.is_canceled,
            is_deleted: event.is_deleted,
        }
    }

    fn get_filtered_events(
        events: Vec<EventResponse>,
        filters: Vec<EventFilter>,
        filter_type: FilterType,
    ) -> Vec<EventResponse> {
        // filter out deleted events
        let mut filtered_events: Vec<EventResponse> = events
            .into_iter()
            .filter(|event| !event.is_deleted)
            .collect();

        use FilterType::*;
        match filter_type {
            And => {
                for filter in filters {
                    use EventFilter::*;
                    match filter {
                        Identifiers(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| value.contains(&event.identifier))
                                .collect();
                        }
                        Name(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| event.name.contains(&value))
                                .collect();
                        }
                        StartDate(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.date.start_date >= value.start_date
                                            && event.date.start_date <= value.end_date;
                                    } else {
                                        return event.date.start_date >= value.start_date;
                                    }
                                })
                                .collect()
                        }
                        EndDate(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.date.end_date >= value.start_date
                                            && event.date.end_date <= value.end_date;
                                    } else {
                                        return event.date.end_date >= value.start_date;
                                    }
                                })
                                .collect();
                        }
                        Owner(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| event.owner == value)
                                .collect();
                        }
                        Tag(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| event.tags.contains(&value))
                                .collect();
                        }
                        UpdatedOn(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.updated_on >= value.start_date
                                            && event.updated_on <= value.end_date;
                                    } else {
                                        return event.updated_on >= value.start_date;
                                    }
                                })
                                .collect();
                        }
                        CreatedOn(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.created_on >= value.start_date
                                            && event.created_on <= value.end_date;
                                    } else {
                                        return event.created_on >= value.start_date;
                                    }
                                })
                                .collect();
                        }
                        IsCanceled(value) => {
                            filtered_events = filtered_events
                                .into_iter()
                                .filter(|event| value == event.is_canceled.0)
                                .collect();
                        }
                    }
                }

                filtered_events
            }
            Or => {
                let mut hashmap_events: HashMap<Principal, EventResponse> = HashMap::new();
                for filter in filters {
                    use EventFilter::*;
                    match filter {
                        Identifiers(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| value.contains(&event.identifier))
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        Name(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| event.name.contains(&value))
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        StartDate(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.date.start_date >= value.start_date
                                            && event.date.start_date <= value.end_date;
                                    } else {
                                        return event.date.start_date >= value.start_date;
                                    }
                                })
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        EndDate(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.date.end_date >= value.start_date
                                            && event.date.end_date <= value.end_date;
                                    } else {
                                        return event.date.end_date >= value.start_date;
                                    }
                                })
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        Owner(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| event.owner == value)
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        Tag(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| event.tags.contains(&value))
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        UpdatedOn(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.updated_on >= value.start_date
                                            && event.updated_on <= value.end_date;
                                    } else {
                                        return event.updated_on >= value.start_date;
                                    }
                                })
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        CreatedOn(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| {
                                    if value.end_date > 0 {
                                        return event.created_on >= value.start_date
                                            && event.created_on <= value.end_date;
                                    } else {
                                        return event.created_on >= value.start_date;
                                    }
                                })
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                        IsCanceled(value) => {
                            filtered_events
                                .iter()
                                .filter(|event| value == event.is_canceled.0)
                                .for_each(|v| {
                                    hashmap_events.insert(v.identifier.clone(), v.clone());
                                });
                        }
                    }
                }
                hashmap_events.into_iter().map(|v| v.1).collect()
            }
        }
    }

    fn get_ordered_events(mut events: Vec<EventResponse>, sort: EventSort) -> Vec<EventResponse> {
        use EventSort::*;
        use SortDirection::*;
        match sort {
            CreatedOn(direction) => match direction {
                Asc => events.sort_by(|a, b| a.created_on.cmp(&b.created_on)),
                Desc => events.sort_by(|a, b| b.created_on.cmp(&a.created_on)),
            },
            UpdatedOn(direction) => match direction {
                Asc => events.sort_by(|a, b| a.updated_on.cmp(&b.updated_on)),
                Desc => events.sort_by(|a, b| b.updated_on.cmp(&a.updated_on)),
            },
            StartDate(direction) => match direction {
                Asc => events.sort_by(|a, b| a.date.start_date.cmp(&b.date.start_date)),
                Desc => events.sort_by(|a, b| b.date.start_date.cmp(&a.date.start_date)),
            },
            EndDate(direction) => match direction {
                Asc => events.sort_by(|a, b| a.date.end_date.cmp(&b.date.end_date)),
                Desc => events.sort_by(|a, b| b.date.end_date.cmp(&a.date.end_date)),
            },
            AttendeeCount(direction) => match direction {
                Asc => events.sort_by(|a, b| a.attendee_count.cmp(&b.attendee_count)),
                Desc => events.sort_by(|a, b| b.attendee_count.cmp(&a.attendee_count)),
            },
        };

        events
    }

    // TODO: double check if this is the correct way to do this
    pub fn update_attendee_count_on_event(
        event_identifier: Principal,
        event_attendee_canister: Principal,
        attendee_count: usize,
    ) -> Result<(), bool> {
        let (_, _, _event_kind) = Identifier::decode(&event_identifier);

        if "evt" != _event_kind {
            return Err(false);
        };

        DATA.with(|data| {
            let existing = Data::get_entry(data, event_identifier);
            match existing {
                Ok((_, mut _event)) => {
                    _event
                        .attendee_count
                        .insert(event_attendee_canister, attendee_count);
                    let _ = Data::update_entry(data, event_identifier, _event);
                    Ok(())
                }
                Err(_) => Err(false),
            }
        })
    }

    pub async fn can_write(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Write,
        )
        .await
    }

    pub async fn can_read(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Read,
        )
        .await
    }

    pub async fn can_edit(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Edit,
        )
        .await
    }

    pub async fn can_delete(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Delete,
        )
        .await
    }

    async fn check_permission(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
        permission: PermissionActionType,
    ) -> Result<Principal, ApiError> {
        let group_roles = get_group_roles(group_identifier).await;
        let member_roles = get_member_roles(member_identifier, group_identifier).await;

        match member_roles {
            Ok((_principal, _roles)) => {
                if caller != _principal {
                    return Err(api_error(
                        ApiErrorType::Unauthorized,
                        "PRINCIPAL_MISMATCH",
                        "Principal mismatch",
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "check_permission",
                        None,
                    ));
                }

                match group_roles {
                    Ok(mut _group_roles) => {
                        _group_roles.append(&mut default_roles());
                        let has_permission = has_permission(
                            &_roles,
                            &PermissionType::Event(None),
                            &_group_roles,
                            &permission,
                        );

                        if !has_permission {
                            return Err(api_error(
                                ApiErrorType::Unauthorized,
                                "NO_PERMISSION",
                                "No permission",
                                DATA.with(|data| Data::get_name(data)).as_str(),
                                "check_permission",
                                None,
                            ));
                        }

                        Ok(caller)
                    }
                    Err(err) => Err(api_error(
                        ApiErrorType::Unauthorized,
                        "NO_PERMISSION",
                        err.as_str(),
                        DATA.with(|data| Data::get_name(data)).as_str(),
                        "check_permission",
                        None,
                    )),
                }
            }
            Err(err) => Err(api_error(
                ApiErrorType::Unauthorized,
                "NO_PERMISSION",
                err.as_str(),
                DATA.with(|data| Data::get_name(data)).as_str(),
                "check_permission",
                None,
            )),
        }
    }

    async fn add_owner_as_attendee(
        user_principal: &Principal,
        event_identifier: &Principal,
        group_identifier: Principal,
        event_attendee_canister: &Principal,
    ) -> Result<(), bool> {
        let add_owner_response: Result<(Result<(), bool>,), _> = call::call(
            event_attendee_canister.clone(),
            "add_owner_as_attendee",
            (user_principal, event_identifier, group_identifier),
        )
        .await;

        match add_owner_response {
            Ok((Ok(_),)) => Ok(()),
            Ok((Err(_),)) => Err(false),
            Err(err) => {
                println!("{:?}", err.1);
                Err(false)
            }
        }
    }
}
