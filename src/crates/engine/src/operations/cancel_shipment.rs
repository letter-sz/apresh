use crate::{
    models::shipment::{ShipmentActions, ShipmentId},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct CancelShipmentOp {
    shipper: ActorId,
    shipment_id: ShipmentId,
}

impl CancelShipmentOp {
    pub fn new(shipper: ActorId, shipment_id: ShipmentId) -> Self {
        Self {
            shipper,
            shipment_id,
        }
    }
}

impl StateOp<Cost> for CancelShipmentOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        let shipper = state
            .shipper_mut(&self.shipper)
            .ok_or(crate::Error::ShipperNotFound)?;

        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        shipment.action(ShipmentActions::Cancel {
            shipper: self.shipper,
        })?;

        Ok(shipment.info().value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actors::Actor,
        models::shipment::{
            Shipment, ShipmentInfo, ShipmentLocation, ShipmentStatus, SizeCategory,
        },
        operations::{BuyShipmentOp, RegisterActorOp},
        utils::hash_secret,
        ActorId, Error,
    };
    use candid::Principal;

    const REGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[1, 2, 3, 4]));
    const UNREGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[5, 6, 7, 8]));
    const REGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));

    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID.into(),
            name: "Test Shipper".to_string(),
        };
        register_shipper.apply(&mut state);

        let register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID.into(),
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let info = ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        );

        let shipment = Shipment::new(
            1234567890,
            REGISTERED_SHIPPER_ID,
            0,
            &hash_secret(b"test_secret"),
            "Test Shipment",
            &info,
        );

        state.create_shipment(shipment).unwrap();
        state
    }

    #[test]
    fn test_cancel_shipment_unregistered_shipper() {
        let mut state = setup_test_state();
        let op = CancelShipmentOp::new(UNREGISTERED_SHIPPER_ID, 0);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::ShipperNotFound)));
    }

    #[test]
    fn test_cancel_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, 999);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::ShipmentNotFound)));
    }

    #[test]
    fn test_cancel_shipment_wrong_shipper() {
        let mut state = setup_test_state();

        let other_shipper_id = ActorId(Principal::from_slice(&[13, 14, 15, 16]));
        let register_other_shipper = RegisterActorOp::AddShipper {
            id: other_shipper_id.into(),
            name: "Other Shipper".to_string(),
        };
        register_other_shipper.apply(&mut state);

        let op = CancelShipmentOp::new(other_shipper_id, 0);
        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::NotAuthorizedAsShipper)));
    }

    #[test]
    fn test_cancel_shipment_already_bought() {
        let mut state = setup_test_state();

        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 0);
        buy_op.apply(&mut state).unwrap();

        let cancel_op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, 0);
        let result = cancel_op.apply(&mut state);
        assert!(matches!(result, Err(Error::ShipmentNotReadyToBeCanceled)));
    }

    #[test]
    fn test_cancel_shipment_success() {
        let mut state = setup_test_state();
        let shipment_id = 0;
        let op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, shipment_id);

        let initial_shipment = state.shipment(shipment_id).unwrap();
        assert_eq!(initial_shipment.status(), &ShipmentStatus::Pending);

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        let shipment = state.shipment(shipment_id).unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::Cancelled);
    }

    #[test]
    fn test_carrier_cancel_shipment() {
        let mut state = setup_test_state();
        let shipment_id = 0;

        // register carrier as shipper to prevent first error
        let register_carrier = RegisterActorOp::AddShipper {
            id: REGISTERED_CARRIER_ID.into(),
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state); 

        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id);
        buy_op.apply(&mut state);

        let op = CancelShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id);
        let result = op.apply(&mut state);

        assert!(matches!(result, Err(Error::NotAuthorizedAsShipper)));
    }
}
