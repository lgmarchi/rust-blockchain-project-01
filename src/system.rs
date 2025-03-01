use std::collections::BTreeMap;

use num::{
    CheckedAdd,
    One,
    Zero,
};

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone + ToString,
    BlockNumber: Ord + One + Clone + Copy + Zero + CheckedAdd,
    Nonce: Ord + Clone + Copy + One + Zero + std::ops::AddAssign,
{
    pub fn new() -> Self {
        Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn increase_block_number(&mut self) {
        if let Some(_) = self.block_number.checked_add(&BlockNumber::one()) {
            self.block_number = self.block_number + BlockNumber::one()
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
        *self.nonce.entry(who.clone()).or_insert(Nonce::zero()) += Nonce::one();
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        let default_nonce = Nonce::zero();
        let nonce = self.nonce.get(who).unwrap_or(&default_nonce);
        *nonce
    }
}

#[cfg(test)]
mod test {
    use crate::{
        system::Pallet,
        utils::{
            AccountId,
            BlockNumber,
            Nonce,
        },
    };

    #[test]
    fn init_system() {
        let system: Pallet<AccountId, BlockNumber, Nonce> =
            super::Pallet::new();
        assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn increase_block_number() {
        let mut system: Pallet<AccountId, BlockNumber, Nonce> =
            super::Pallet::new();
        system.increase_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn increase_nonce() {
        let mut system: Pallet<AccountId, BlockNumber, Nonce> =
            super::Pallet::new();
        let alice = &String::from("Alice");
        system.increase_nonce(alice);
        assert_eq!(system.get_nonce(alice), 1);
    }
}
