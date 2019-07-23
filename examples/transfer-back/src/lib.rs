//! A simple smart contract that sends half of the PERLs it receives
//! back to its respective sender.
//!
//! Overall a simple example of registering a function to get called when
//! the smart contract receives PERLs, and on how to create and send PERLs
//! to a provided destination wallet address.
use smart_contract::payload::Parameters;
use smart_contract::transaction::{Transaction, Transfer};
use smart_contract_macros::smart_contract;

pub struct Contract;

#[smart_contract]
impl Contract {
    fn init(_params: &mut Parameters) -> Self {
        Self {}
    }

    fn on_money_received(&mut self, params: &mut Parameters) -> Result<(), String> {
        // Create and send transaction.
        Transfer {
            destination: params.sender,
            amount: (params.amount + 1) / 2,
            invocation: None,
        }
        .send_transaction();

        Ok(())
    }
}
