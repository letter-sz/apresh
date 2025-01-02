import type { LoadEvent } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad } */
export async function load({ url }: LoadEvent) {
	return {};
}
