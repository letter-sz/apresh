import { mintBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = true;

export const load: LayoutLoad = async ({ depends, fetch }): Promise<{ balance?: bigint }> => {
	depends('token:balance');

	let balance: bigint | undefined;

	if (connection.identity !== null) {
		const tokenActor = mintBackend(fetch);

		balance = await tokenActor.icrc1_balance_of({
			owner: connection.identity.getPrincipal(),
			subaccount: []
		});
	}

	return {
		balance
	};
};
