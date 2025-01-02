import { fileURLToPath, URL } from 'url';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import environment from 'vite-plugin-environment';
import dotenv from 'dotenv';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

dotenv.config({ path: '../../.env' });

export default defineConfig({
	build: {
		emptyOutDir: true,
		chunkSizeWarningLimit: 1024000
	},
	optimizeDeps: {
		esbuildOptions: {
			define: {
				global: 'globalThis'
			}
		}
	},
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:4943',
				changeOrigin: true
			}
		}
	},
	plugins: [
		topLevelAwait(),
		wasm(),
		sveltekit(),
		environment('all', { prefix: 'CANISTER_' }),
		environment('all', { prefix: 'DFX_' })
	],
	resolve: {
		alias: [
			{
				find: 'declarations',
				replacement: fileURLToPath(new URL('../declarations', import.meta.url))
			},
			{
				find: 'components',
				replacement: fileURLToPath(new URL('./src/components', import.meta.url))
			}
		]
	}
});
