import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import sveltePreprocess from "svelte-preprocess";

export default defineConfig({
  plugins: [
    svelte({
      preprocess: [sveltePreprocess({ typescript: true })],
    }),
  ],
});
