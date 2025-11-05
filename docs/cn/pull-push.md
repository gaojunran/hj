# 拉取和推送

## 推送

::: details 对于熟悉 git 的用户

因为 `jj` 没有当前分支（当前书签）的概念，因此拉取和推送的操作都需要显式地声明被操作的分支；而 git 则需要切换到被操作的分支。

`hj push main`近似于：

```sh
git checkout main
git push
```

:::

::: details 对于熟悉 jj 的用户

和 `git` 不同，`jj` 不会在提交时自动移动书签（分支 HEAD）。因此提交后直接 `push` 会提示 `Nothing changed.`。`hj push main` 的实际操作是：

```sh
jj bookmark set -r "@-" main
jj git push --allow-new --bookmark main
```

你也可以禁用自动移动书签功能（这个功能在绝大多数场景下是有用的）：

```sh
HJ_PUSH_KEEPUP=false hj push main
```

我注意到 jj 中有一个 [PR](https://github.com/jj-vcs/jj/pull/3129) 实现了一个实验性的自动推进书签功能。当它稳定之后，我会修改 hj 的逻辑。

`hj push` 的实际操作是：

```sh
jj bookmark move --from "heads(::@- & bookmarks())" --to @-
jj git push --allow-new
```

:::

进行完一些提交工作后，可以使用：

```sh
hj push main
```

来推送 `main` 分支到默认的 `origin` 远程。你也可以在一个命令中携带多个参数，推送多个分支。

你甚至可以省略被推送的分支名。在这种情况下，将推送 **能到达当前工作副本路径** 上的所有分支：

```sh
hj push
```

:::warning

在推送分支时，**当前工作副本必须是被推送书签的后代**。否则将影响书签的移动而无法推送。

:::

### 从远程获取书签

为了更方便地控制从远程获取书签（bookmark），增加了 `hj fetch` 命令：

- 拉取远程变更：
  ```sh
  hj fetch
  ```
  
- 跟踪分支，并仅获取指定分支的远程变更：
  ```sh
  hj fetch main feature-A
  ```


## 拉取

::: details 对于熟悉 git 的用户

git 有 **远程分支** 的概念，所以需要进行实际的变基才能将远程变更同步到本地。

`hj pull BRANCH`近似于：

```sh
git checkout BRANCH
git pull --rebase
```

:::

::: details 对于熟悉 jj 的用户

jj 没有原生的 `jj git pull` 命令。`jj git fetch` 命令可以实现类似的功能，但不会将本地的新提交变基到远程新提交之上。`hj pull BRANCH` 的实际操作是：

```sh
jj git fetch   # 拉取所有跟踪分支
jj rebase -d BRANCH  # 如果不指定 branch，就没有这个操作
```

:::

使用：

```sh
hj pull main
```

来拉取 `main` 分支的远程变更到本地。此命令的默认变基行为使得本地分支如下图所示地排列：

```
本地分支和远程分支相同的部分 
                -> 远程分支相比本地分支新增的部分 
                                -> 本地分支相比远程分支新增的部分
```

如果不指定分支，执行：

```sh
hj pull
```

则只会拉取远程变更，不会执行任何变基操作。

更多操作参见 [与 GitHub PR 相关的操作](/cn/pr)。

## 拉取并推送

此外，还可以使用：

```sh
hj push main --pull
```

先拉取该远程分支的最新变更，再将变基后的本地分支推送到远程。

## 更新主干

::: details 对于熟悉 git 的用户

`hj upbase featA` 大致相当于：

```sh
git checkout main
git pull --rebase
git checkout featA
git rebase main
```

这个操作流程中省略了确保工作目录干净的步骤。git 的恼人之处之一在于，执行切换分支或变基操作时，经常需要你确保工作目录干净，你经常要与 `stash` 打交道。

而对于 `hj upbase`（更新所有基于主分支的分支），git 需要非常复杂的脚本来支持。

:::

::: details 对于熟悉 jj 的用户

`hj upbase` 的实际操作是：

```sh
jj git fetch   # 可以通过 upbase.fetch 配置项禁用
jj rebase -d "trunk()" -s "all:(::trunk())+ & mutable()"
```

`hj upbase featA featB` 的实际操作是：

```sh
jj git fetch   # 可以通过 upbase.fetch 配置项禁用
jj rebase -d "trunk()" -s featA
jj rebase -d "trunk()" -s featB
```

:::


假设一个工作场景：你在基于主分支 `main` 的一个分支 `feature-A` 上工作，在你发起 PR 之前，`main` 分支更新了。

此时的一个推荐做法是将你的分支更新到基于 **最新 `main` 分支** 的状态，再发起 PR：

```sh
hj upbase
```

这将更新所有基于主分支的分支。你也可以指定要更新的分支：

```sh
hj upbase featA featB
```

:::tip

如果你不在本地处理这些冲突，PR 创建后合并会失败，CI/CD 可能会失败，合并按钮会被禁用。

通过在本地提前合并/变基，你可以在发 PR 之前就解决冲突，减少代码审查者的负担。

:::

这个 `upbase` 操作往往是在推送前进行的，因此 `hj push` 命令也支持 `--upbase` 选项。
