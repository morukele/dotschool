mod balances;
mod system;

impl system::Config for Runtime {
    type AccoundId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::<Self>::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&alice, 100);

    // Block emulation
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // First transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(&alice, &bob, 30)
        .map_err(|e| eprint!("{}", e));

    // Second transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime)
}
