use apresh_store::Record;
use apresh_types::{ActorId, Carrier, ChannelKey, Shipment, ShipmentActions, ShipmentId};

use super::{StateOp, ValidatedStateOp};
use crate::state::CanisterState;
use crate::state::{CanisterActors, CanisterShipments};

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier_id: ActorId,
    shipment_id: ShipmentId,
    channel_key: ChannelKey,
}

impl BuyShipmentOp {
    pub fn new(carrier_id: ActorId, shipment_id: ShipmentId, channel_key: ChannelKey) -> Self {
        Self {
            carrier_id,
            shipment_id,
            channel_key,
        }
    }
}

impl ValidatedStateOp<Cost> for BuyShipmentOp {
    type ValidationResult = u64;

    fn validate(&self, state: &CanisterState) -> Result<u64, Self::Error> {
        if state.carrier(&self.carrier_id).is_none() {
            return Err(crate::EngineError::CarrierNotFound);
        }

        let shipment = state
            .shipment(self.shipment_id)
            .ok_or(crate::EngineError::ShipmentNotFound)?;

        let value = shipment.info().value();
        Ok(value)
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = crate::EngineError;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        if state.carrier(&self.carrier_id).is_none() {
            return Err(crate::EngineError::CarrierNotFound);
        }

        let value = {
            let mut shipment = state
                .shipment(self.shipment_id)
                .ok_or(crate::EngineError::ShipmentNotFound)?;

            shipment.action(ShipmentActions::Buy(self.carrier_id))?;
            shipment.add_guest_to_channel(self.channel_key.clone());
            let value = shipment.info().value();
            Shipment::set(shipment);

            value
        };

        let mut carrier = state.carrier(&self.carrier_id).unwrap();
        carrier.add_shipment(self.shipment_id);
        Carrier::set(carrier);

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        operations::{CancelShipmentOp, RegisterActorOp},
        EngineError,
    };
    use apresh_crypto::hash_secret;
    use candid::Principal;
    use types::{
        Actor, ActorId, Shipment, ShipmentActions, ShipmentError, ShipmentId, ShipmentInfo,
        ShipmentLocation, ShipmentStatus, SizeCategory,
    };

    const REGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[1, 2, 3, 4]));
    const UNREGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[5, 6, 7, 8]));
    const REGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));
    const CHANNEL_KEY: &[u8] = b"channel_key_123";
    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID,
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID,
            name: "Test Shipper".to_string(),
        };
        register_shipper.apply(&mut state);

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

        state.create_shipment(shipment).unwrap();
        state
    }

    #[test]
    fn test_buy_shipment_unregistered_carrier() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(UNREGISTERED_CARRIER_ID, 0, CHANNEL_KEY.to_vec());

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(EngineError::CarrierNotFound)));
    }

    #[test]
    fn test_buy_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 999, CHANNEL_KEY.to_vec());

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(EngineError::ShipmentNotFound)));
    }

    #[test]
    fn test_buy_shipment_already_bought() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 0, CHANNEL_KEY.to_vec());

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentCannotBeBought
            ))
        ));
    }

    #[test]
    fn test_buy_shipment_success_and_return_value() {
        let mut state = setup_test_state();
        let shipment_id = 0;
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id, CHANNEL_KEY.to_vec());

        let initial_carrier = state.carrier(&REGISTERED_CARRIER_ID).unwrap();
        assert!(!initial_carrier
            .get_active_shipments()
            .contains(&shipment_id));

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        let carrier = state.carrier(&REGISTERED_CARRIER_ID).unwrap();
        let shipment = state.shipment(shipment_id).unwrap();

        assert!(carrier.get_active_shipments().contains(&shipment_id));
        assert_eq!(shipment.status(), &ShipmentStatus::InTransit);
        assert_eq!(shipment.carrier_id(), Some(REGISTERED_CARRIER_ID));
    }

    #[test]
    fn test_canceled_shipment_cannot_be_bought() {
        let mut state = setup_test_state();

        let info = ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        );
        let new_shipment = Shipment::new(
            1234567890,
            REGISTERED_SHIPPER_ID,
            1,
            hash_secret(b"test_secret"),
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
        );

        state.create_shipment(new_shipment).unwrap();

        let shipment_id = 1;
        let op = CancelShipmentOp::new(REGISTERED_SHIPPER_ID, shipment_id);
        op.apply(&mut state).unwrap();

        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id, CHANNEL_KEY.to_vec());
        let result = buy_op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentCannotBeBought
            ))
        ));
    }
}
