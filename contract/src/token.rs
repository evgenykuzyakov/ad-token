use crate::*;

use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_contract_standards::fungible_token::core_impl::{
    ext_fungible_token, ext_fungible_token_receiver,
};

const NO_DEPOSIT: Balance = 0;
const ONE: Balance = 1 * 10u128.pow(18);

const TGAS: Gas = 10u64.pow(12);
const GAS_FOR_RESOLVE_TRANSFER: Gas = 5 * TGAS;
const GAS_FOR_FT_TRANSFER_CALL: Gas = 25 * TGAS + GAS_FOR_RESOLVE_TRANSFER;

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[allow(unused_variables)]
    #[payable]
    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: U128, memo: Option<String>) {
        assert_one_yocto();
        assert!(
            self.account_ids.contains(receiver_id.as_ref()),
            "Not registered"
        );
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }

    #[allow(unused_variables)]
    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_one_yocto();
        assert!(
            self.account_ids.contains(receiver_id.as_ref()),
            "Not registered"
        );
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
        ext_fungible_token_receiver::ft_on_transfer(
            env::predecessor_account_id(),
            amount.into(),
            msg,
            receiver_id.as_ref(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
        )
        .then(ext_fungible_token::ft_balance_of(
            receiver_id.into(),
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }

    fn ft_total_supply(&self) -> U128 {
        U128(Balance::from(self.num_accounts) * ONE)
    }

    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128 {
        if self.account_ids.contains(account_id.as_ref()) {
            U128(ONE)
        } else {
            U128(0)
        }
    }
}
