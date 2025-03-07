use crate::support::Dispatch;
use system::Config;
mod balances;
mod support;
mod system;

mod types {
    use crate::{support, RuntimeCall};
    pub type AccoundId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccoundId, RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    BalanceTransfer {
        to: types::AccoundId,
        amount: types::Balance,
    },
}

impl system::Config for Runtime {
    type AccoundId = types::AccoundId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
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

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if self.system.block_number() != block.header.block_number {
            return Err("block number does not match what is expected");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }
        Ok(())
    }
}

impl support::Dispatch for Runtime {
    type Caller = <Runtime as Config>::AccoundId;
    type Call = RuntimeCall;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::BalanceTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            }
        }

        Ok(())
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
