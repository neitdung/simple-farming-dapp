use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use std::collections::HashSet;
use crate::farm::FarmId;

pub(crate) type SeedId = AccountId;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Seed {
    pub seed_id: SeedId,
    pub farms: HashSet<FarmId>,
    pub next_index: u32,
}

impl Seed {
    pub fn new(seed_id: SeedId) -> Self {
        Self {
            seed_id: seed_id,
            farms: HashSet::new(),
            next_index: 0
        }
    }
}
