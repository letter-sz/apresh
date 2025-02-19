use crate::{
    models::shipment::{ShipmentActions, ShipmentId},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::StateOp;
use crate::{actors::Actor, state::CanisterState};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct FinalizeShipmentResult {
    carrier_id: ActorId,
    value: u64,
    price: u64,
}

impl FinalizeShipmentResult {
    pub fn carrier_id(&self) -> &ActorId {
        &self.carrier_id
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn price(&self) -> u64 {
        self.price
    }
}

#[derive(Debug, Clone)]
pub struct FinalizeShipmentOp {
    caller: ActorId,
    shipment_id: ShipmentId,
    secret_key: Option<String>,
}

impl FinalizeShipmentOp {
    pub fn new(shipment_id: ShipmentId, secret_key: Option<String>, caller: ActorId) -> Self {
        Self {
            shipment_id,
            secret_key,
            caller,
        }
    }
}

impl StateOp<FinalizeShipmentResult> for FinalizeShipmentOp {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<FinalizeShipmentResult, anyhow::Error> {
        let shipment = state
            .shipment_mut(self.shipment_id)
            .ok_or(crate::errors::Error::ShipmentNotFound)?;

        let carrier_id = shipment
            .carrier_id()
            .ok_or(crate::errors::Error::CarrierNotSet)?;

        let value = shipment.info().value();
        let price = shipment.info().price();

        shipment.action(ShipmentActions::MarkDelivered {
            secret_key: self.secret_key.clone(),
            caller: self.caller,
        })?;

        let carrier = state
            .carrier_mut(&carrier_id)
            .ok_or(crate::errors::Error::CarrierNotFound)?;

        Ok(FinalizeShipmentResult {
            carrier_id: carrier.id(),
            value,
            price,
        })
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
    const REGISTERED_CARRIER_ID: ActorId = ActorId(Principal::from_slice(&[5, 6, 7, 8]));
    const UNREGISTERED_ACTOR_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));
    const ANONYMOUS_ACTOR_ID: ActorId = ActorId(Principal::anonymous());
    const SECRET_KEY: &str = "test_secret";
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
            hash_secret(SECRET_KEY.as_bytes()),
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
        );

        state.create_shipment(shipment).unwrap();
        state
    }

    fn setup_bought_shipment_state() -> CanisterState {
        let mut state = setup_test_state();
        let buy_op = BuyShipmentOp::new(REGISTERED_CARRIER_ID, 0, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state).unwrap();
        state
    }

    #[test]
    fn test_finalize_shipment_nonexistent() {
        let mut state = setup_test_state();
        let op = FinalizeShipmentOp::new(999, Some(SECRET_KEY.to_string()), ANONYMOUS_ACTOR_ID);

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::ShipmentNotFound)
        ));
    }

    #[test]
    fn test_finalize_shipment_not_bought() {
        let mut state = setup_test_state();
        let op = FinalizeShipmentOp::new(0, Some(SECRET_KEY.to_string()), ANONYMOUS_ACTOR_ID);

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::CarrierNotSet)
        ));
    }

    #[test]
    fn test_finalize_shipment_wrong_secret() {
        let mut state = setup_bought_shipment_state();
        let op =
            FinalizeShipmentOp::new(0, Some("wrong_secret".to_string()), REGISTERED_CARRIER_ID);

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::SecretKeyIsInvalid)
        ));
    }

    #[test]
    fn test_finalize_shipment_no_secret_if_caller_is_anonymous() {
        let mut state = setup_bought_shipment_state();
        let op = FinalizeShipmentOp::new(0, None, ANONYMOUS_ACTOR_ID);

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::SecretKeyNotPresent)
        ));
    }

    #[test]
    fn test_finalize_shipment_no_secret_if_caller_is_carrier() {
        let mut state = setup_bought_shipment_state();
        let op = FinalizeShipmentOp::new(0, None, REGISTERED_CARRIER_ID);

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::SecretKeyNotPresent)
        ));
    }

    #[test]
    fn test_finalize_shipment_success_if_caller_is_shipper() {
        let mut state = setup_bought_shipment_state();
        let op = FinalizeShipmentOp::new(0, None, REGISTERED_SHIPPER_ID);

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        let shipment = state.shipment(0).unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::DeliveryCompleted);
    }

    #[test]
    fn test_finalize_shipment_success() {
        let mut state = setup_bought_shipment_state();
        let op = FinalizeShipmentOp::new(0, Some(SECRET_KEY.to_string()), REGISTERED_CARRIER_ID);

        let initial_shipment = state.shipment(0).unwrap();
        assert_eq!(initial_shipment.status(), &ShipmentStatus::InTransit);

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        let finalize_result = result.unwrap();
        assert_eq!(finalize_result.carrier_id(), &REGISTERED_CARRIER_ID);
        assert_eq!(finalize_result.value(), 100);
        assert_eq!(finalize_result.price(), 10);

        let shipment = state.shipment(0).unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::DeliveryCompleted);
    }

    #[test]
    fn test_finalize_already_delivered_shipment() {
        let mut state = setup_bought_shipment_state();
        let op = FinalizeShipmentOp::new(0, Some(SECRET_KEY.to_string()), REGISTERED_CARRIER_ID);

        assert!(op.apply(&mut state).is_ok());

        let result = op.apply(&mut state);
        assert!(matches!(
            result.unwrap_err().downcast_ref::<Error>(),
            Some(&Error::ShipmentNotReadyToBeFinalized)
        ));
    }
}
