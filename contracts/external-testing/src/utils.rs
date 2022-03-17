use near_sdk::{ext_contract, Gas};
use near_contract_standards::storage_management::StorageBalance;
pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;
pub const MIN_SEED_DEPOSIT: u128 = 1_000_000_000_000_000_000;
pub type TimestampSec = u32;

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    // change methods
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    fn ft_transfer_call(&mut self, receiver_id: String, amount: String, memo: Option<String>, msg: String) -> U128;
    fn storage_deposit(
        account_id: String,
        registration_only: bool
    ) -> StorageBalance;
    // view methods
    fn ft_total_supply(&self) -> String;
    fn ft_balance_of(&self, account_id: String) -> String;
    fn storage_balance_of(account_id: String) -> StorageBalance;
}