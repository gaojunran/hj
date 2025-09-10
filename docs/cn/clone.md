# 克隆和下载仓库

## 克隆仓库

::: details 对于熟悉 git 的用户

`hj clone` 近似于 `git clone`，但允许你使用仓库全名（`owner/repo`）而不是 URL 来克隆 GitHub 上的仓库。

:::

::: details 对于熟悉 jj 的用户

`hj clone` 近似于 `jj git clone`，但允许你使用仓库全名（`owner/repo`）而不是 URL 来克隆 GitHub 上的仓库。

相比于 `git clone`，`jj` 有清晰的进度条来查看进度。

因为 `git clone` 和 `gh repo clone` 得到的是以 `.git` 作为版本管理的本地目录，所以并不兼容 `jj` 和 `hj`。

:::

你可以使用命令：

```sh
hj clone https://github.com/gaojunran/hj.git
```

来克隆一个仓库。

### 使用仓库全名替代 URL

使用 URL 并不简练！你需要去网页中手动复制粘贴一个地址。你可以用仓库全名来替代一个长 URL：

```sh
hj clone gaojunran/hj
```

这建立在默认 host 是 `github.com` 的前提下。你可以通过 [配置项](/cn/config) 来修改默认行为：

```toml
# ./hj.toml
[clone]
default_host = "gitlab.com"
default_user = "gaojunran"
```

配置了 `default_user` 后，你可以省略用户名：

```sh
hj clone gaojunran/hj  # 默认 host 是 gitlab.com
hj clone hj # 默认 user 是 gaojunran
```

### 指定目标文件夹

也可以指定目标文件夹路径。默认是远程仓库的名字。如果你这样执行：

```sh
hj clone gaojunran/hj .  # 关注最后的 . 表示当前文件夹
```

那么仓库内容将被克隆在当前文件夹内，而不是新建一个文件夹。

### Fork 并克隆

对于一个 GitHub 仓库，你可以用一行命令 Fork 某个仓库并克隆自己的分叉：

```sh
hj clone gaojunran/hj --fork
```

---

## 下载仓库

很多情况下我们使用 GitHub 上的仓库，并不需要其历史版本，例如：

- 学习仓库中的代码；
- 将仓库作为模板用于自己的项目；
- 只想要最新的代码快照。

在这种情况下，我们为你提供了一个方便的命令：

```sh
hj download gaojunran/hj
```

它将从 GitHub 上下载仓库的最新代码快照（不含版本控制），并解压到一个文件夹中。这比克隆仓库要快得多！

:::details

此部分功能实现借鉴自 [degit-rs](https://github.com/psnszsn/degit-rs)。

一个较大的不同是，[degit](https://github.com/Rich-Harris/degit)、[tiged](https://github.com/tiged/tiged) 及 [degit-rs](https://github.com/psnszsn/degit-rs) 均是将代码克隆在当前目录中（且要求这个文件夹必须为空），而 `hj download` 默认是将代码解压到当前目录的一个新文件夹下，与 `hj clone` 保持一致。和 `hj clone` 一样，你也可以指定具体路径（名字），例如：

```sh
hj download vuejs/core vue  # 如果不指定名字，默认名为仓库名，即 core
```

:::

---

你还可以从一个仓库中下载指定的文件或文件夹，例如：

```sh
hj download vuejs/core -e package.json   # 下载 package.json 文件
hj download vuejs/core -e src -e package.json   # 下载 src 文件夹和 package.json 文件
```

:::details

下载指定文件（夹）的功能借鉴自 [cloneit](https://github.com/alok8bb/cloneit)。

此功能高度依赖 GitHub API，故如果您有 API 速率限制问题，请自行导出 `GITHUB_TOKEN` 环境变量，程序将会自动使用。

未来我们将会支持 GitLab。

:::

:::tip

下载仓库与 **版本控制** 完全无关！它只是我们给你提供的一个方便的小礼物 🎁；下载仓库后，你可以继续使用 `hj init` 来进行自己的版本控制。

:::
