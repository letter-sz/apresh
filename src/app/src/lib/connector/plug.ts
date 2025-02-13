import {
	canisterId as contractCanisterId,
	idlFactory as contractIdlFactory
} from '$declarations/contract';
import type { _SERVICE as _CONTRACT_SERVICE } from '$declarations/contract/contract.did';
import type { _SERVICE as _ICRC1_SERVICE } from '$declarations/icrc1_ledger_canister/icrc1_ledger_canister.did';

import {
	createActor as createTokenActor,
	canisterId as tokenCanisterId,
	idlFactory as tokenIdlFactory
} from '$declarations/icrc1_ledger_canister';
import { type IConnection } from '$lib/canisters';
import { Principal } from '@dfinity/principal';

export const connectPlug = async (): Promise<IConnection | undefined> => {
	if (!window.ic?.plug) return undefined;

	try {
		const isConnected = await window.ic.plug.isConnected();

		if (!isConnected) {
			const hasAllowed = await window.ic.plug.requestConnect({
				whitelist: [contractCanisterId, tokenCanisterId],
				host: 'http://localhost:4943'
				// timeout: 10000
			});
			if (!hasAllowed) return undefined;

			// const hasCreatedAgent = await window.ic.plug.createAgent({
			// 	whitelist: [contractCanisterId, tokenCanisterId],
			// 	host: 'http://localhost:4943'
			// 	// timeout: 10000
			// });

			// if (!hasCreatedAgent) return undefined;
		}

		const principal = Principal.from(window.ic.plug.agent.principal);

		console.log('createActor', window.ic.plug.createActor);

		const actor = await window.ic.plug.createActor<_CONTRACT_SERVICE>({
			canisterId: contractCanisterId,
			interfaceFactory: contractIdlFactory
		});

		const tokenActor = await window.ic.plug.createActor<_ICRC1_SERVICE>({
			canisterId: tokenCanisterId,
			interfaceFactory: tokenIdlFactory
		});

		return { actor, tokenActor, identity: principal };
	} catch {
		console.log('Plug wallet not found');
		return undefined;
	}
};
