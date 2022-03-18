
use near_sdk::{env, near_bindgen, Promise, assert_one_yocto};
use near_sdk::json_types::{U128};
use crate::farm::{Farm, Terms};
use crate::farm_seed::{Seed};
use crate::errors::*;
use crate::utils::{MIN_SEED_DEPOSIT, DENOM};


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn stake(&mut self, farm_id: FarmId) {
        let amount = env::attached_deposit();
        assert!(
            amount > self.farms.get(farm_id).terms.min_deposit,
            "You need attach more than min deposit of this farm term."
        );
        
        self.internal_stake_farm(amount.into() - 1, farm_id.clone());
    }

    #[payable]
    pub fn withdraw(&mut self, farm_id: FarmId, amount: U128) {
        assert_one_yocto();
        self.internal_withdraw(amount.into(), farm_id.clone());
    }
    /// View methods.
    pub fn get_number_of_farms(&self) -> u64 {
        self.farms.len()
    }

    pub fn get_number_of_outdated_farms(&self) -> u64 {
        self.outdated_farms.len()
    }

    // pub fn list_farms(&self, from_index: u64, limit: u64) -> Vec<Farm> {
    //     let keys = self.farms.keys_as_vector();

    //     (from_index..std::cmp::min(from_index + limit, keys.len()))
    //         .map(|index| 
    //             (&self.farms.get(&keys.get(index).unwrap())
    //         )
    //         .collect()
    // }

    // pub fn list_outdated_farms(&self, from_index: u64, limit: u64) -> Vec<Farm> {
    //     let keys = self.outdated_farms.keys_as_vector();

    //     (from_index..std::cmp::min(from_index + limit, keys.len()))
    //         .map(|index|
    //             (&self.outdated_farms.get(&keys.get(index).unwrap())
    //         )
    //         .collect()
    // }
}

impl Contract {
    pub(crate) fn create_farm(&mut self, terms: Terms, min_deposit: Option<U128>) -> FarmId {        
        let prev_storage = env::storage_usage();

        let min_deposit: u128 = min_deposit.unwrap_or(U128(MIN_SEED_DEPOSIT)).0;

        let farm_id = self.internal_add_farm(&terms, min_deposit);

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

        farm_id
    }

    fn internal_stake_farm(&mut self, amount: Balance, farm_id: &FarmId) {
        let sender_id = env::predecessor_account_id();
        let new_staked_at = env::block_timestamp();
        let mut farmer = self.farmers.get(sender_id);
        let mut farm = self.farms.get(farm_id);

        if let Some(mut stake_info) = farmer.staking.get(farm_id) {
            let (seed_id, _) = parse_farm_id(farm_id);
            if let Some(seed) = self.seeds.get(&seed_id) {
                if new_staked_at > farm.staked_at {
                    let remain_amount = farm.amount_of_reward;
                    let will_claim_amount = (new_staked_at - stake_info.staked_at) * farm.terms.reward_per_session * stake_info.amount /(DENOM * farm.terms.session_interval);
                    
                    if (remain_amount <= will_claim_amount) {
                        self.internal_claim_user_reward(remain_amount, sender_id, seed_id);
                        farm.set_ended(remain_amount);
                    } else {
                        self.internal_claim_user_reward(will_claim_amount, sender_id, seed_id);
                        stake_info.amount += amount;
                        farm.staking += amount;
                        farm.amount_of_claimed += will_claim_amount;
                        farm.amount_of_reward -= will_claim_amount;
                        stake_info.staked_at = new_staked_at;
                        farmer.staking.insert(farm_id, stake_info);
                    }
                } else {
                    stake_info.amount += amount;
                }
            }
        } else {
            let new_stake_info = StakeInfo {
                staked_at:  ? new_staked_at : farm.staked_at;
                amount: amount;
            }
        }
        self.farmers.insert(sender_id, farmer);
        self.farms.insert(farm_id, farm);
    }

    fn internal_withdraw(&mut self, amount: Balance, farm_id: &FarmId) {
        let sender_id = env::predecessor_account_id();
        let new_staked_at = env::block_timestamp();
        let mut farmer = self.farmers.get(sender_id);
        let mut farm = self.farms.get(farm_id);

        if let Some(mut stake_info) = farmer.staking.get(farm_id) {
            let (seed_id, _) = parse_farm_id(farm_id);
            if let Some(seed) = self.seeds.get(&seed_id) {
                assert!(amount <= stake_info.amount, "You can not withdraw more than your balance you staked in this farm");
                if new_staked_at > farm.staked_at {
                    let remain_amount = farm.amount_of_reward;
                    let will_claim_amount = (new_staked_at - stake_info.staked_at) * farm.terms.reward_per_session * stake_info.amount /(DENOM * farm.terms.session_interval);
                    
                    if (remain_amount <= will_claim_amount) {
                        self.internal_claim_user_reward(remain_amount, sender_id, seed_id);
                        farm.set_ended(remain_amount);
                    } else {
                        self.internal_claim_user_reward(will_claim_amount, sender_id, seed_id);
                        stake_info.amount -= amount;
                        farm.staking -= amount;
                        farm.amount_of_claimed += will_claim_amount;
                        farm.amount_of_reward -= will_claim_amount;
                        stake_info.staked_at = new_staked_at;
                        farmer.staking.insert(farm_id, stake_info);
                    }
                } else {
                    stake_info.amount -= amount;
                }
                Promise::new(sender_id).transfer(amount);
            }
        } else {
            env::panic("You're not stake on this farm");
        }
        self.farmers.insert(sender_id, farmer);
        self.farms.insert(farm_id, farm);
    }

    fn internal_add_farm(&mut self, terms: &Terms, min_deposit: Balance) -> FarmId {
        let mut seed: Seed;
        if let Some(s) = self.seeds.get(&terms.seed_id) {
            seed = s;
            env::log(
                format!(
                    "New farm created In seed {}, with existed min_deposit {}",
                    terms.seed_id, seed.min_deposit
                )
                .as_bytes(),
            );
        } else {
            seed = Seed::new(&terms.seed_id);
            env::log(
                format!(
                    "The first farm created In seed {}, with min_deposit {}",
                    terms.seed_id, min_deposit
                )
                .as_bytes(),
            );
        }

        let farm_id: FarmId = format!("{}{}{}", &terms.seed_id, INDEX_HASHTAG,farm_seed.get_ref().next_index as usize);

        let farm = Farm::new(
            farm_id.clone(),
            terms.into(),
        );
        
        seed.farms.insert(farm_id.clone());
        seed.next_index += 1;
        self.seeds.insert(&terms.seed_id, &farm_seed);
        self.farms.insert(&farm_id.clone(), &farm);
        farm_id
    }
}
