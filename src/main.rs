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
    Balances(balances::Call<Runtime>),
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
            self.system.inc_nonce(&caller);
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
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
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

    let block = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob,
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block).expect("invalid block");

    println!("{:#?}", runtime)
}
