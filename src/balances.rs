use std::collections::BTreeMap;

const ERR_INSUFFICIENT_BALANCE: &str = "Insufficient balance";
const ERR_OVERFLOW_BALANCE: &str = "Overflow when adding to balance";

#[derive(Clone, Debug)]
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &String) -> u128 {
        let balance = self.clone().balances.get(who).map(|f| *f).unwrap_or(0);
        balance
    }

    /// Transfer `amount` from one account to another
    /// This function verifies that `from` has at least `amount` balance to
    /// transfer and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_ballance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or(ERR_INSUFFICIENT_BALANCE)?;

        let new_to_ballance =
            to_ballance.checked_add(amount).ok_or(ERR_OVERFLOW_BALANCE)?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_ballance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::u128;

    const ALICE_BALANCE: &str = "Alice";
    const BOB_BALANCE: &str = "Bob";

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 0);
        balances.set_balance(&ALICE_BALANCE.to_string(), 100);
        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 100);
        assert_eq!(balances.get_balance(&BOB_BALANCE.to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::new();

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
        let mut balances = super::Pallet::new();

        balances.set_balance(&ALICE_BALANCE.to_string(), 100);
        balances.set_balance(&BOB_BALANCE.to_string(), u128::MAX);

        let transfer_result = balances.transfer(
            ALICE_BALANCE.to_string(),
            BOB_BALANCE.to_string(),
            1,
        );

        assert_eq!(transfer_result, Err(super::ERR_OVERFLOW_BALANCE));
        assert_eq!(balances.get_balance(&ALICE_BALANCE.to_string()), 100);
        assert_eq!(balances.get_balance(&BOB_BALANCE.to_string()), u128::MAX);
    }

    #[test]
    fn insufficient_found_to_transfer() {
        let mut balances = super::Pallet::new();

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
