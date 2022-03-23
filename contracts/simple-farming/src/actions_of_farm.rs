
use near_sdk::{env, near_bindgen, Promise, assert_one_yocto, Balance};
use near_sdk::json_types::{U64, U128};
use crate::farm::{Farm, Terms, FarmId, FarmInfo, Status};
use crate::farmer::{StakeInfo};
use crate::seed::{Seed};
use crate::utils::{DENOM, parse_farm_id};
use crate::*;


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn stake(&mut self, farm_id: FarmId) {
        let amount = env::attached_deposit();
        self.internal_stake_farm(amount, farm_id);
    }

    #[payable]
    pub fn withdraw(&mut self, farm_id: FarmId, amount: U128) {
        assert_one_yocto();
        self.internal_withdraw(amount.into(), &farm_id);
    }

    #[payable]
    pub fn create_farm(&mut self, start_at: U64, reward_per_session: U128, session_interval: U64) -> FarmId {        
        let prev_storage = env::storage_usage();
        let account_call = env::predecessor_account_id();
        if let Some(call_msg) = self.account_calls.get(&account_call) {
            let v: Vec<&str> = call_msg.split(":").collect();
            if v.len() == 3 && v[2] == "create_farm" {
                let terms = Terms {
                    seed_id: v[0].to_string(),
                    start_at: start_at.0,
                    reward_per_session: reward_per_session.0,
                    session_interval: session_interval.0
                };
                
                let farm_id = self.internal_add_farm(terms, v[1].parse::<u128>().unwrap());
        
                // Check how much storage cost and refund the left over back.
                let storage_needed = env::storage_usage() - prev_storage;
                let storage_cost = storage_needed as u128 * env::storage_byte_cost();
                assert!(
                    storage_cost <= env::attached_deposit(),
                    "{}: {}", "Need more storage", storage_needed
                );
        
                let refund = env::attached_deposit() - storage_cost;
                if refund > 0 {
                    Promise::new(env::predecessor_account_id()).transfer(refund);
                }
                self.account_calls.remove(&account_call);
                return farm_id;
            }
        }
        String::from("Not found call")
    }
    /// View methods.
    pub fn get_number_of_farms(&self) -> u64 {
        self.farms.len()
    }

    pub fn get_call(&self) -> Option<String> {
        let account_call = env::predecessor_account_id();
        self.account_calls.get(&account_call)
    }

    pub fn list_farms(&self, from_index: u64, limit: u64) -> Vec<FarmInfo> {
        let keys = self.farms.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| 
                (&self.farms.get(&keys.get(index).unwrap()).unwrap()).into()
            )
            .collect()
    }
}

impl Contract {
    fn internal_stake_farm(&mut self, amount: Balance, farm_id: FarmId) {
        let sender_id = env::predecessor_account_id();
        let new_staked_at = env::block_timestamp();
        let mut farmer = self.farmers.get(&sender_id).unwrap();
        let mut farm = self.farms.get(&farm_id).unwrap();
        assert!(
            farm.status != Status::Ended,
            "Farm is ended"
        );
        if let Some(mut stake_info) = farmer.staking.get(&farm_id) {
            let (seed_id, _) = parse_farm_id(&farm_id);
            if let Some(_seed) = self.seeds.get(&seed_id) {
                if new_staked_at > farm.terms.start_at {
                    let remain_amount = farm.amount_of_reward;
                    let mut will_claim_amount = stake_info.amount / DENOM;
                    will_claim_amount *= farm.terms.reward_per_session;
                    will_claim_amount *= (new_staked_at - stake_info.staked_at) as u128 / 10u128.pow(9);
                    will_claim_amount /= farm.terms.session_interval as u128;
                    env::log(
                        format!(
                            "Will claim: {}, staked: {}, reward per sesssion: {}, session_interval: {}",
                            will_claim_amount, stake_info.amount, farm.terms.reward_per_session, farm.terms.session_interval, 
                        )
                        .as_bytes(),
                    );
                    if remain_amount <= will_claim_amount {
                        self.internal_claim_user_reward(remain_amount, &sender_id, &seed_id);
                        farm.set_ended(Some(remain_amount));
                    } else if will_claim_amount > 0 {
                        self.internal_claim_user_reward(will_claim_amount, &sender_id, &seed_id);
                        stake_info.amount += amount;
                        farm.staking += amount;
                        farm.amount_of_claimed += will_claim_amount;
                        farm.amount_of_reward -= will_claim_amount;
                        stake_info.staked_at = new_staked_at;
                        farmer.staking.insert(&farm_id, &stake_info);
                    }
                } else {
                    stake_info.amount += amount;
                }
            }
        } else {
            let mut time_staked = new_staked_at;
            if new_staked_at > farm.terms.start_at {
                time_staked = new_staked_at;
            }
            let new_stake_info = StakeInfo {
                staked_at: time_staked,
                amount: amount
            };
            farmer.staking.insert(&farm_id, &new_stake_info);
        }
        self.farmers.insert(&sender_id, &farmer);
        self.farms.insert(&farm_id, &farm);
    }

    fn internal_withdraw(&mut self, amount: Balance, farm_id: &FarmId) {
        let sender_id: AccountId = env::predecessor_account_id();
        let new_staked_at = env::block_timestamp();
        let mut farmer = self.farmers.get(&sender_id).unwrap();
        let mut farm = self.farms.get(farm_id).unwrap();

        if let Some(mut stake_info) = farmer.staking.get(farm_id) {
            let (seed_id, _) = parse_farm_id(farm_id);
            assert!(amount <= stake_info.amount, "You can not withdraw more than your balance you staked in this farm");
            if let Some(_seed) = self.seeds.get(&seed_id) {
                if new_staked_at > farm.terms.start_at {
                    let remain_amount = farm.amount_of_reward;
                    let mut will_claim_amount = stake_info.amount / DENOM;
                    will_claim_amount *= farm.terms.reward_per_session;
                    will_claim_amount *= (new_staked_at - stake_info.staked_at) as u128 / 10u128.pow(9);
                    will_claim_amount /= farm.terms.session_interval as u128;
                    if remain_amount <= will_claim_amount {
                        self.internal_claim_user_reward(remain_amount, &sender_id, &seed_id);
                        farm.set_ended(Some(remain_amount));
                    } else if will_claim_amount > 0 {
                        self.internal_claim_user_reward(will_claim_amount, &sender_id, &seed_id);
                        stake_info.amount -= amount;
                        farm.staking -= amount;
                        farm.amount_of_claimed += will_claim_amount;
                        farm.amount_of_reward -= will_claim_amount;
                        stake_info.staked_at = new_staked_at;
                        farmer.staking.insert(farm_id, &stake_info);
                    }
                } else {
                    stake_info.amount -= amount;
                }
            }
            env::log(format!("amount: {}, receiver_id: {}", amount, sender_id.clone()).as_bytes());
            Promise::new(sender_id.clone()).transfer(amount);
        } else {
            env::panic(b"You're not stake on this farm");
        }
        self.farmers.insert(&sender_id, &farmer);
        self.farms.insert(farm_id, &farm);
    }

    fn internal_add_farm(&mut self, terms: Terms, amount: u128) -> FarmId {
        let mut seed: Seed;
        if let Some(s) = self.seeds.get(&terms.seed_id.clone()) {
            seed = s;
            env::log(
                format!(
                    "New farm created In seed {}",
                    terms.seed_id.clone()
                )
                .as_bytes(),
            );
        } else {
            seed = Seed::new(terms.seed_id.clone());
            env::log(
                format!(
                    "The first farm created In seed {}",
                    terms.seed_id.clone()
                )
                .as_bytes(),
            );
        }
        let seed_id = terms.seed_id.clone();
        let farm_id: FarmId = format!("{}#{}", &terms.seed_id.clone(), seed.next_index as usize);

        let farm = Farm::new(
            env::predecessor_account_id(),
            farm_id.clone(),
            terms,
            amount
        );
        
        seed.farms.insert(farm_id.clone());
        seed.next_index += 1;
        self.seeds.insert(&seed_id, &seed);
        self.farms.insert(&farm_id.clone(), &farm);
        farm_id
    }
}
