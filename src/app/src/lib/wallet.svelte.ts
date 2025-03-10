import { Principal } from '@dfinity/principal';
import { connection } from './connection.svelte';
import { unwrap } from './utils';
import type {
	ApproveResult,
	TransferResult
} from '$declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { invalidate } from '$app/navigation';
import { CANISTER_ID_CONTRACT, mintBackend } from './canisters';

class Wallet {
	decimalPoints = 8n;

	decimals() {
		return Number(this.decimalPoints);
	}

	denominator() {
		return 10n ** this.decimalPoints;
	}

	amountToPrettyFull(amount: bigint) {
		const whole = amount / this.denominator();
		const fractional = amount % this.denominator();
		return `${whole.toString()}.${fractional.toString().padStart(Number(this.decimalPoints), '0')}`;
	}

	amountToPretty(amount: bigint) {
		const pretty = this.amountToPrettyFull(amount);
		return pretty.replace(/\.?0+$/, '');
	}

	amountFromPretty(amount: string) {
		if (!amount.includes('.')) {
			amount = amount + '.';
		}
		let [whole, fractional] = amount.split('.');

		while (fractional.length < this.decimals()) {
			fractional += '0';
		}

		if (whole.length === 0) {
			whole = '0';
		}

		return BigInt(whole) * this.denominator() + BigInt(fractional);
	}

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

		const spender = Principal.fromText(CANISTER_ID_CONTRACT);
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

	async approveDoubleFee(amount: bigint) {
		await this.approve(amount + (await this.getTransferFee()));
	}

	async balance(fetchFunction: typeof fetch = fetch) {
		const tokenActor = mintBackend(fetchFunction);
		const owner = await connection.getIdentity();

		const balance = await tokenActor.icrc1_balance_of({
			owner,
			subaccount: []
		});

		return balance;
	}

	async mint() {
		const amount = this.denominator() * 10n;
		console.log('Minting', amount);
		const tokenActor = mintBackend(fetch);
		const owner = await connection.getIdentity();
		const mintResult = await tokenActor.icrc1_transfer({
			amount,
			to: { owner, subaccount: [] },
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
