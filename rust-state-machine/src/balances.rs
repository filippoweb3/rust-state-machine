use std::collections::BTreeMap;
use num::Zero;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + Copy + std::ops::Add<Output = Balance> + std::ops::Sub<Output = Balance> + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        if caller_balance < amount {
            return Err("Not enough funds.");
        }

        let new_caller_balance = caller_balance - amount;
        let new_to_balance = self.balance(&to) + amount;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type AccountId = String;
    type Balance = u128;

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<AccountId, Balance>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        // Ensure transfer fails when Alice has no funds
        assert_eq!(balances.transfer(alice.clone(), bob.clone(), 51), Err("Not enough funds."));

        // Set Alice's balance and retry
        balances.set_balance(&alice, 100);
        assert_eq!(balances.transfer(alice.clone(), bob.clone(), 51), Ok(()));
        assert_eq!(balances.balance(&alice), 49);
        assert_eq!(balances.balance(&bob), 51);

        // Ensure Alice can't transfer more than she has
        assert_eq!(balances.transfer(alice.clone(), bob.clone(), 51), Err("Not enough funds."));
    }
}

