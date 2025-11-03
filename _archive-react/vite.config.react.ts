import juno from "@junobuild/vite-plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import tailwindcss from "@tailwindcss/vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(), 
    nodePolyfills(), 
    juno(), 
    tailwindcss(),
    wasm(),
    topLevelAwait()
  ],
  optimizeDeps: {
    exclude: ['@noble/secp256k1'],
    include: ['tiny-secp256k1']
  },
  define: {
    global: 'globalThis',
  },
  server: {
    fs: {
      allow: ['..']
    }
  }
});
