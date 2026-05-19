import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  // Needed for `file:///opt/volt/desktop/app/index.html` so assets resolve
  // relative (Vite defaults to absolute `/assets/...` which breaks offline file://).
  base: "./",
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:3170",
        changeOrigin: true
      },
      "/health": {
        target: "http://127.0.0.1:3170",
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: "dist",
    sourcemap: true
  }
});
