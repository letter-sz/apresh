import { browser } from '$app/environment';
import { mintBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import '$lib/i18n';
import { locale, waitLocale } from 'svelte-i18n';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = true;

export const load: LayoutLoad = async ({ depends, fetch }): Promise<{ balance?: bigint }> => {
	if (browser) {
		locale.set(window.navigator.language);
	}
	await waitLocale();

	depends('token:balance');

	let balance: bigint | undefined;

	if (connection.identity !== null) {
		const tokenActor = mintBackend(fetch);

		balance = await tokenActor.icrc1_balance_of({
			owner: connection.identity,
			subaccount: []
		});
	}

	return {
		balance
	};
};
