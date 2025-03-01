use utils::*;

mod balances;
mod error_messages;
mod system;
mod utils;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<AccountId, BlockNumber, Nonce>,
    balances: balances::Pallet<AccountId, Balance>,
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
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.increase_nonce(&alice);

    let _ = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime);

    let _ = runtime.system.get_nonce(&alice);

    // let mut balance = balances::Pallet::new();
    // let mut system = system::Pallet::new();
}
