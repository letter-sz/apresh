use apresh_store::Record;
use apresh_types::{ActorId, Carrier, Shipper};

use crate::state::CanisterState;

use super::StateOp;

pub enum RegisterActorOp {
    AddShipper { id: ActorId, name: String },
    AddCarrier { id: ActorId, name: String },
}

impl StateOp<()> for RegisterActorOp {
    type Error = crate::EngineError;

    fn apply(self, state: &mut CanisterState) -> Result<(), Self::Error> {
        match self {
            RegisterActorOp::AddShipper { id, name } => {
                Shipper::new(id, name.as_str()).set();
            }
            RegisterActorOp::AddCarrier { id, name } => {
                Carrier::new(id, name.as_str()).set();
            }
        }

        Ok(())
    }
}
