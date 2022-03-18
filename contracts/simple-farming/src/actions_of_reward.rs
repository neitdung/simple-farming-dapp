use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{Promise};

use crate::utils::{ext_ft, GAS_FOR_FT_TRANSFER};
use crate::errors::*;
use crate::*;

impl Contract {
    pub(crate) fn internal_claim_user_reward(
        &mut self,
        amount: Balance,
        sender_id: &AccountId, 
        seed_id: &SeedId,
    ) -> Promise {
        ext_ft::ft_transfer(
            sender_id,
            U128(amount),
            seed_id,
            None,
            1,
            GAS_FOR_FT_TRANSFER
        )
    }
}
