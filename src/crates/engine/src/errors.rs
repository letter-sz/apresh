pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Shipment not found")]
    ShipmentNotFound,
    #[error("Carrier not found")]
    CarrierNotFound,
    #[error("Carrier not set")]
    CarrierNotSet,
    #[error("Shipper not found")]
    ShipperNotFound,
    #[error("Carrier already set")]
    CarrierAlreadySet,
    #[error("Shipment not ready to be finalized")]
    ShipmentNotReadyToBeFinalized,
    #[error("Not authorized to finalize shipment")]
    NotAuthorizedToFinalizeShipment,
    #[error("Secret key is invalid")]
    SecretKeyIsInvalid,
    #[error("Secret key not present")]
    SecretKeyNotPresent,
    #[error("Not authorized as carrier")]
    NotAuthorizedAsCarrier,
    #[error("Not authorized as shipper")]
    NotAuthorizedAsShipper,
    #[error("Not authorized as neither carrier nor shipper")]
    NotAuthorizedAsNeitherCarrierNorShipper,
}
