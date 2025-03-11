use candid::{CandidType, Principal};

#[derive(CandidType)]
pub struct Metadata {
    key: String,
    value: String,
}

#[derive(CandidType)]
pub struct InitArgs {
    pub minting_account: Account,
    pub transfer_fee: u128,
    pub token_symbol: String,
    pub token_name: String,
    pub metadata: Vec<(String, MetadataValue)>,
    pub initial_balances: Vec<(Account, u128)>,
    pub archive_options: ArchiveOptions,
    pub feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType)]
#[allow(dead_code)]
pub enum MetadataValue {
    Nat(u128),
    Int(i128),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(CandidType)]
pub struct ArchiveOptions {
    pub num_blocks_to_archive: u64,
    pub trigger_threshold: u64,
    pub controller_id: Principal,
}

#[derive(CandidType)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(CandidType)]
pub enum LedgerArg {
    Init(InitArgs),
}

#[derive(CandidType)]
pub struct IcrcAccount {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType)]
pub struct IcrcTransferArg {
    from_subaccount: Option<Vec<u8>>,
    to: IcrcAccount,
    amount: u128,
    fee: Option<u128>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType)]
pub struct ApproveArgs {
    pub spender: Account,
    pub amount: u128,
    pub fee: Option<u128>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub expected_allowance: Option<u128>,
    pub expires_at: Option<u64>,
}
