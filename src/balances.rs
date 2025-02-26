use std::collections::BTreeMap;

#[derive(Clone)]
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

    pub fn get_balance(&mut self, who: &String) -> u128 {
        let balance = self.clone().balances.get(who).map(|f| *f).unwrap_or(0);
        balance
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.get_balance(&"Alice".to_string()), 0);
        balances.set_balance(&"Alice".to_string(), 100);
        assert_eq!(balances.get_balance(&"Alice".to_string()), 100);
        assert_eq!(balances.get_balance(&"Bob".to_string()), 0);
    }
}
