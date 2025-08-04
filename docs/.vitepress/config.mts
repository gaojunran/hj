import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "hj",
  description: "Fast, opinionated version control experience.",
  markdown: {
    theme: "vitesse-dark"
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
          { text: 'Home', link: '/' },
        ],

        sidebar: [
          {
            text: 'Quick Tutorial',
            items: [
              { text: 'Initialize a repo', link: '/init' },
              { text: 'Clone / Download a repo', link: '/clone' },
              { text: 'Repo Status', link: '/status' },
              { text: 'Commit', link: '/commit' },
              { text: 'Branch', link: '/branch' },
              { text: 'Pull / Push', link: '/pull-push' },
            ]
          },
          {
            text: 'Advanced',
            items: [
              { text: 'Config', link: '/config' },
              { text: 'FAQ', link: '/faq' },
            ]
          }
        ],

        socialLinks: [
          { icon: 'github', link: 'https://github.com/gaojunran/hj' }
        ]
      }
    },
    cn: {
      label: '中文',
      lang: 'zh-CN',
      themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        nav: [
          { text: '主页', link: '/cn' },
        ],

        sidebar: [
          {
            text: '快速上手',
            items: [
              { text: '初始化仓库', link: '/cn/init' },
              { text: '克隆和下载仓库', link: '/cn/clone' },
              { text: '查看状态', link: '/cn/status' },
              { text: '提交', link: '/cn/commit' },
              { text: '分支/书签', link: '/cn/branch' },
              { text: '拉取和推送', link: '/cn/pull-push' },
            ]
          },
          {
            text: '进阶',
            items: [
              { text: '钩子', link: '/cn/hooks' },
              { text: '配置', link: '/cn/config' },
              { text: '常见问题', link: '/cn/faq' },
            ]
          }
        ],

        socialLinks: [
          { icon: 'github', link: 'https://github.com/gaojunran/hj' }
        ]
      }
    }
  },
  
})
