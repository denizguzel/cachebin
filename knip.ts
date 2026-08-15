import type { KnipConfig } from 'knip';

const config: KnipConfig = {
  project: [
    '**/*.{js,mjs,cjs,jsx,ts,tsx,mts,cts}!',
    '**/*.css',
    '**/*.{svg,png,jpg,jpeg,gif,webp,ico}',
    '!src-tauri/**',
    'public/**/*',
  ],
  compilers: {
    svg: true,
    png: true,
    jpg: true,
    jpeg: true,
    gif: true,
    webp: true,
    ico: true,
  },
  ignoreExportsUsedInFile: true,
  ignoreFiles: ['public/favicon.svg'],
  ignoreIssues: {
    'src/components/ui/**': ['exports', 'types'],
  },
};

export default config;
