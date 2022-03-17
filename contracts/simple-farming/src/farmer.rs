use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};
use std::collections::HashMap;
use near_sdk::collections::LookupMap;
use crate::StorageKeys;
use crate::{SeedId, FarmId, RPS};


#[derive(BorshSerialize, BorshDeserialize)]
pub struct Farmer {
    amount: Balance,
    /// Amounts of various reward tokens the farmer claimed.
    rewards: HashMap<SeedId, Balance>,
    /// Amounts of various seed tokens the farmer staked.
    seeds: HashMap<SeedId, Balance>,
    /// record user_last_rps of farms
    user_rps: LookupMap<FarmId, RPS>,
    rps_count: u32
}

impl Farmer {
    pub fn new(farmer_id: AccountId, amount: Balance) -> Self {
        Self {
            amount: amount,
            rewards: HashMap::new(),
            seeds: HashMap::new(),
            user_rps: LookupMap::new(StorageKeys::UserRPS {
                account_id: farmer_id.clone(),
            }),
            rps_count: 0,
        })
    }
}
