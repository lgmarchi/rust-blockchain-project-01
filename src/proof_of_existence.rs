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

    pub fn revoke_claim(
        &mut self,
        caller: T::AccountId,
        claim: T::Content,
    ) -> DispatchResult {
        let claim_owner =
            self.get_claim(&claim).ok_or("Claim doest not exist")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::AccountIdentifier;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl AccountIdentifier for TestConfig {
        type AccountId = String;
    }

    impl crate::utils::SystemConfig for TestConfig {
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut proof_of_existence = super::Pallet::<TestConfig>::new();

        let _ = proof_of_existence
            .create_claim(String::from("Alice"), "my_documents");

        assert_eq!(
            proof_of_existence.get_claim(&"my_documents"),
            Some(&String::from("Alice"))
        );

        let _ = proof_of_existence
            .revoke_claim(String::from("Alice"), "my_documents");
        assert_eq!(proof_of_existence.get_claim(&"my_documents"), None);

        let _ = proof_of_existence
            .create_claim(String::from("Bob"), "my_documents");

        let res = proof_of_existence
            .create_claim(String::from("Charlie"), "my_documents");

        assert_eq!(res, Err("Claim already exist."));

        let res02 = proof_of_existence
            .revoke_claim(String::from("Alice"), "my_documents");
        assert_eq!(res02, Err("Caller is not the owner of the claim"));
    }
}
