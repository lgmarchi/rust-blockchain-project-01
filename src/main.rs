#![warn(
    // clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use support::Dispatch;
use utils::{
    AccountId,
    AccountIdentifier,
    Balance,
    BalancesConfig,
    BlockNumber,
    Nonce,
    RuntimeCall,
    SystemConfig,
};

mod balances;
mod declarative_marcros;
mod error_messages;
mod support;
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

    fn execute_block(
        &mut self,
        block: utils::Block,
    ) -> support::DispatchResult {
        self.system.increase_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in
            block.extrinsics.into_iter().enumerate()
        {
            self.system.increase_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!("Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}", block.header.block_number, i, e);
            });
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Self as AccountIdentifier>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice: String = String!("alice");
    let bob: String = String!("bob");
    let charlie: String = String!("charlie");

    runtime.balances.set_balance(&alice, 100);

    let block_1 = support::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    let block_2 = support::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob,
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Wrong block execution!");
    runtime.execute_block(block_2).expect("Wrong block execution!");

    println!("{runtime:#?}");
}
