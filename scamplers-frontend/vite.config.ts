import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import * as mod from "node:dns";

mod.setDefaultResultOrder("verbatim");

export default defineConfig({
  plugins: [sveltekit()],
});
