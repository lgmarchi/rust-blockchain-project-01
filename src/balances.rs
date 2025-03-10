use std::{
    collections::BTreeMap,
    fmt::Debug,
};

use num::{
    CheckedAdd,
    CheckedSub,
    Zero,
};

pub trait Config: AccountIdentifier {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Debug;
}

use crate::{
    error_messages::*,
    utils::AccountIdentifier,
};

/// The `Pallet` struct represents a module for managing account balances.
/// It uses a `BTreeMap` to store the balances of accounts, where the key
/// is the account ID and the value is the balance associated with that
/// account.
///
/// # Type Parameters
///
/// * `T` - A type that implements the `BalancesConfig` trait, which defines the
///   associated types for `AccountId` and `Balance`.
///
/// # Fields
///
/// * `balances` - A `BTreeMap` that maps account IDs to their respective
///   balances.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
///
/// use crate::{
///     balances::Pallet,
///     utils::BalancesConfig,
/// };
///
/// struct TestConfig;
///
/// impl BalancesConfig for TestConfig {
///     type AccountId = String;
///     type Balance = u128;
/// }
///
/// let mut balances: Pallet<TestConfig> = Pallet::new();
/// balances.set_balance(&"Alice".to_string(), 100);
/// assert_eq!(balances.get_balance(&"Alice".to_string()), 100);
/// ```
#[derive(Clone, Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub const fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        let balance =
            self.balances.get(who).map_or_else(T::Balance::zero, |f| *f);
        balance
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfer `amount` from one account to another
    /// This function verifies that `from` has at least `amount` balance to
    /// transfer and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_ballance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or(ERR_INSUFFICIENT_BALANCE)?;

        let new_to_ballance =
            to_ballance.checked_add(&amount).ok_or(ERR_OVERFLOW_BALANCE)?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_ballance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::{
        balances::Pallet,
        utils::{
            AccountIdentifier,
            Balance,
        },
    };

    const ALICE_BALANCE: &str = "Alice";
    const BOB_BALANCE: &str = "Bob";

    struct TestConfig;

    impl AccountIdentifier for TestConfig {
        type AccountId = String;
    }

    impl Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 0);
        balances.set_balance(&ALICE_BALANCE.to_string(), 100);
        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 100);
        assert_eq!(balances.get_balance(&BOB_BALANCE.to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&ALICE_BALANCE.to_string(), 100);
        balances.set_balance(&BOB_BALANCE.to_string(), 20);

        let _ = balances.transfer(
            ALICE_BALANCE.to_string(),
            BOB_BALANCE.to_string(),
            80,
        );

        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 20);
        assert_eq!(balances.get_balance(&BOB_BALANCE.to_string()), 100);
    }

    #[test]
    fn test_balance_overflow() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&ALICE_BALANCE.to_string(), 100);
        balances.set_balance(&BOB_BALANCE.to_string(), Balance::MAX);

        let transfer_result = balances.transfer(
            ALICE_BALANCE.to_string(),
            BOB_BALANCE.to_string(),
            1,
        );

        assert_eq!(transfer_result, Err(super::ERR_OVERFLOW_BALANCE));
        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 100);
        assert_eq!(
            balances.get_balance(&BOB_BALANCE.to_string()),
            Balance::MAX
        );
    }

    #[test]
    fn insufficient_found_to_transfer() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&ALICE_BALANCE.to_string(), 30);
        balances.set_balance(&BOB_BALANCE.to_string(), 20);

        let transfer_result = balances.transfer(
            ALICE_BALANCE.to_string(),
            BOB_BALANCE.to_string(),
            50,
        );

        assert_eq!(transfer_result, Err(super::ERR_INSUFFICIENT_BALANCE));
        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 30);
        assert_eq!(balances.get_balance(&BOB_BALANCE.to_string()), 20);
    }
}
