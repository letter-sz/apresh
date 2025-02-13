use crate::{
    models::shipment::{Shipment, ShipmentId, ShipmentInfo},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::{CanisterState, StateOp};
use crate::actors::Actor;

#[derive(Debug)]
pub struct CreateShipmentOp<'a> {
    creator: ActorId,
    hashed_secret: Vec<u8>,
    shipment_name: &'a str,
    info: ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator: ActorId,
        hashed_secret: Vec<u8>,
        shipment_name: &'a str,
        info: ShipmentInfo,
        timestamp: u64,
    ) -> Self {
        Self {
            creator,
            hashed_secret,
            shipment_name,
            info,
            timestamp,
        }
    }
}

impl<'a> StateOp<ShipmentId> for CreateShipmentOp<'a> {
    type Error = crate::errors::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<ShipmentId> {
        let new_shipment_id = state.shipment_counter();

        let mut shipper = state
            .shipper_mut(self.creator)
            .ok_or(crate::Error::ShipperNotFound)?;

        let shipment = Shipment::new(
            self.timestamp,
            shipper.id(),
            new_shipment_id,
            self.hashed_secret.clone(),
            self.shipment_name,
            self.info.clone(),
        );

        shipper.add_shipment(new_shipment_id);
        state.create_shipment(shipment);

        Ok(new_shipment_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::shipment::{ShipmentLocation, SizeCategory},
        operations::RegisterActorOp,
        utils::hash_secret,
        ActorId,
    };

    use super::*;
    use candid::Principal;

    #[test]
    fn test_create_shipment_success() {
        let mut state = CanisterState::default();

        let creator_id = Principal::anonymous();
        let creator_name = "John Doe";
        let hashed_secret = hash_secret(b"secret_key_123");
        let shipment_name = "Test Shipment";
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let info = ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        );

        let register_op = RegisterActorOp::AddShipper {
            id: creator_id.into(),
            name: creator_name.to_string(),
        };

        register_op.apply(&mut state);

        let op = CreateShipmentOp::new(
            creator_id.into(),
            hashed_secret,
            shipment_name,
            info.clone(),
            timestamp,
        );

        let result = op.apply(&mut state);

        assert!(result.is_ok());
        let shipment_id = result.unwrap();
        assert_eq!(shipment_id, 0);
        assert_eq!(state.shipment_counter(), 1);

        let shipment = state.shipments.get(&shipment_id).unwrap();
        assert_eq!(shipment.shipper_id(), ActorId(creator_id));
        assert_eq!(shipment._id(), shipment_id);
        assert_eq!(shipment._name(), shipment_name);

        let shipper = state.shippers.get(&creator_id).unwrap();
        assert_eq!(shipper.id(), ActorId(creator_id));
        assert!(shipper.get_active_shipments().contains(&shipment_id));
        assert_eq!(shipper.name(), creator_name);
    }
}
