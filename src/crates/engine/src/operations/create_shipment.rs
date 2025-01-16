use crate::models::shipment::{InternalShipment, ShipmentId, ShipmentInfo};

use super::{CanisterState, StateOp};
use crate::actors::{shipper::Shipper, Actor};

#[derive(Debug)]
pub struct CreateShipmentOp<'a> {
    creator: Shipper,
    hashed_secret: &'a str,
    shipment_name: &'a str,
    info: ShipmentInfo,
    timestamp: u64,
}

impl<'a> CreateShipmentOp<'a> {
    pub fn new(
        creator: Shipper,
        hashed_secret: &'a str,
        shipment_name: &'a str,
        info: ShipmentInfo,
        timestamp: u64,
    ) -> Self {
        Self {
            creator,
            hashed_secret,
            shipment_name,
            info,
            timestamp,
        }
    }
}

impl<'a> StateOp<ShipmentId> for CreateShipmentOp<'a> {
    type Error = anyhow::Error;

    fn apply(&self, state: &mut CanisterState) -> Result<ShipmentId, anyhow::Error> {
        let new_shipment_id = state.shipment_counter;
        state.shipment_counter += 1;

        let shipper = match state.shippers.get_mut(&self.creator.id()) {
            Some(shipper) => shipper,
            None => state.shippers.create(self.creator.clone()),
        };

        let shipment = InternalShipment::new(
            self.timestamp,
            shipper.id(),
            new_shipment_id,
            self.hashed_secret,
            self.shipment_name,
            self.info.clone(),
        );

        state.shipments.insert(new_shipment_id, shipment);
        shipper.add_shipment(new_shipment_id);

        Ok(new_shipment_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::shipment::{ShipmentLocation, SizeCategory};

    use super::*;
    use candid::Principal;

    #[test]
    fn test_create_shipment_success() {
        let mut state = CanisterState::default();

        let creator_id = Principal::anonymous();
        let creator_name = "John Doe";
        let hashed_secret = "hashed_secret_123";
        let shipment_name = "Test Shipment";
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let info = ShipmentInfo::new(
            100,
            10,
            ShipmentLocation::new("Warsaw".to_string(), 52.23, 21.01),
            ShipmentLocation::new("Krakow".to_string(), 54.44, 18.23),
            SizeCategory::Envelope,
        );

        let creator = Shipper::new(creator_id, creator_name);

        let op = CreateShipmentOp::new(
            creator,
            hashed_secret,
            shipment_name,
            info.clone(),
            timestamp,
        );

        let result = op.apply(&mut state);

        assert!(result.is_ok());
        let shipment_id = result.unwrap();
        assert_eq!(shipment_id, 0);
        assert_eq!(state.shipment_counter, 1);

        let shipment = state.shipments.get(&shipment_id).unwrap();
        assert_eq!(shipment.shipper_id(), creator_id);
        assert_eq!(shipment._id(), shipment_id);
        assert_eq!(shipment._name(), shipment_name);

        let shipper = state.shippers.get(&creator_id).unwrap();
        assert_eq!(shipper.id(), creator_id);
        assert!(shipper.get_active_shipments().contains(&shipment_id));
        assert_eq!(shipper.name(), creator_name);
    }
}
