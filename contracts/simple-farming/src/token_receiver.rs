use near_sdk::json_types::U128;
use near_sdk::{PromiseOrValue, env};

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use crate::*;


#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Callback on receiving tokens by this contract.
    /// transfer reward token with specific msg indicate
    /// which farm to be deposited to.
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let sender: AccountId = sender_id.into();
        let amount: u128 = amount.into();
        let seed_id = env::predecessor_account_id();
        if !msg.is_empty() {
            let msg_transfer = format!("{}:{}:{}", seed_id, amount, msg);
            self.internal_add_call(sender, msg_transfer);
        }
        PromiseOrValue::Value(U128(0))
    }
}

impl Contract {
    pub fn internal_add_call(&mut self, sender_id: String, call: String) {
        self.account_calls.insert(&sender_id, &call);
    } 
}