use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Default, Deserialize)]
pub struct CanisterState {
    shipment_counter: u64,
}

impl CanisterState {
    pub fn get_new_shipment_id(&mut self) -> u64 {
        self.shipment_counter += 1;
        self.shipment_counter
    }

    #[cfg(test)]
    pub fn shipment_counter(&self) -> u64 {
        self.shipment_counter
    }

    #[cfg(test)]
    pub fn set_shipment_counter(&mut self, counter: u64) {
        self.shipment_counter = counter;
    }
}
