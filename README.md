# Obsidian Rust MCP

[English](#english) | [中文](#中文)

---

## English

A high-performance MCP (Model Context Protocol) server for Obsidian knowledge base management, built with Rust.

### Features

- 📂 **File Tree Indexing** - Get complete vault structure and tag overview
- 🔍 **Smart Search** - Query notes by tags, exact name, or fuzzy keyword matching
- 📝 **Note Management** - Read and write notes with automatic frontmatter generation
- 🏷️ **Tag System** - Organize notes with tags and aliases
- ⚡ **High Performance** - Built with Rust for speed and reliability

### Installation

```bash
cargo build --release
```

### Usage

The server provides the following MCP tools:

#### `note_index_tree`
Get the complete file tree and all available tags in your vault.

#### `query_note`
Search notes using:
- **Tags**: Filter by one or more tags (intersection)
- **Exact name**: Match exact filename (without .md)
- **Keyword**: Fuzzy search across filenames, aliases, and tags

Example:
```json
{"tags": ["docker"]}
{"exact_name": "docker-guide"}
{"keyword": "Docker"}
{"tags": ["rust"], "keyword": "mcp"}
```

#### `read_note`
Read the complete content of a note by its relative path.

Example:
```json
{"path": "tech/docker-guide.md"}
```

#### `write_note`
Create or append to notes with automatic frontmatter generation.

Example:
```json
{
  "directory": "tech",
  "filename": "nginx-guide",
  "tags": ["nginx"],
  "aliases": ["Nginx Guide"],
  "status": "active",
  "content": "> [!abstract] Overview\n> Content\n\n## Related Notes\n\n- [[docker-guide]]"
}
```

#### `write_note_tips`
Get the complete writing guidelines for the vault (directory structure, naming conventions, frontmatter format, etc.).

### Configuration

Edit the `VAULT_ROOT` constant in `src/main.rs` to point to your Obsidian vault:

```rust
const VAULT_ROOT: &str = r"D:\notes\Fromsko";
```

### Valid Directories

Notes can be organized in the following directories:
- `tech` - Technical notes
- `ai` - AI/ML related notes
- `projects` - Project documentation
- `methods` - Methodologies and processes
- `career` - Career development
- `ideas` - Ideas and brainstorming
- `cheatsheet` - Quick reference guides
- `journal` - Daily journals

### License

MIT

---

## 中文

基于 Rust 构建的高性能 Obsidian 知识库 MCP（模型上下文协议）服务器。

### 功能特性

- 📂 **文件树索引** - 获取完整的知识库结构和标签概览
- 🔍 **智能搜索** - 通过标签、精确文件名或模糊关键词查询笔记
- 📝 **笔记管理** - 读写笔记，自动生成 Frontmatter
- 🏷️ **标签系统** - 使用标签和别名组织笔记
- ⚡ **高性能** - 使用 Rust 构建，速度快且可靠

### 安装

```bash
cargo build --release
```

### 使用方法

服务器提供以下 MCP 工具：

#### `note_index_tree`
获取知识库的完整文件树和所有可用标签。

#### `query_note`
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

#### `read_note`
通过相对路径读取笔记的完整内容。

示例：
```json
{"path": "tech/docker-guide.md"}
```

#### `write_note`
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

#### `write_note_tips`
获取知识库的完整写入规范（目录结构、命名规范、Frontmatter 格式等）。

### 配置

编辑 `src/main.rs` 中的 `VAULT_ROOT` 常量，指向你的 Obsidian 知识库：

```rust
const VAULT_ROOT: &str = r"D:\notes\Fromsko";
```

### 有效目录

笔记可以组织在以下目录中：
- `tech` - 技术笔记
- `ai` - AI/机器学习相关笔记
- `projects` - 项目文档
- `methods` - 方法论和流程
- `career` - 职业发展
- `ideas` - 想法和头脑风暴
- `cheatsheet` - 快速参考指南
- `journal` - 日常日志

### 许可证

MIT
