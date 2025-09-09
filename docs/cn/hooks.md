# 钩子

在特定 hj 命令运行前/后，可以指定命令运行。例如：

- 可以在提交前，运行代码格式化工具；

- 在推送前，运行公司提供的代码检查工具；

- 在推送后，运行自定义脚本发送邮件给自己。

- ……

## 触发时机

你可以在本地配置 `hj.toml` 中指定钩子。目前有以下配置项提供：

- `hooks.pre_commit`
- `hooks.post_commit`
- `hooks.pre_push`
- `hooks.post_push`

未来会提供更多钩子触发的时机，以媲美和 Git Hooks 一样的体验。

## 指定命令

我们会启动一个默认 Shell 来运行您的命令，例如：

```toml
[hooks]
pre_commit = "npm run lint"
```

## 使用 [Just](https://github.com/casey/just) 命令运行器

命令运行是一个看似简单，实则需求繁多的功能。在指定钩子的命令时，您可能会有以下需求：

- 指定 Windows 与 Unix-like 不同的命令；
- 指定不同的 Shell；
- 指定工作目录；
- 编写脚本而不是命令；
- 指定环境变量……

作为一个版本控制工具，我们并不聚焦于将命令运行的功能做到完美。需求复杂时，推荐您使用 [Just](https://github.com/casey/just) 命令运行器。只需在配置中编写：

```toml
[hooks]
use_just = true  # 当前系统中无 just 时自动安装
pre_commit = "just lint"
```

然后，编写 `justfile` 文件：

```
lint:
  npm run lint
```

你可以参阅 [just 手册](https://just.systems/man/en/introduction.html) 来查看更多高级功能。
