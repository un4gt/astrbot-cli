# AstrBot CLI

用于管理 AstrBot 实例的命令行界面，具有增强的身份验证调试和持久凭证存储。

## 目录

- [介绍](#介绍)
- [安装](#安装)
- [使用](#使用)

## 介绍

AstrBot CLI 是一个强大的命令行工具，专为管理 AstrBot 实例而设计。它提供了插件管理和用户登录功能，支持从本地路径或 Git 仓库安装插件，并具备详细的日志输出选项。该工具基于 Rust 开发，确保高性能和可靠性。

### 主要功能

- **插件管理**：获取插件列表、安装、启用/禁用、重新加载和卸载插件。
- **用户登录**：支持用户名、密码和服务器 URL 的身份验证。
- **详细日志**：通过 `--verbose` 选项启用详细输出，便于调试。
- **持久凭证存储**：安全存储登录凭证，避免重复输入。

## 安装

### 前置要求

- [Rust](https://www.rust-lang.org/) (版本 1.70 或更高)
- Cargo (Rust 的包管理器，随 Rust 一起安装)

### 源码安装

1. **克隆仓库**：
   ```bash
   git clone https://github.com/un4gt/astrbot-cli.git
   cd astrbot-cli
   ```

2. **构建项目**：
   ```bash
   cargo build --release
   ```

3. **安装到系统**（可选）：
   ```bash
   cargo install --path .
   ```

### [GitHub Releases](https://github.com/un4gt/astrbot-cli/releases) 安装

- Linux
  ```bash
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/un4gt/astrbot-cli/releases/download/v0.1.2/astrbot-cli-installer.sh | sh
  ```
- Windows powershell
  ```powershell
  powershell -ExecutionPolicy Bypass -c "irm https://github.com/un4gt/astrbot-cli/releases/download/v0.1.2/astrbot-cli-installer.ps1 | iex"
  ```
- Macos
  ```bash
  brew install astrbot-cli
  ```

安装完成后，您可以使用 `astrbot` 命令开始使用工具。

## 使用

AstrBot CLI 提供了四个主要命令：`login`、`plugin`、`stat` 和 `log`。以下是详细的使用示例。

### 全局选项

- `--verbose` 或 `-v`：启用详细输出模式。

### 登录命令

用于登录到 AstrBot 服务器。

```bash
astrbot login --username <用户名> --password <密码> --server <服务器URL>
```

示例：
```bash
astrbot login --username myuser --password mypass --server https://astrbot.example.com
```

### 插件管理命令

#### 获取插件列表

```bash
astrbot plugin get
```

#### 安装插件

从本地路径安装：
```bash
astrbot plugin install --from-local
```

从 Git 仓库安装：
```bash
astrbot plugin install --from-git https://github.com/example/plugin-repo.git
```

#### 禁用插件

```bash
astrbot plugin off <插件名称>
```

示例：
```bash
astrbot plugin off my-plugin
```

#### 启用插件

```bash
astrbot plugin on <插件名称>
```

#### 重新加载插件

```bash
astrbot plugin reload <插件名称>
```

#### 卸载插件

```bash
astrbot plugin uninstall <插件名称>
```

### 统计命令

获取 AstrBot 实例的运行统计信息，包括消息数量、插件数量、运行时间、内存使用和 CPU 负载等。

```bash
astrbot stat
```

示例输出：
```
开始时间: 2025-09-13 17:59:36
消息平台:
  aiocqhttp: 60 (最后更新 2025-10-02 20:14:39)
消息总数: 1027
插件数量: 7
已运行:
482小时15分3秒
内存占用:
  进程: 239
  系统: 3400
CPU 负载: 2
```

### 日志命令

获取 AstrBot 实例的实时日志输出，用于监控和调试。

```bash
astrbot log
```

此命令将持续输出 AstrBot 的实时日志，直到手动停止（Ctrl+C）。

### 完整示例

启用详细输出并获取插件列表：
```bash
astrbot --verbose plugin get
```

登录并安装插件：
```bash
astrbot login --username admin --password secret --server https://api.astrbot.com
astrbot plugin install --from-git https://github.com/un4gt/astrbot-plugin-example.git
astrbot plugin on astrbot-plugin-example
```
