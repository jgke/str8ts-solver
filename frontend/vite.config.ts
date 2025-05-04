import { defineConfig, loadEnv, searchForWorkspaceRoot } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const basePath = env.BASE_URL || "/";
  return {
    base: basePath,
    plugins: [react(), tailwindcss()],
    server: {
      fs: {
        allow: [
          // search up for workspace root
          searchForWorkspaceRoot(process.cwd()),
          // your custom rules
          "../solver_wasm/target/pkg-web",
        ],
      },
    },
    worker: { format: "es" },
  };
});
