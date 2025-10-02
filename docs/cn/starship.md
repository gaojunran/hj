# 与 [Starship](https://starship.rs) 集成

![alt text](/image6.png)

目前，你可以使用 Starship 的 [自定义命令](https://starship.rs/config/#custom-commands) 功能，来显示当前 jj 仓库的状态。

:::tip 使用 Git 还是 Jujutsu？

Jujutsu 支持 [与 Git 仓库共存](/cn/init.html#与-git-仓库共存)，但目前做得还不够好，经常出现 Git HEAD 游离的情况。因此我们建议你在 Starship 中禁用 Git 相关的配置，只使用下文提及的 Jujutsu 相关的配置。

:::

## 显示当前分支

在 hj 中，「当前分支」被解释为距离当前工作副本提交最近的书签。你可以构建自定义命令：

```toml
# ~/.config/starship.toml
[custom.jj_branch]
symbol = ""
command = "jj log -r 'heads(::@ & bookmarks())' -T bookmarks --color never --no-graph --ignore-working-copy"
when = "jj --ignore-working-copy root"
shell = ["sh", "--norc", "--noprofile"]
format = '[[ $symbol $output ](fg:color_fg0 bg:color_aqua)]($style)'
```

## 显示当前工作副本提交的状态

你可以使用 jj 的 [Templating Language](https://jj-vcs.github.io/jj/latest/templates/) 功能来显示当前工作副本提交的状态：

```toml
# ~/.config/starship.toml
[custom.jj_status]
command = '''
jj log -r @ --color never --no-graph --ignore-working-copy --limit 1 --template '
  concat(
    if(!empty, "M")
  )'
'''
when = "jj --ignore-working-copy root"
format = '[[$output ](fg:color_fg0 bg:color_aqua bold)]($style)'
shell = ["sh", "--norc", "--noprofile"]
```

这是我的 Starship 配置，供你参考：[starship.toml](https://github.com/gaojunran/dotfiles/commit/521bb23be8f5794def0eb6f4638191c1d4cfc3ff)

你可以参阅 [jj wiki](https://github.com/jj-vcs/jj/wiki/Starship) 来获取更多示例！
