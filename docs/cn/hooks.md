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

## 与 Git 的兼容性

我们提供了运行 Git Hooks 相关的功能。在特定时期，hj 会自动运行：

```sh
git hook run [HOOK_NAME]
```

这提供了与 Git 兼容的体验。如果您的项目使用 [husky](https://github.com/typicode/husky)、[lefthook](https://github.com/evilmartians/lefthook)、[hk](https://github.com/jdx/hk) 等工具，它们可以在 hj 中正常运行。

### 忽略 Git Hooks

如果有些 Git Hooks 提供了仅 Git 可用的逻辑，而不是代码格式化等通用逻辑，您可以禁止 hj 运行这些 Hooks，在配置项中配置：

```toml
[hooks]
ignore_git_hooks = ["pre-commit", "post-commit"]
```

### 对比 hj Hooks 和 Git Hooks

我们推荐您仅在单人项目，或者个性配置中使用 hj Hooks。hj Hooks 使用类似 `package.json` 脚本的配置方式，配置更简单，您无需考虑将脚本复制到 `.git/hooks` 目录下，并为脚本设置执行权限等问题。

在多人项目中，因为 Git 庞大的用户基数，Git Hooks 更为广泛地被接受。正如我们之前所说，hj 同样支持运行 Git Hooks。

如果您要结合使用 hj Hooks 和 Git Hooks，请确保它们不会冲突。hj 会先运行 Git Hooks，再运行 hj Hooks。

## 在命令中禁用钩子

在命令中，您可以使用 `--no-pre-hooks`、`--no-post-hooks` 等参数来禁用钩子。例如有一个提交仅修改了文档而没有修改代码，可能不需要运行代码格式化钩子：

```sh
hj commit "chore: update docs" --no-pre-hooks
```
