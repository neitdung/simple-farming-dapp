
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, Promise};
use crate::utils::{ext_ft, GAS_FOR_FT_TRANSFER};
use crate::actions_of_farm::*;
use crate::actions_of_reward::*;
use crate::actions_of_seed::*;
use crate::farm_seed::*;
use crate::farmer::*;
use crate::farm::*;
near_sdk::setup_alloc!();

mod actions_of_farm;
mod actions_of_reward;
mod actions_of_seed;
mod farm_seed;
mod farmer;
mod farm;
mod utils;

pub(crate) const INDEX_HASHTAG = "#";

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
    outdated_farms: UnorderedMap<FarmId, Farm>,

    // for statistic
    farmer_count: u64
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Farm,
    OutdatedFarm,
    Seed,
    Farmer,
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
            outdated_farms: UnorderedMap::new(StorageKeys::OutdatedFarm)
        }
    }
}