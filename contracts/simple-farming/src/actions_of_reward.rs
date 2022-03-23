use std::convert::TryInto;
use near_sdk::{Promise, Balance};

use crate::utils::{ext_ft, GAS_FOR_FT_TRANSFER};
use crate::*;

impl Contract {
    pub(crate) fn internal_claim_user_reward(
        &mut self,
        amount: Balance,
        sender_id: &AccountId, 
        seed_id: &SeedId,
    ) -> Promise {
        ext_ft::ft_transfer(
            sender_id.clone().try_into().unwrap(),
            amount.to_string(),
            None,            
            seed_id,
            1,
            GAS_FOR_FT_TRANSFER
        )
    }
}
