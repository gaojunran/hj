import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "hj document",
  description: "Fast, opinionated version control experience.",
  locales: {
    root: {
      label: 'English',
      lang: 'en',
    },
    cn: {
      label: '中文',
      lang: 'zh-CN',
    }
  },
  markdown: {
    theme: 'red'
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Quick Tutorial',
        items: [
          { text: 'Initialize a repo', link: '/init' },
          { text: 'Clone / Download a repo', link: '/clone' },
          { text: 'Commit', link: '/commit' },
          { text: 'Pull / Push', link: '/pull-push' },
        ]
      },
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: '/markdown-examples' },
          { text: 'Runtime API Examples', link: '/api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
