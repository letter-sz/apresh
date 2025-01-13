use candid::CandidType;
use serde::{Deserialize, Serialize};

// LOCATION

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct ShipmentLocation {
    street: String,
    lat: f64,
    lng: f64,
}

impl ShipmentLocation {
    pub fn new(street: String, lat: f64, lng: f64) -> Self {
        Self { street, lat, lng }
    }
}


// SIZE CATEGORY
#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub enum SizeCategory {
    Envelope,
    Parcel {
        max_width: u64,
        max_height: u64,
        max_depth: u64,
    },
}

// INFO

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
pub struct ShipmentInfo {
    /// Shipment value, used in insurance
    value: u64,
    /// Shipment price for a delivery
    price: u64,
    /// Shipment source location
    source: ShipmentLocation,
    /// Shipment destination location
    destination: ShipmentLocation,
    /// Shipment size category
    size_category: SizeCategory,
}

impl ShipmentInfo {
    pub fn price(&self) -> u64 {
        self.price
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn new(
        value: u64,
        price: u64,
        source: ShipmentLocation,
        destination: ShipmentLocation,
        size_category: SizeCategory,
    ) -> Self {
        Self {
            value,
            price,
            source,
            destination,
            size_category,
        }
    }
}
