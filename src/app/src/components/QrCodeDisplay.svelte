<script lang="ts">
	import { fetchBackend } from '$lib/canisters';
	import { generate_qr } from 'wasm';
	import PillButton from './common/PillButton.svelte';

	let { settleId, settleSecret } = $props<{
		settleId: bigint;
		settleSecret: string | null;
	}>();

	const baseUrl = 'http://localhost:3000';
	// const settleUrl = $derived(`${baseUrl}/?settleId=${settleId}&settleSecret=${settleSecret ?? ''}`);
	const settleUrl = $derived(
		`${baseUrl}/shipment/confirm?id=${settleId}&secret=${settleSecret ?? ''}`
	);

	async function getQrCode(url: string) {
		// const data = await fetchBackend(fetch).generateQr(url, BigInt(320));
		// if (Object.keys(data)[0] == 'Ok') {
		// 	const blob = new Blob([Object.values(data)[0]], { type: 'image/png' });
		// 	const url = await convertToDataUrl(blob);
		// 	return url as string;
		// }
		try {
			const data = await generate_qr(url, 320);
			if (data) {
				const blob = new Blob([data], { type: 'image/png' });
				const url = await convertToDataUrl(blob);
				return url as string;
			}
		} catch (error) {
			console.error('Cannot get QR code: ' + error);
		}

		return null;
	}

	function convertToDataUrl(blob: Blob) {
		return new Promise((resolve, _) => {
			const fileReader = new FileReader();
			fileReader.readAsDataURL(blob);
			fileReader.onloadend = function () {
				resolve(fileReader.result);
			};
		});
	}
</script>

{#await getQrCode(settleUrl)}
	<span>Loading...</span>
{:then image}
	<div class="flex flex-col space-y-6">
		<div class="h-72 w-72 rounded-3xl bg-gradient-to-r from-blue-500 to-rose-400 p-0.5">
			<img src={image} alt="settlement QR code" class="rounded-3xl" />
		</div>

		<PillButton
			onClick={() => navigator.clipboard.writeText(settleUrl)}
			text="Copy link"
			className="w-1/2 mx-auto"
		/>
	</div>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
