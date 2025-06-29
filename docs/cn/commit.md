# 提交

## 创建提交

::: details 对于熟悉 git 的用户

与 git 不同，jj：

- 没有暂存区（或叫索引区）；
- 工作区的所有变更默认提交到一个叫 `working-copy` 的特殊顶端提交中；
- 新添加的文件不需要显式地 `add` 即可被追踪，但 `.gitignore` 仍然有效。

`hj commit <message>`近似于：

```sh
git add -p
git commit -m message
```

:::

::: details 对于熟悉 jj 的用户

`hj commit <message>`的实质是：

```sh
jj commit --interactive --message message
```

:::

在初始化仓库后，你就可以开始工作了！你在此仓库里做的 **所有变更操作** 都会被自动跟踪（被忽略的文件除外）。

每当你实现了一部分工作，例如新增了一个功能、修复了一个 bug 等，就可以对这部分修改创建一个**提交（commit）**：

```sh
hj commit "feat: add new feature"
```

在 `commit` 子命令后加一个可选参数，填写此次提交的描述信息。

命令会弹出一个终端交互式界面，可以在界面中选择要提交的文件。按下 `f` 键可以展开单个文件，选择具体的变更行。选择完成后，按下 `c` 键提交。

如果你在 `hj commit` 命令中没有给出提交描述信息，命令会认为你想先选择文件、后提供描述。选择完成后，命令会提示你输入提交描述信息。

完成后，可以查看当前仓库的状态：

```sh
hj
```

:::tip

为了极致的效率，大多数 hj 命令都有短缩写。 例如 `commit` 可以简写为 `cm`，`status` 可以简写为 `st`。这是我们最常用的两个缩写。

:::

## 创建增补提交

::: details 对于熟悉 git 的用户

`hj amend`近似于：

```sh
git add -p
git commit --amend
```

:::

::: details 对于熟悉 jj 的用户

`hj amend`的实质是：

```sh
jj squash --interactive
```

:::
