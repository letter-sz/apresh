use crate::models::shipment::InternalShipment;
use candid::Principal;
use derive_deref::{Deref, DerefMut};

use std::collections::HashMap;
type ShipmentsStore = HashMap<u64, InternalShipment<Principal>>;

#[derive(Default, Deref, DerefMut)]

pub struct Shipments(ShipmentsStore);
