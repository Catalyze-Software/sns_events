use candid::Principal;
use ic_cdk::api::{call, time};
use ic_scalable_canister::ic_scalable_misc::{
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
use ic_scalable_canister::store::Data;

use shared::event_models::{Event, EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent};

use std::{cell::RefCell, collections::HashMap, iter::FromIterator};

use crate::IDENTIFIER_KIND;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    {DefaultMemoryImpl, StableBTreeMap, StableCell},
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
        pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
            RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

        // NEW STABLE
        pub static STABLE_DATA: RefCell<StableCell<Data, Memory>> = RefCell::new(
            StableCell::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
                Data::default(),
            ).expect("failed")
        );

        // NEW STABLE
        pub static ENTRIES: RefCell<StableBTreeMap<String, Event, Memory>> = RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            )
        );

        // OLD NON-STABLE
        pub static DATA: RefCell<ic_scalable_canister::ic_scalable_misc::models::original_data::Data<Event>> = RefCell::new(ic_scalable_canister::ic_scalable_misc::models::original_data::Data::default());
}

pub struct Store;

impl Store {
    // This method is used to store a new event
    pub async fn add_event(
        caller: Principal,
        post_event: PostEvent,
        group_identifier: Principal,
        event_attendee_canister: Principal,
    ) -> Result<EventResponse, ApiError> {
        // Create a new event with the post_event data
        let new_event = Event {
            name: post_event.name,
            description: post_event.description,
            date: post_event.date,
            privacy: post_event.privacy,
            created_by: caller,
            owner: post_event.owner,
            website: post_event.website,
            location: post_event.location,
            image: post_event.image,
            banner_image: post_event.banner_image,
            tags: post_event.tags,
            // The attendee count is a hashmap with the canister id as key and the count as value
            attendee_count: HashMap::from_iter(vec![(event_attendee_canister, 1)]),
            is_canceled: (false, "".to_string()),
            is_deleted: false,
            updated_on: time(),
            created_on: time(),
            group_identifier,
            metadata: post_event.metadata,
        };

        // TODO: Validate the event data

        match STABLE_DATA.with(|data| {
            ENTRIES.with(|entries| {
                Data::add_entry(
                    data,
                    entries,
                    new_event.clone(),
                    Some(IDENTIFIER_KIND.to_string()),
                )
            })
        }) {
            // If the canister is at capacity, we spawn a new canister
            Err(err) => match err {
                ApiError::CanisterAtCapacity(message) => {
                    let _data = STABLE_DATA.with(|d| d.borrow().get().clone());
                    // Spawn a sibling canister and pass the event data to it
                    match Data::spawn_sibling(&_data, new_event).await {
                        Ok(_) => Err(ApiError::CanisterAtCapacity(message)),
                        Err(err) => Err(err),
                    }
                }
                _ => Err(err),
            },
            // If the event is stored successfully, we add the owner as an attendee on the event_attendee canister (inter-canister call)
            Ok((_identifier, event)) => {
                let add_attendee_result = Self::add_owner_as_attendee(
                    &event.owner,
                    &_identifier,
                    &group_identifier,
                    &event_attendee_canister,
                )
                .await;

                // If the attendee is added successfully, we return the event response
                match add_attendee_result {
                    Ok(_) => Ok(Self::map_to_event_response(_identifier.to_string(), event)),
                    Err(_) => {
                        ENTRIES.with(|entries| Data::remove_entry(entries, &_identifier));
                        return Err(api_error(
                            ApiErrorType::Unauthorized,
                            "ATTENDEE_ADD_FAILED",
                            "Storing the attendee failed",
                            STABLE_DATA
                                .with(|data| Data::get_name(data.borrow().get()))
                                .as_str(),
                            "add_event",
                            None,
                        ));
                    }
                }
            }
        }
    }

    // This method is used to edit an event
    pub async fn edit_event(
        identifier: Principal,
        update_event: UpdateEvent,
        event_attendee_canister: Principal,
    ) -> Result<EventResponse, ApiError> {
        // Get the event from the canister
        let response = STABLE_DATA.with(|data| {
            ENTRIES.with(|entries| match Data::get_entry(data, entries, identifier) {
                // If the event is not found, we return an error
                Err(err) => Err(err),
                // If the event is found, we check if the caller is the owner of the event
                Ok((_identifier, mut _existing_event)) => {
                    _existing_event.name = update_event.name;
                    _existing_event.description = update_event.description;
                    _existing_event.date = update_event.date;
                    _existing_event.privacy = update_event.privacy;
                    _existing_event.website = update_event.website;
                    _existing_event.location = update_event.location;
                    _existing_event.image = update_event.image;
                    _existing_event.banner_image = update_event.banner_image;
                    _existing_event.owner = update_event.owner;
                    _existing_event.metadata = update_event.metadata;
                    _existing_event.tags = update_event.tags;
                    _existing_event.updated_on = time();

                    // Update the event
                    match Data::update_entry(data, entries, _identifier, _existing_event) {
                        Err(err) => Err(err),
                        Ok((__identifier, event)) => Ok((
                            Ok(Self::map_to_event_response(
                                __identifier.to_string(),
                                event.clone(),
                            )),
                            event.group_identifier.clone(),
                        )),
                    }
                }
            })
        });

        let _response = response.clone().unwrap();
        let user_principal = &_response.0.unwrap().owner;
        let group_identifier = &_response.1;

        let _ = Self::add_owner_as_attendee(
            &user_principal,
            &identifier,
            group_identifier,
            &event_attendee_canister,
        )
        .await;

        response.unwrap().0
    }

    // This method is used to delete an event
    pub fn delete_event(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<(), ApiError> {
        // Get the event from the data store
        match STABLE_DATA
            .with(|data| ENTRIES.with(|entries| Data::get_entry(data, entries, identifier)))
        {
            Err(err) => Err(err),
            // If the event is found, we check if the event belongs to the group
            Ok((_identifier, mut _event)) => {
                if &_event.group_identifier != &group_identifier {
                    return Err(api_error(
                        ApiErrorType::NotFound,
                        "EVENT_NOT_FOUND",
                        "No event found for this group",
                        STABLE_DATA
                            .with(|data| Data::get_name(data.borrow().get()))
                            .as_str(),
                        "delete_event",
                        None,
                    ));
                }
                // Set the is_deleted flag to true
                _event.is_deleted = true;

                // Update the event
                match STABLE_DATA.with(|data| {
                    ENTRIES.with(|entries| Data::update_entry(data, entries, _identifier, _event))
                }) {
                    Err(err) => Err(err),
                    Ok((_identifier, _event)) => Ok(()),
                }
            }
        }
    }

    // This method is used to cancel an event
    pub fn cancel_event(
        identifier: Principal,
        reason: String,
        group_identifier: Principal,
    ) -> Result<(), ApiError> {
        // Get the event from the data store
        match STABLE_DATA
            .with(|data| ENTRIES.with(|entries| Data::get_entry(data, entries, identifier)))
        {
            // If the event is not found, we return an error
            Err(err) => Err(err),
            // If the event is found, we check if the event belongs to the group
            Ok((_identifier, mut _event)) => {
                if &_event.group_identifier != &group_identifier {
                    return Err(api_error(
                        ApiErrorType::NotFound,
                        "EVENT_NOT_FOUND",
                        "No event found for this group",
                        STABLE_DATA
                            .with(|data| Data::get_name(data.borrow().get()))
                            .as_str(),
                        "cancel_event",
                        None,
                    ));
                }
                // Set the is_canceled flag to true and specify a reason of cancellation
                _event.is_canceled = (true, reason);

                // Update the event
                match STABLE_DATA.with(|data| {
                    ENTRIES.with(|entries| Data::update_entry(data, entries, _identifier, _event))
                }) {
                    Err(err) => Err(err),
                    Ok((_identifier, _event)) => Ok(()),
                }
            }
        }
    }

    // This method is used to get an event
    pub fn get_event(
        identifier: Principal,
        group_identifier: Option<Principal>,
    ) -> Result<EventResponse, ApiError> {
        // Get the event from the data store
        STABLE_DATA.with(|data| {
            ENTRIES.with(|entries| match Data::get_entry(data, entries, identifier) {
                // If the event is not found, we return an error
                Err(err) => Err(err),
                // If the event is found, we check if the event belongs to the group
                Ok((_identifier, event)) => {
                    if let Some(_group_identifier) = group_identifier {
                        if &event.group_identifier != &_group_identifier {
                            return Err(api_error(
                                ApiErrorType::NotFound,
                                "EVENT_NOT_FOUND",
                                "No event found for this group",
                                Data::get_name(data.borrow().get()).as_str(),
                                "get_event",
                                None,
                            ));
                        }
                        return Ok(Self::map_to_event_response(_identifier.to_string(), event));
                    }
                    Ok(Self::map_to_event_response(_identifier.to_string(), event))
                }
            })
        })
    }

    // This method is used to get the privacy and owner of an event
    pub fn get_event_privacy_and_owner(
        identifier: Principal,
        group_identifier: Principal,
    ) -> Result<(Principal, Privacy), ApiError> {
        // Get the event from the data store
        match Self::get_event(identifier, Some(group_identifier)) {
            Err(err) => Err(err),
            Ok(_response) => Ok((_response.owner, _response.privacy)),
        }
    }

    // This method is used to get the events for a group filtered, sorted and paginated
    pub fn get_events(
        limit: usize,
        page: usize,
        sort: EventSort,
        filters: Vec<EventFilter>,
        filter_type: FilterType,
        group_identifier: Option<Principal>,
    ) -> PagedResponse<EventResponse> {
        // Get all the events
        let entries = ENTRIES.with(|entries| Data::get_entries(entries));

        // Filter the events by group identifier
        let events: Vec<EventResponse> = entries
            .into_iter()
            .filter(|(_, _event)| !_event.is_deleted)
            .filter(|(_, _event)| {
                if let Some(_group_identifier) = group_identifier {
                    return &_event.group_identifier == &_group_identifier;
                }
                true
            })
            .map(|(id, event)| Self::map_to_event_response(id, event))
            .collect();

        // Filter the events by the filters
        let filtered_events = Self::get_filtered_events(events, filters, filter_type);

        // Sort the events
        let ordered_events = Self::get_ordered_events(filtered_events, sort);

        get_paged_data(ordered_events, limit, page)
    }

    // This method is used to get the events count for a set of groups
    pub fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
        // Initialize the vector that will contain the events count for each group
        let mut events_counts: Vec<(Principal, usize)> = vec![];

        ENTRIES.with(|entries| {
            // For each group, we count the number of events
            for group_identifier in group_identifiers {
                let count = Data::get_entries(entries)
                    .into_iter()
                    .filter(|(_, _event)| &_event.group_identifier == &group_identifier)
                    .count();

                // We add the group identifier and the count to the vector
                events_counts.push((group_identifier, count));
            }
        });
        events_counts
    }

    // Used for composite_query calls from the parent canister
    //
    // Method to get filtered events serialized and chunked
    pub fn get_chunked_data(
        filters: Vec<EventFilter>,
        filter_type: FilterType,
        chunk: usize,
        max_bytes_per_chunk: usize,
    ) -> (Vec<u8>, (usize, usize)) {
        // Get all the events
        let events = ENTRIES.with(|entries| Data::get_entries(entries));
        // Filter out deleted events and map the events to EventResponse
        let mapped_events: Vec<EventResponse> = events
            .iter()
            .filter(|(_identifier, _event_data)| !_event_data.is_deleted)
            .map(|(_identifier, _event_data)| {
                Self::map_to_event_response(_identifier.clone(), _event_data.clone())
            })
            .collect();

        // Filter the events by the filters specified in the method arguments
        let filtered_events = Self::get_filtered_events(mapped_events, filters, filter_type);
        // Serialize the events
        if let Ok(bytes) = serialize(&filtered_events) {
            // Check if the bytes of the serialized events are greater than the max bytes per chunk specified as an argument
            if bytes.len() >= max_bytes_per_chunk {
                // Get the start and end index of the bytes to be returned
                let start = chunk * max_bytes_per_chunk;
                let end = (chunk + 1) * (max_bytes_per_chunk);

                // Get the bytes to be returned, if the end index is greater than the length of the bytes, return the remaining bytes
                let response = if end >= bytes.len() {
                    bytes[start..].to_vec()
                } else {
                    bytes[start..end].to_vec()
                };

                // Determine the max number of chunks that can be returned, a float is used because the number of chunks can be a decimal in this step
                let mut max_chunks: f64 = 0.00;
                if max_bytes_per_chunk < bytes.len() {
                    max_chunks = (bytes.len() / max_bytes_per_chunk) as f64;
                }

                // return the response and start and end chunk index, the end chunk index is calculated by rounding up the max chunks
                return (response, (chunk, max_chunks.ceil() as usize));
            }

            // if the bytes of the serialized groups are less than the max bytes per chunk specified as an argument, return the bytes and start and end chunk index as 0
            return (bytes, (0, 0));
        } else {
            // if the groups cant be serialized return an empty vec and start and end chunk index as 0
            return (vec![], (0, 0));
        }
    }

    // Method to map events to a default reponse that can be used on the frontend
    fn map_to_event_response(identifier: String, event: Event) -> EventResponse {
        EventResponse {
            identifier: Principal::from_text(identifier).unwrap_or(Principal::anonymous()),
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
            // Sum the attendee count for each event
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
            metadata: event.metadata,
            group_identifier: event.group_identifier,
        }
    }

    // Method to filter events
    fn get_filtered_events(
        events: Vec<EventResponse>,
        filters: Vec<EventFilter>,
        filter_type: FilterType,
    ) -> Vec<EventResponse> {
        if let FilterType::Or = filter_type {
            if filters.len() == 0 {
                return events;
            }
        }

        // filter out deleted events
        let mut filtered_events: Vec<EventResponse> = events
            .into_iter()
            .filter(|event| !event.is_deleted)
            .collect();

        use FilterType::*;
        match filter_type {
            // this filter type will return events that match all the filters
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
            // This filter type will return events that match any of the filters
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

    // This function will sort the events based on the sort type
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

    pub fn update_attendee_count_on_event(
        event_identifier: Principal,
        event_attendee_canister: Principal,
        attendee_count: usize,
    ) -> Result<(), bool> {
        // Check if the event identifier is valid
        let (_, _, _event_kind) = Identifier::decode(&event_identifier);

        if IDENTIFIER_KIND != _event_kind {
            return Err(false);
        };

        STABLE_DATA.with(|data| {
            ENTRIES.with(|entries| {
                // Get the event
                let existing = Data::get_entry(data, entries, event_identifier);
                match existing {
                    // Update the event
                    Ok((_, mut _event)) => {
                        _event
                            .attendee_count
                            .insert(event_attendee_canister, attendee_count);
                        let _ = Data::update_entry(data, entries, event_identifier, _event);
                        Ok(())
                    }
                    Err(_) => Err(false),
                }
            })
        })
    }

    // This method is used for role / permission based access control
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

    // This method is used for role / permission based access control
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

    // This method is used for role / permission based access control
    pub async fn can_edit(
        caller: Principal,
        event_identifier: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        if let Ok(event) = Self::get_event(event_identifier, Some(group_identifier)) {
            if event.owner == caller {
                return Ok(caller);
            }
        }

        return Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Edit,
        )
        .await;
    }

    // This method is used for role / permission based access control
    pub async fn can_delete(
        caller: Principal,
        event_identifier: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
    ) -> Result<Principal, ApiError> {
        if let Ok(event) = Self::get_event(event_identifier, Some(group_identifier)) {
            if event.owner == caller {
                return Ok(caller);
            }
        }
        Self::check_permission(
            caller,
            group_identifier,
            member_identifier,
            PermissionActionType::Delete,
        )
        .await
    }

    // This method is used for role / permission based access control
    async fn check_permission(
        caller: Principal,
        group_identifier: Principal,
        member_identifier: Principal,
        permission: PermissionActionType,
    ) -> Result<Principal, ApiError> {
        let group_roles = get_group_roles(group_identifier).await;
        let member_roles = get_member_roles(member_identifier, group_identifier).await;
        let name = STABLE_DATA
            .with(|data| Data::get_name(data.borrow().get()))
            .to_string();

        match member_roles {
            Ok((_principal, _roles)) => {
                if caller != _principal {
                    return Err(api_error(
                        ApiErrorType::Unauthorized,
                        "PRINCIPAL_MISMATCH",
                        "Principal mismatch",
                        name.as_str(),
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
                                name.as_str(),
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
                        name.as_str(),
                        "check_permission",
                        None,
                    )),
                }
            }
            Err(err) => Err(api_error(
                ApiErrorType::Unauthorized,
                "NO_PERMISSION",
                err.as_str(),
                name.as_str(),
                "check_permission",
                None,
            )),
        }
    }

    // Add the owner as an attendee to the event attendee canister (inter-canister call)
    async fn add_owner_as_attendee(
        user_principal: &Principal,
        event_identifier: &Principal,
        group_identifier: &Principal,
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
