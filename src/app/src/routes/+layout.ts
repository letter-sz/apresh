import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { mintBackend } from '$lib/canisters';
import { connection } from '$lib/connection.svelte';
import '$lib/i18n';
import type { Principal } from '@dfinity/principal';
import { locale, waitLocale } from 'svelte-i18n';
import type { LayoutLoad } from './$types';

export const ssr = false;
export const prerender = true;

type FetchType = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export const load: LayoutLoad = async ({ depends, fetch, url }): Promise<{ balance?: bigint }> => {
	const isModalPage = url.pathname.startsWith('/shipment/');

	if (isModalPage) {
		const redirect = url.searchParams.get('redirect');

		if (redirect) {
			goto(`/${redirect}`);
		} else goto('/');
	}

	if (browser) {
		locale.set(window.navigator.language);
	}

	await waitLocale();

	depends('token:balance');

	let balance: bigint | undefined;

	if (connection.identity !== null) balance = await fetchActorBalance(connection.identity, fetch);

	return {
		balance
	};
};

const fetchActorBalance = async (owner: Principal, fetch: FetchType) => {
	const tokenActor = mintBackend(fetch);

	return await tokenActor.icrc1_balance_of({
		owner,
		subaccount: []
	});
};
