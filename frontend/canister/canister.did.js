export const idlFactory = ({ IDL }) => {
  const ErrorMessage = IDL.Record({
    'tag' : IDL.Text,
    'message' : IDL.Text,
    'inputs' : IDL.Opt(IDL.Vec(IDL.Text)),
    'location' : IDL.Text,
  });
  const ValidationResponse = IDL.Record({
    'field' : IDL.Text,
    'message' : IDL.Text,
  });
  const UpdateMessage = IDL.Record({
    'canister_principal' : IDL.Principal,
    'message' : IDL.Text,
  });
  const ApiError = IDL.Variant({
    'SerializeError' : ErrorMessage,
    'DeserializeError' : ErrorMessage,
    'NotFound' : ErrorMessage,
    'ValidationError' : IDL.Vec(ValidationResponse),
    'CanisterAtCapacity' : ErrorMessage,
    'UpdateRequired' : UpdateMessage,
    'Unauthorized' : ErrorMessage,
    'Unexpected' : ErrorMessage,
    'BadRequest' : ErrorMessage,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : ApiError });
  const WasmVersion = IDL.Variant({
    'None' : IDL.Null,
    'Version' : IDL.Nat64,
    'Custom' : IDL.Null,
  });
  const CanisterType = IDL.Variant({
    'Empty' : IDL.Null,
    'Foundation' : IDL.Null,
    'Custom' : IDL.Null,
    'ScalableChild' : IDL.Null,
    'Scalable' : IDL.Null,
  });
  const ScalableCanisterDetails = IDL.Record({
    'entry_range' : IDL.Tuple(IDL.Nat64, IDL.Opt(IDL.Nat64)),
    'principal' : IDL.Principal,
    'wasm_version' : WasmVersion,
    'is_available' : IDL.Bool,
    'canister_type' : CanisterType,
  });
  const Result_1 = IDL.Variant({
    'Ok' : ScalableCanisterDetails,
    'Err' : IDL.Text,
  });
  const DateRange = IDL.Record({
    'end_date' : IDL.Nat64,
    'start_date' : IDL.Nat64,
  });
  const ProfileFilter = IDL.Variant({
    'Interest' : IDL.Nat32,
    'Email' : IDL.Text,
    'Skill' : IDL.Nat32,
    'DisplayName' : IDL.Text,
    'UpdatedOn' : DateRange,
    'City' : IDL.Text,
    'FirstName' : IDL.Text,
    'LastName' : IDL.Text,
    'Cause' : IDL.Nat32,
    'StateOrProvince' : IDL.Text,
    'Country' : IDL.Text,
    'CreatedOn' : DateRange,
    'Username' : IDL.Text,
  });
  const FilterType = IDL.Variant({ 'Or' : IDL.Null, 'And' : IDL.Null });
  const SortDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const ProfileSort = IDL.Variant({
    'Email' : SortDirection,
    'DisplayName' : SortDirection,
    'UpdatedOn' : SortDirection,
    'City' : SortDirection,
    'FirstName' : SortDirection,
    'LastName' : SortDirection,
    'StateOrProvince' : SortDirection,
    'Country' : SortDirection,
    'CreatedOn' : SortDirection,
    'Username' : SortDirection,
  });
  const ChunkData = IDL.Record({
    'chunk_id' : IDL.Nat64,
    'canister' : IDL.Principal,
    'index' : IDL.Nat64,
  });
  const Manifest = IDL.Record({ 'entries' : IDL.Vec(ChunkData) });
  const CanisterStorage = IDL.Variant({
    'None' : IDL.Null,
    'Manifest' : Manifest,
    'Chunk' : ChunkData,
  });
  const Asset = IDL.Variant({
    'Url' : IDL.Text,
    'None' : IDL.Null,
    'CanisterStorage' : CanisterStorage,
  });
  const ProfilePrivacy = IDL.Variant({
    'Private' : IDL.Null,
    'Public' : IDL.Null,
  });
  const WalletResponse = IDL.Record({
    'principal' : IDL.Principal,
    'provider' : IDL.Text,
    'is_primary' : IDL.Bool,
  });
  const CodeOfConductDetails = IDL.Record({
    'approved_date' : IDL.Nat64,
    'approved_version' : IDL.Nat64,
  });
  const ApplicationRole = IDL.Variant({
    'Blocked' : IDL.Null,
    'Guest' : IDL.Null,
    'Member' : IDL.Null,
    'Banned' : IDL.Null,
    'Admin' : IDL.Null,
    'Moderator' : IDL.Null,
    'Leader' : IDL.Null,
    'Owner' : IDL.Null,
    'Watcher' : IDL.Null,
  });
  const ProfileResponse = IDL.Record({
    'updated_on' : IDL.Nat64,
    'profile_image' : Asset,
    'principal' : IDL.Principal,
    'banner_image' : Asset,
    'about' : IDL.Text,
    'country' : IDL.Text,
    'username' : IDL.Text,
    'interests' : IDL.Vec(IDL.Nat32),
    'city' : IDL.Text,
    'created_on' : IDL.Nat64,
    'email' : IDL.Text,
    'website' : IDL.Text,
    'display_name' : IDL.Text,
    'extra' : IDL.Text,
    'privacy' : ProfilePrivacy,
    'wallets' : IDL.Vec(WalletResponse),
    'state_or_province' : IDL.Text,
    'first_name' : IDL.Text,
    'last_name' : IDL.Text,
    'member_identifier' : IDL.Principal,
    'causes' : IDL.Vec(IDL.Nat32),
    'code_of_conduct' : CodeOfConductDetails,
    'date_of_birth' : IDL.Nat64,
    'identifier' : IDL.Principal,
    'skills' : IDL.Vec(IDL.Nat32),
    'application_role' : ApplicationRole,
  });
  const PagedResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(ProfileResponse),
    'page' : IDL.Nat64,
    'limit' : IDL.Nat64,
    'number_of_pages' : IDL.Nat64,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'accept_cycles' : IDL.Func([], [IDL.Nat64], []),
    'close_child_canister_and_spawn_sibling' : IDL.Func(
        [IDL.Principal, IDL.Nat64, IDL.Vec(IDL.Nat8), IDL.Opt(IDL.Principal)],
        [Result],
        [],
      ),
    'get_available_canister' : IDL.Func([], [Result_1], ['query']),
    'get_canisters' : IDL.Func(
        [],
        [IDL.Vec(ScalableCanisterDetails)],
        ['query'],
      ),
    'get_latest_wasm_version' : IDL.Func([], [WasmVersion], ['query']),
    'get_profiles' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Vec(ProfileFilter), FilterType, ProfileSort],
        [PagedResponse],
        ['query'],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Text, IDL.Principal, IDL.Principal];
};
