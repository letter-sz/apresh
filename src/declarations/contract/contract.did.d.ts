import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Channel {
  'messages' : Array<Uint8Array | number[]>,
  'host_key' : Uint8Array | number[],
  'guest_keys' : Array<Uint8Array | number[]>,
}
export interface PrintableShipment {
  'id' : bigint,
  'shipper' : string,
  'status' : ShipmentStatus,
  'info' : ShipmentInfo,
  'name' : string,
  'created_at' : bigint,
  'hashed_secret' : Uint8Array | number[],
  'carrier' : [] | [string],
  'channel' : Channel,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Uint8Array | number[] } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : Channel } |
  { 'Err' : string };
export interface ShipmentInfo {
  'destination' : ShipmentLocation,
  'value' : bigint,
  'source' : ShipmentLocation,
  'size_category' : SizeCategory,
  'price' : bigint,
}
export interface ShipmentLocation {
  'lat' : number,
  'lng' : number,
  'street' : string,
}
export type ShipmentStatus = { 'InTransit' : null } |
  { 'DeliveryScheduled' : null } |
  { 'DeliveryCompleted' : null } |
  { 'PickupScheduled' : null } |
  { 'Bought' : null } |
  { 'Cancelled' : null } |
  { 'PickupCompleted' : null } |
  { 'Pending' : null };
export type SizeCategory = {
    'Parcel' : {
      'max_height' : bigint,
      'max_width' : bigint,
      'max_depth' : bigint,
    }
  } |
  { 'Envelope' : null };
export interface _SERVICE {
  'addWhitelisted' : ActorMethod<[Principal], undefined>,
  'add_message' : ActorMethod<[Uint8Array | number[], bigint], Result>,
  'buyShipment' : ActorMethod<
    [[] | [string], bigint, Uint8Array | number[]],
    Result
  >,
  'cancel_shipment' : ActorMethod<[bigint], Result>,
  'carrier_shipments' : ActorMethod<[], Array<PrintableShipment>>,
  'createShipment' : ActorMethod<
    [
      [] | [string],
      string,
      Uint8Array | number[],
      Uint8Array | number[],
      ShipmentInfo,
    ],
    Result_1
  >,
  'finalizeShipment' : ActorMethod<[bigint, [] | [string]], Result>,
  'generateQr' : ActorMethod<[string, bigint], Result_2>,
  'getTransferFee' : ActorMethod<[], bigint>,
  'is_mainnet' : ActorMethod<[], boolean>,
  'listPendingShipments' : ActorMethod<[], Array<PrintableShipment>>,
  'lockCanister' : ActorMethod<[], undefined>,
  'migrateCarriers' : ActorMethod<[], undefined>,
  'migrateShipments' : ActorMethod<[], undefined>,
  'migrateShippers' : ActorMethod<[], undefined>,
  'read_channel' : ActorMethod<[bigint], Result_3>,
  'roles' : ActorMethod<[], [boolean, boolean]>,
  'setTransferFee' : ActorMethod<[bigint], undefined>,
  'shipment' : ActorMethod<[bigint], [] | [PrintableShipment]>,
  'shipments' : ActorMethod<[], Array<PrintableShipment>>,
  'shipper_shipments' : ActorMethod<[], Array<PrintableShipment>>,
  'unlockCanister' : ActorMethod<[], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
