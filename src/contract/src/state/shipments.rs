use crate::{impl_deref_deref_mut, models::shipment::InternalShipment};
use std::collections::HashMap;
type ShipmentsStore = HashMap<u64, InternalShipment>;

#[derive(Default)]
pub struct Shipments(ShipmentsStore);

impl_deref_deref_mut!(Shipments, ShipmentsStore);
