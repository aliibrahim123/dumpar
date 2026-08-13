import { defineConfig } from 'vite';
import { neocomp } from '@neocomp/vite-plugin';

// config
export default defineConfig({
	build: {
		target: 'esnext',
		outDir: './dist',
		manifest: true,
		rollupOptions: {
			input: {
				index: './src/index.ts',
			},
			preserveEntrySignatures: 'allow-extension',
			output: {
				entryFileNames: '[name].js',
				chunkFileNames: 'chunks/[name]-[hash].js',
			},
		},
	},
	plugins: [neocomp()],
});
