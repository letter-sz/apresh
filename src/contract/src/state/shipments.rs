use candid::Principal;
use derive_deref::{Deref, DerefMut};
use engine::models::shipment::InternalShipment;

use std::collections::HashMap;
type ShipmentsStore = HashMap<u64, InternalShipment<Principal>>;

#[derive(Default, Deref, DerefMut)]

pub struct Shipments(ShipmentsStore);

#[derive(Default, Deref, DerefMut)]

pub struct A(i32);
