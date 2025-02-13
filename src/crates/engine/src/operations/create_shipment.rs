use crate::{
    models::shipment::{ChannelKey, Shipment, ShipmentId, ShipmentInfo},
    state::{CanisterActors, CanisterShipments},
    ActorId,
};

use super::{CanisterState, StateOp};
use crate::actors::Actor;

#[derive(Debug)]
pub struct CreateShipmentOp<'a> {
    creator: ActorId,
    hashed_secret: Vec<u8>,
    channel_key: ChannelKey,
    shipment_name: &'a str,
    info: &'a ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator: ActorId,
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

impl<'a> StateOp<ShipmentId> for CreateShipmentOp<'a> {
    type Error = crate::errors::Error;

    fn apply(&self, state: &mut CanisterState) -> crate::Result<ShipmentId> {
        let new_shipment_id = state.shipment_counter();

        if new_shipment_id == u64::MAX {
            return Err(crate::Error::ShipmentLimitReached);
        }

        let mut shipper = state
            .shipper_mut(&self.creator)
            .ok_or(crate::Error::ShipperNotFound)?;

        let shipment = Shipment::new(
            self.timestamp,
            shipper.id(),
            new_shipment_id,
            self.hashed_secret.clone(),
            self.channel_key.clone(),
            self.shipment_name,
            &self.info,
        );

        shipper.add_shipment(new_shipment_id);
        state.create_shipment(shipment)?;

        Ok(new_shipment_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::shipment::{ShipmentLocation, ShipmentStatus, SizeCategory},
        operations::RegisterActorOp,
        utils::hash_secret,
        ActorId, Error,
    };

    use super::*;
    use candid::Principal;

    pub const REGISTERED_SHIPPER_NAME: &str = "Ben Dover";
    pub const REGISTERED_SHIPPER_ACTOR_ID: ActorId = ActorId(Principal::from_slice(&[1, 3, 3, 7]));
    pub const UNREGISTERED_SHIPPER_ACTOR_ID: ActorId =
        ActorId(Principal::from_slice(&[2, 1, 3, 7]));
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
            id: REGISTERED_SHIPPER_ACTOR_ID.into(),
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

        unsafe { state.set_shipment_counter(u64::MAX - 1) };

        let op1 = CreateShipmentOp::new(
            REGISTERED_SHIPPER_ACTOR_ID,
            hashed_secret.clone(),
            CHANNEL_KEY.to_vec(),
            "Last Valid Shipment",
            &info,
            1234567890,
        );

        assert!(op1.apply(&mut state).is_ok());

        let op2 = CreateShipmentOp::new(
            REGISTERED_SHIPPER_ACTOR_ID,
            hashed_secret,
            CHANNEL_KEY.to_vec(),
            "Should Fail",
            &info,
            1234567890,
        );

        assert!(matches!(
            op2.apply(&mut state),
            Err(Error::ShipmentLimitReached)
        ));
    }

    #[test]
    fn test_create_shipment_unregistered_shipper() {
        let mut state = setup_test_state();
        let hashed_secret = hash_secret(b"secret_key_123");
        let info = default_shipper_info();

        let op = CreateShipmentOp::new(
            UNREGISTERED_SHIPPER_ACTOR_ID,
            hashed_secret,
            CHANNEL_KEY.to_vec(),
            "Test Shipment",
            &info,
            1234567890,
        );

        let result = op.apply(&mut state);
        assert!(matches!(result, Err(Error::ShipperNotFound)));
    }

    #[test]
    fn test_create_shipment_state_consistency() {
        let mut state = setup_test_state();
        let hashed_secret = hash_secret(b"secret_key_123");
        let info = default_shipper_info();
        let timestamp = 1234567890;
        let shipment_name = "Test Shipment";
        let expected_shipment_id = 0;

        let initial_counter = state.shipment_counter();
        let initial_shipments = state
            .shipper(&REGISTERED_SHIPPER_ACTOR_ID)
            .unwrap()
            .get_active_shipments()
            .len();

        let op = CreateShipmentOp::new(
            REGISTERED_SHIPPER_ACTOR_ID,
            hashed_secret.clone(),
            CHANNEL_KEY.to_vec(),
            shipment_name,
            &info,
            timestamp,
        );

        let expected_shipment = Shipment::new(
            timestamp,
            REGISTERED_SHIPPER_ACTOR_ID,
            expected_shipment_id,
            hashed_secret,
            CHANNEL_KEY.to_vec(),
            shipment_name,
            &info,
        );

        let result = op.apply(&mut state);
        assert!(result.is_ok());

        let shipment_id = result.unwrap();
        let shipper = state.shipper(&REGISTERED_SHIPPER_ACTOR_ID).unwrap();
        let state_shipment_id = state.shipment_counter();
        let state_shipment = state.shipment(expected_shipment_id);
        let shipper_shipments = shipper.get_active_shipments();

        assert_eq!(shipment_id, expected_shipment_id);
        assert!(shipper_shipments.contains(&expected_shipment_id));
        assert!(state_shipment.is_some());
        assert_eq!(shipper_shipments.len(), initial_shipments + 1);
        assert_eq!(state.shipment_counter(), initial_counter + 1);
        assert!(state.shipment(state_shipment_id).is_none());

        let shipment = state_shipment.unwrap();
        assert_eq!(shipment.shipper_id(), REGISTERED_SHIPPER_ACTOR_ID);
        assert_eq!(shipment._name(), shipment_name);
        assert_eq!(shipment.info(), &info);
        assert_eq!(shipment.status(), &ShipmentStatus::Pending);
        assert_eq!(shipment.id(), expected_shipment_id);
    }
}
