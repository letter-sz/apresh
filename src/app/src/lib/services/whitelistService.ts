import type { Principal } from '@dfinity/principal';

export interface IWhitelistService {
	isIdentityWhitelisted(identity: Principal): boolean;
	linkIdentityWithEmail(identity: Principal, email: string): void;
}

export class WhitelistService implements IWhitelistService {
	#whitelist: string[] = [];

	isIdentityWhitelisted(identity: Principal): boolean {
		return this.#whitelist.some((e) => e == identity.toString());
	}

	linkIdentityWithEmail(identity: Principal, email: string): void {
		this.#whitelist.push(identity.toString());
	}
}
