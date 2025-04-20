use anyhow::anyhow;
use apresh_store::Record;
use apresh_types::{ActorId, Shipment, ShipmentActions, ShipmentId, ShipmentKey, ShipperKey};

use super::StateOp;
use crate::state::CanisterState;

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
    type Error = crate::EngineError;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        let shipper = ShipperKey(self.shipper)
            .get()
            .ok_or(crate::EngineError::ShipperNotFound)?;

        let mut shipment = ShipmentKey(self.shipment_id)
            .get()
            .ok_or(crate::EngineError::ShipmentNotFound)?;

        shipment.action(ShipmentActions::Cancel {
            shipper: self.shipper,
        })?;

        let value = shipment.info().value();
        Shipment::set(shipment);

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operations::{BuyShipmentOp, RegisterActorOp},
        EngineError,
    };
    use apresh_crypto::hash_secret;
    use apresh_types::{
        ShipmentError, ShipmentInfo, ShipmentLocation, ShipmentStatus, SizeCategory,
    };
    use candid::Principal;

    const REGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[1, 2, 3, 4]));
    const UNREGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[5, 6, 7, 8]));
    const REGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));
    const CHANNEL_KEY: &[u8] = b"channel_key_123";
    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID,
            name: "Test Shipper".to_string(),
        };
        register_shipper.apply(&mut state);

        let register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID,
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
            hash_secret(b"test_secret"),
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
        );

        shipment.set();
        state
    }

    #[test]
    fn test_cancel_shipment_unregistered_shipper() {
        let mut state = setup_test_state();
        let op = CancelShipmentOp::new(UNREGISTERED_SHIPPER_ID, 0);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(EngineError::ShipperNotFound)));
    }

    #[test]
    fn test_cancel_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, 999);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(EngineError::ShipmentNotFound)));
    }

    #[test]
    fn test_cancel_shipment_wrong_shipper() {
        let mut state = setup_test_state();

        let other_shipper_id = ActorId(Principal::from_slice(&[13, 14, 15, 16]));
        let register_other_shipper = RegisterActorOp::AddShipper {
            id: other_shipper_id,
            name: "Other Shipper".to_string(),
        };
        register_other_shipper.apply(&mut state);

        let op = CancelShipmentOp::new(other_shipper_id, 0);
        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::NotAuthorizedAsShipper
            ))
        ));
    }

    #[test]
    fn test_cancel_shipment_already_bought() {
        let mut state = setup_test_state();

        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 0, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state).unwrap();

        let cancel_op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, 0);
        let result = cancel_op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentNotReadyToBeCanceled
            ))
        ));
    }

    #[test]
    fn test_cancel_shipment_success() {
        let mut state = setup_test_state();
        let shipment_id = 0;
        let op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, shipment_id);

        let initial_shipment = ShipmentKey(shipment_id).get().unwrap();
        assert_eq!(initial_shipment.status(), &ShipmentStatus::Pending);

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        let shipment = ShipmentKey(shipment_id).get().unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::Cancelled);
    }

    #[test]
    fn test_carrier_cannot_cancel_shipment() {
        let mut state = setup_test_state();
        let shipment_id = 0;

        // register carrier as shipper to prevent first error
        let register_carrier = RegisterActorOp::AddShipper {
            id: REGISTERED_CARRIER_ID,
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state);

        let op = CancelShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id);
        let result = op.apply(&mut state);

        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::NotAuthorizedAsShipper
            ))
        ));
    }
}
