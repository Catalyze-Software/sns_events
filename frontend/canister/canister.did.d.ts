import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ApiError = { 'SerializeError' : ErrorMessage } |
  { 'DeserializeError' : ErrorMessage } |
  { 'NotFound' : ErrorMessage } |
  { 'ValidationError' : Array<ValidationResponse> } |
  { 'CanisterAtCapacity' : ErrorMessage } |
  { 'UpdateRequired' : UpdateMessage } |
  { 'Unauthorized' : ErrorMessage } |
  { 'Unexpected' : ErrorMessage } |
  { 'BadRequest' : ErrorMessage };
export type ApplicationRole = { 'Blocked' : null } |
  { 'Guest' : null } |
  { 'Member' : null } |
  { 'Banned' : null } |
  { 'Admin' : null } |
  { 'Moderator' : null } |
  { 'Leader' : null } |
  { 'Owner' : null } |
  { 'Watcher' : null };
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
export interface CodeOfConductDetails {
  'approved_date' : bigint,
  'approved_version' : bigint,
}
export interface DateRange { 'end_date' : bigint, 'start_date' : bigint }
export interface ErrorMessage {
  'tag' : string,
  'message' : string,
  'inputs' : [] | [Array<string>],
  'location' : string,
}
export type FilterType = { 'Or' : null } |
  { 'And' : null };
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
export interface Manifest { 'entries' : Array<ChunkData> }
export interface PagedResponse {
  'total' : bigint,
  'data' : Array<ProfileResponse>,
  'page' : bigint,
  'limit' : bigint,
  'number_of_pages' : bigint,
}
export type ProfileFilter = { 'Interest' : number } |
  { 'Email' : string } |
  { 'Skill' : number } |
  { 'DisplayName' : string } |
  { 'UpdatedOn' : DateRange } |
  { 'City' : string } |
  { 'FirstName' : string } |
  { 'LastName' : string } |
  { 'Cause' : number } |
  { 'StateOrProvince' : string } |
  { 'Country' : string } |
  { 'CreatedOn' : DateRange } |
  { 'Username' : string };
export type ProfilePrivacy = { 'Private' : null } |
  { 'Public' : null };
export interface ProfileResponse {
  'updated_on' : bigint,
  'profile_image' : Asset,
  'principal' : Principal,
  'banner_image' : Asset,
  'about' : string,
  'country' : string,
  'username' : string,
  'interests' : Uint32Array | number[],
  'city' : string,
  'created_on' : bigint,
  'email' : string,
  'website' : string,
  'display_name' : string,
  'extra' : string,
  'privacy' : ProfilePrivacy,
  'wallets' : Array<WalletResponse>,
  'state_or_province' : string,
  'first_name' : string,
  'last_name' : string,
  'member_identifier' : Principal,
  'causes' : Uint32Array | number[],
  'code_of_conduct' : CodeOfConductDetails,
  'date_of_birth' : bigint,
  'identifier' : Principal,
  'skills' : Uint32Array | number[],
  'application_role' : ApplicationRole,
}
export type ProfileSort = { 'Email' : SortDirection } |
  { 'DisplayName' : SortDirection } |
  { 'UpdatedOn' : SortDirection } |
  { 'City' : SortDirection } |
  { 'FirstName' : SortDirection } |
  { 'LastName' : SortDirection } |
  { 'StateOrProvince' : SortDirection } |
  { 'Country' : SortDirection } |
  { 'CreatedOn' : SortDirection } |
  { 'Username' : SortDirection };
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
export interface WalletResponse {
  'principal' : Principal,
  'provider' : string,
  'is_primary' : boolean,
}
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
  'get_latest_wasm_version' : ActorMethod<[], WasmVersion>,
  'get_profiles' : ActorMethod<
    [bigint, bigint, Array<ProfileFilter>, FilterType, ProfileSort],
    PagedResponse
  >,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}
