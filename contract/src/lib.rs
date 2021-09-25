mod storage;
mod token;

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    assert_one_yocto, env, log, near_bindgen, AccountId, Balance, BorshStorageKey, Gas,
    PanicOnDefault, PromiseOrValue,
};

const MIN_DEPOSIT: Balance = 10u128.pow(24);
const MAX_LIMIT: u32 = 10;

near_sdk::setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Accounts,
    FtMeta,
    Metas,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub account_ids: LookupSet<AccountId>,
    pub num_accounts: u32,
    pub meta: LazyOption<FungibleTokenMetadata>,
    pub metas: LookupMap<u32, FungibleTokenMetadata>,
    pub first_meta: u32,
    pub next_meta: u32,
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let number = self.random_meta_id();
        if number == 2 {
            self.meta.get().unwrap()
        } else {
            self.metas.get(&number).unwrap()
        }
    }
}

impl Contract {
    pub(crate) fn random_meta_id(&self) -> u32 {
        let num_metas = self.next_meta - self.first_meta + 1;
        let number = (env::block_index() % (num_metas as u64)) as u32;
        if number == num_metas - 1 {
            2
        } else {
            number + self.first_meta
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            account_ids: LookupSet::new(StorageKey::Accounts),
            num_accounts: 0,
            meta: LazyOption::new(StorageKey::FtMeta, None),
            metas: LookupMap::new(StorageKey::Metas),
            first_meta: 3,
            next_meta: 3,
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn upgrade() -> Self {
        #[derive(BorshDeserialize)]
        pub struct OldContract {
            pub account_ids: LookupSet<AccountId>,
            pub num_accounts: u32,
            pub meta: LazyOption<FungibleTokenMetadata>,
        }

        let OldContract {
            account_ids,
            num_accounts,
            meta,
        } = env::state_read().unwrap();

        Self {
            account_ids,
            num_accounts,
            meta,
            metas: LookupMap::new(StorageKey::Metas),
            first_meta: 3,
            next_meta: 3,
        }
    }

    #[payable]
    pub fn post_ad(&mut self, title: String, icon: Option<String>) {
        assert!(
            env::predecessor_account_id() == env::current_account_id()
                || env::attached_deposit() >= MIN_DEPOSIT
        );
        let meta = self.meta.get().unwrap();
        self.metas.insert(
            &self.next_meta,
            &FungibleTokenMetadata {
                spec: meta.spec,
                name: format!("AD#{}: {}", self.next_meta, title),
                symbol: meta.symbol,
                icon: icon.or(meta.icon),
                reference: meta.reference,
                reference_hash: meta.reference_hash,
                decimals: meta.decimals,
            },
        );
        self.next_meta += 1;
        if self.next_meta - self.first_meta > MAX_LIMIT {
            self.metas.remove(&self.first_meta);
            self.first_meta += 1;
        }
    }

    pub fn get_num_accounts(&self) -> u32 {
        self.num_accounts
    }

    #[private]
    pub fn update_meta(&mut self, meta: FungibleTokenMetadata) {
        self.meta.set(&meta);
    }

    #[private]
    pub fn ban_meta(&mut self, id: u32) {
        assert!(self.first_meta <= id && id < self.next_meta);
        let meta = self.meta.get().unwrap();
        self.metas.insert(&id, &meta);
    }

    pub fn get_meta_limits(&self) -> (u32, u32) {
        (self.first_meta, self.next_meta)
    }

    pub fn get_metas(
        &self,
        from_index: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<(u32, FungibleTokenMetadata)> {
        let from_index = from_index.unwrap_or(self.first_meta);
        let limit = limit.unwrap_or(self.next_meta - self.first_meta);
        (from_index..limit)
            .filter_map(|index| self.metas.get(&index).map(|meta| (index, meta)))
            .collect()
    }
}
