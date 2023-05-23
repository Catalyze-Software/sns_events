# Event canister

Catalyze proposes a powerful event management system where communities can organize and plan their events. Community managers have the ability to fine-tune their events through various options including online vs. IRL, public vs. private, single-day vs, multi-day events. These events will then be showcased on the community’s profile and their calendar.

## setup

The parent canister is SNS controlled, the child canisters are controlled by their parent. Upgrading the child canister is done through the parent canister as the (gzipped) child wasm is included in the parent canister.

When the parent canister is upgraded it checks if the child wasm has changed (currently it generates a new wasm hash every time you run the script). if changed it upgrades the child canisters automatically.

## Project structure

**|- candid**
Contains the candid files for the `parent` and `child` canister.

**|- frontend**
Contains all declarations that are needed for the frontend

**|- scripts**
Contains a single script that generates the following files for the parent and child canisters;

- candid files
- frontend declarations
- wasms (gzipped and regular)

**|- src/child**
Contains codebase related to the child canisters
**|- src/parent**
Contains codebase related to the child canisters
**|- src/shared**
Contains data used by both codebases

**|- wasm**
Contains

- child wasm
- child wasm (gzipped)
- parent wasm
- parent wasm (gzipped)

## Parent canister

The parent canister manages all underlying child canisters.

#### This canister is responsible for;

- keeping track of all event child canisters
- spinning up a new child canisters
- composite query call to the children (preperation)

#### methods

Described methods can be found below, for more details you can check out the code which is inline commented

###### DEFAULT

```
// Stores the data in stable storage before upgrading the canister.
pub fn pre_upgrade() {}

// Restores the data from stable- to heap storage after upgrading the canister.
pub fn post_upgrade() {}

// Init methods thats get triggered when the canister is installed
pub fn init() {}
```

##

###### QUERY CALLS

```
// Method to retrieve an available canister to write updates to
fn get_available_canister() -> Result<ScalableCanisterDetails, String> {}

// Method to retrieve all the canisters
fn get_canisters() -> Vec<ScalableCanisterDetails> {}

// Method to retrieve the latest wasm version of the child canister that is currently stored
fn get_latest_wasm_version() -> WasmVersion {}

// HTTP request handler (canister metrics are added to the response)
fn http_request(req: HttpRequest) -> HttpResponse {}

// Method used to get all the events from the child canisters filtered, sorted and paged
// requires composite queries to be released to mainnet
async fn get_events(
    limit: usize,
    page: usize,
    filters: Vec<EventFilter>,
    filter_type: FilterType,
    sort: EventSort,
) -> PagedResponse<EventResponse> {}
```

##

###### UPDATE CALLS

```
// Method called by child canister once full (inter-canister call)
// can only be called by a child canister
async fn close_child_canister_and_spawn_sibling(
    last_entry_id: u64,
    entry: Vec<u8>
    ) -> Result<Principal, ApiError> {}

// Method to accept cycles when send to this canister
fn accept_cycles() -> u64 {}
```

## Child canister

The child canister is where the data is stored that the app uses.

This canister is responsible for;

- storing data records
- data validation
- messaging the parent to spin up a new sibling

#### methods

Described methods can be found below, for more details you can check out the code which is inline commented

###### DEFAULT

```
// Stores the data in stable storage before upgrading the canister.
pub fn pre_upgrade() {}

// Restores the data from stable- to heap storage after upgrading the canister.
pub fn post_upgrade() {}

// Init methods thats get triggered when the canister is installed
pub fn init(parent: Principal, name: String, identifier: usize) {}
```

##

###### QUERY CALLS

```
// This method is used to get an event
fn get_event(
    identifier: Principal,
    group_identifier: Principal,
) -> Result<EventResponse, ApiError> {}

// This method is used to get the privacy and owner of an event
fn get_event_privacy_and_owner(
    identifier: Principal,
    group_identifier: Principal,
) -> Result<(Principal, Privacy), ApiError> {}

// This method is used to get events filtered and sorted with pagination
fn get_events(
    limit: usize,
    page: usize,
    sort: EventSort,
    filter: Vec<EventFilter>,
    filter_type: FilterType,
    group_identifier: Principal,
) -> Result<PagedResponse<EventResponse>, ApiError> {}

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
fn get_events_count(group_identifiers: Vec<Principal>) -> Vec<(Principal, usize)> {}

// HTTP request handler, canister metrics are added to the response by default
fn http_request(req: HttpRequest) -> HttpResponse {}
```

###

###### UPDATE CALLS

```
// This method is used to add a event to the canister,
// The method is async because it optionally creates a new canister
async fn add_event(
    value: PostEvent,
    group_identifier: Principal,
    member_identifier: Principal,
    event_attendee_canister: Principal,
) -> Result<EventResponse, ApiError> {}

// This method is used to update an existing event
async fn edit_event(
    identifier: Principal,
    value: UpdateEvent,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<EventResponse, ApiError> {}

// This method is used to delete an existing event
async fn delete_event(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {}

// This method is used to cancel an event
async fn cancel_event(
    identifier: Principal,
    reason: String,
    group_identifier: Principal,
    member_identifier: Principal,
) -> Result<(), ApiError> {}

// This method is used to update the attendee count on an event (inter-canister call)
pub fn update_attendee_count_on_event(
    event_identifier: Principal,
    event_attendee_canister: Principal,
    attendee_count: usize,
) -> Result<(), bool> {}

// This call get triggered when a new canister is spun up
// the data is passed along to the new canister as a byte array
async fn add_entry_by_parent(entry: Vec<u8>) -> Result<(), ApiError> {}

// Method to accept cycles when send to this canister
fn accept_cycles() -> u64 {}
```

## SNS controlled

// TBD

## Testing

// TBD
