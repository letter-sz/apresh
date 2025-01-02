<script lang="ts">
	import PillButton from './common/PillButton.svelte';

	let { settleId, settleSecret } = $props<{
		settleId: bigint;
		settleSecret: string | null;
	}>();

	// const baseUrl = 'http://localhost:3000';
	// const settleUrl = $derived(`${baseUrl}/?settleId=${settleId}&settleSecret=${settleSecret ?? ''}`);

	async function getQrCode(text: string) {
		// const response = await fetch(`/api/qr?text=${encodeURIComponent(text)}`);
		// if (!response.ok) {
		// 	throw new Error('Failed to generate QR code');
		// }
		// return await response.text();
		'';
	}
</script>

<span>Not yet implemented</span>

<!-- {#await getQrCode(settleUrl)}
	<span></span>
{:then image}
	<div class="flex flex-col space-y-6">
		<div class="h-72 w-72 rounded-3xl bg-gradient-to-r from-blue-500 to-rose-400 p-0.5">
			<img src={image} alt="qr code" class="rounded-3xl" />
		</div>

		<PillButton
			onClick={() => navigator.clipboard.writeText(settleUrl)}
			text="Copy link"
			className="w-1/2 mx-auto"
		/>
	</div>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await} -->
