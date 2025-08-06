import { defineConfig } from 'vitepress'

export default defineConfig({
  title: '🐊 krokfmt',
  description: 'A highly opinionated, zero-configuration TypeScript code formatter',
  lang: 'en-US',
  
  // Set base URL for GitHub Pages (repo name)
  base: process.env.GITHUB_ACTIONS ? '/krokfmt/' : '/',
  
  head: [
    ['link', { rel: 'icon', href: 'data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🐊</text></svg>' }],
  ],

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Playground', link: '/playground' },
      { text: 'API', link: '/api' },
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'What is krokfmt?', link: '/guide/what-is-krokfmt' },
          { text: 'Getting Started', link: '/guide/getting-started' },
        ]
      },
      {
        text: 'Features',
        items: [
          { text: 'Import Organization', link: '/guide/import-organization' },
          { text: 'Member Ordering', link: '/guide/member-ordering' },
          { text: 'Comment Preservation', link: '/guide/comment-preservation' },
          { text: 'TypeScript Support', link: '/guide/typescript-support' },
        ]
      },
      {
        text: 'Usage',
        items: [
          { text: 'CLI Usage', link: '/guide/cli-usage' },
          { text: 'Editor Integration', link: '/guide/editor-integration' },
          { text: 'CI/CD Integration', link: '/guide/ci-integration' },
        ]
      },
      {
        text: 'Reference',
        items: [
          { text: 'Configuration', link: '/guide/configuration' },
          { text: 'Rules', link: '/guide/rules' },
          { text: 'API Reference', link: '/api' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/skeswa/krokfmt' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present krokfmt contributors'
    },

    search: {
      provider: 'local'
    }
  },

  vite: {
    server: {
      fs: {
        allow: ['../..']
      }
    },
    optimizeDeps: {
      exclude: ['monaco-editor']
    }
  }
})