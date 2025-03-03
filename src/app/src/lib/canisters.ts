import { createActor } from '$declarations/contract';
import {
	createActor as createTokenActor
} from '$declarations/icrc1_ledger_canister';
import { AuthClient } from '@dfinity/auth-client';
import { type ActorSubclass } from '@dfinity/agent';
import type { _SERVICE } from '$declarations/contract/contract.did';
import type { _SERVICE as _ICRC1_SERVICE } from '$declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';
import { connectPlug } from './connector/plug';
import type { Principal } from '@dfinity/principal';

// export const host = `http://localhost:4943`;
export const host = `https://icp0.io`;

export const dev = () => false;

export const CANISTER_ID_CONTRACT = 'vujqm-syaaa-aaaag-at46q-cai';
export const CANISTER_ID_TOKEN = 'ryjl3-tyaaa-aaaaa-aaaba-cai';

export function fetchBackend(fetchFunction: typeof fetch) {
	return createActor(CANISTER_ID_CONTRACT, { agentOptions: { host, fetch: fetchFunction } });
}

export function mintBackend(fetchFunction: typeof fetch) {
	return createTokenActor(CANISTER_ID_TOKEN, {
		agentOptions: {
			host,
			fetch: fetchFunction
		}
	});
}

export interface IConnection {
	actor: ActorSubclass<_SERVICE>;
	tokenActor: ActorSubclass<_ICRC1_SERVICE>;
	identity: Principal;
}

export const connect = async (allowReconnect: boolean = true): Promise<IConnection> => {
	// Assuming user prefers to connect with plug wallet, if present.
	const plugConnection = await connectPlug();

	if (plugConnection) {
		console.log('Plug connection', plugConnection);
		if (plugConnection) return plugConnection;
	} else {
		console.log('No plug connection');
	}

	console.log('Connecting to backend');
	const authClient = await AuthClient.create();

	const authenticated = await authClient.isAuthenticated();
	const canReconnect = authenticated && allowReconnect;

	if (!canReconnect) {
		// Login manually (opens a new tab)
		await new Promise((resolve) => {
			authClient.login({
				// identityProvider: `http://${identityCanisterId}.localhost:4943/`, // 'https://identity.ic0.app'
				identityProvider: 'https://identity.ic0.app',
				onSuccess: () => resolve(undefined)
			});
		});
	}

	return initActors(authClient);
};

const initActors = (authClient: AuthClient): IConnection => {
	const identity = authClient.getIdentity();
	const actor = createActor(CANISTER_ID_CONTRACT, {
		agentOptions: {
			identity,
			host,
			fetch
		}
	});
	const tokenActor = createTokenActor(CANISTER_ID_TOKEN, {
		agentOptions: {
			identity,
			host,
			fetch
		}
	});

	console.log(`Connected to backend as ${identity.getPrincipal().toText()} in ${dev() ? 'dev' : 'prod'} mode`);
	const principal = identity.getPrincipal();

	return { actor, tokenActor, identity: principal };
};

// export const contractCanister = canisterIds.contract.local;
