use crate::{
    models::shipment::{ShipmentActions, ShipmentId}, state::{CanisterActors, CanisterShipments}, ActorId
};

use super::StateOp;
use crate::{actors::carrier::Carrier, state::CanisterState};
use anyhow::anyhow;

pub type Cost = u64;

pub struct BuyShipmentOp {
    carrier_id: ActorId,
    shipment_id: ShipmentId,
}

impl BuyShipmentOp {
    pub fn new(carrier_id: ActorId, shipment_id: ShipmentId) -> Self {
        Self {
            carrier_id,
            shipment_id,
        }
    }
}

impl StateOp<Cost> for BuyShipmentOp {
    type Error = crate::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<Cost> {
        if state.carrier(&self.carrier_id).is_none() {
            return Err(crate::Error::CarrierNotFound);
        }
        
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::Error::ShipmentNotFound)?;

        shipment.action(ShipmentActions::Buy(self.carrier_id))?;
        let value = shipment.info().value();

        let carrier = state.carrier_mut(&self.carrier_id).unwrap();
        carrier.add_shipment(self.shipment_id);

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actors::Actor, models::shipment::{Shipment, ShipmentInfo, ShipmentLocation, ShipmentStatus, SizeCategory}, operations::RegisterActorOp, utils::hash_secret, ActorId, Error
    };
    use super::*;
    use candid::Principal;

    const REGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[1, 2, 3, 4]));
    const UNREGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[5, 6, 7, 8]));
    const REGISTERED_SHIPPER_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));

    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_carrier = RegisterActorOp::AddCarrier {
            id: REGISTERED_CARRIER_ID.into(),
            name: "Test Carrier".to_string(),
        };
        register_carrier.apply(&mut state);

        let register_shipper = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ID.into(),
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
            &hash_secret(b"test_secret"),
            "Test Shipment",
            &info,
        );

        state.create_shipment(shipment).unwrap();
        state
    }

    #[test]
    fn test_buy_shipment_unregistered_carrier() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(UNREGISTERED_CARRIER_ID, 0);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::CarrierNotFound)));
    }

    #[test]
    fn test_buy_shipment_nonexistent_shipment() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 999);

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::ShipmentNotFound)));
    }

    #[test]
    fn test_buy_shipment_already_bought() {
        let mut state = setup_test_state();
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 0);

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::CarrierAlreadySet)));
    }

    #[test]
    fn test_buy_shipment_success_and_return_value() {
        let mut state = setup_test_state();
        let shipment_id = 0;
        let op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, shipment_id);

        let initial_carrier = state.carrier(&REGISTERED_CARRIER_ID).unwrap();
        assert!(!initial_carrier.get_active_shipments().contains(&shipment_id));

        let result = op.apply(&mut state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100); 

        let carrier = state.carrier(&REGISTERED_CARRIER_ID).unwrap();
        let shipment = state.shipment(shipment_id).unwrap();

        assert!(carrier.get_active_shipments().contains(&shipment_id));
        assert_eq!(shipment.status(), &ShipmentStatus::InTransit);
        assert_eq!(shipment.carrier_id(), Some(REGISTERED_CARRIER_ID));
    }
}
