# 贡献指南

## ALL in [mise](https://mise.jdx.dev)

本项目的开发环境配置均在项目目录的 `mise.toml` 中定义。

## 开发环境

本项目中使用了 Rust 1.91.0（目前仍为 nightly 版本）；要构建文档，需要 Node.js 22 和较新版本的 pnpm。你可以通过 `mise` 来批量安装项目需要的开发环境：

```bash
mise trust  # 信任 mise.toml
mise install
```

## 任务

`mise.toml` 的 `tasks` 里定义了项目中可执行的任务。你可以通过运行命令：

```bash
mise run [TASK]
```

来执行任务。

## 提交前钩子

在提交前会自动运行 hj 的 `pre-commit` 钩子，用于做 lint 和 format 操作。如果你习惯于用 VS Code 或其它自带 lint & format 的开发环境，可以禁用这个钩子。
