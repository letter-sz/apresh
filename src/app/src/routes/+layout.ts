import { connection } from '$lib/connection.svelte';
import { wallet } from '$lib/wallet.svelte';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = true;

export const load: LayoutLoad = async ({ depends }) => {
	depends('token:balance');

	let balance = 0n;

	if (connection.identity !== null) {
		balance = await wallet.balance();
	}

	return {
		balance
	};
};
