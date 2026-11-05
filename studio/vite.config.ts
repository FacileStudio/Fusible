import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  define: { "process.env.NODE_ENV": '"production"', "process.env": "{}" },
  build: {
    outDir: "../src/assets",
    emptyOutDir: false,
    cssCodeSplit: false,
    lib: { entry: "src/main.ts", formats: ["es"], fileName: () => "studio.bundle.js" },
    rollupOptions: { output: { inlineDynamicImports: true, assetFileNames: "studio[extname]" } },
    chunkSizeWarningLimit: 4000,
  },
  optimizeDeps: { exclude: ["@facile/muse"] },
});
