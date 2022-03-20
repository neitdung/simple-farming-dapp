use near_sdk::{env, ext_contract, Gas};
use near_contract_standards::storage_management::StorageBalance;
use crate::{FarmId};
pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;
pub const DENOM: u128 = 1_000_000_000_000_000_000_000_000;

#[ext_contract(ext_ft)]
trait FungibleToken {
    // change methods
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo: Option<String>);
    fn ft_transfer_call(&mut self, receiver_id: String, amount: String, memo: Option<String>, msg: String) -> U128;

    // view methods
    fn ft_total_supply(&self) -> String;
    fn ft_balance_of(&self, account_id: String) -> String;
    fn storage_balance_of(account_id: String) -> StorageBalance;
}

pub fn parse_farm_id(farm_id: &FarmId) -> (String, usize) {
    let v: Vec<&str> = farm_id.split("#").collect();
    if v.len() != 2 {
        env::panic(b"Farm id not found")
    }
    (v[0].to_string(), v[1].parse::<usize>().unwrap())
}