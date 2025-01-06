import { expect, test } from 'vitest';
import { testContract } from '$lib/canisters';

test('should handle a basic greeting', async () => {
	const result1 = await testContract.listPendingShipments();
	expect(result1).toEqual([]);
});
