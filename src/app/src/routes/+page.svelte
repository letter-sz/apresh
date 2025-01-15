<script lang="ts">
	import { goto } from '$app/navigation';
	import Card from '$components/landing/Card.svelte';

	type Chosen = 'shipper' | 'carrier' | 'customer';

	let selectedCard: Chosen | null = $state(null);

	function roleToColor(role: Chosen | null) {
		switch (role) {
			case 'shipper':
				return 'orange';
			case 'carrier':
				return 'blue';
			case 'customer':
				return 'violet';
			default:
				return 'red';
		}
	}
</script>

<div class="grid h-screen grid-cols-3 gap-8 bg-{roleToColor(selectedCard)} px-12 py-12">
	<Card
		onMouseEnter={() => {
			selectedCard = 'shipper';
		}}
		title="Nadaj"
		color={roleToColor('shipper')}
		onClick={() => {
			goto('/shipper');
		}}
	>
		Twórz zlecenia i znajdź najlepszego przewoźnika dla swojej przesyłki.
	</Card>

	<Card
		title="Przewoźnik"
		color={roleToColor('carrier')}
		onMouseEnter={() => {
			selectedCard = 'carrier';
		}}
		onClick={() => {
			goto('/carrier');
		}}>Znajdź zlecenia i przewieź wybrane przesyłki.</Card
	>
	<Card
		title="Śledź"
		color={roleToColor('customer')}
		onMouseEnter={() => {
			selectedCard = 'customer';
		}}
		onClick={() => {
			goto('/customer');
		}}>Zobacz status swoich przesyłek i kiedy będą u Ciebie.</Card
	>
</div>
