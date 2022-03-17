use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, Promise, AccountId, Gas};
use crate::utils::{ext_ft};
near_sdk::setup_alloc!();

mod utils;
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct StatusMessage {
    owner_id: AccountId,
}
pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            owner_id: String::from("external-testing.dungnt.testnet")
        }
    }
}

#[near_bindgen]
impl StatusMessage {

    #[payable]
    pub fn ft_deposit(&mut self) -> Promise {
        assert!(env::attached_deposit() > 1250000000000000000001, "You need attach more yocto");

        ext_ft::storage_deposit(
            self.owner_id.clone(),
            true,
            &"ft.luftmensch.testnet",
            1250000000000000000000,  // one yocto near
            GAS_FOR_FT_TRANSFER,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
