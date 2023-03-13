use candid::candid_method;
use ic_cdk::query;
use ic_scalable_misc::{
    enums::filter_type::FilterType, models::paged_response_models::PagedResponse,
};

use crate::models::event_models::{EventFilter, EventResponse, EventSort};

use super::store::ScalableData;

#[query(composite = true)]
#[candid_method(query)]
async fn get_events(
    limit: usize,
    page: usize,
    filters: Vec<EventFilter>,
    filter_type: FilterType,
    sort: EventSort,
) -> PagedResponse<EventResponse> {
    ScalableData::get_child_canister_data(limit, page, filters, filter_type, sort).await
}
