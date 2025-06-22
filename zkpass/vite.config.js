import { defineConfig } from "vite";

export default defineConfig({
  base: process.env.NODE_ENV === "production" ? "/web5claims/zkpass/" : "/",
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
  server: {
    port: 8000,
    proxy: {
      "/verify-age": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
  },
});
