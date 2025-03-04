#![warn(
    // clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use utils::{
    AccountId,
    AccountIdentifier,
    Balance,
    BalancesConfig,
    BlockNumber,
    Nonce,
    SystemConfig,
};

mod balances;
mod error_messages;
mod system;
mod utils;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl AccountIdentifier for Runtime {
    type AccountId = AccountId;
}

impl SystemConfig for Runtime {
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl BalancesConfig for Runtime {
    type Balance = Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.increase_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.increase_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), bob, 30)
        .map_err(|e| println!("Error: {e:?}"));

    runtime.system.increase_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), charlie, 20)
        .map_err(|e| println!("Error: {e:?}"));

    println!("{runtime:#?}");

    let _ = runtime.system.get_nonce(&alice);

    // let mut balance = balances::Pallet::new();
    // let mut system = system::Pallet::new();
}
