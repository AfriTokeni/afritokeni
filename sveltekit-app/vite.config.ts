import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import juno from '@junobuild/vite-plugin';
import yaml from '@rollup/plugin-yaml';

export default defineConfig({
	plugins: [sveltekit(), juno(), yaml()],
	optimizeDeps: {
		include: ['@lucide/svelte']
	},
	ssr: {
		noExternal: ['@lucide/svelte']
	}
});
