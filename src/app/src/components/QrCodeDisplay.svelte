<script lang="ts">
	import { page } from '$app/state';
	import { Jumper } from 'svelte-loading-spinners';
	import { generate_qr } from 'wasm';
	import PillButton from './common/Buttons/PillButton.svelte';

	let { settleId, settleSecret } = $props<{
		settleId: bigint;
		settleSecret: string | null;
	}>();

	const settleUrl = $derived(
		`${page.url.origin}/shipment/confirm?id=${settleId}&secret=${settleSecret ?? ''}`
	);
	let copied = $state(false);

	async function getQrCode(url: string) {
		try {
			const data = generate_qr(url, 320);
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

	function copyLink() {
		navigator.clipboard.writeText(settleUrl);
		copied = true;

		setTimeout(() => {
			copied = false;
		}, 1000);
	}
</script>

{#await getQrCode(settleUrl)}
	<div class="flex flex-col">
		<div class="flex h-72 w-72 items-center justify-center">
			<Jumper size="60" color="#FF3E00" unit="px" duration="1.5s" />
		</div>
	</div>
{:then image}
	<div class="flex flex-col space-y-6">
		<div class="h-72 w-72 rounded-3xl bg-gradient-to-r from-blue-500 to-rose-400 p-0.5">
			<img src={image} alt="settlement QR code" class="rounded-3xl" />
		</div>

		<PillButton
			onClick={copyLink}
			text={copied ? 'Copied!' : 'Copy link'}
			className="w-1/2 mx-auto"
		/>
	</div>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
