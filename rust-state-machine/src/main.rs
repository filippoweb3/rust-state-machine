mod balances;

fn main() {
	println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::balances;

    #[test]
    fn init_balances() {
        let mut balances = balances::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = balances::Pallet::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Not enough funds.")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
        assert_eq!(balances.balance(&"alice".to_string()), 49);
        assert_eq!(balances.balance(&"bob".to_string()), 51);

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Not enough funds.")
        );
    }
}

