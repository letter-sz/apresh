import { expect, test } from 'vitest';
import { testContract } from '$lib/canisters.svelte';

// For easier CI use, tests moved to the rust side
test('should handle a basic greeting', async () => {
	const result1 = await testContract.list_pending_shipments();
	expect(result1).toEqual([]);
});
