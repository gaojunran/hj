# 分支



## 分支的「书签」模型

:::details 对于熟悉 git 的用户

hj 中的分支模型和 git 中不同。每次提交（commit）后，hj 不会像 git 一样将分支的头指针指向新的提交；更新分支头指针只会发生在 `hj push` 和 `hj switch` 时。

:::

:::details 对于熟悉 jj 的用户

hj 的分支（书签）模型基于 jj 改进而来。`hj push` 和 `hj switch` 的操作都会自动推进书签到当前最新的提交。注意，如果你使用其它命令，例如 `hj new / edit / prev` 等能改变工作副本指向的命令，书签不会自动推进！你需要手动调用 `hj keepup` 命令来推进书签后，再尝试改变工作副本指向。

:::

TODO

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
hj keepup  # jj bookmark move --from "heads(::@- & bookmarks())" --to @-
jj edit "latest(main+)" 
```

如果目标分支存在多个工作副本提交，`hj switch` 默认会选择最新的一个。

不像 git 一样，hj 切换分支一定会成功，因为其工作副本被视作一个提交。 

:::

切换分支操作将移动你的工作副本，使其指向另一个分支的头提交。你可以使用：

```sh
hj switch BRANCH
```

来切换到一个分支。



