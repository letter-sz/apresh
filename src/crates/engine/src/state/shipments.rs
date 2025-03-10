use candid::CandidType;
use derive_deref::{Deref, DerefMut};
use serde::Deserialize;

use crate::models::shipment::Shipment;
use std::collections::HashMap;

type ShipmentsStore = HashMap<u64, Shipment>;

#[cfg(feature = "icp")]
#[derive(CandidType, Default, Deref, DerefMut, Deserialize)]
pub struct Shipments(ShipmentsStore);

#[cfg(feature = "icp")]
impl Shipments {
    pub fn insert_multiple(&mut self, shipments: Vec<Shipment>) {
        for shipment in shipments {
            self.insert(shipment.id(), shipment);
        }
    }
}
