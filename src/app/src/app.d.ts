import type { Shipment } from '$declarations/contract/contract.did';
import type { PageData as CreateShipmentPageData } from './routes/shipment/create/$types';
import type { PageData as BuyShipmentPageData } from './routes/shipment/buy/$types';
import type { PageData as SettleShipmentPageData } from './routes/shipment/settle/$types';

/// <reference types="@sveltejs/kit" />
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			page: { mode: 'map' } | CreatePageState | BuyPageState | SettlePageState;
		}
		// interface Platform {}
	}
}

interface CreatePageState extends CreateShipmentPageData {
	mode: 'create';
}

interface BuyPageState extends BuyShipmentPageData {
	mode: 'buy';
}

interface SettlePageState extends SettleShipmentPageData {
	mode: 'settle';
}

export {};
