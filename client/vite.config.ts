import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import cesium from 'vite-plugin-cesium'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte(), cesium()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1704,
    strictPort: true,
  },
  assetsInclude: ['**/*.glb'],
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  resolve: {
    alias: {
      $assets: path.resolve('./assets'),
      $translations: path.resolve('./translations'),
      $lib: path.resolve('./src/lib'),
      $components: path.resolve('./src/components'),
      $bindings: path.resolve('./src/bindings'),
      $stores: path.resolve('./src/stores'),
      $services: path.resolve('./src/services'),
      $datasource: path.resolve('./src/datasource'),
    }
  }
}));
