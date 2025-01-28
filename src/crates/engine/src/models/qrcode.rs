use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct QrCodeOptions {
    pub link: String,
    pub size: usize,
    pub gradient: bool,
    pub transparent: bool,
}
