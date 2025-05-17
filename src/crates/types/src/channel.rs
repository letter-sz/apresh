use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type Message = Vec<u8>;
pub type ChannelKey = Vec<u8>;

#[cfg(feature = "icp")]
#[derive(CandidType)] //
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    host_key: ChannelKey,
    messages: Vec<Message>,
    guest_keys: Vec<ChannelKey>, // Guests understand host but not each other
}

impl Channel {
    pub fn new(host_key: ChannelKey) -> Self {
        Self {
            host_key,
            messages: Vec::new(),
            guest_keys: Vec::new(),
        }
    }

    pub fn add_guest(&mut self, guest_key: ChannelKey) {
        self.guest_keys.push(guest_key);
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}
