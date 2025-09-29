# 安装

:::tip 请先安装 [Jujutsu](https://github.com/jj-vcs/jj) 🚀
[Jujutsu](https://github.com/jj-vcs/jj) 是 hj 的核心依赖。参阅 [安装 jj](https://jj-vcs.github.io/jj/latest/install-and-setup/)。
:::

:::tip 在寻找系统级包管理工具吗？

我很快会提供平台特定的包管理工具安装方式：

- scoop for Windows;
- homebrew for macOS;

此外，[aqua](https://aquaproj.github.io/) 标准注册表的安装方式也很快会提供。

:::

## 使用 [mise](https://mise.jdx.dev) 安装（推荐）

[mise](https://mise.jdx.dev) 是一个集开发工具安装、开发工具多版本管理、环境变量管理和任务管理为一体的工具 💪🏻，支持安装 GitHub Release Assets 中的二进制文件。

```bash
mise use -g jj  # 先安装 jj
mise use -g github:gaojunran/hj
```

## 使用 [stew](https://github.com/marwanhawari/stew) 安装

[stew](https://github.com/marwanhawari/stew) 可以安装 GitHub Release Assets 中的二进制文件。

```bash
stew install jj-vcs/jj  # 先安装 jj
stew install gaojunran/hj
```

## 使用 `cargo install` 手动编译

hj 已编译的二进制文件有如下版本：

- Linux X64 / ARM
- macOS X64 / ARM
- Windows X64

如果您的操作系统或架构不在上述列表中，则只能使用 cargo 自行编译。你需要确保本地有合适版本的 Rust 环境。

```bash
cargo install hj-vcs
```
