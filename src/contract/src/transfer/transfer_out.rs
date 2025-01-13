use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};

use super::TransferOutParams;
use crate::transfer::utils::get_ledger_principal;
use anyhow::anyhow;

pub async fn transfer_out(args: TransferOutParams) -> anyhow::Result<()> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.params.amount,
        &args.to,
    );

    let transfer_args = TransferArg {
        from_subaccount: None,
        memo: args.params.memo,
        amount: args.params.amount,
        to: args.to,
        fee: None,
        created_at_time: None,
    };

    let block_index = ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        get_ledger_principal(),
        "icrc1_transfer",
        (transfer_args,),
    )
    .await
    .map_err(|e| anyhow!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| anyhow!("ledger transfer error {:?}", e))?;

    ic_cdk::println!("Transfer successful. Block index: {:?}", block_index);

    Ok(())
}
