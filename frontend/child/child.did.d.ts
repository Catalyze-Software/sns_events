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
export interface Event {
  'updated_on' : bigint,
  'banner_image' : Asset,
  'group_identifier' : Principal,
  'owner' : Principal,
  'metadata' : [] | [string],
  'date' : DateRange,
  'attendee_count' : Array<[Principal, bigint]>,
  'name' : string,
  'tags' : Uint32Array | number[],
  'description' : string,
  'created_by' : Principal,
  'created_on' : bigint,
  'website' : string,
  'privacy' : Privacy,
  'is_canceled' : [boolean, string],
  'image' : Asset,
  'location' : Location,
  'is_deleted' : boolean,
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
  'metadata' : [] | [string],
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
export type GatedType = { 'Neuron' : Array<NeuronGated> } |
  { 'Token' : Array<TokenGated> };
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
  { 'Physical' : PhysicalLocation } |
  { 'MultiLocation' : MultiLocation };
export interface Manifest { 'entries' : Array<ChunkData> }
export interface MultiLocation {
  'physical' : PhysicalLocation,
  'digital' : string,
}
export interface NeuronGated {
  'governance_canister' : Principal,
  'name' : string,
  'description' : string,
  'ledger_canister' : Principal,
  'rules' : Array<NeuronGatedRules>,
}
export type NeuronGatedRules = { 'IsDisolving' : boolean } |
  { 'MinStake' : bigint } |
  { 'MinAge' : bigint } |
  { 'MinDissolveDelay' : bigint };
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
export interface PostEvent {
  'banner_image' : Asset,
  'metadata' : [] | [string],
  'date' : DateRange,
  'name' : string,
  'tags' : Uint32Array | number[],
  'description' : string,
  'website' : string,
  'privacy' : Privacy,
  'image' : Asset,
  'location' : Location,
}
export type Privacy = { 'Gated' : GatedType } |
  { 'Private' : null } |
  { 'Public' : null } |
  { 'InviteOnly' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : ApiError };
export type Result_1 = { 'Ok' : EventResponse } |
  { 'Err' : ApiError };
export type Result_2 = { 'Ok' : [Principal, Privacy] } |
  { 'Err' : ApiError };
export type Result_3 = { 'Ok' : PagedResponse } |
  { 'Err' : ApiError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : boolean };
export type SortDirection = { 'Asc' : null } |
  { 'Desc' : null };
export interface TokenGated {
  'principal' : Principal,
  'name' : string,
  'description' : string,
  'amount' : bigint,
  'standard' : string,
}
export interface UpdateMessage {
  'canister_principal' : Principal,
  'message' : string,
}
export interface ValidationResponse { 'field' : string, 'message' : string }
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'accept_cycles' : ActorMethod<[], bigint>,
  'add_entry_by_parent' : ActorMethod<[Uint8Array | number[]], Result>,
  'add_event' : ActorMethod<
    [PostEvent, Principal, Principal, Principal],
    Result_1
  >,
  'cancel_event' : ActorMethod<
    [Principal, string, Principal, Principal],
    Result
  >,
  'delete_event' : ActorMethod<[Principal, Principal, Principal], Result>,
  'edit_event' : ActorMethod<
    [Principal, PostEvent, Principal, Principal],
    Result_1
  >,
  'get_chunked_data' : ActorMethod<
    [Array<EventFilter>, FilterType, bigint, bigint],
    [Uint8Array | number[], [bigint, bigint]]
  >,
  'get_event' : ActorMethod<[Principal, Principal], Result_1>,
  'get_event_privacy_and_owner' : ActorMethod<[Principal, Principal], Result_2>,
  'get_events' : ActorMethod<
    [bigint, bigint, EventSort, Array<EventFilter>, FilterType, Principal],
    Result_3
  >,
  'get_events_count' : ActorMethod<
    [Array<Principal>],
    Array<[Principal, bigint]>
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'migration_add_events' : ActorMethod<[Array<[Principal, Event]>], undefined>,
  'update_attendee_count_on_event' : ActorMethod<
    [Principal, Principal, bigint],
    Result_4
  >,
}
