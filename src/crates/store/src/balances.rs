use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::{Guard, Writable, BALANCES};

pub fn balances(caller_bytes: Vec<u8>) -> Guard<Balances> {
    BALANCES.with_borrow_mut(|balances| {
        let balances = balances.get(&caller_bytes).unwrap_or_default();
        Guard::new_with_key(caller_bytes, balances)
    })
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Balances {
    regular: BalanceAndLocked,
    prepaid: BalanceAndLocked,
    _shared: BalanceAndLocked,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BalanceAndLocked {
    balance: u64,
    locked: u64, // Locked means balance was not yet paid, but could be required to be so it can't be withdrawn
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum BalanceAndLockedError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Balance too great to be stored")]
    BalanceTooGreat,
    #[error("Inconsistency - not enough locked balance")]
    InsufficientLockedBalance,
}

impl Storable for Balances {
    const BOUND: Bound = Bound::Bounded {
        max_size: 48,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        bcs::to_bytes(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        bcs::from_bytes::<Self>(&bytes).unwrap()
    }
}

impl Writable for Balances {
    fn commit(self, key: Vec<u8>) {
        BALANCES.with_borrow_mut(|balances| {
            balances.insert(key, self);
        })
    }
}

type Result<T> = std::result::Result<T, BalanceAndLockedError>;

impl Balances {
    pub fn balance(&self) -> u64 {
        self.regular.balance + self.prepaid.balance
    }

    pub fn locked(&self) -> u64 {
        self.regular.locked + self.prepaid.locked
    }

    pub fn withdrawable(&self) -> u64 {
        self.regular.balance
    }

    pub fn prepaid_balance(&self) -> u64 {
        self.prepaid.balance
    }

    pub fn prepaid_locked(&self) -> u64 {
        self.prepaid.locked
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.regular.deposit(amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.regular.withdraw(amount)
    }

    pub fn prepay(&mut self, amount: u64) -> Result<()> {
        self.prepaid.deposit(amount)
    }

    pub fn lock(&mut self, amount: u64) -> Result<()> {
        if self.prepaid.balance == 0 {
            self.regular.lock(amount)
        } else if self.prepaid.balance > amount {
            self.prepaid.lock(amount)
        } else {
            self.regular.lock(amount - self.prepaid.balance)?;
            self.prepaid.lock(self.prepaid.balance)
        }
    }

    pub fn unlock(&mut self, amount: u64) -> Result<()> {
        if self.regular.locked > amount {
            self.regular.unlock(amount)
        } else {
            self.prepaid.unlock(amount - self.regular.locked)?;
            self.regular.unlock(self.regular.locked)
        }
    }

    pub fn transfer_from(&mut self, other: &mut Self, amount: u64) -> Result<()> {
        if self.regular.locked > amount {
            self.regular.transfer_locked_out(amount)?;
        } else {
            self.prepaid
                .transfer_locked_out(amount - self.regular.locked)?;
            self.regular.transfer_locked_out(self.regular.locked)?;
        }

        other.deposit(amount)
    }
}

impl BalanceAndLocked {
    fn deposit(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(BalanceAndLockedError::BalanceTooGreat)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(BalanceAndLockedError::InsufficientBalance)?;
        Ok(())
    }

    fn transfer_locked_out(&mut self, amount: u64) -> Result<()> {
        self.locked = self
            .locked
            .checked_sub(amount)
            .ok_or(BalanceAndLockedError::InsufficientLockedBalance)?;
        Ok(())
    }

    fn lock(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(BalanceAndLockedError::InsufficientBalance)?;
        self.locked = self
            .locked
            .checked_add(amount)
            .ok_or(BalanceAndLockedError::BalanceTooGreat)?;
        Ok(())
    }

    fn unlock(&mut self, amount: u64) -> Result<()> {
        self.locked = self
            .locked
            .checked_sub(amount)
            .ok_or(BalanceAndLockedError::InsufficientLockedBalance)?;
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(BalanceAndLockedError::BalanceTooGreat)?;
        Ok(())
    }
}
