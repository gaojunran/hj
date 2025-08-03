# 提交

## 创建提交

::: details 对于熟悉 git 的用户

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

未来我们将支持在终端编辑器中编辑提交描述信息。这对于多行描述信息很有用。

:::

如果你在 `hj commit` 命令中没有给出提交描述信息，命令会认为你想先选择文件、后提供描述。选择完成后，命令会提示你输入提交描述信息。

:::tip

为了极致的效率，大多数 hj 命令都有短缩写。 例如 `commit` 可以简写为 `cm`，`status` 可以简写为 `st`。这是我们最常用的两个缩写。

:::

## 重命名提交

::: details 对于熟悉 git 的用户

要在 Git 中重命名一个历史提交，可以通过交互式变基 `reword` 命令实现。

:::

::: details 对于熟悉 jj 的用户

`hj describe` 和 `jj describe` 完全等价。

:::

`hj describe` 命令可以用来重命名一个提交：

```sh
hj describe REVSET -m "new message"
```

`REVSET` 处填写一个具体的提交，如 `@-` 表示最近的一次提交；`-m` 参数填写新的提交描述信息。

## 增补提交

::: details 对于熟悉 git 的用户

`hj amend`近似于：

```sh
git add -p
git commit --amend
```

而 `hj amend <revset>` 需要复杂的交互式变基才能实现。

git 没有不可变提交的机制，所以 `hj amend --force` 在 git 中没有对应项；但是 git 在推送时如果会覆写远程仓库提交，需要 `git push --force`，这在 jj 中是不需要的。

:::

::: details 对于熟悉 jj 的用户

`hj amend`的实质是：

```sh
jj squash --interactive --from @ --into @-
```

`hj amend REVSET`的实质是：

```sh
jj squash --interactive --from @ --into REVSET
```

`--force` 这个标志对应 `jj squash`的 `--ignore-immutable` 标志。

事实上 `squash` 命令已经足够简单，但 `amend` 语义更加明确、功能更加单一，所以我们提供了 `amend` 命令。

:::

如果你在提交后，发现漏掉了一些内容、或者有些已提交的代码有误，可以继续编辑文件之后后进行 **提交的增补（amend）**。增补的部分会合入 **最近一个提交** 中：

```sh
hj amend
```

同样会弹出一个交互式界面，你可以选择要提交的文件和变更行。

你也可以指定将工作副本中的一部分变更增补到 **任意提交** 中：

```sh
hj amend REVSET
```

已推送（push）的提交很可能是不可变的，增补它可能是 **不安全** 的。此时可以加上 `--force` 参数来强制增补。

## 删减提交

::: details 对于熟悉 git 的用户

`hj reset`近似于：

```sh
git reset -p HEAD^  # 针对改动块
git reset HEAD^ -- file1 file2 file3  # 针对文件
```

而 `hj reset REVSET` 需要复杂的交互式变基才能实现。

:::

::: details 对于熟悉 jj 的用户

`hj reset`的实质是：

```sh
jj squash --interactive --from @- --into @
```

`hj reset REVSET`的实质是：

```sh
jj squash --interactive --from REVSET --into @
```


撤回一个提交和增补一个提交一样，本质上都是将两个提交合并成一个。

事实上 `squash` 命令已经足够简单，但 `reset` 语义更加明确、功能更加单一，所以我们提供了 `reset` 命令。

:::


`reset` 操作是 `amend` 操作的逆操作，可以将一个提交的部分或全部内容撤回到工作副本中（如果没有指定，这个提交默认是最近一次提交）：

```sh
hj reset REVSET
```

特别地，如果你撤回了一个提交的全部内容，这个提交将会被删除（abandoned）。

和增补一样，可以加上 `--force` 参数来强制删减。

## 在提交间移动变更

::: details 对于熟悉 git 的用户

对于跨分支的操作，可以使用 `git diff` + `git apply`：

```sh
git diff C~ C -- path/to/file > patch.diff
git checkout B
git apply patch.diff
git commit -am "手动移动变更"
```

这只是其中一种实现方式。git 中不存在能完成这个操作的单行命令。

:::

::: details 对于熟悉 jj 的用户

`hj squash` 和 `jj squash` 完全等价。

:::

我们可以总结一下前面提到的两个操作：

- `hj amend`：将 **工作副本** 中的变更增补到某次提交（默认为最近一次提交）中。
- `hj reset`：将某次提交（默认为最近一次提交）中的部分或全部内容撤回到 **工作副本** 中。

如果我们不局限于在工作副本和提交中间移动变更，而是 **在两个任意提交中移动变更**，则可以：

```sh
hj squash -i --from A --into B
```

这会将提交 A 中的部分变更移动到提交 B 中。其中 `-i` 参数表示交互式选择变更，如果不提供则表示将提交 A 整体合并入提交 B 中。

更多用法参见 [jj squash](https://jj-vcs.github.io/jj/latest/cli-reference/#jj-squash)。

---

## 拆分提交

::: details 对于熟悉 git 的用户

你可以用交互式 rebase：

```bash
git rebase -i HEAD~3
```

- 找到要拆分的提交（例如 pick abc123）
- 将它改为 edit
- 保存并退出编辑器
- `git reset HEAD^` → `git add -p` → 多次提交

这只是其中一种实现方式。git 中不存在能完成这个操作的单行命令。

:::

::: details 对于熟悉 jj 的用户

`hj split` 和 `jj split` 完全等价。

:::

很多时候你会发现自己的提交粒度太大，应该拆分成多个。你可以使用：

```sh
hj split -r A -d B
```

这将将提交 A 中的部分变更拆分出来，作为一个新提交放到已有提交 B 之上。

更多用法参见 [jj split](https://jj-vcs.github.io/jj/latest/cli-reference/#jj-split)。

## 移动提交的位置

::: details 对于熟悉 git 的用户

如果你想改变提交的顺序，在同一个分支中可以使用交互式变基 `git rebase -i`；跨分支的操作需要 `git cherry-pick` 等操作。

git 中不存在能完成这个操作的单行命令。

:::

::: details 对于熟悉 jj 的用户

`hj rebase` 和 `jj rebase` 完全等价。

:::

一个非常好用的工具是 `rebase`，它可以将一个提交移动到另一个提交之上，不论这两个提交是否位于同一分支：

```sh
hj rebase -s A -d B
```

这将把提交 A 移动到提交 B 之上。

更多用法参见 [jj rebase](https://jj-vcs.github.io/jj/latest/cli-reference/#jj-rebase)。

## 删除提交

::: details 对于熟悉 git 的用户

`hj abandon B` 近似于：

```sh
git rebase --onto B^ B
```

特别地，如果 B 是合并提交，`B^`（ B 的父提交）可能存在多个。这种情况下你需要显式地指定，否则命令会失败。

:::

::: details 对于熟悉 jj 的用户

`hj abandon` 和 `jj abandon` 完全等价。

:::

`abandon` 命令可以用来删除一个提交：

```sh
hj abandon REVSET
```

这将移除这个提交，并将其子提交变基到其父提交上。

特别地，如果不指定 `REVSET`，将默认为工作副本（`@`）。此时会清空工作副本的所有变更，并自动地创建一个新工作副本提交。
