use apresh_store::Record;
use apresh_types::{
    ActorId, Carrier, ChannelKey, Shipment, ShipmentActions, ShipmentId, ShipmentKey,
};

use super::StateOp;
use crate::state::CanisterState;

pub type Cost = u64;

pub struct BuyShipmentOp<'a> {
    carrier: &'a mut Carrier,
    shipment: &'a mut Shipment,
    channel_key: ChannelKey,
}

impl<'a> BuyShipmentOp<'a> {
    pub fn new(
        carrier: &'a mut Carrier,
        shipment: &'a mut Shipment,
        channel_key: ChannelKey,
    ) -> Self {
        Self {
            carrier,
            shipment,
            channel_key,
        }
    }
}

impl StateOp<Cost> for BuyShipmentOp<'_> {
    type Error = crate::EngineError;

    fn apply(self, state: &mut CanisterState) -> crate::Result<Cost> {
        self.shipment
            .action(ShipmentActions::Buy(self.carrier.id()))?;
        self.shipment.add_guest_to_channel(self.channel_key.clone());
        self.carrier.add_shipment(*self.shipment.id());
        Ok(self.shipment.info().value())
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
        Actor, ActorId, CarrierKey, Shipment, ShipmentActions, ShipmentError, ShipmentId,
        ShipmentInfo, ShipmentLocation, ShipmentStatus, ShipperKey, SizeCategory,
    };
    use candid::Principal;

    const REGISTERED_CARRIER_ID: CarrierKey =
        CarrierKey(ActorId(Principal::from_slice(&[1, 2, 3, 4])));
    const UNREGISTERED_CARRIER_ID: CarrierKey =
        CarrierKey(ActorId(Principal::from_slice(&[5, 6, 7, 8])));
    const REGISTERED_SHIPPER_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[9, 10, 11, 12])));
    const CHANNEL_KEY: &[u8] = b"channel_key_123";

    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let mut register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID.0,
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let mut register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID.0,
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
    fn test_buy_shipment_unregistered_carrier() {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let mut carrier = UNREGISTERED_CARRIER_ID
            .get()
            .ok_or(EngineError::CarrierNotFound);
        assert_eq!(carrier, Err(EngineError::CarrierNotFound));
        shipment.revert();
    }

    #[test]
    fn test_buy_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(999).get().ok_or(EngineError::ShipmentNotFound);
        assert_eq!(shipment, Err(EngineError::ShipmentNotFound));
    }

    #[test]
    fn test_buy_shipment_already_bought() {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        let op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());
        let result = op.apply(&mut state);
        assert_eq!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentCannotBeBought
            ))
        );

        carrier.revert();
        shipment.revert();
    }

    #[test]
    fn test_buy_shipment_success_and_return_value() {
        let mut state = setup_test_state();
        let shipment_id = 0;
        let mut shipment = ShipmentKey(shipment_id).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());

        let initial_carrier = REGISTERED_CARRIER_ID.get().unwrap();
        assert!(!initial_carrier
            .get_active_shipments()
            .contains(&shipment_id));

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);

        shipment.commit();
        carrier.commit();

        let shipment = ShipmentKey(shipment_id).get().unwrap();
        let carrier = REGISTERED_CARRIER_ID.get().unwrap();
        assert!(carrier.get_active_shipments().contains(&shipment_id));
        assert_eq!(shipment.status(), &ShipmentStatus::InTransit);
        assert_eq!(*shipment.carrier_id(), Some(REGISTERED_CARRIER_ID.0));
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
            REGISTERED_SHIPPER_ID.0,
            1,
            hash_secret(b"test_secret"),
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
        );

        new_shipment.set();

        let shipment_id = ShipmentKey(1);
        let mut shipper = REGISTERED_SHIPPER_ID.get().unwrap();
        let mut shipment = shipment_id.get().unwrap();
        let op = CancelShipmentOp::new(&shipper, &mut shipment);
        op.apply(&mut state).unwrap();
        shipment.commit();
        shipper.commit();

        let mut shipment = ShipmentKey(1).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let buy_op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());
        let result = buy_op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentCannotBeBought
            ))
        ));

        carrier.revert();
        shipment.revert();
    }
}
