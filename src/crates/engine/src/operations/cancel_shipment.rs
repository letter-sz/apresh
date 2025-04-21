use anyhow::anyhow;
use apresh_store::Record;
use apresh_types::{
    ActorId, Shipment, ShipmentActions, ShipmentId, ShipmentKey, Shipper, ShipperKey,
};

use super::StateOp;
use crate::state::CanisterState;

pub type Cost = u64;

pub struct CancelShipmentOp<'a> {
    shipper: &'a Shipper,
    shipment: &'a mut Shipment,
}

impl<'a> CancelShipmentOp<'a> {
    pub fn new(shipper: &'a Shipper, shipment: &'a mut Shipment) -> Self {
        Self { shipper, shipment }
    }
}

impl StateOp<Cost> for CancelShipmentOp<'_> {
    type Error = crate::EngineError;

    fn apply(self, state: &mut CanisterState) -> crate::Result<Cost> {
        if self.shipment.shipper_id() != self.shipper.id() {
            return Err(crate::EngineError::NotAuthorizedAsShipper);
        }

        self.shipment.action(ShipmentActions::Cancel {
            shipper: self.shipper.id(),
        })?;

        Ok(self.shipment.info().value())
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
        Carrier, CarrierKey, ShipmentError, ShipmentInfo, ShipmentLocation, ShipmentStatus,
        SizeCategory,
    };
    use candid::Principal;

    const REGISTERED_SHIPPER_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[1, 2, 3, 4])));
    const UNREGISTERED_SHIPPER_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[5, 6, 7, 8])));
    const REGISTERED_CARRIER_ID: CarrierKey =
        CarrierKey(ActorId(Principal::from_slice(&[9, 10, 11, 12])));
    const CHANNEL_KEY: &[u8] = b"channel_key_123";

    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID.0,
            name: "Test Shipper".to_string(),
        };
        register_shipper.apply(&mut state);

        let register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID.0,
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
            REGISTERED_SHIPPER_ID.0,
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
        let mut shipper = UNREGISTERED_SHIPPER_ID
            .get()
            .ok_or(EngineError::ShipperNotFound);
        assert!(matches!(shipper, Err(EngineError::ShipperNotFound)));
    }

    #[test]
    fn test_cancel_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let mut shipper = REGISTERED_SHIPPER_ID.get().unwrap();
        let mut shipment = ShipmentKey(999).get().ok_or(EngineError::ShipmentNotFound);
        assert!(matches!(shipment, Err(EngineError::ShipmentNotFound)));
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

        let mut shipper = ShipperKey(other_shipper_id).get().unwrap();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = CancelShipmentOp::new(&shipper, &mut shipment);
        let result = op.apply(&mut state);
        assert_eq!(result, Err(EngineError::NotAuthorizedAsShipper));
    }

    #[test]
    fn test_cancel_shipment_already_bought() {
        let mut state = setup_test_state();

        let mut shipment = ShipmentKey(0).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let buy_op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state).unwrap();

        shipment.consume();

        let mut shipper = REGISTERED_SHIPPER_ID.get().unwrap();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = CancelShipmentOp::new(&shipper, &mut shipment);
        let result = op.apply(&mut state);
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
        let mut shipper = REGISTERED_SHIPPER_ID.get().unwrap();
        let mut shipment = ShipmentKey(shipment_id).get().unwrap();
        let op = CancelShipmentOp::new(&shipper, &mut shipment);

        let initial_shipment = ShipmentKey(shipment_id).get().unwrap();
        assert_eq!(initial_shipment.status(), &ShipmentStatus::Pending);

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        shipment.consume();

        let shipment = ShipmentKey(shipment_id).get().unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::Cancelled);
    }

    #[test]
    fn test_carrier_cannot_cancel_shipment() {
        let mut state = setup_test_state();
        let shipment_id = 0;

        // register carrier as shipper to prevent first error
        let register_carrier = RegisterActorOp::AddShipper {
            id: REGISTERED_CARRIER_ID.0,
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let mut shipment = ShipmentKey(shipment_id).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let buy_op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state).unwrap();

        shipment.consume();

        let mut shipper = REGISTERED_SHIPPER_ID.get().unwrap();
        let mut shipment = ShipmentKey(shipment_id).get().unwrap();
        let op = CancelShipmentOp::new(&shipper, &mut shipment);
        let result = op.apply(&mut state);

        assert_eq!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentNotReadyToBeCanceled
            ))
        );
    }
}
