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
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : ApiError });
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
  const DateRange = IDL.Record({
    'end_date' : IDL.Nat64,
    'start_date' : IDL.Nat64,
  });
  const NeuronGatedRules = IDL.Variant({
    'IsDisolving' : IDL.Bool,
    'MinStake' : IDL.Nat64,
    'MinAge' : IDL.Nat64,
    'MinDissolveDelay' : IDL.Nat64,
  });
  const NeuronGated = IDL.Record({
    'governance_canister' : IDL.Principal,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'ledger_canister' : IDL.Principal,
    'rules' : IDL.Vec(NeuronGatedRules),
  });
  const TokenGated = IDL.Record({
    'principal' : IDL.Principal,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'amount' : IDL.Nat64,
    'standard' : IDL.Text,
  });
  const GatedType = IDL.Variant({
    'Neuron' : IDL.Vec(NeuronGated),
    'Token' : IDL.Vec(TokenGated),
  });
  const Privacy = IDL.Variant({
    'Gated' : GatedType,
    'Private' : IDL.Null,
    'Public' : IDL.Null,
    'InviteOnly' : IDL.Null,
  });
  const Address = IDL.Record({
    'street' : IDL.Text,
    'country' : IDL.Text,
    'city' : IDL.Text,
    'postal_code' : IDL.Text,
    'label' : IDL.Text,
    'state_or_province' : IDL.Text,
    'house_number' : IDL.Text,
    'house_number_addition' : IDL.Text,
  });
  const PhysicalLocation = IDL.Record({
    'longtitude' : IDL.Float32,
    'address' : Address,
    'lattitude' : IDL.Float32,
  });
  const Location = IDL.Variant({
    'None' : IDL.Null,
    'Digital' : IDL.Text,
    'Physical' : PhysicalLocation,
  });
  const PostEvent = IDL.Record({
    'banner_image' : Asset,
    'date' : DateRange,
    'name' : IDL.Text,
    'tags' : IDL.Vec(IDL.Nat32),
    'description' : IDL.Text,
    'website' : IDL.Text,
    'privacy' : Privacy,
    'image' : Asset,
    'location' : Location,
  });
  const EventResponse = IDL.Record({
    'updated_on' : IDL.Nat64,
    'banner_image' : Asset,
    'owner' : IDL.Principal,
    'date' : DateRange,
    'attendee_count' : IDL.Nat64,
    'name' : IDL.Text,
    'tags' : IDL.Vec(IDL.Nat32),
    'description' : IDL.Text,
    'created_by' : IDL.Principal,
    'created_on' : IDL.Nat64,
    'website' : IDL.Text,
    'privacy' : Privacy,
    'is_canceled' : IDL.Tuple(IDL.Bool, IDL.Text),
    'image' : Asset,
    'identifier' : IDL.Principal,
    'location' : Location,
    'is_deleted' : IDL.Bool,
  });
  const Result_1 = IDL.Variant({ 'Ok' : EventResponse, 'Err' : ApiError });
  const EventFilter = IDL.Variant({
    'Tag' : IDL.Nat32,
    'UpdatedOn' : DateRange,
    'Name' : IDL.Text,
    'Identifiers' : IDL.Vec(IDL.Principal),
    'IsCanceled' : IDL.Bool,
    'StartDate' : DateRange,
    'Owner' : IDL.Principal,
    'CreatedOn' : DateRange,
    'EndDate' : DateRange,
  });
  const FilterType = IDL.Variant({ 'Or' : IDL.Null, 'And' : IDL.Null });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Tuple(IDL.Principal, Privacy),
    'Err' : ApiError,
  });
  const SortDirection = IDL.Variant({ 'Asc' : IDL.Null, 'Desc' : IDL.Null });
  const EventSort = IDL.Variant({
    'UpdatedOn' : SortDirection,
    'AttendeeCount' : SortDirection,
    'StartDate' : SortDirection,
    'CreatedOn' : SortDirection,
    'EndDate' : SortDirection,
  });
  const PagedResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(EventResponse),
    'page' : IDL.Nat64,
    'limit' : IDL.Nat64,
    'number_of_pages' : IDL.Nat64,
  });
  const Result_3 = IDL.Variant({ 'Ok' : PagedResponse, 'Err' : ApiError });
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
  const Event = IDL.Record({
    'updated_on' : IDL.Nat64,
    'banner_image' : Asset,
    'group_identifier' : IDL.Principal,
    'owner' : IDL.Principal,
    'date' : DateRange,
    'attendee_count' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64)),
    'name' : IDL.Text,
    'tags' : IDL.Vec(IDL.Nat32),
    'description' : IDL.Text,
    'created_by' : IDL.Principal,
    'created_on' : IDL.Nat64,
    'website' : IDL.Text,
    'privacy' : Privacy,
    'is_canceled' : IDL.Tuple(IDL.Bool, IDL.Text),
    'image' : Asset,
    'location' : Location,
    'is_deleted' : IDL.Bool,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Bool });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'accept_cycles' : IDL.Func([], [IDL.Nat64], []),
    'add_entry_by_parent' : IDL.Func([IDL.Vec(IDL.Nat8)], [Result], []),
    'add_event' : IDL.Func(
        [PostEvent, IDL.Principal, IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'cancel_event' : IDL.Func(
        [IDL.Principal, IDL.Text, IDL.Principal, IDL.Principal],
        [Result],
        [],
      ),
    'delete_event' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Principal],
        [Result],
        [],
      ),
    'edit_event' : IDL.Func(
        [IDL.Principal, PostEvent, IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'get_chunked_data' : IDL.Func(
        [IDL.Vec(EventFilter), FilterType, IDL.Nat64, IDL.Nat64],
        [IDL.Vec(IDL.Nat8), IDL.Tuple(IDL.Nat64, IDL.Nat64)],
        ['query'],
      ),
    'get_event' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_1],
        ['query'],
      ),
    'get_event_privacy_and_owner' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_2],
        ['query'],
      ),
    'get_events' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Nat64,
          EventSort,
          IDL.Vec(EventFilter),
          FilterType,
          IDL.Principal,
        ],
        [Result_3],
        ['query'],
      ),
    'get_events_count' : IDL.Func(
        [IDL.Vec(IDL.Principal)],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat64))],
        ['query'],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'migration_add_events' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Principal, Event))],
        [],
        [],
      ),
    'update_attendee_count_on_event' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat64],
        [Result_4],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Principal, IDL.Text, IDL.Nat64];
};
