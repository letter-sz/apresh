import canisterIds from '../../../../.dfx/local/canister_ids.json';
import { Principal } from '@dfinity/principal';
import { connection } from './connection.svelte';

class Wallet {
	async getTransferFee() {
		const tokenActor = await connection.getTokenActor();
		const fee = await tokenActor.icrc1_fee();
		return fee;
	}

	async approve(amount: bigint) {
		const owner = await connection.getIdentity();
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
	}

	async balance() {
		const tokenActor = await connection.getTokenActor();
		const owner = await connection.getIdentity();
		const balance = await tokenActor.icrc1_balance_of({
			owner: owner.getPrincipal(),
			subaccount: []
		});
		return balance;
	}
}

export const wallet = $state<Wallet>(new Wallet());
