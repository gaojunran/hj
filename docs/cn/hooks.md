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

作为一个版本控制工具，我们并不聚焦于将命令运行的功能做到完美。需求复杂时，推荐您使用 [Just](https://github.com/casey/just) 命令运行器或 [mise](https://mise.jdx.dev)。这里我们以 Just 为例，只需在配置中编写：

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

## 失败的命令

对于 `pre` 钩子，如果钩子命令失败，hj 会阻止版本控制命令的执行。

## 与 Git 的兼容性

我们提供了运行 Git Hooks 相关的功能。在特定时期，hj 会自动运行：

```sh
git hook run [HOOK_NAME]
```

这提供了与 Git 兼容的体验。如果您的项目使用 [husky](https://github.com/typicode/husky)、[lefthook](https://github.com/evilmartians/lefthook)、[hk](https://github.com/jdx/hk) 等工具，它们可以在 hj 中正常运行。

### 忽略 Git Hooks

如果有些 Git Hooks 提供了仅 Git 可用的逻辑（如和 `staged area` 相关的逻辑），您可以禁止 hj 运行这些 Hooks，在配置项中配置：

```toml
[hooks]
ignore_git_hooks = ["pre-commit", "post-commit"]
```

> [!TIP]
> 未来我们会支持一个有趣的功能：在执行 `pre-commit` 钩子前，先清空 Git 的暂存区，再从 jj 中导出即将提交的 diff，并执行 `git apply --cached`，从而将 jj 即将提交的变更放入 Git 的暂存区中，从而让 Git Hooks 能够访问这些变更。

### 对比 hj Hooks 和 Git Hooks

我们推荐您仅在单人项目，或者个性配置中使用 hj Hooks。hj Hooks 使用类似 `package.json` 脚本的配置方式，配置更简单，您无需考虑将脚本复制到 `.git/hooks` 目录下，并为脚本设置执行权限等问题。

在多人项目中，因为 Git 庞大的用户基数，Git Hooks 更为广泛地被接受。正如我们之前所说，hj 同样支持运行 Git Hooks（但 Git 用户在不配置的情况下无法运行 hj Hooks）。

如果您要结合使用 hj Hooks 和 Git Hooks，请确保它们不会冲突。hj 会先运行 Git Hooks，再运行 hj Hooks。

## 在命令中禁用钩子

在命令中，您可以使用 `--no-pre-hooks`、`--no-post-hooks` 等参数来禁用钩子。例如有一个提交仅修改了文档而没有修改代码，可能不需要运行代码格式化钩子：

```sh
hj commit "chore: update docs" --no-pre-hooks
```

## 关于「提交钩子」(`pre_commit` / `post_commit`) 的设计

实际上 jj 的设计中不存在「提交」的概念。提交仅是 hj 版本控制命令中 `describe` + `split` 的组合。因此提交钩子在 hj 中比较模糊，目前我们的设计如下：

- 仅在用户期待将类似 Git 中「工作区/暂存区」的内容提交到版本库时，才应该触发提交钩子：
- 因此，仅在 `hj commit` 和 `hj amend` 命令中，运行 `hj.toml` 中定义的提交钩子和 Git 提交钩子；
- 我们假设您在使用 `hj split`、`hj squash` 等 **在提交中移动变更** 的命令时的意图是不期待提交钩子运行的。如果您要将工作副本中的变更转移到版本库并触发钩子运行，请使用 `hj commit` 来创建一个新提交、`hj amend` 来增补到已有提交；
- 我们假设您在使用 `hj new` 时的意图是不期待钩子运行的，因为这个命令通常创建一个空提交，仅包含元数据。
- jj 目前的 [讨论](https://github.com/jj-vcs/jj/issues/3577#issuecomment-2087816381) 中提及到，因为使用 jj 时总是隐式地创造提交，所以预提交 (`pre_commit`) 钩子似乎没有预上传 (`pre_push`) 钩子直观，所以他们会优先支持后者。hj 会在 jj 原生功能支持后保持 100% 对 jj 的兼容性。

## 关于「描述钩子」（`pre_describe` / `post_describe`）的设计

目前 hj 还没有实现描述钩子，因为如果从命令的角度来看，一共有 11 个命令支持传入 `--message` 参数，依次实现命令行参数改写似乎不太聪明；而 jj 从内部实现又非常简单，因此我们会等待 jj 原生支持描述钩子。
