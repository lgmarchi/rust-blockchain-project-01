use std::fmt::Debug;

use num::{
    CheckedAdd,
    CheckedSub,
    One,
    Zero,
};

use crate::support;

pub type AccountId = String;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Balance = u128;
pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
pub type Header = support::Header<BlockNumber>;
pub type Block = support::Block<Header, Extrinsic>;

pub enum RuntimeCall {}

pub trait AccountIdentifier {
    type AccountId: Ord + Clone + Debug;
}

pub trait SystemConfig: AccountIdentifier {
    type BlockNumber: Ord + One + Clone + Copy + Zero + CheckedAdd + Debug;
    type Nonce: Ord + Clone + Copy + One + Zero + std::ops::AddAssign + Debug;
}

pub trait BalancesConfig: AccountIdentifier {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Debug;
}
