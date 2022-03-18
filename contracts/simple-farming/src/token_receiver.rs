use crate::farm::Terms;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

/// Message parameters to receive via token function call.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct TokenReceiverMessage {
    action: String,
    terms: Option<Terms>, 
    min_deposit: Option<U128>
}

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
        if !msg.is_empty() {
            let message = serde_json::from_str::<TokenReceiverMessage>(&msg).expect("Wrong formeat!!!");
            match message.action.as_str() {
                "create_farm" => self.create_farm(message.terms.into(), message.min_deposit),
            }
            message.action
        }
        String::from("No action found")
    }
}