mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
	pub type BlockNumber = u32;
    pub type Nonce = u32;
}
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<types::AccountId, types::BlockNumber, types::Nonce>,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
    }
}

fn main() {
	let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    // start emulating a block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice.clone(), bob, 30)
        .map_err(|e| eprintln!("{}", e));

    // second transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));
	
	println!("{:#?}", runtime);
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

