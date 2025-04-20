use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Default, Deserialize)]
pub struct CanisterState {
    shipment_counter: u64,
}

impl CanisterState {
    pub fn set_shipment_counter(&mut self, counter: u64) {
        self.shipment_counter = counter;
    }

    pub fn shipment_counter(&self) -> u64 {
        self.shipment_counter
    }
}
