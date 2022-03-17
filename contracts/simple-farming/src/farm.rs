use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};
use crate::SeedId;

pub type RPS = [u8; 32];

pub(crate) type FarmId = String;

pub const DENOM: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Status {
    Created, Running, Ended, Cleared
}

impl From<&Status> for String {
    fn from(status: &Status) -> Self {
        match *status {
            Status::Created => { String::from("Created") },
            Status::Running => { String::from("Running") },
            Status::Ended => { String::from("Ended") },
            Status::Cleared => { String::from("Cleared") },
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Terms {
    pub seed_id: SeedId,
    pub reward_token: AccountId,
    pub start_at: TimestampSec,
    pub reward_per_session: Balance,
    pub session_interval: TimestampSec,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Default)]
pub struct RewardDistribution {
    pub undistributed: Balance,
    pub unclaimed: Balance,
    pub rps: RPS,
    pub rr: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Farm {
    pub owner_id: AccountId
    pub farm_id: FarmId,
    pub terms: Terms,
    pub status: Status,
    pub last_distribution: RewardDistribution,
    pub amount_of_reward: Balance,
    pub amount_of_claimed: Balance,
    pub amount_of_beneficiary: Balance,
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
            last_distribution: RewardDistribution::default(),
            amount_of_claimed: 0,
            amount_of_reward: 0,
            amount_of_beneficiary: 0
        }
    }
}