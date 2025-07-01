# 克隆和下载仓库

## 克隆仓库

::: details 对于熟悉 git 的用户

`hj clone` 近似于 `git clone`，但允许你使用仓库全名（`owner/repo`）来克隆 GitHub 上的仓库。

:::

::: details 对于熟悉 jj 的用户

`hj clone` 近似于 `jj git clone`，但允许你使用仓库全名（`owner/repo`）来克隆 GitHub 上的仓库。

相比于 `git clone`，`jj` 有清晰的进度条来查看进度。

因为 `git clone` 和 `gh repo clone` 得到的是以 `.git` 作为版本管理的本地目录，所以并不兼容 `jj` 和 `hj`。

:::

你可以使用命令：

```sh
hj clone https://github.com/gaojunran/hj.git
```

来克隆一个仓库。

这样写并不简练！你可以用仓库全名来替代一个长 url：

```sh
hj clone gaojunran/hj
```

这建立在默认 host 是 `github.com` 的前提下。你可以通过覆写环境变量来修改默认行为：

```sh
HJ_DEFAULT_HOST=gitlab.com hj clone gaojunran/hj
```

:::tip 未来计划

未来我们可能会提供一个 flag，如 `--host` 来指定不同的源。

对于 GitHub 仓库，未来我们会补充一个克隆自己仓库省略 owner 的功能。

:::

## 下载仓库

很多情况下我们使用 GitHub 上的仓库，并不需要其历史版本，例如：

- 学习仓库中的代码；
- 将仓库作为模板用于自己的项目；
- 只想要最新的代码快照。

在这种情况下，我们为你提供了一个方便的命令：

```sh
hj download gaojunran/hj
```

它将从 GitHub 上下载仓库的最新代码快照（不含版本控制），并解压到当前目录下。这比克隆仓库要快得多！

:::details

此部分功能实现借鉴自 [degit-rs](https://github.com/psnszsn/degit-rs)。

一个较大的不同是，[degit](https://github.com/Rich-Harris/degit)、[tiged](https://github.com/tiged/tiged) 及 [degit-rs](https://github.com/psnszsn/degit-rs) 均是将代码克隆在一个空文件夹中，而 `hj download` 则是将代码解压到当前目录的一个新文件夹下，与 `hj clone` 保持一致。

:::

:::warning

再次强调，此功能与 **版本控制** 完全无关！它只是我们给你提供的一个方便的小礼物 🎁；下载仓库后，你可以继续使用 `hj init` 来进行自己的版本控制。

:::
