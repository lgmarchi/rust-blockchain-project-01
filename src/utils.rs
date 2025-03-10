use std::fmt::Debug;

use num::{
    CheckedAdd,
    One,
    Zero,
};

use crate::{
    Runtime,
    balances,
    proof_of_existence,
    support,
};

pub type AccountId = String;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Balance = u128;
pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;
pub type Content = &'static str;

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

pub trait AccountIdentifier {
    type AccountId: Ord + Clone + Debug;
}

pub trait SystemConfig: AccountIdentifier {
    type BlockNumber: Ord + One + Clone + Copy + Zero + CheckedAdd + Debug;
    type Nonce: Ord + Clone + Copy + One + Zero + std::ops::AddAssign + Debug;
}
