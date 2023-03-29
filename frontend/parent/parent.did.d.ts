import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Address {
  'street' : string,
  'country' : string,
  'city' : string,
  'postal_code' : string,
  'label' : string,
  'state_or_province' : string,
  'house_number' : string,
  'house_number_addition' : string,
}
export type ApiError = { 'SerializeError' : ErrorMessage } |
  { 'DeserializeError' : ErrorMessage } |
  { 'NotFound' : ErrorMessage } |
  { 'ValidationError' : Array<ValidationResponse> } |
  { 'CanisterAtCapacity' : ErrorMessage } |
  { 'UpdateRequired' : UpdateMessage } |
  { 'Unauthorized' : ErrorMessage } |
  { 'Unexpected' : ErrorMessage } |
  { 'BadRequest' : ErrorMessage };
export type Asset = { 'Url' : string } |
  { 'None' : null } |
  { 'CanisterStorage' : CanisterStorage };
export type CanisterStorage = { 'None' : null } |
  { 'Manifest' : Manifest } |
  { 'Chunk' : ChunkData };
export type CanisterType = { 'Empty' : null } |
  { 'Foundation' : null } |
  { 'Custom' : null } |
  { 'ScalableChild' : null } |
  { 'Scalable' : null };
export interface ChunkData {
  'chunk_id' : bigint,
  'canister' : Principal,
  'index' : bigint,
}
export interface DateRange { 'end_date' : bigint, 'start_date' : bigint }
export interface ErrorMessage {
  'tag' : string,
  'message' : string,
  'inputs' : [] | [Array<string>],
  'location' : string,
}
export type EventFilter = { 'Tag' : number } |
  { 'UpdatedOn' : DateRange } |
  { 'Name' : string } |
  { 'Identifiers' : Array<Principal> } |
  { 'IsCanceled' : boolean } |
  { 'StartDate' : DateRange } |
  { 'Owner' : Principal } |
  { 'CreatedOn' : DateRange } |
  { 'EndDate' : DateRange };
export interface EventResponse {
  'updated_on' : bigint,
  'banner_image' : Asset,
  'owner' : Principal,
  'date' : DateRange,
  'attendee_count' : bigint,
  'name' : string,
  'tags' : Uint32Array | number[],
  'description' : string,
  'created_by' : Principal,
  'created_on' : bigint,
  'website' : string,
  'privacy' : Privacy,
  'is_canceled' : [boolean, string],
  'image' : Asset,
  'identifier' : Principal,
  'location' : Location,
  'is_deleted' : boolean,
}
export type EventSort = { 'UpdatedOn' : SortDirection } |
  { 'AttendeeCount' : SortDirection } |
  { 'StartDate' : SortDirection } |
  { 'CreatedOn' : SortDirection } |
  { 'EndDate' : SortDirection };
export type FilterType = { 'Or' : null } |
  { 'And' : null };
export interface Gated {
  'principal' : Principal,
  'name' : string,
  'description' : string,
  'amount' : bigint,
  'standard' : string,
}
export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type Location = { 'None' : null } |
  { 'Digital' : string } |
  { 'Physical' : PhysicalLocation };
export interface Manifest { 'entries' : Array<ChunkData> }
export interface PagedResponse {
  'total' : bigint,
  'data' : Array<EventResponse>,
  'page' : bigint,
  'limit' : bigint,
  'number_of_pages' : bigint,
}
export interface PhysicalLocation {
  'longtitude' : number,
  'address' : Address,
  'lattitude' : number,
}
export type Privacy = { 'Gated' : Array<Gated> } |
  { 'Private' : null } |
  { 'Public' : null } |
  { 'InviteOnly' : null };
export type Result = { 'Ok' : Principal } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : ScalableCanisterDetails } |
  { 'Err' : string };
export interface ScalableCanisterDetails {
  'entry_range' : [bigint, [] | [bigint]],
  'principal' : Principal,
  'wasm_version' : WasmVersion,
  'is_available' : boolean,
  'canister_type' : CanisterType,
}
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface UpdateMessage {
  'canister_principal' : Principal,
  'message' : string,
}
export interface ValidationResponse { 'field' : string, 'message' : string }
export type WasmVersion = { 'None' : null } |
  { 'Version' : bigint } |
  { 'Custom' : null };
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'accept_cycles' : ActorMethod<[], bigint>,
  'close_child_canister_and_spawn_sibling' : ActorMethod<
    [Principal, bigint, Uint8Array | number[], [] | [Principal]],
    Result
  >,
  'get_available_canister' : ActorMethod<[], Result_1>,
  'get_canisters' : ActorMethod<[], Array<ScalableCanisterDetails>>,
  'get_events' : ActorMethod<
    [bigint, bigint, Array<EventFilter>, FilterType, EventSort],
    PagedResponse
  >,
  'get_latest_wasm_version' : ActorMethod<[], WasmVersion>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}