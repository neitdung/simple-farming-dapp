use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, Timestamp};
use crate::SeedId;

pub(crate) type FarmId = String;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Status {
    Created, Running, Ended, Cleared
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
    pub reward_token: AccountId,
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
    pub(crate) new(
        owner_id: AccountId,
        farm_id: FarmId,
        terms: Terms
    ) -> Self {
        Self {
            owner_id: owner_id,
            farm_id: farm_id.clone(),
            terms: terms,
            status: Status::from("Created"),
            staking: 0,
            amount_of_claimed: 0,
            amount_of_reward: 0
        }
    }

    pub fn set_ended(&mut self, amount: Option<Balance>) {
        self.amount_of_reward = 0;
        self.amount_of_claimed += Some(amount);
        self.status = Status::from("Ended");
    }
}