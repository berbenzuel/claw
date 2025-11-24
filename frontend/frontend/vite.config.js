import { defineConfig } from "vite";
import path from "path";

export default defineConfig({
  server: {
    watch: {
      include: [path.resolve(__dirname, "../pkg/**")],
    },
  },
});
