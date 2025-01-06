import type { Meta, StoryObj } from '@storybook/svelte';
import CreateShipment from '$components/forms/CreateShipment.svelte';
import type { SvelteComponent } from 'svelte';

const meta = {
	title: 'Forms/CreateShipment',
	component: CreateShipment,
	tags: ['autodocs']
} satisfies Meta<SvelteComponent>;

export default meta;
type Story = StoryObj<typeof meta>;

// Basic story showing the create shipment form
export const Basic: Story = {
	args: {
		showModal: true,
		onClose: () => {
			console.log('closeHandler');
		}
	}
};
