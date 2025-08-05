# 安装

:::tip 请先安装 [jj-vcs](https://github.com/jj-vcs/jj) 🚀
[jj-vcs](https://github.com/jj-vcs/jj) 是 hj 的核心依赖。参阅 [安装 jj](https://jj-vcs.github.io/jj/latest/install-and-setup/)。

特别地，如果你使用的是 scoop、brew 等包管理器的安装方式，我们在它们的安装脚本中包含了对 jj-vcs 的安装命令，因此你无需手动安装 jj-vcs。
:::

## 使用 [stew](https://github.com/marwanhawari/stew) 安装

[stew](https://github.com/marwanhawari/stew) 可以安装 GitHub Release Assets 中的二进制文件。

```bash
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
