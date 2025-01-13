use crate::{impl_deref_deref_mut, models::shipment::Shipment};
use std::collections::HashMap;
type ShipmentsStore = HashMap<u64, Shipment>;

#[derive(Default)]
pub struct Shipments(ShipmentsStore);

impl_deref_deref_mut!(Shipments, ShipmentsStore);
