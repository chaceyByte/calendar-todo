import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  // 设置根目录
  root: resolve(__dirname, './'),
  // 防止 vite 输出复杂的 rust 错误
  clearScreen: false,
  // Tauri 使用固定端口，若端口不可用则自动切换，因此无需设置严格端口
  server: {
    strictPort: true,
    port: 1420,
    host: '127.0.0.1'
  },
  // 使用 `TAURI_PLATFORM`、`TAURI_ARCH`、`TAURI_FAMILY`、`TAURI_PLATFORM_VERSION`、`TAURI_PLATFORM_TYPE` 和 `TAURI_DEBUG` 环境变量
  envPrefix: ['VITE_', 'TAURI_'],
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
      '@components': resolve(__dirname, './src/components'),
      '@stores': resolve(__dirname, './src/stores'),
      '@views': resolve(__dirname, './src/views')
    }
  },
  build: {
    // Tauri 支持 Chromium 最新版本，设置最低 target 为 ES2020
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // 不为调试构建生成 sourcemap
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // 为调试构建生成源代码映射
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: 'dist',
  },
})