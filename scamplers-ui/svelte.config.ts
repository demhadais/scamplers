import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { Config } from "@sveltejs/kit";

const config: Config = {
  compilerOptions: {
    experimental: {
      async: true,
    },
  },
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    csp: {
      directives: {
        "base-uri": ["self"],
        "default-src": ["self"],
        "frame-ancestors": ["none"],
        "form-action": ["self"],
        "trusted-types": [],
        "require-trusted-types-for": ["script"],
      },
      mode: "auto",
    },
  },
};

export default config;
