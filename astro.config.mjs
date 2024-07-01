import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";


export default defineConfig({
  srcDir : "web/src",
  publicDir : "web/public",
  outDir : "web/build",
  cacheDir : "web/cache",

  output : "static",
  compressHTML : true,

	integrations: [tailwind()],

  build : {
    // directory based routing
    // pages/index.astro      -> /
    // pages/post/index.astro -> /post
    format : "directory",
    assets : "__assets",
    redirects : true,
    inlineStylesheets : 'auto',
  },
})
