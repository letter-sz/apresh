use apresh_types::{ActorId, Carrier, Shipper};

use crate::state::{CanisterActors, CanisterState};

use super::StateOp;

pub enum RegisterActorOp {
    AddShipper { id: ActorId, name: String },
    AddCarrier { id: ActorId, name: String },
}

impl StateOp<()> for RegisterActorOp {
    type Error = crate::EngineError;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        match self {
            RegisterActorOp::AddShipper { id, name } => {
                state.create_shipper(Shipper::new(*id, name));
            }
            RegisterActorOp::AddCarrier { id, name } => {
                state.create_carrier(Carrier::new(*id, name));
            }
        }

        Ok(())
    }
}
