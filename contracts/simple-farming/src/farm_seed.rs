use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};
use std::collections::HashSet;
use crate::farm::FarmId;

pub(crate) type SeedId = AccountId;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Seed {
    seed_id: SeedId,
    farms: HashSet<FarmId>,
    next_index: u32,
    amount: Balance,
    min_deposit: Balance
}

impl Seed {
    pub fn new(seed_id: SeedId, min_deposit: Balance) -> Self {
        Self {
            seed_id: seed_id,
            farms: HashSet::new(),
            next_index: 0,
            amount: 0,
            min_deposit: min_deposit
        }
    }
}
