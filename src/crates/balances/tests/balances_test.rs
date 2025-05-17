use balances::{BalanceAndLockedError, Balances};

#[test]
fn balances() {
    let mut balances = Balances::default();
    balances.deposit(10).unwrap();
    assert_eq!(balances.balance(), 10);

    balances.lock(10).unwrap();
    assert_eq!(balances.locked(), 10);

    let r = balances.lock(1);
    assert_eq!(r, Err(BalanceAndLockedError::InsufficientBalance));
}
