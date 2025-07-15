# 分支

:::details 对于熟悉 git 的用户

hj 中的分支模型和 git 中不同。每次提交（commit）后，hj 不会像 git 一样将分支的头指针指向新的提交；更新分支头指针只会发生在 `hj push` 和 `hj switch` 时。

:::

:::details 对于熟悉 jj 的用户

hj 的分支（书签）模型基于 jj 改进而来。`hj push` 和 `hj switch` 的操作都会自动推进书签到当前最新的提交。注意，如果你使用其它命令，例如 `hj new / edit / prev` 等能改变工作副本指向的命令，书签不会自动推送！你需要手动调用 `hj keepup` 命令来推送书签后，再尝试改变工作副本指向。

:::
