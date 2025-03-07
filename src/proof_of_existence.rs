use std::{collections::BTreeMap, fmt::Debug};

use crate::support::{self, DispatchResult};

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccoundId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccoundId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccoundId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccoundId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.claims.get(&claim).ok_or("claim does not exist")?;
        if *claim_owner != caller {
            return Err("caller does not own claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> support::Dispatch for Pallet<T> {
    type Caller = T::AccoundId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::system;

    use super::*;

    struct TestConfig;
    impl Config for TestConfig {
        type Content = &'static str;
    }

    impl system::Config for TestConfig {
        type AccoundId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = Pallet::<TestConfig>::new();
        assert!(poe.claims.is_empty());
        assert_eq!(poe.get_claim(&"Hello, world!"), None);
        assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
        assert_eq!(
            poe.create_claim("bob", "Hello, world!"),
            Err("this content is already claimed")
        );
        assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
        assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
    }
}
