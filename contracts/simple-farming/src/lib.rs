
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{near_bindgen, PanicOnDefault, AccountId, BorshStorageKey, Promise, Gas, env};
use near_sdk::json_types::{ValidAccountId};
use crate::seed::*;
use crate::farmer::*;
use crate::farm::*;
use crate::utils::{ext_ft};
near_sdk::setup_alloc!();

mod actions_of_farm;
mod actions_of_reward;
mod seed;
mod farmer;
mod farm;
mod utils;
mod token_receiver;
mod storage_impl;
pub const GAS_FOR_FT_DEPOSIT: Gas = 10_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // owner of this contract
    owner_id: AccountId,
    
    // record seeds and the farms under it.
    // seeds: UnorderedMap<SeedId, FarmSeed>,
    seeds: UnorderedMap<SeedId, Seed>,

    // each farmer has a structure to describe
    // farmers: LookupMap<AccountId, Farmer>,
    farmers: LookupMap<AccountId, Farmer>,

    farms: UnorderedMap<FarmId, Farm>,
    account_calls: UnorderedMap<AccountId, String>,
    // for statistic
    farmer_count: u64
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Farm,
    Seed,
    Farmer,
    AccountCall,
    FarmerClaimed { account_id: AccountId },
    FarmerStaking { account_id: AccountId }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        Self {
            owner_id: owner_id.into(),
            farmer_count: 0,
            seeds: UnorderedMap::new(StorageKeys::Seed),
            farmers: LookupMap::new(StorageKeys::Farmer),
            farms: UnorderedMap::new(StorageKeys::Farm),
            account_calls: UnorderedMap::new(StorageKeys::AccountCall)
        }
    }

    #[payable]
    pub fn ft_deposit(&mut self, ft_account: ValidAccountId) -> Promise {
        assert!(env::attached_deposit() >= 1250000000000000000000, "You need attach more yocto");
        ext_ft::storage_deposit(
            self.owner_id.clone(),
            true,
            &ft_account,
            1250000000000000000000,
            GAS_FOR_FT_DEPOSIT,
        )
    }

    pub fn view_farmer_exists(&self, account_id: AccountId) -> bool {
        if let Some(farmer) = self.farmers.get(&account_id) {
            return true;
        }
        return false;
    }
}