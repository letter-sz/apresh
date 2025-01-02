// import * as vetkd from 'ic-vetkd-utils';

import type { Principal } from '@dfinity/principal';
import type { IConnection } from './canisters';

export async function ibe_decrypt(ibe_ciphertext_hex: string) {
	// const tsk_seed = window.crypto.getRandomValues(new Uint8Array(32));
	// const tsk = new vetkd.TransportSecretKey(tsk_seed);
	// console.log('tsk.public_key()', tsk.public_key());
	// const ek_bytes_hex = await stateWallet.actor!.encrypted_ibe_decryption_key_for_caller(
	// 	tsk.public_key()
	// );
	// console.log('stop');
	// const pk_bytes_hex = await stateWallet.actor!.ibe_encryption_key();

	// console.log('ek_bytes_hex', ek_bytes_hex);
	// console.log('pk_bytes_hex', pk_bytes_hex);
	// const k_bytes = tsk.decrypt(
	// 	hex_decode(ek_bytes_hex),
	// 	hex_decode(pk_bytes_hex),
	// 	(await Actor.agentOf(stateWallet.actor!)?.getPrincipal())!.toUint8Array()
	// );
	// console.log('k_bytes', k_bytes);

	// const ibe_ciphertext = vetkd.IBECiphertext.deserialize(hex_decode(ibe_ciphertext_hex));
	// const ibe_plaintext = ibe_ciphertext.decrypt(k_bytes);
	// return new TextDecoder().decode(ibe_plaintext);
	return 'vetkd not yet supported';
}

const hex_decode = (hexString: string) =>
	Uint8Array.from(hexString.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16)));

const hex_encode = (bytes: Uint8Array) =>
	bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

export async function ibe_encrypt(wallet: IConnection, message: string, principal: Principal) {
	const pk_bytes_hex = await wallet.actor.ibe_encryption_key();
	const message_encoded = new TextEncoder().encode(message);
	const seed = window.crypto.getRandomValues(new Uint8Array(32));

	// const ibe_ciphertext = vetkd.IBECiphertext.encrypt(
	// 	hex_decode(pk_bytes_hex),
	// 	principal.toUint8Array(),
	// 	message_encoded,
	// 	seed
	// );

	// return hex_encode(ibe_ciphertext.serialize());
	return hex_encode(new Uint8Array());
}
