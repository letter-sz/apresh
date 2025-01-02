import type { Shipment } from '$declarations/contract/contract.did';

/// <reference types="@sveltejs/kit" />
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			mode: 'none' | 'create' | 'buy' | 'settle';
			selected: bigint | null;
		}
		// interface Platform {}
	}
}

export {};
