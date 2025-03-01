use std::collections::BTreeMap;

use crate::utils::*;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn increase_block_number(&mut self) {
        if let Some(_) = self.block_number.checked_add(1) {
            self.block_number = self.block_number + 1
        }
    }

    // Increment the nonce of an account. This helps us keep track of how many
    // transactions account has made.
    pub fn increase_nonce(&mut self, who: &AccountId) {
        // let account = self.nonce.get_mut(who);
        // if let Some(nonce_number) = account {
        //     *nonce_number += 1;
        // }

        // let nonce = self.nonce.get(who).unwrap_or(&0);
        // self.nonce.insert(who.clone(), nonce + 1);

        // Short version
        *self.nonce.entry(who.to_string()).or_insert(0) += 1;
    }

    fn get_nonce(&self, who: &AccountId) -> Nonce {
        let default_nonce = 0;
        let nonce = self.nonce.get(who).unwrap_or(&default_nonce);
        *nonce
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let system = super::Pallet::new();
        assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn increase_block_number() {
        let mut system = super::Pallet::new();
        system.increase_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn increase_nonce() {
        let mut system = super::Pallet::new();
        let alice = &String::from("Alice");
        system.increase_nonce(alice);
        assert_eq!(system.get_nonce(alice), 1);
    }
}
