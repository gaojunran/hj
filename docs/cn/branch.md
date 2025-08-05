# 分支/书签

## 分支的「书签」模型

:::details 对于熟悉 git 的用户

hj 中的分支模型和 git 中不同。每次提交（commit）后，hj 不会像 git 一样将分支的头指针指向新的提交；更新分支头指针只会发生在 `hj push` 和 `hj switch` 时。

:::

:::details 对于熟悉 jj 的用户

hj 的分支（书签）模型基于 jj 改进而来。`hj push` 和 `hj switch` 的操作都会自动推进书签到当前最新的提交。注意，如果你使用其它命令，例如 `hj new / edit / prev` 等能改变工作副本指向的命令，书签不会自动推进！你需要手动调用 `hj keepup` 命令来推进书签后，再尝试改变工作副本指向。

:::

在 hj 中，分支功能以「书签」的形式提供。你可以使用：

```sh
hj bookmark set BRANCH -r REVSET
```

来创建一个书签，并指向一个特定的提交。

从这个书签到根提交的路径上的所有提交构成了这个分支。


## 切换分支

:::details 对于熟悉 git 的用户

`hj switch main` 命令大致相当于：

```sh
git checkout main
```

这省略了保持工作区干净的步骤。在 git 中，只有工作区是干净的，才能切换分支。你可以使用 stash 或 restore 来保持工作区干净。

:::

:::details 对于熟悉 jj 的用户

`hj switch main` 命令实际上执行的是：

```sh
hj keepup  # 内部调用 `jj bookmark move`

# 检查目标书签的子提交中，有几个描述为空的提交
jj log -r "((main+) ~ bookmarks()) & description(exact:\"\")" 

# 如果数量为 0，则：
jj new main

# 如果数量大于 0，则：
jj edit "latest(((main+) ~ bookmarks()) & description(exact:\"\"))" 
```

如果目标分支存在多个工作副本提交，`hj switch` 默认会选择最新的一个。

不像 git 一样，hj 切换分支一定会成功，因为其工作副本被视作一个提交。 

---

`hj wip` 实际执行的是：

```sh
jj log -r "heads(:: ~ description(exact:'')).." --summary
```

找到所有描述为空的提交，并展示其变更的大致信息（`--summary`）。你还可以使用 `hj wip -p` 来展示详细变更。

:::

切换分支操作将移动你的工作副本，使其指向另一个分支的头提交。你可以使用：

```sh
hj switch BRANCH
```

来切换到另一个分支。你可以在这个分支的工作副本上继续工作、提交、推送；并随时切换回以前的分支。

事实上，如果你从一个 **工作副本中变更不为空** 的分支切换到另一个分支，原先的工作内容仍然以提交的形式存在。你可以运行：

```sh
hj wip
```

来查看项目中所有正在进行的工作内容。还可以附加 `-p` 参数来查看详细变更。

## 合并分支

:::details 对于熟悉 git 的用户

Git 和 Git Forge 都推崇基于分支的工作流。很多时候你需要将一个分支合入另一个分支中：

```sh
git checkout main
git merge feature
```

这将把 `feature` 分支的变更合入 `main` 分支中。

:::

:::details 对于熟悉 jj 的用户

`hj new` 和 `jj new` 完全等价。

:::

一个特性分支想要合入到主分支中，可以运行：

```sh
hj new feat-xxx main
```

这将创建一个新的提交，基于 `main` 书签指向的节点和 `feat-xxx` 书签指向的节点。

:::warning

合并多个节点而创建的新提交通常不应该作为工作副本！这个提交中只应该处理合并冲突和与合并节点相关的操作。要基于这个合并后的提交进行新工作，请使用：

```sh
hj new @
```

创建一个新提交。

:::

## 变基分支

:::details 对于熟悉 git 的用户

`hj rebase -b feat-xxx -d main` 大致等价于：

```sh
git checkout feat-xxx
git rebase main
```

变基在工作区不干净或者有冲突时可能会失败。

:::

:::details 对于熟悉 jj 的用户

`hj rebase` 和 `jj rebase` 完全等价。

详见 [更多用法](https://jj-vcs.github.io/jj/latest/cli-reference/#jj-rebase)。

:::

如果你想在合并两个分支的同时，保持提交历史的线性，可以使用：

```sh
hj rebase -b feat-xxx -d main
```

这将把 `feat-xxx` 分支分叉于 `main` 分支的部分「摘下来」，放在 `main` 书签之上。下图是变基前的状态，描述为 `3`, `4`, `5` 的提交为我们想要变基的部分。

![rebase1](/image4.png)

下图是变基后的状态：

![rebase2](/image5.png)

:::tip

`hj rebase` 命令非常强大！

- 你可以用 `-s ul` 替换 `-b` 参数，表示变基 `ul` 及 `ul` 的子节点。
- 你也可以用 `-r` 手动指定要变基的提交。参见 [移动提交的位置](/cn/commit.html#移动提交的位置)。
- 除了用 `-d` 表示变基的目标之外，你也可以用 `-A` 表示插入到目标之后、`-B` 表示插入到目标之前。

:::
