use std::{collections::HashMap, iter::FromIterator};

use candid::{candid_method, Principal};
use ic_cdk::caller;
use ic_cdk_macros::{query, update};
use ic_scalable_misc::{
    enums::{api_error_type::ApiError, filter_type::FilterType, privacy_type::Privacy},
    models::paged_response_models::PagedResponse,
};

use super::store::{Store, DATA};
use shared::event_models::{Event, EventFilter, EventResponse, EventSort, PostEvent, UpdateEvent};

#[update]
#[candid_method(update)]
pub fn migration_add_events(events: Vec<(Principal, Event)>) -> () {
    if caller()
        == Principal::from_text("ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe")
            .unwrap()
    {
        DATA.with(|data| {
            data.borrow_mut().current_entry_id = events.clone().len() as u64;
            data.borrow_mut().entries = HashMap::from_iter(events);
        })
    }
}

// This method is used to add a event to the canister,
// The method is async because it optionally creates a new canister
#[update]
#[candid_method(update)]
async fn add_event(
    value: PostEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> Result<EventResponse, ApiError> {
    match Store::can_write(caller(), group_identifier, member_identifier).await {
        Ok(_caller) => {
            Store::add_event(_caller, value, group_identifier, event_attendee_canister).await
        }
        Err(err) => Err(err),
    }
}

// This method is used to get an event
#[query]
#[candid_method(query)]
fn get_event(
    identifier: Principal,
    group_identifier: Option<Principal>,
) -> Result<EventResponse, ApiError> {
    Store::get_event(identifier, group_identifier)
}

// This method is used to get the privacy and owner of an event
#[query]
#[candid_method(query)]
fn get_event_privacy_and_owner(
    identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Privacy), ApiError> {
    Store::get_event_privacy_and_owner(identifier, group_identifier)
}

// This method is used to get events filtered and sorted with pagination
#[query]
#[candid_method(query)]
fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filter: Vec<EventFilter>,
    filter_type: FilterType,
    group_identifier: Option<Principal>,
) -> Result<PagedResponse<EventResponse>, ApiError> {
    Ok(Store::get_events(
        limit,
        page,
        sort,
        filter,
        filter_type,
        group_identifier,
    ))
}

// COMPOSITE_QUERY PREPARATION
// This methods is used by the parent canister to get filtered events the (this) child canister
// Data serialized and send as byte array chunks ` (bytes, (start_chunk, end_chunk)) `
// The parent canister can then deserialize the data and pass it to the frontend
#[query]
#[candid_method(query)]
fn get_chunked_data(
    filters: Vec<EventFilter>,
    filter_type: FilterType,
    chunk: usize,
    max_bytes_per_chunk: usize,
) -> (Vec<u8>, (usize, usize)) {
    if caller() != DATA.with(|data| data.borrow().parent) {
        return (vec![], (0, 0));
    }

    Store::get_chunked_data(filters, filter_type, chunk, max_bytes_per_chunk)
}

// This method is used to get the amount of events for a list of groups
#[query]
#[candid_method(query)]
fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {
    Store::get_events_count(group_identifiers)
}

// This method is used to update an existing event
#[update]
#[candid_method(update)]
async fn edit_event(
    identifier: Principal,
    value: UpdateEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> Result<EventResponse, ApiError> {
    match Store::can_edit(caller(), identifier, group_identifier, member_identifier).await {
        Ok(_caller) => Store::edit_event(identifier, value, event_attendee_canister).await,
        Err(err) => Err(err),
    }
}

// This method is used to delete an existing event
#[update]
#[candid_method(update)]
async fn delete_event(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    match Store::can_delete(caller(), identifier, group_identifier, member_identifier).await {
        Ok(_caller) => Store::delete_event(identifier, group_identifier),
        Err(err) => Err(err),
    }
}

// This method is used to cancel an event
#[update]
#[candid_method(update)]
async fn cancel_event(
    identifier: Principal,
    reason: String,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {
    match Store::can_edit(caller(), identifier, group_identifier, member_identifier).await {
        Ok(_caller) => Store::cancel_event(identifier, reason, group_identifier),
        Err(err) => Err(err),
    }
}

// This method is used to update the attendee count on an event (inter-canister call)
#[update]
#[candid_method(update)]
pub fn update_attendee_count_on_event(
    event_identifier: Principal,
    event_attendee_canister: Principal,
    attendee_count: usize,
) -> Result<(), bool> {
    let _caller = caller();
    if _caller == event_attendee_canister {
        return Store::update_attendee_count_on_event(
            event_identifier,
            event_attendee_canister,
            attendee_count,
        );
    }
    return Err(false);
}
