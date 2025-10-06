import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite配置，用于在开发和生产环境清除控制台日志
  clearScreen: false,
  // Tauri需要固定端口进行开发
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // 监听tauri配置文件变化
      ignored: ["**/src-tauri/**"],
    },
  },
}));
