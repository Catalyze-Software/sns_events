use ic_cdk::query;
use ic_scalable_canister::ic_scalable_misc::{
    enums::filter_type::FilterType, models::paged_response_models::PagedResponse,
};

use shared::event_models::{EventFilter, EventResponse, EventSort};

use super::store::ScalableData;

// Method used to get all the events from the child canisters filtered, sorted and paged
// requires composite queries to be released to mainnet
// TODO: Add group identifier
// #[query(composite = true)]
#[query]
async fn get_events(
    limit: usize,
    page: usize,
    filters: Vec<EventFilter>,
    filter_type: FilterType,
    sort: EventSort,
) -> PagedResponse<EventResponse> {
    ScalableData::get_child_canister_data(limit, page, filters, filter_type, sort).await
}
