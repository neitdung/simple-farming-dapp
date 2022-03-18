use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, Timestamp};
use near_sdk::collections::UnorderedMap;
use crate::StorageKeys;
use crate::{SeedId, FarmId};


#[derive(BorshSerialize, BorshDeserialize)]
pub struct StakeInfo {
    pub staked_at: Timestamp,
    pub amount: Balance,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Farmer {
    claimed: UnorderedMap<SeedId, Balance>,
    staking: UnorderedMap<FarmId, StakeInfo>
}

impl Farmer {
    pub fn new(farmer_id: AccountId) -> Self {
        Self {
            claimed: UnorderedMap::new(StorageKeys::FarmerClaimed {
                account_id: farmer_id.clone(),
            }),
            staking: UnorderedMap::new(StorageKeys::FarmerStaking {
                account_id: farmer_id.clone()
            }),
        }
    }
}
