import { createActor, canisterId } from '../../../declarations/contract';
import { canisterId as identityCanisterId } from '../../../declarations/internet_identity';
import {
	canisterId as tokenCanisterId,
	createActor as createTokenActor
} from '../../../declarations/icrc1_ledger_canister';
import { AuthClient } from '@dfinity/auth-client';
import { type ActorSubclass, type Identity } from '@dfinity/agent';
import { type _SERVICE } from '../../../declarations/contract/contract.did.js';
import type { _SERVICE as _ICRC1_SERVICE } from '../../../declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';

const host = `http://localhost:4943`;

export const anonymousBackend = createActor(canisterId, { agentOptions: { host } });

export interface IConnection {
	actor: ActorSubclass<_SERVICE>;
	tokenActor: ActorSubclass<_ICRC1_SERVICE>;
	identity: Identity;
}

const tryConnect = async (): Promise<ConnectionOrConnect> => {
	console.log('Trying to connect to backend');
	const authClient = await AuthClient.create();
	const isAuthenticated = await authClient.isAuthenticated();
	if (!isAuthenticated)
		return {
			connected: false,
			connect: async () => await connect()
		};

	return {
		connected: true,
		...initActors(authClient)
	};
};

type ConnectionOrConnect =
	| {
			connected: false;
			connect: () => Promise<IConnection>;
	  }
	| ({
			connected: true;
	  } & IConnection);

export const connect = async (allowReconnect: boolean = true): Promise<IConnection> => {
	console.log('Connecting to backend');
	let authClient = await AuthClient.create();

	const authenticated = await authClient.isAuthenticated();
	const canReconnect = authenticated && allowReconnect;

	if (!canReconnect) {
		// Login manually (opens a new tab)
		await new Promise((resolve) => {
			authClient.login({
				identityProvider: `http://${identityCanisterId}.localhost:4943/`, // 'https://identity.ic0.app'
				onSuccess: () => resolve(undefined)
			});
		});
	}

	return initActors(authClient);
};

const initActors = (authClient: AuthClient): IConnection => {
	const identity = authClient.getIdentity();
	const actor = createActor(canisterId, {
		agentOptions: {
			identity,
			host
		}
	});
	const tokenActor = createTokenActor(tokenCanisterId, {
		agentOptions: {
			identity,
			host
		}
	});

	console.log('Connected to backend as', identity.getPrincipal().toText());

	return { actor, tokenActor, identity };
};

// export const contractCanister = canisterIds.contract.local;
