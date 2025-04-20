use apresh_store::Record;
use apresh_types::{
    ActorId, Carrier, CarrierKey, ChannelKey, Shipment, ShipmentActions, ShipmentId, ShipmentKey,
};

use super::{StateOp, ValidatedStateOp};
use crate::state::CanisterState;

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier_id: CarrierKey,
    shipment_id: ShipmentKey,
    channel_key: ChannelKey,
}

impl BuyShipmentOp {
    pub fn new(carrier_id: ActorId, shipment_id: ShipmentId, channel_key: ChannelKey) -> Self {
        Self {
            carrier_id: CarrierKey(carrier_id),
            shipment_id: ShipmentKey(shipment_id),
            channel_key,
        }
    }
}

impl ValidatedStateOp<Cost> for BuyShipmentOp {
    type ValidationResult = u64;

    fn validate(&self, state: &CanisterState) -> Result<u64, Self::Error> {
        if (self.carrier_id).get().is_none() {
            return Err(crate::EngineError::CarrierNotFound);
        }

        let shipment = (self.shipment_id)
            .get()
            .ok_or(crate::EngineError::ShipmentNotFound)?;

        let value = shipment.info().value();
        Ok(value)
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = crate::EngineError;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        if (self.carrier_id).get().is_none() {
            return Err(crate::EngineError::CarrierNotFound);
        }

        let value = {
            let mut shipment = (self.shipment_id)
                .get()
                .ok_or(crate::EngineError::ShipmentNotFound)?;

            shipment.action(ShipmentActions::Buy(self.carrier_id.0))?;
            shipment.add_guest_to_channel(self.channel_key.clone());
            let value = shipment.info().value();
            Shipment::set(shipment);

            value
        };

        let mut carrier = (self.carrier_id).get().unwrap();
        carrier.add_shipment(self.shipment_id.0);
        carrier.set();

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
    use apresh_types::{
        Actor, ActorId, Shipment, ShipmentActions, ShipmentError, ShipmentId, ShipmentInfo,
        ShipmentLocation, ShipmentStatus, SizeCategory,
    };
    use candid::Principal;

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

        shipment.set();
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

        let initial_carrier = CarrierKey(REGISTERED_CARRIER_ID).get().unwrap();
        assert!(!initial_carrier
            .get_active_shipments()
            .contains(&shipment_id));

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        let carrier = CarrierKey(REGISTERED_CARRIER_ID).get().unwrap();
        let shipment = ShipmentKey(shipment_id).get().unwrap();

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

        new_shipment.set();

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
