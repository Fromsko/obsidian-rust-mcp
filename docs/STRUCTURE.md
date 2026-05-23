# 项目结构

```
obsidian-rust-mcp/
├── Cargo.toml                 # crate 元数据与依赖
├── CHANGELOG.md
├── README.md / README_CN.md
│
├── docs/
│   ├── arch.md                # 架构与命令契约
│   ├── todo.md                # 实现清单
│   ├── STRUCTURE.md           # 本文件
│   ├── write-note-tips.md     # obsidian.guide 内容源
│   └── obsidian-mcp.example.json
│
├── .cursor/skills/obsidian-vault-mcp/
│
├── src/
│   ├── main.rs                # 二进制入口（stdio MCP）
│   ├── lib.rs                 # 库入口，供测试与集成
│   │
│   ├── config/                # AppConfig：env + JSON 文件
│   ├── validation/            # 目录/文件名/路径校验（依赖 config）
│   │
│   ├── vault/                 # 存储抽象
│   │   ├── backend.rs         # VaultBackend trait
│   │   ├── local.rs           # 本地文件系统
│   │   ├── cloud.rs           # 本地缓存 + HTTP 同步
│   │   └── mod.rs             # VaultHandle 工厂
│   │
│   ├── note/                  # 笔记域逻辑（无 I/O 策略）
│   │   ├── frontmatter.rs     # serde_yaml 解析/生成
│   │   ├── index.rs           # 全库索引构建
│   │   ├── file_tree.rs       # 树状目录渲染
│   │   └── semantic.rs        # 加权语义搜索
│   │
│   ├── service/               # ObsidianService 业务编排
│   ├── command/               # registry / help / dispatch
│   ├── mcp/                   # MCP 传输层（help + executeCommand）
│   └── types.rs               # 共享 DTO / 索引结构
│
└── tests/
    ├── service_integration.rs # 命令端到端（temp vault）
    ├── mcp_stdio.rs           # MCP 子进程集成
    └── registry.rs            # 命令注册表完整性
```

## 模块职责

| 层 | 模块 | 职责 |
|----|------|------|
| 入口 | `main` / `mcp` | stdio 传输，仅暴露 2 个 MCP tool |
| 命令 | `command` | CLI 风格路由、help 渲染、参数反序列化 |
| 业务 | `service` | guide / search / write / read / index / delete / semantic_search |
| 笔记 | `note` | frontmatter、索引、语义评分（纯函数为主） |
| 存储 | `vault` | Local / Cloud 后端，统一 read/write/delete |
| 配置 | `config` + `validation` | 可配置 VALID_DIRS、backend、cloud URL |

## 配置来源（优先级：env > 文件 > 默认）

| 变量 / 文件 | 说明 |
|-------------|------|
| `OBSIDIAN_VAULT_ROOT` | vault 根目录 |
| `OBSIDIAN_CONFIG` | JSON 配置文件路径 |
| `{vault}/obsidian-mcp.json` | vault 内默认配置文件 |
| `OBSIDIAN_VALID_DIRS` | 逗号分隔顶级目录白名单 |
| `OBSIDIAN_VALID_STATUSES` | 逗号分隔 status 白名单 |
| `OBSIDIAN_VAULT_BACKEND` | `local` \| `cloud` |
| `OBSIDIAN_CLOUD_URL` | Cloud API 基址（启用 cloud 同步） |
| `OBSIDIAN_CLOUD_TOKEN` | 可选 Bearer token |

## 典型调用链

```
MCP help / executeCommand
  → command::dispatch
    → service::ObsidianService
      → validation（路径校验）
      → vault（读写删）
      → note（索引 / frontmatter / semantic）
```
