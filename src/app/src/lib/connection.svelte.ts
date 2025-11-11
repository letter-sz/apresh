import { invalidateAll } from '$app/navigation';
import { type _SERVICE } from '$declarations/contract/contract.did.js';
import type { _SERVICE as _ICRC1_SERVICE } from '$declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { type ActorSubclass } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { connect, type IConnection } from './canisters';
import { WhitelistService } from './services/whitelistService';

export class Connection {
	identity: Principal | null = $state(null);
	actor: ActorSubclass<_SERVICE> | null = $state(null);
	tokenActor: ActorSubclass<_ICRC1_SERVICE> | null = $state(null);

	#whitelistService: WhitelistService = new WhitelistService();
	#isWhitelisted: boolean = $state(false);

	// Connects to the backend if not already connected
	async ensureConnected(): Promise<IConnection> {
		if (this.identity !== null)
			return {
				identity: this.identity!,
				actor: this.actor!,
				tokenActor: this.tokenActor!
			};

		return this.connect(true);
	}

	async connect(allowReconnect: boolean = true): Promise<IConnection> {
		const connection = await connect(allowReconnect);

		this.identity = connection.identity;
		this.actor = connection.actor;
		this.tokenActor = connection.tokenActor;

		// this.#isWhitelisted = this.#whitelistService.isIdentityWhitelisted(this.identity);

		await invalidateAll();

		return connection;
	}

	async disconnect(): Promise<void> {
		this.identity = null;
		this.actor = null;
		this.tokenActor = null;

		await invalidateAll();
	}

	async getConnection(): Promise<IConnection> {
		return await this.ensureConnected();
	}

	async getActor(): Promise<ActorSubclass<_SERVICE>> {
		return (await this.ensureConnected()).actor;
	}

	tryGetActor(): ActorSubclass<_SERVICE> | null {
		return this.actor;
	}

	async getTokenActor(): Promise<ActorSubclass<_ICRC1_SERVICE>> {
		return (await this.ensureConnected()).tokenActor;
	}

	async getIdentity(): Promise<Principal> {
		return (await this.ensureConnected()).identity;
	}

	isConnected(): boolean {
		return this.identity !== null;
	}

	isWhitelisted(): boolean {
		return this.#isWhitelisted;
	}

	async linkIdentityWithEmail(): Promise<void> {
		if (this.identity) this.#whitelistService.linkIdentityWithEmail(this.identity, '');

		await invalidateAll();
	}
}

export const connection = $state<Connection>(new Connection());
