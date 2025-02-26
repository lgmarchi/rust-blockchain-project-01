use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
        unimplemented!();
    }

    pub fn get_balance(self, who: &String) -> u128 {
        let balance = self.balances.get(who).map(|f| *f).unwrap_or(0);
        balance
    }
}
