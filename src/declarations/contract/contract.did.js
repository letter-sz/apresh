export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const ShipmentStatus = IDL.Variant({
    'InTransit' : IDL.Null,
    'DeliveryScheduled' : IDL.Null,
    'DeliveryCompleted' : IDL.Null,
    'PickupScheduled' : IDL.Null,
    'Bought' : IDL.Null,
    'Cancelled' : IDL.Null,
    'PickupCompleted' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const ShipmentLocation = IDL.Record({
    'lat' : IDL.Float64,
    'lng' : IDL.Float64,
    'street' : IDL.Text,
  });
  const SizeCategory = IDL.Variant({
    'Parcel' : IDL.Record({
      'max_height' : IDL.Nat64,
      'max_width' : IDL.Nat64,
      'max_depth' : IDL.Nat64,
    }),
    'Envelope' : IDL.Null,
  });
  const ShipmentInfo = IDL.Record({
    'destination' : ShipmentLocation,
    'value' : IDL.Nat64,
    'source' : ShipmentLocation,
    'size_category' : SizeCategory,
    'price' : IDL.Nat64,
  });
  const Channel = IDL.Record({
    'messages' : IDL.Vec(IDL.Vec(IDL.Nat8)),
    'host_key' : IDL.Vec(IDL.Nat8),
    'guest_keys' : IDL.Vec(IDL.Vec(IDL.Nat8)),
  });
  const PrintableShipment = IDL.Record({
    'id' : IDL.Nat64,
    'shipper' : IDL.Text,
    'status' : ShipmentStatus,
    'info' : ShipmentInfo,
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'hashed_secret' : IDL.Vec(IDL.Nat8),
    'carrier' : IDL.Opt(IDL.Text),
    'channel' : Channel,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({ 'Ok' : Channel, 'Err' : IDL.Text });
  return IDL.Service({
    'addWhitelisted' : IDL.Func([IDL.Principal], [], []),
    'add_message' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Nat64], [Result], []),
    'buyShipment' : IDL.Func(
        [IDL.Opt(IDL.Text), IDL.Nat64, IDL.Vec(IDL.Nat8)],
        [Result],
        [],
      ),
    'cancel_shipment' : IDL.Func([IDL.Nat64], [Result], []),
    'carrier_shipments' : IDL.Func([], [IDL.Vec(PrintableShipment)], ['query']),
    'createShipment' : IDL.Func(
        [
          IDL.Opt(IDL.Text),
          IDL.Text,
          IDL.Vec(IDL.Nat8),
          IDL.Vec(IDL.Nat8),
          ShipmentInfo,
        ],
        [Result_1],
        [],
      ),
    'finalizeShipment' : IDL.Func([IDL.Nat64, IDL.Opt(IDL.Text)], [Result], []),
    'generateQr' : IDL.Func([IDL.Text, IDL.Nat64], [Result_2], ['query']),
    'getTransferFee' : IDL.Func([], [IDL.Nat64], ['query']),
    'is_mainnet' : IDL.Func([], [IDL.Bool], ['query']),
    'listPendingShipments' : IDL.Func(
        [],
        [IDL.Vec(PrintableShipment)],
        ['query'],
      ),
    'lockCanister' : IDL.Func([], [], []),
    'migrateCarriers' : IDL.Func([], [], []),
    'migrateShipments' : IDL.Func([], [], []),
    'migrateShippers' : IDL.Func([], [], []),
    'read_channel' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'roles' : IDL.Func([], [IDL.Bool, IDL.Bool], ['query']),
    'setTransferFee' : IDL.Func([IDL.Nat64], [], []),
    'shipment' : IDL.Func([IDL.Nat64], [IDL.Opt(PrintableShipment)], ['query']),
    'shipments' : IDL.Func([], [IDL.Vec(PrintableShipment)], ['query']),
    'shipper_shipments' : IDL.Func([], [IDL.Vec(PrintableShipment)], ['query']),
    'unlockCanister' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
