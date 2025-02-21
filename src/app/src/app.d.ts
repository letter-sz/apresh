import type { PageData as BuyShipmentPageData } from './routes/(header)/shipment/buy/$types';
import type { PageData as CancelShipmentPageData } from './routes/(header)/shipment/cancel/$types';
import type { PageData as CreateShipmentPageData } from './routes/(header)/shipment/create/$types';
import type { PageData as InfoShipmentPageData } from './routes/(header)/shipment/info/$types';
import type { PageData as SettleShipmentPageData } from './routes/(header)/shipment/settle/$types';
/// <reference types="@sveltejs/kit" />
// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			page:
				| { mode: 'map' }
				| CreatePageState
				| BuyPageState
				| SettlePageState
				| CancelPageState
				| InfoPageState;
		}
		// interface Platform {}
	}
	declare interface Window {
		ic?: {
			plug?: {
				createAgent: ({
					whitelist,
					host
				}: {
					whitelist: string[];
					host: string;
				}) => Promise<boolean>;
				agent: HttpAgent;
				requestConnect: ({
					whitelist,
					host,
					timeout
				}: {
					whitelist?: string[];
					host?: string;
					timeout?: number;
				}) => Promise<any>;
				// fetchRootKey: () => Promise<void>;
				createActor: <T>({
					canisterId,
					interfaceFactory
				}: CreateActorArgs) => Promise<ActorSubclass<T>>;
				isConnected: () => Promise<boolean>;
				disconnect: () => Promise<void>;
				principalId: string;
				getPrincipal: () => Promise<Principal>;
				onExternalDisconnect: (callback: () => void) => void;
				onLockStateChange: (callback: (isLocked: boolean) => void) => void;
			};
		};
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

interface CancelPageState extends CancelShipmentPageData {
	mode: 'cancel';
}

interface InfoPageState extends InfoShipmentPageData {
	mode: 'info';
}

export {};
