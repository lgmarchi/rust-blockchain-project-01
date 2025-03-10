use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::utils::SystemConfig {
    /// The type which represents the content that can be claimed using this
    /// pallet. Could be the content directly as bytes, or better yet the
    /// hash of that content. We leave that decision to the runtime
    /// developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only
    /// have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> DispatchResult {
        if self.get_claim(&claim).is_some() {
            Err("Claim already exist.")
        } else {
            self.claims.insert(claim, caller);
            Ok(())
        }
    }
}
