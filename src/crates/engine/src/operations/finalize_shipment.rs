use crate::state::CanisterState;

use super::{buy_shipment::Cost, StateOp};
use anyhow::anyhow;
use apresh_store::Record;
use apresh_types::{ActorId, CarrierKey, Shipment, ShipmentActions, ShipmentId, ShipmentKey};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct FinalizeShipmentOp<'a> {
    caller: ActorId,
    shipment: &'a mut Shipment,
    secret_key: Option<String>,
}

impl<'a> FinalizeShipmentOp<'a> {
    pub fn new(shipment: &'a mut Shipment, secret_key: Option<String>, caller: ActorId) -> Self {
        Self {
            shipment,
            secret_key,
            caller,
        }
    }
}

impl StateOp<FinalizeShipmentResult> for FinalizeShipmentOp<'_> {
    type Error = crate::EngineError;

    fn apply(
        self,
        state: &mut CanisterState,
    ) -> Result<FinalizeShipmentResult, crate::EngineError> {
        let carrier_id = self
            .shipment
            .carrier_id()
            .ok_or(crate::errors::EngineError::CarrierNotSet)?;

        let value = self.shipment.info().value();
        let price = self.shipment.info().price();

        self.shipment.action(ShipmentActions::MarkDelivered {
            secret_key: self.secret_key.clone(),
            caller: self.caller,
        })?;

        Ok(FinalizeShipmentResult {
            carrier_id,
            value,
            price,
        })
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
        Shipment, ShipmentError, ShipmentInfo, ShipmentLocation, ShipmentStatus, ShipperKey,
        SizeCategory,
    };
    use candid::Principal;

    const REGISTERED_SHIPPER_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[1, 2, 3, 4])));
    const REGISTERED_CARRIER_ID: CarrierKey =
        CarrierKey(ActorId(Principal::from_slice(&[5, 6, 7, 8])));
    const UNREGISTERED_ACTOR_ID: ActorId = ActorId(Principal::from_slice(&[9, 10, 11, 12]));
    const ANONYMOUS_ACTOR_ID: ActorId = ActorId(Principal::anonymous());
    const SECRET_KEY: &str = "test_secret";
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
            hash_secret(SECRET_KEY.as_bytes()),
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
        );

        Shipment::set(shipment);
        state
    }

    fn setup_bought_shipment_state() -> CanisterState {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let mut carrier = REGISTERED_CARRIER_ID.get().unwrap();
        let buy_op = BuyShipmentOp::new(&mut carrier, &mut shipment, CHANNEL_KEY.to_vec());
        buy_op.apply(&mut state).unwrap();
        state
    }

    #[test]
    fn test_finalize_shipment_nonexistent() {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(999).get().ok_or(EngineError::ShipmentNotFound);

        assert!(matches!(shipment, Err(EngineError::ShipmentNotFound)));
    }

    #[test]
    fn test_finalize_shipment_not_bought() {
        let mut state = setup_test_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(
            &mut shipment,
            Some(SECRET_KEY.to_string()),
            REGISTERED_CARRIER_ID.0,
        );

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(EngineError::CarrierNotSet)));
    }

    #[test]
    fn test_finalize_shipment_wrong_secret() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(
            &mut shipment,
            Some("wrong_secret".to_string()),
            REGISTERED_CARRIER_ID.0,
        );

        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::SecretKeyIsInvalid
            ))
        ));
    }

    #[test]
    fn test_finalize_shipment_no_secret_if_caller_is_anonymous() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(&mut shipment, None, REGISTERED_CARRIER_ID.0);

        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::SecretKeyNotPresent
            ))
        ));
    }

    #[test]
    fn test_finalize_shipment_no_secret_if_caller_is_carrier() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(&mut shipment, None, REGISTERED_CARRIER_ID.0);

        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::SecretKeyNotPresent
            ))
        ));
    }

    #[test]
    fn test_finalize_shipment_success_if_caller_is_shipper() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(&mut shipment, None, REGISTERED_SHIPPER_ID.0);

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        shipment.consume();

        let shipment = ShipmentKey(0).get().unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::DeliveryCompleted);
    }

    #[test]
    fn test_finalize_shipment_success() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(
            &mut shipment,
            Some(SECRET_KEY.to_string()),
            REGISTERED_CARRIER_ID.0,
        );

        let initial_shipment = ShipmentKey(0).get().unwrap();
        assert_eq!(initial_shipment.status(), &ShipmentStatus::InTransit);

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        let finalize_result = result.unwrap();
        assert_eq!(finalize_result.carrier_id(), &REGISTERED_CARRIER_ID.0);
        assert_eq!(finalize_result.value(), 100);
        assert_eq!(finalize_result.price(), 10);

        shipment.consume();

        let shipment = ShipmentKey(0).get().unwrap();
        assert_eq!(shipment.status(), &ShipmentStatus::DeliveryCompleted);
    }

    #[test]
    fn test_finalize_already_delivered_shipment() {
        let mut state = setup_bought_shipment_state();
        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(
            &mut shipment,
            Some(SECRET_KEY.to_string()),
            REGISTERED_CARRIER_ID.0,
        );

        assert_eq!(
            op.apply(&mut state),
            Ok(FinalizeShipmentResult {
                carrier_id: REGISTERED_CARRIER_ID.0,
                value: 100,
                price: 10
            })
        );

        shipment.consume();

        let mut shipment = ShipmentKey(0).get().unwrap();
        let op = FinalizeShipmentOp::new(
            &mut shipment,
            Some(SECRET_KEY.to_string()),
            REGISTERED_CARRIER_ID.0,
        );
        let result = op.apply(&mut state);
        assert!(matches!(
            result,
            Err(EngineError::ShipmentError(
                ShipmentError::ShipmentNotReadyToBeFinalized
            ))
        ));
    }
}
