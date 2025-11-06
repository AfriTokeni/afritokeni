import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import juno from '@junobuild/vite-plugin';
import yaml from '@rollup/plugin-yaml';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), juno(), yaml()],
	optimizeDeps: {
		include: ['@lucide/svelte']
	},
	ssr: {
		noExternal: ['@lucide/svelte']
	}
});
