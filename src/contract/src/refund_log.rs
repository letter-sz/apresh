use crate::transfer::Refund;
use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, Log,
};
use std::cell::RefCell;

thread_local! {
  static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
      MemoryManager::init(DefaultMemoryImpl::default())
  );
}

const REFUND_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(0);
const REFUND_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(1);

pub struct RefundLog(
    Log<Refund, VirtualMemory<DefaultMemoryImpl>, VirtualMemory<DefaultMemoryImpl>>,
);

impl Default for RefundLog {
    fn default() -> Self {
        let index_mem = MEMORY_MANAGER.with(|m| m.borrow().get(REFUND_LOG_INDEX_MEMORY_ID));
        let data_mem = MEMORY_MANAGER.with(|m| m.borrow().get(REFUND_LOG_DATA_MEMORY_ID));

        Self(Log::init(index_mem, data_mem).unwrap())
    }
}

impl RefundLog {
    pub fn append(&self, amount: u64, recipient: Principal, memo: String) -> Result<u64, String> {
        let refund = Refund {
            amount,
            recipient,
            memo,
            timestamp: ic_cdk::api::time(),
        };

        self.0
            .append(&refund)
            .map_err(|err| format!("Failed to log refund: {:?}", err))
    }

    pub fn get(&self, idx: u64) -> Option<Refund> {
        self.0.get(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = Refund> + '_ {
        self.0.iter()
    }
}