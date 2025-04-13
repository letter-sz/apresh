use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, ShipmentError>;

#[derive(Error, Debug)]
pub enum ShipmentError {
    #[error("Carrier already set")]
    CarrierAlreadySet,
    #[error("Shipment already bought")]
    ShipmentNotReadyToBeCanceled,
    #[error("Shipment not ready to be finalized")]
    ShipmentNotReadyToBeFinalized,
    #[error("Secret key is invalid")]
    SecretKeyIsInvalid,
    #[error("Secret key not present")]
    SecretKeyNotPresent,
    #[error("Not authorized as shipper")]
    NotAuthorizedAsShipper,
    #[error("Shipment cannot be bought")]
    ShipmentCannotBeBought,
}
