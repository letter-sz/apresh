import { type ActorSubclass, type Identity } from '@dfinity/agent';
import { type _SERVICE } from '../../../declarations/contract/contract.did.js';
import type { _SERVICE as _ICRC1_SERVICE } from '../../../declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { connect, type IConnection } from './canisters.svelte';

export class Connection {
	identity: Identity | null = $state(null);
	actor: ActorSubclass<_SERVICE> | null = $state(null);
	tokenActor: ActorSubclass<_ICRC1_SERVICE> | null = $state(null);

	// Connects to the backend if not already connected
	async ensureConnected(): Promise<IConnection> {
		if (this.identity !== null)
			return {
				identity: this.identity!,
				actor: this.actor!,
				tokenActor: this.tokenActor!
			};

		const connection = await connect();

		this.identity = connection.identity;
		this.actor = connection.actor;
		this.tokenActor = connection.tokenActor;

		return connection;
	}

	async getConnection(): Promise<IConnection> {
		return await this.ensureConnected();
	}

	async getActor(): Promise<ActorSubclass<_SERVICE>> {
		return (await this.ensureConnected()).actor;
	}

	async getTokenActor(): Promise<ActorSubclass<_ICRC1_SERVICE>> {
		return (await this.ensureConnected()).tokenActor;
	}

	async getIdentity(): Promise<Identity> {
		return (await this.ensureConnected()).identity;
	}

	isConnected(): boolean {
		return this.identity !== null;
	}
}

// TODO: confirm that this will not slow down the app
// export const connection = $state<ConnectionOrConnect>(await tryConnect());
export let connection = $state<Connection>(new Connection());
