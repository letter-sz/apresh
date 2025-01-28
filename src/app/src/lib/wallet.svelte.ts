import canisterIds from '../../../../.dfx/local/canister_ids.json';
import { Principal } from '@dfinity/principal';
import { connection } from './connection.svelte';
import { unwrap } from './utils';
import type {
	ApproveResult,
	TransferResult
} from '$declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { invalidate } from '$app/navigation';
import { mintBackend } from './canisters';

class Wallet {
	async owner() {
		return await connection.getIdentity();
	}

	async tokenName() {
		return (await connection.getTokenActor()).icrc1_name();
	}

	async getTransferFee() {
		const tokenActor = await connection.getTokenActor();
		const fee = await tokenActor.icrc1_fee();
		return fee;
	}

	async approve(amount: bigint) {
		const tokenActor = await connection.getTokenActor();

		const spender = Principal.fromText(canisterIds.contract.local);
		const fee = await tokenActor.icrc1_fee();
		const approveResult = await tokenActor.icrc2_approve({
			fee: [],
			from_subaccount: [],
			memo: [],
			created_at_time: [],
			amount: amount + fee,
			expected_allowance: [],
			expires_at: [],
			spender: { owner: spender, subaccount: [] }
		});

		unwrap<ApproveResult>(approveResult);
		return approveResult;
	}

	async balance(fetchFunction: typeof fetch = fetch) {
		const tokenActor = mintBackend(fetchFunction);
		const owner = await connection.getIdentity();

		const balance = await tokenActor.icrc1_balance_of({
			owner: owner.getPrincipal(),
			subaccount: []
		});

		return balance;
	}

	async mint(amount: bigint) {
		console.log('Minting', amount);
		const tokenActor = mintBackend(fetch);
		const owner = await connection.getIdentity();
		const mintResult = await tokenActor.icrc1_transfer({
			amount,
			to: { owner: owner.getPrincipal(), subaccount: [] },
			fee: [],
			memo: [],
			from_subaccount: [],
			created_at_time: []
		});

		unwrap<TransferResult>(mintResult);

		await invalidate('token:balance');

		return;
	}
}

export const wallet = $state<Wallet>(new Wallet());
