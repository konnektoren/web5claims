import { defineConfig } from "vite";

export default defineConfig(({ mode }) => {
  const isProduction = mode === "production";

  return {
    base: isProduction ? "/zkpass/" : "/",
    build: {
      outDir: "dist",
      emptyOutDir: true,
      assetsDir: "assets",
      rollupOptions: {
        output: {
          // Ensure consistent asset naming
          assetFileNames: "assets/[name]-[hash][extname]",
          chunkFileNames: "assets/[name]-[hash].js",
          entryFileNames: "assets/[name]-[hash].js",
        },
      },
    },
  };
});
