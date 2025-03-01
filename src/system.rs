use std::{
    collections::BTreeMap,
    fmt::Debug,
};

use num::{
    CheckedAdd,
    One,
    Zero,
};

pub trait Config {
    type AccountId: Ord + Clone + Debug;
    type BlockNumber: Ord + One + Clone + Copy + Zero + CheckedAdd + Debug;
    type Nonce: Ord + Clone + Copy + One + Zero + std::ops::AddAssign + Debug;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn increase_block_number(&mut self) {
        if let Some(_) = self.block_number.checked_add(&T::BlockNumber::one()) {
            self.block_number = self.block_number + T::BlockNumber::one()
        }
    }

    // Increment the nonce of an account. This helps us keep track of how many
    // transactions account has made.
    pub fn increase_nonce(&mut self, who: &T::AccountId) {
        // let account = self.nonce.get_mut(who);
        // if let Some(nonce_number) = account {
        //     *nonce_number += 1;
        // }

        // let nonce = self.nonce.get(who).unwrap_or(&0);
        // self.nonce.insert(who.clone(), nonce + 1);

        // Short version
        *self.nonce.entry(who.clone()).or_insert(T::Nonce::zero()) +=
            T::Nonce::one();
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        let default_nonce = T::Nonce::zero();
        let nonce = self.nonce.get(who).unwrap_or(&default_nonce);
        *nonce
    }
}

#[cfg(test)]
mod test {
    use crate::system::{
        Config,
        Pallet,
    };

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn increase_block_number() {
        let mut system: Pallet<TestConfig> = super::Pallet::new();
        system.increase_block_number();
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn increase_nonce() {
        let mut system: Pallet<TestConfig> = super::Pallet::new();
        let alice = &String::from("Alice");
        system.increase_nonce(alice);
        assert_eq!(system.get_nonce(alice), 1);
    }
}
