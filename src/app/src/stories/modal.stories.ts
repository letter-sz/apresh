// import type { Meta, StoryObj } from '@storybook/svelte';
// import Modal from '$components/modal/Modal.svelte';
// import { createRawSnippet } from 'svelte';

// // More on how to set up stories at: https://storybook.js.org/docs/writing-stories
// const meta = {
// 	title: 'Modal/Modal',
// 	component: Modal,
// 	tags: ['autodocs'],
// 	argTypes: {}
// } satisfies Meta<Modal>;

// export default meta;
// type Story = StoryObj<typeof meta>;

// const defaultArgs = {
// 	showModal: true,
// 	onClose: () => {
// 		console.log('closeHandler');
// 	},
// 	children: createRawSnippet(() => {
// 		return {
// 			render: () => `<div>Hello</div>`
// 		};
// 	})
// };

// const header = createRawSnippet(() => {
// 	return {
// 		render: () => `<h1>World</h1>`
// 	};
// });

// // More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
// export const Basic: Story = { args: { ...defaultArgs } };
// export const Hidden: Story = { args: { ...defaultArgs, showModal: false } };
// export const WithHeader: Story = { args: { ...defaultArgs, header } };
