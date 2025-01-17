use crate::{actors::carrier::Carrier, state::CanisterState};

use super::StateOp;

pub enum RegisterActorOp {
    AddShipper { name: String },
    AddCarrier { carrier: Carrier },
}

impl StateOp<()> for RegisterActorOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
