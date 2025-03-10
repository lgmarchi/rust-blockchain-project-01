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
    BlockNumber,
    Content,
    Nonce,
    PoeConfig,
    RuntimeCall,
    SystemConfig,
};

mod balances;
mod declarative_macros;
mod error_messages;
mod proof_of_existence;
mod support;
mod system;
mod utils;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
}

impl AccountIdentifier for Runtime {
    type AccountId = AccountId;
}

impl SystemConfig for Runtime {
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl balances::Config for Runtime {
    type Balance = Balance;
}

impl PoeConfig for Runtime {
    type Content = Content;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
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
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let lucas: String = String!("Lucas");
    let matheus: String = String!("Matheus");
    let marcos: String = String!("Marcos");

    runtime.balances.set_balance(&lucas, 100);
    let block_1 = support::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: lucas.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: matheus.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: lucas.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: marcos.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    let block_2 = support::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: lucas.clone(),
                call: RuntimeCall::ProofOfExistence(
                    proof_of_existence::Call::CreateClaim {
                        claim: "lucas_document",
                    },
                ),
            },
            support::Extrinsic {
                caller: matheus.clone(),
                call: RuntimeCall::ProofOfExistence(
                    proof_of_existence::Call::CreateClaim {
                        claim: "matheus_document",
                    },
                ),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Wrong block execution!");
    runtime.execute_block(block_2).expect("Wrong block execution!");

    println!("{runtime:#?}");
}
