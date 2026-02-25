<!--
```yaml
project: Obsidian Rust MCP
description: 基于 Rust 构建的高性能 Obsidian 知识库 MCP 服务器
language: Rust
version: 0.1.2
author: Fromsko
email: fromsko@example.com
license: MIT
keywords:
  - MCP
  - Obsidian
  - Rust
  - 知识管理
  - 模型上下文协议
  - 笔记管理
  - 文件树索引
  - 智能搜索
  - 标签系统
  - 高性能
repository: https://github.com/fromsko/obsidian-rust-mcp
documentation: https://github.com/fromsko/obsidian-rust-mcp/blob/main/README_CN.md
```
-->


# Obsidian Rust MCP

[English](./README.md)

基于 Rust 构建的高性能 Obsidian 知识库 MCP（模型上下文协议）服务器。

## 功能特性

- 📂 **文件树索引** - 获取完整的知识库结构和标签概览
- 🔍 **智能搜索** - 通过标签、精确文件名或模糊关键词查询笔记
- 📝 **笔记管理** - 读写笔记，自动生成 Frontmatter
- 🏷️ **标签系统** - 使用标签和别名组织笔记
- ⚡ **高性能** - 使用 Rust 构建，速度快且可靠

## 安装

```bash
cargo build --release
```

## 使用方法

服务器提供以下 MCP 工具：

### `note_index_tree`
获取知识库的完整文件树和所有可用标签。

### `query_note`
使用以下方式搜索笔记：
- **标签**: 按一个或多个标签过滤（取交集）
- **精确文件名**: 匹配精确的文件名（不含 .md）
- **关键词**: 在文件名、别名和标签中模糊搜索

示例：
```json
{"tags": ["docker"]}
{"exact_name": "docker-guide"}
{"keyword": "Docker"}
{"tags": ["rust"], "keyword": "mcp"}
```

### `read_note`
通过相对路径读取笔记的完整内容。

示例：
```json
{"path": "tech/docker-guide.md"}
```

### `write_note`
创建或追加笔记内容，自动生成 Frontmatter。

示例：
```json
{
  "directory": "tech",
  "filename": "nginx-guide",
  "tags": ["nginx"],
  "aliases": ["Nginx 指南"],
  "status": "active",
  "content": "> [!abstract] 概述\n> 内容\n\n## 相关笔记\n\n- [[docker-guide]]"
}
```

### `write_note_tips`
获取知识库的完整写入规范（目录结构、命名规范、Frontmatter 格式等）。

## 配置

### 选项 1：环境变量（推荐）
设置 `OBSIDIAN_VAULT_ROOT` 环境变量指向您的 Obsidian 知识库：

```bash
# Linux/macOS
export OBSIDIAN_VAULT_ROOT="/path/to/your/vault"

# Windows (cmd)
set OBSIDIAN_VAULT_ROOT=D:\notes\Fromsko

# Windows (PowerShell)
$env:OBSIDIAN_VAULT_ROOT="D:\notes\Fromsko"
```

### 选项 2：硬编码路径
编辑 `src/main.rs` 中的 `VAULT_ROOT` 常量：

```rust
const VAULT_ROOT: &str = r"D:\notes\Fromsko";
```

### 选项 3：MCP 客户端配置（推荐用于 MCP 客户端）
在您的 MCP 客户端配置中直接设置知识库路径：

```json
{
  "fromsko-note": {
    "command": "/path/to/obsidian-mcp",
    "disabled": false,
    "env": {
      "OBSIDIAN_VAULT_ROOT": "/path/to/your/vault"
    }
  }
}
```

将 `/path/to/obsidian-mcp` 替换为编译后的二进制文件实际路径，将 `/path/to/your/vault` 替换为您的 Obsidian 知识库路径。

**注意**：当与 Claude Desktop、Cursor 或其他 MCP 兼容工具一起使用时，这是推荐的方法。

## 有效目录

笔记可以组织在以下目录中：
- `tech` - 技术笔记
- `ai` - AI/机器学习相关笔记
- `projects` - 项目文档
- `methods` - 方法论和流程
- `career` - 职业发展
- `ideas` - 想法和头脑风暴
- `cheatsheet` - 快速参考指南
- `journal` - 日常日志

## 截图

### 代理集成
![代理集成](docs/imgs/agents_readme.png)

### 笔记示例
![笔记示例](docs/imgs/random_note.png)

## 许可证

MIT - 详见 [LICENSE](./LICENSE) 文件。
