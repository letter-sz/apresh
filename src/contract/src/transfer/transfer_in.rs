use crate::transfer::utils::{
    get_account_from_principal, get_canister_default_account, get_ledger_principal,
};
use anyhow::anyhow;
use icrc_ledger_types::{
    icrc1::transfer::BlockIndex,
    icrc2::transfer_from::{TransferFromArgs, TransferFromError},
};

use super::TransferInParams;

pub async fn transfer_in(args: TransferInParams) -> anyhow::Result<()> {
    ic_cdk::println!(
        "Transferring {} tokens from account {}",
        &args.params.amount,
        &args.from,
    );

    let transfer_from_args = TransferFromArgs {
        from: get_account_from_principal(ic_cdk::caller()),
        // maybe add something valuable here
        memo: args.params.memo,
        amount: args.params.amount,
        spender_subaccount: None,
        // TODO: inspect what id exactly do
        fee: None,
        to: get_canister_default_account(),
        created_at_time: None,
    };

    let block_index =
        ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
            get_ledger_principal(),
            "icrc2_transfer_from",
            (transfer_from_args,),
        )
        .await
        .map_err(|e| anyhow!("failed to call ledger: {:?}", e))?
        .0
        .map_err(|e| anyhow!("ledger transfer error {:?}", e))?;

    ic_cdk::println!("Transfer successful. Block index: {:?}", block_index);

    Ok(())
}
