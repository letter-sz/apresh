use apresh_types::{
    Actor, ActorId, ChannelKey, Shipment, ShipmentId, ShipmentInfo, Shipper, ShipperKey,
};

use apresh_store::{Guard, Record};

use super::{CanisterState, StateOp};

#[derive(Debug)]
pub struct CreateShipmentOp<'a> {
    creator: &'a mut Shipper,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_name: &'a str,
    info: &'a ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator: &'a mut Shipper,
        hashed_secret: Vec<u8>,
        channel_key: ChannelKey,
        shipment_name: &'a str,
        info: &'a ShipmentInfo,
        timestamp: u64,
    ) -> Self {
        Self {
            creator,
            hashed_secret,
            channel_key,
            shipment_name,
            info,
            timestamp,
        }
    }
}

impl StateOp<Shipment> for CreateShipmentOp<'_> {
    type Error = crate::errors::EngineError;

    fn apply(self, state: &mut CanisterState) -> crate::Result<Shipment> {
        let new_shipment_id = state.get_new_shipment_id();

        if new_shipment_id == u64::MAX {
            return Err(crate::EngineError::ShipmentLimitReached);
        }

        let mut shipment = Shipment::new(
            self.timestamp,
            self.creator.id(),
            new_shipment_id,
            self.hashed_secret.clone(),
            self.channel_key.clone(),
            self.shipment_name,
            self.info,
        );

        self.creator.add_shipment(new_shipment_id);
        Ok(shipment)
    }
}

#[cfg(test)]
mod tests {
    use crate::{operations::RegisterActorOp, EngineError};

    use super::*;
    use apresh_crypto::hash_secret;
    use apresh_types::{ShipmentKey, ShipmentLocation, ShipmentStatus, SizeCategory};
    use candid::Principal;

    pub const REGISTERED_SHIPPER_NAME: &str = "Ben Dover";
    pub const REGISTERED_SHIPPER_ACTOR_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[1, 3, 3, 7])));
    pub const UNREGISTERED_SHIPPER_ACTOR_ID: ShipperKey =
        ShipperKey(ActorId(Principal::from_slice(&[2, 1, 3, 7])));
    pub const CHANNEL_KEY: &[u8] = b"channel_key_123";

    fn default_shipper_info() -> ShipmentInfo {
        ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        )
    }

    fn setup_test_state() -> CanisterState {
        let mut state = CanisterState::default();

        let register_op = RegisterActorOp::AddShipper {
            id: REGISTERED_SHIPPER_ACTOR_ID.0,
            name: REGISTERED_SHIPPER_NAME.to_string(),
        };

        register_op.apply(&mut state);

        state
    }

    #[test]
    fn test_create_shipment_with_max_counter() {
        let mut state = setup_test_state();
        let hashed_secret = hash_secret(b"secret_key");
        let info = default_shipper_info();

        state.set_shipment_counter(u64::MAX - 2);

        let mut shipper = REGISTERED_SHIPPER_ACTOR_ID.get().unwrap();
        let op1 = CreateShipmentOp::new(
            &mut shipper,
            hashed_secret.clone(),
            CHANNEL_KEY.to_vec(),
            "Last Valid Shipment",
            &info,
            1234567890,
        );

        let new_shipment = op1.apply(&mut state).unwrap();
        assert_eq!(new_shipment.id(), u64::MAX - 1);
        new_shipment.set();
        shipper.commit();

        let mut shipper = REGISTERED_SHIPPER_ACTOR_ID.get().unwrap();
        let op2 = CreateShipmentOp::new(
            &mut shipper,
            hashed_secret,
            CHANNEL_KEY.to_vec(),
            "Should Fail",
            &info,
            1234567890,
        );

        assert_eq!(
            op2.apply(&mut state),
            Err(EngineError::ShipmentLimitReached)
        );

        shipper.revert();
    }

    #[test]
    fn test_create_shipment_unregistered_shipper() {
        let mut state = setup_test_state();
        let hashed_secret = hash_secret(b"secret_key_123");
        let info = default_shipper_info();

        let mut shipper = UNREGISTERED_SHIPPER_ACTOR_ID
            .get()
            .ok_or(EngineError::ShipperNotFound);
        assert!(matches!(shipper, Err(EngineError::ShipperNotFound)));
    }

    #[test]
    fn test_create_shipment_state_consistency() {
        let mut state = setup_test_state();
        let hashed_secret = hash_secret(b"secret_key_123");
        let info = default_shipper_info();
        let timestamp = 1234567890;
        let shipment_name = "Test Shipment";
        let expected_shipment_id = 1;

        let initial_counter = state.shipment_counter();
        let initial_shipments = REGISTERED_SHIPPER_ACTOR_ID
            .get()
            .unwrap()
            .get_active_shipments()
            .len();

        let mut shipper = REGISTERED_SHIPPER_ACTOR_ID.get().unwrap();
        let op = CreateShipmentOp::new(
            &mut shipper,
            hashed_secret.clone(),
            CHANNEL_KEY.to_vec(),
            shipment_name,
            &info,
            timestamp,
        );

        let expected_shipment = Shipment::new(
            timestamp,
            REGISTERED_SHIPPER_ACTOR_ID.0,
            expected_shipment_id,
            hashed_secret,
            CHANNEL_KEY.to_vec(),
            shipment_name,
            &info,
        );

        let new_shipment = op.apply(&mut state).unwrap();
        let shipment_id = new_shipment.id();
        assert_eq!(shipment_id, expected_shipment_id);
        new_shipment.set();
        shipper.commit();

        let shipper = (REGISTERED_SHIPPER_ACTOR_ID).get().unwrap();
        let state_shipment_id = state.shipment_counter();
        let state_shipment = ShipmentKey(expected_shipment_id).get();
        let shipper_shipments = shipper.get_active_shipments();

        assert_eq!(shipment_id, expected_shipment_id);
        assert!(shipper_shipments.contains(&expected_shipment_id));
        assert!(state_shipment.is_some());
        assert_eq!(shipper_shipments.len(), initial_shipments + 1);
        assert_eq!(state.shipment_counter(), initial_counter + 1);
        assert!(ShipmentKey(state_shipment_id + 1).get().is_none());

        let shipment = state_shipment.unwrap();
        assert_eq!(shipment.shipper_id(), REGISTERED_SHIPPER_ACTOR_ID.0);
        assert_eq!(shipment._name(), shipment_name);
        assert_eq!(shipment.info(), &info);
        assert_eq!(shipment.status(), &ShipmentStatus::Pending);
        assert_eq!(shipment.id(), expected_shipment_id);
    }
}
