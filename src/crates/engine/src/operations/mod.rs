#![allow(unused)]

use crate::state::CanisterState;

mod add_message;
mod buy_shipment;
mod create_shipment;
mod finalize_shipment;
mod read_message;

pub use add_message::AddMessageOp;
pub use buy_shipment::BuyShipmentOp;
pub use create_shipment::CreateShipmentOp;
pub use finalize_shipment::FinalizeShipmentOp;
pub use read_message::ReadMessageOp;
pub trait StateOp<R> {
    type Error;

    fn apply(&self, state: &mut CanisterState) -> Result<R, Self::Error> {
        unimplemented!()
    }

    fn read(&self, state: &CanisterState) -> Result<R, Self::Error> {
        unimplemented!()
    }
}
