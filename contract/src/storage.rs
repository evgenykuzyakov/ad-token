use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};

const MIN_STORAGE_BALANCE: U128 = U128(109 * env::STORAGE_PRICE_PER_BYTE);

#[allow(unused_variables)]
#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        assert!(env::attached_deposit() >= MIN_STORAGE_BALANCE.0);
        let account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);
        if self.account_ids.insert(&account_id) {
            self.num_accounts += 1;
        }
        StorageBalance {
            total: MIN_STORAGE_BALANCE,
            available: U128(0),
        }
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        env::panic(b"Can't withdraw");
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        env::panic(b"Can't withdraw");
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: MIN_STORAGE_BALANCE,
            max: Some(MIN_STORAGE_BALANCE),
        }
    }

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
        if self.account_ids.contains(account_id.as_ref()) {
            Some(StorageBalance {
                total: MIN_STORAGE_BALANCE,
                available: U128(0),
            })
        } else {
            None
        }
    }
}
