# 常见问题

## 已经有了 Git / jj-vcs，为什么还要自己造一个工具？

[Git](https://git-scm.com/) 有着悠久的历史，并且绝大多数开源代码都托管在基于 Git 的平台上。几乎不可能撼动 Git 的地位。

但是你会发现 Git **并不好用**。

大多数人的工作流是单一的，而 Git 做了太多事情。很多资深的程序员一旦遇到一些相对复杂的 Git 使用场景，都要去反复查阅各种文档。Git 的命令繁杂、手册冗长。它的很多设计理念新手难以理解。

如果你感兴趣的话，这里有一篇关于 Git 的文章：[关于 Git 的礼节](https://www.yinwang.org/blog-cn/2015/03/11/git-etiquette)。

[jj-vcs](https://github.com/jj-vcs/jj) 是一个革新性的工具，具备了很多新的理念：

- 工作副本（工作区）也是一个提交。这样不需要 stash，而且切换分支和变基永远不会失败。
- 不需要索引区（暂存区），所有变更默认被提交，也包括 Git 中的未追踪文件。
- 不再需要交互式变基，子提交会被自动变基。
- 使用函数式语言指定文件和变更，而不是使用上古时期的方式解析文本。
- 有着默认的根提交。
- ……

这些理念上的革新使得 jj 的本地版本控制非常优秀。

但是 jj 在与 Git 远程的互作性上有待进步。例如书签（分支 HEAD）并不会随着提交而推进；pull / push 的模型依然需要非常熟练才能掌握。jj 与 Git Forge（如 GitHub）的互作性也不够好。

而且 jj 是另一套工具和使用理念，不论你是否学过 Git 都有着不小的学习成本。

所以有了 hj，致力于基于 jj 的优秀理念，提供一个 90% 场景下极简的版本控制体验，降低你的学习成本（你甚至可以在不了解 Git 和 jj 的情况下上手），并且在一些特殊场景中你可以随时切换回 jj 进行操作。这符合 [渐进性](https://talks.antfu.me/2024/vue-amsterdam/) 的设计理念。

让我们来举几个例子说明 hj 相较于 jj 的极简性：

- 你可以一行命令开始版本控制、新建一个默认的主分支、创建一个 GitHub 仓库、并把远程仓库关联到本地：

```sh
hj init --gh
```

- 你可以一行命令提交指定文件，并给出提交信息：

```sh
hj commit "feat: add new feature"
```

- 你可以一行命令拉取当前分支的变更，并推送到远程：

```sh
hj push main --pull
```

hj 将不断进步！💪🏻

## 为什么不向 jj 贡献代码，而是直接封装 jj ？

jj 是一个有着优秀设计理念的工具，为了保持其功能的 [正交性](https://en.wikipedia.org/wiki/Orthogonality#Computer_science)，不可避免地要提供尽量 **通用** 的工具，以满足所有用户的版本控制需求。

hj 的理念有所不同，「简单」是我们的第一设计理念。我们封装了许多有用的子命令使得你可以以 **最小的心智负担** 来完成 Git 和 jj 中复杂的操作。目前我们的工具是对多个命令行工具的组合，以可维护性极强的 Rust 脚本形式为您提供。

我们额外提供的功能中，我确信其中有很多 jj 在短期内一定不会提供，甚至永远不会提供，因为这些功能与 jj 的设计理念相悖。如果你追求简单，不妨来尝试一下我们的工具。 💪🏻

## 目前存在哪些已知缺陷？

- `hj undo` 是基于 `jj undo` 的。而 hj 的很多操作不是原子化的，例如 `hj switch` 内部执行了两行 jj 命令，所以 `hj undo` 不能够完全撤销一个操作。未来我们会根据 `hj op restore` 封装一个命令。

- 因为 hj 基于 jj，所以短期内几乎不可能实现各种 IDE 或插件的原生适配（因为 jj 的知名度远远不够）。如果你非常需要这些适配的话，可以考虑 [与 Git 仓库共存](/cn/init.html#与-git-仓库共存)。

- hj 的很多改变工作副本指向的操作不会自动推进书签。这是 jj 的特意设计，hj 正在尝试解决。

- hj 目前仅在 GitHub 中发布各架构的 Release。除了从 GitHub 手动下载二进制文件外，你仅能通过 `cargo install --path .` 来从源码编译。未来我们将会至少支持以下安装方式： 
  - [scoop](https://scoop.sh/) (Windows)、
  - PowerShell script (Windows)、
  - [homebrew](https://brew.sh/) (macOS, Linux X86)、
  - bash script (Unix，包括 WSL, Linux 和 macOS)。

## 路线图

> ✅ 表示已经实现，🚧 表示正在实现，🤔 表示未来计划。你也可以在 [Issues](https://github.com/gaojunran/hj/issues) 里提出你的想法！

- ✅　`clone` 支持省略 `owner`。
- ✅　`hj clone --fork`
- ✅　`hj commit` 和 `hj describe` 时支持打开编辑器进行多行描述的编辑。
- ✅　提供 Hooks 支持。
- 🤔　发布到 scoop、homebrew；编写 PowerShell script 和 bash script。
- 🤔　`hj download` 下载单个文件(夹) 时直接下载到指定的目录，而不包含远程仓库中的文件夹结构。
- 🤔　`hj init` 时支持下载 `.gitignore`。（[来源](https://github.com/github/gitignore)）
- 🤔　全面支持 GitLab。
- 🤔　`hj rollback`：交互式选择要回退的命令。
- 🤔　优化与 Git 仓库共存时的体验。
- 🤔　提供更多与 `hj log` 相关的简化命令。
- 🤔　完善的 Stacked PR 支持。

## hj 是如何实现的？

hj 用 Rust 语言编写的一个命令行工具。

目前多数子命令我们使用 Shell Command 的形式构建基于 jj、Git、gh 等工具的命令。

:::tip 移除 jj、git、gh 依赖的计划

目前 hj 强依赖 jj 可执行文件；涉及 GitHub 的操作强依赖 gh（有些操作如下载仓库是调用的 GitHub API）；涉及[与 Git 仓库共存](/cn/init.html#与-git-仓库共存)的操作强依赖 Git。

未来我们将逐步移除这些依赖。

- 首先，jj 是 hj 的核心依赖。但 [jj-cli](https://crates.io/crates/jj-cli) 和 [jj-lib](https://crates.io/crates/jj-lib) 均未稳定，所以我们目前以调用和解析 jj CLI 输出的形式使用 jj。

- 其次，gh 是 GitHub 的官方 CLI 工具，但其很大程度上依赖本地的 Git 仓库，而且所有操作都可以换用 GitHub API 实现。未来我们会使用 GitHub API 的 Rust Client 来替换 gh。

- 最后，当我们与 Git 仓库共存时会依赖 Git。未来我们会使用 Git 的 Rust 绑定来替换对 Git 的调用。

:::

而一些子命令（如 `hj squash/split/rebase` 等），直接调用 jj 来实现，不做任何命令行参数的转换和命令的封装，这是因为它们已经做得足够完善了，不需要多余的封装。

## 我应该从 Git 切换到 Jujutsu 或 hj 吗？

切换成本很低，因为它可以与您现有的工作流程无缝集成。坦白说，最多一天就能习惯使用，而且使用好东西确实有道理。当然，你可以在任何食品安全的壶里泡出好茶，但如果你有一个漂亮的茶壶，每次泡茶都会享受。你一定经常使用版本控制系统，为什么不让它也变得愉快呢？

> 这个比喻来自 [kubamartin 的博文](https://kubamartin.com/posts/introduction-to-the-jujutsu-vcs/)。
