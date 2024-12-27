import type { Meta, StoryObj } from '@storybook/svelte';
// import CreateShipment from '$components/forms/CreateShipment.svelte';
import CreateShipment from '$routes/shipment/create/+page.svelte';
import type { SvelteComponent } from 'svelte';
import type { PageData } from '../routes/shipment/create/$types';

const meta = {
	title: 'Forms/CreateShipment',
	component: CreateShipment,
	tags: ['autodocs']
} satisfies Meta<SvelteComponent>;

export default meta;
type Story = StoryObj<typeof meta>;

const data: PageData = {
	created: [],
	shipments: [],
	carried: [],
	registeredCarrier: false,
	registeredCustomer: false,
	settleSecret: null,
	settleId: null
};

// Basic story showing the create shipment form
export const Basic: Story = {
	args: {
		data
	}
};
