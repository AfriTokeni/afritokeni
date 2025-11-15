import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import path from 'path';

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		globals: true,
		environment: 'happy-dom',
		include: ['src/**/*.{test,spec}.{js,ts}'],
		coverage: {
			provider: 'v8',
			reporter: ['text', 'json', 'html'],
			exclude: [
				'node_modules/',
				'src/declarations/**',
				'**/*.config.{js,ts}',
				'**/types/**',
				'**/*.d.ts'
			]
		},
		setupFiles: ['./src/lib/services/__tests__/setup.ts'],
		// Prevent network calls during tests
		testTimeout: 10000,
		hookTimeout: 10000
	},
	resolve: {
		alias: {
			$: path.resolve(__dirname, './src'),
			$lib: path.resolve(__dirname, './src/lib'),
			$app: path.resolve(__dirname, './node_modules/@sveltejs/kit/src/runtime/app')
		}
	}
});
