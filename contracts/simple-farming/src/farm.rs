use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::{AccountId, Balance, Timestamp};
use crate::SeedId;

pub(crate) type FarmId = String;

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub enum Status {
    Created, Running, Ended
}

impl From<&Status> for String {
    fn from(status: &Status) -> Self {
        match *status {
            Status::Created => { String::from("Created") },
            Status::Running => { String::from("Running") },
            Status::Ended => { String::from("Ended") }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Terms {
    pub seed_id: SeedId,
    pub start_at: Timestamp,
    pub reward_per_session: Balance,
    pub session_interval: Timestamp,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Farm {
    pub owner_id: AccountId,
    pub farm_id: FarmId,
    pub terms: Terms,
    pub status: Status,
    pub staking: Balance,
    pub amount_of_reward: Balance,
    pub amount_of_claimed: Balance,
}

impl Farm {
    pub(crate) fn new(
        owner_id: AccountId,
        farm_id: FarmId,
        terms: Terms,
        amount_of_reward: Balance
    ) -> Self {
        Self {
            owner_id: owner_id,
            farm_id: farm_id.clone(),
            terms: terms,
            status: Status::Created,
            staking: 0,
            amount_of_claimed: 0,
            amount_of_reward: amount_of_reward
        }
    }

    pub fn set_ended(&mut self, amount: Option<Balance>) {
        self.amount_of_reward = 0;
        self.amount_of_claimed += amount.unwrap_or_else(|| 0);
        self.status = Status::Ended;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FarmInfo {
    pub farm_id: FarmId,
    pub farm_status: String,
    pub seed_id: SeedId,
    pub start_at: u64,
    pub reward_per_session: U128,
    pub session_interval: u64,

    pub total_reward: U128,
    pub claimed_reward: U128,
}

impl From<&Farm> for FarmInfo {
    fn from(farm: &Farm) -> Self {
        Self {
            farm_id: farm.farm_id.clone(),
            farm_status: (&farm.status).into(),
            seed_id: farm.terms.seed_id.clone(),
            start_at: farm.terms.start_at.into(),
            reward_per_session: farm.terms.reward_per_session.into(),
            session_interval: farm.terms.session_interval.into(),
            total_reward: farm.amount_of_reward.into(),
            claimed_reward: farm.amount_of_claimed.into(),
        }
    }
}
