#![allow(unused)]

use crate::state::CanisterState;

mod add_message;
mod buy_shipment;
mod cancel_shipment;
mod create_shipment;
mod finalize_shipment;
mod read_message;
mod register_actor;

pub use add_message::AddMessageOp;
pub use buy_shipment::BuyShipmentOp;
pub use cancel_shipment::CancelShipmentOp;
pub use create_shipment::CreateShipmentOp;
pub use finalize_shipment::FinalizeShipmentOp;
pub use read_message::ReadMessageOp;
pub use register_actor::RegisterActorOp;

pub trait StateOp<R> {
    type Error;

    fn apply(&self, state: &mut CanisterState) -> Result<R, Self::Error> {
        unimplemented!()
    }

    fn read(&self, state: &CanisterState) -> Result<R, Self::Error> {
        unimplemented!()
    }
}

/// A trait for operations that need validation before being applied
pub trait ValidatedStateOp<R>: StateOp<R> {
    type ValidationResult;

    /// Validates the operation without modifying state
    fn validate(&self, state: &CanisterState) -> Result<Self::ValidationResult, Self::Error>;

    /// Validates and then applies the operation
    fn validate_and_apply(&self, state: &mut CanisterState) -> Result<R, Self::Error> {
        self.validate(state)?;
        self.apply(state)
    }
}
