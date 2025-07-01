# 提交

## 创建提交

::: details 对于熟悉 git 的用户

与 git 不同，jj：

- 没有暂存区（或叫索引区）；
- 工作区的所有变更默认提交到一个叫 **工作副本** 的特殊顶端提交中；
- 新添加的文件不需要显式地 `add` 即可被追踪，但 `.gitignore` 仍然有效。

`hj commit <message>`近似于：

```sh
git add .  # 先将未追踪的文件添加到工作区
git add -p
git commit -m message
```

:::

::: details 对于熟悉 jj 的用户

`hj commit <message>`的实质是：

```sh
jj commit --interactive --message message
```

这本质上是拆分一个提交，使用 `jj split` 也可以实现。

:::

在初始化仓库后，你就可以开始工作了！你在此仓库里做的 **所有变更操作** 都会被自动跟踪（被忽略的文件除外）。

每当你实现了一部分工作，例如新增了一个功能、修复了一个 bug 等，就可以对这部分修改创建一个**提交（commit）**：

```sh
hj commit "feat: add new feature"
```

在 `commit` 子命令后加一个可选参数，填写此次提交的描述信息。

命令会弹出一个终端交互式界面，可以在界面中选择要提交的文件。按下 `f` 键可以展开单个文件，选择具体的变更行。选择完成后，按下 `c` 键提交。

:::tip 未来计划

未来我们会支持向命令行参数中传入文件，而不只是交互式地选择，以支持 hj 在更广泛地场景下被使用，例如脚本。

:::

如果你在 `hj commit` 命令中没有给出提交描述信息，命令会认为你想先选择文件、后提供描述。选择完成后，命令会提示你输入提交描述信息。

:::tip

为了极致的效率，大多数 hj 命令都有短缩写。 例如 `commit` 可以简写为 `cm`，`status` 可以简写为 `st`。这是我们最常用的两个缩写。

未来 `hj commit` 会支持选项 `--push` 或 `-p`，将本次提交自动推送到远程。

:::

## 增补提交

::: details 对于熟悉 git 的用户

`hj amend`近似于：

```sh
git add -p
git commit --amend
```

而 `hj amend <revset>` 需要复杂的交互式变基才能实现。

:::

::: details 对于熟悉 jj 的用户

`hj amend`的实质是：

```sh
jj squash --interactive --from @ --into @-
```

`hj amend <revset>`的实质是：

```sh
jj squash --interactive --from @ --into <revset>
```

事实上 `squash` 命令已经足够简单，但 `amend` 语义更加明确、功能更加单一，所以我们提供了 `amend` 命令。

:::

如果你在提交后，发现漏掉了一些内容、或者有些已提交的代码有误，可以继续编辑文件之后后进行 **提交的增补（amend）**。增补的部分会合入上一个提交中：

```sh
hj amend
```

同样会弹出一个交互式界面，你可以选择要提交的文件和变更行。

你也可以指定将工作副本中的一部分变更增补到任意提交中：

```sh
hj amend <revset>
```

## 删减提交

::: details 对于熟悉 git 的用户

`hj reset`近似于：

```sh
git reset -p HEAD^  # 针对改动块
git reset HEAD^ -- file1 file2 file3  # 针对文件
```

而 `hj amend <revset>` 需要复杂的交互式变基才能实现。

:::

::: details 对于熟悉 jj 的用户

`hj reset`的实质是：

```sh
jj squash --interactive --from @- --into @
```

`hj reset <revset>`的实质是：

```sh
jj squash --interactive --from <revset> --into @
```


撤回一个提交和增补一个提交一样，本质上都是将两个提交合并成一个。

事实上 `squash` 命令已经足够简单，但 `reset` 语义更加明确、功能更加单一，所以我们提供了 `reset` 命令。

:::


`reset` 操作是 `amend` 操作的逆操作，可以将一个提交的部分或全部内容撤回到工作副本中（这个提交默认是最近一次提交）：

```sh
hj reset <revset>
```

特别地，如果你撤回了一个提交的全部内容，这个提交将会被删除。
