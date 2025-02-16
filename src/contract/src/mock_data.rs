use candid::Principal;
use engine::{
    models::shipment::{ShipmentInfo, ShipmentLocation, SizeCategory},
    operations::{CreateShipmentOp, RegisterActorOp, StateOp},
    utils::hash_secret,
};

use crate::STATE;

pub fn mock_shipments() {
    ic_cdk::print("Initializing the shipment service");

    // Define a set of realistic coordinates for shipment locations
    let locations = [
        ("A", 40.7128, -74.0060),  // New York, USA
        ("B", 34.0522, -118.2437), // Los Angeles, USA
        ("C", 51.5074, -0.1278),   // London, UK
        ("D", 48.8566, 2.3522),    // Paris, France
        ("E", 35.6895, 139.6917),  // Tokyo, Japan
        ("F", -33.8688, 151.2093), // Sydney, Australia
    ];

    let default_principal =
        Principal::from_text("ryssj-xcbz7-gbw4s-p7fio-lolnx-5nr7a-yxufe-cvpfg-6iujw-2ypsz-rqe")
            .expect("Failed to create principal");

    let names = [
        ("Package 1", "John Doe"),
        ("Package 2", "Jane Doe"),
        ("Package 3", "Alice Smith"),
        ("Package 4", "Bob Smith"),
        ("Package 5", "Charlie Brown"),
        ("Package 6", "Daisy Brown"),
        ("Package 7", "Eve Green"),
        ("Package 8", "Frank Green"),
        ("Package 9", "Grace Black"),
        ("Package 10", "Harry Black"),
    ];

    for (i, (package_name, name)) in names.iter().enumerate() {
        let (origin_label, origin_lat, origin_lng) = &locations[i % locations.len()];
        let (dest_label, dest_lat, dest_lng) = &locations[(i + 1) % locations.len()];

        let shipment_id = STATE
            .with_borrow_mut(|state| {
                RegisterActorOp::AddShipper {
                    id: default_principal.into(),
                    name: name.to_string(),
                }
                .apply(state)
                .unwrap();

                CreateShipmentOp::new(
                    default_principal.into(),
                    hash_secret(b"secret"),
                    b"channel_key_123".to_vec(),
                    package_name,
                    ShipmentInfo::new(
                        100u64 + i as u64,
                        10u64 + i as u64,
                        ShipmentLocation::new(origin_label.to_string(), *origin_lat, *origin_lng),
                        ShipmentLocation::new(dest_label.to_string(), *dest_lat, *dest_lng),
                        SizeCategory::Envelope,
                    ),
                    ic_cdk::api::time(),
                )
                .apply(state)
            })
            .map_err(|e| e.to_string())
            .expect("Failed to create shipment");

        ic_cdk::print(format!(
            "Shipment created: {:?}, shipment_id: {}",
            shipment_id, shipment_id
        ));
    }
}
