pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum EngineError {
    #[error("Store error: {0}")]
    StoreError(#[from] apresh_store::StoreError),
    #[error("Shipment error: {0}")]
    ShipmentError(#[from] apresh_types::ShipmentError),
    #[error("Shipment not found")]
    ShipmentNotFound,
    #[error("Carrier not found")]
    CarrierNotFound,
    #[error("Carrier not set")]
    CarrierNotSet,
    #[error("Shipper not found")]
    ShipperNotFound,
    #[error("Not authorized as neither carrier nor shipper")]
    NotAuthorizedAsNeitherCarrierNorShipper,
    #[error("Message too long")]
    MessageTooLong,
    #[error("Shipment limit reached")]
    ShipmentLimitReached,
    #[error("Not authorized as shipper")]
    NotAuthorizedAsShipper,
}
