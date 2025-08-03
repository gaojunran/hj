# 配置

hj 提供了很多个性化配置，你可以使用它们加速你的工作流。

## 读取配置

hj 的配置分为以下层级：

1. [hj 的默认配置](https://github.com/gaojunran/hj/blob/main/src/config.rs)；

2. 全局配置：尝试读取环境变量 `$XDG_CONFIG_HOME`，若存在则为 `$XDG_CONFIG_HOME/hj/config.toml`；若不存在则为 `~/.config/hj/config.toml`；

3. 项目级配置：项目根目录下的 `hj.toml` 文件；

4. 环境变量：`HJ__*` 开头的环境变量。例如配置项 `open_config.editor` 对应的环境变量为 `HJ__OPEN_CONFIG__EDITOR`（每个 `.` 都应该被替换成双下划线 `__`）。

## 配置项

完整的配置项参见 [config.rs](https://github.com/gaojunran/hj/blob/main/src/config.rs)。
