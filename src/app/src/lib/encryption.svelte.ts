// import * as vetkd from 'ic-vetkd-utils';
import { stateWallet, wallet } from '$lib/wallet.svelte';
import { Principal } from '@dfinity/principal';
import { Actor } from '@dfinity/agent';

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

const hex_decode = (hexString: any) =>
	Uint8Array.from(hexString.match(/.{1,2}/g)!.map((byte: any) => parseInt(byte, 16)));
