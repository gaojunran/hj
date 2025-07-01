# 拉取 / 推送

## 推送

::: details 对于熟悉 git 的用户

因为 `jj` 没有当前分支（当前书签）的概念，因此拉取和推送的操作都需要显式地声明被操作的分支。

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
HJ_PUSH_CONFIG_KEEPUP=false hj push main
```

我注意到 jj 中有一个 [PR](https://github.com/jj-vcs/jj/pull/3129) 实现了一个实验性的自动推进书签功能。当它稳定之后，我会修改 hj 的逻辑。

:::

进行完一些提交工作后，可以使用：

```sh
hj push main
```

来推送 `main` 分支到默认的 `origin` 远程。

:::warning

在推送一个分支时，当前工作副本必须在被推送分支之上（即成为被推送书签的后代）。否则将影响 `jj` 内部书签的移动而无法推送。

:::

:::tip 未来计划

未来 `hj push` 会有一个 `--pr` 选项，用于在推送后创建一个 GitHub PR。

:::

## 拉取

::: details 对于熟悉 git 的用户

git 有 **远程分支** 的概念，所以需要进行实际的变基才能将远程变更同步到本地。

`hj pull <branch>`近似于：

```sh
git checkout <branch>
git pull --rebase
```

:::

::: details 对于熟悉 jj 的用户

jj 没有原生的 `jj git pull` 命令。`jj git fetch` 命令可以实现类似的功能，但不会将本地的新提交变基到远程新提交之上。`hj pull <branch>` 的实际操作是：

```sh
jj git fetch   # 拉取所有跟踪分支
jj rebase -d <branch>
```

:::

使用：

```sh
hj pull main
```

来拉取 `main` 分支的远程变更到本地。此命令的默认行为使得本地分支如下图所示地排列：

```
本地分支和远程分支相同的部分 
                -> 远程分支相比本地分支新增的部分 
                                -> 本地分支相比远程分支新增的部分
```
