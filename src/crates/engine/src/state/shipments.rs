use derive_deref::{Deref, DerefMut};

use crate::models::shipment::Shipment;
use std::collections::HashMap;

type ShipmentsStore = HashMap<u64, Shipment>;

#[derive(Default, Deref, DerefMut)]

pub struct Shipments(ShipmentsStore);
