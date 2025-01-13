mod actor_collection;
mod shipments;
use actor_collection::ActorCollection;
use shipments::Shipments;
use std::cell::RefCell;
use crate::actors::{carrier::Carrier, shipper::Shipper};

#[macro_use]
mod macros;

#[derive(Default)]
pub struct CanisterState {
    pub shippers: ActorCollection<Shipper>,
    pub carriers: ActorCollection<Carrier>,
    pub shipments: Shipments,
    pub shipment_counter: u64,
}

thread_local! {
    pub static STATE: RefCell<CanisterState> = Default::default();
}
