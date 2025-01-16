mod actor_collection;
mod shipments;

use crate::actors::{carrier::Carrier, shipper::Shipper};
use actor_collection::ActorCollection;
use shipments::Shipments;
use std::cell::RefCell;

#[derive(Default)]
pub struct CanisterState {
    pub shippers: ActorCollection<Shipper>,
    pub carriers: ActorCollection<Carrier>,
    pub shipments: Shipments,
    pub shipment_counter: u64,
}
