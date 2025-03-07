use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccoundId, T::Balance>,
}

impl<T> Pallet<T>
where
    T: Config,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccoundId, amount: T::Balance) {
        self.balances.insert(who.to_owned(), amount);
    }

    pub fn balance(&self, who: &T::AccoundId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccoundId,
        to: &T::AccoundId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller.clone(), new_caller_balance);
        self.balances.insert(to.clone(), new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestConfig;
    impl crate::system::Config for TestConfig {
        type AccoundId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = Pallet::<TestConfig>::new();
        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 51),
            Err("Not enough funds.")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 51),
            Ok(())
        );
        assert_eq!(balances.balance(&"alice".to_string()), 49);
        assert_eq!(balances.balance(&"bob".to_string()), 51);

        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 51),
            Err("Not enough funds.")
        );
    }
}
