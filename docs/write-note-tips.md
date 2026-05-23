# Obsidian 知识库操作规范

> 本文件面向 AI 代理（LLM），规范对此 Obsidian Vault 的所有读写操作。

## 核心原则：树状分层，拒绝无意义平铺

知识库采用 **「顶级分区 → 主题子目录 → 具体文档」** 的树状结构组织。

- ✅ **默认多层目录**：同一主题的多篇笔记应归入同一子目录，而不是全部堆在分区根下
- ✅ **路径即语义**：`tech/docker/networking.md` 比根下的 `docker-networking.md` 更清晰
- ✅ **支持任意深度子目录**：按主题逐层细分，如 `ai/mcp/protocol/rmcp.md`、`projects/easytier/docs/api/endpoints.md`
- ❌ **禁止**把同一系列文档无结构地平铺在 `tech/`、`ai/` 等分区根目录
- ❌ **禁止**为了「少一层路径」而牺牲可读性和检索效率

```text
# 推荐 ✅
tech/
  docker/
    guide.md
    networking.md
  rust/
    toolchain.md
ai/
  mcp/
  protocol/
    rmcp.md
    overview.md
projects/
  easytier/
    README.md
    01-overview.md

# 不推荐 ❌（分区根下文件过多、无主题聚合）
tech/
  docker-guide.md
  docker-networking.md
  rust-toolchain.md
  linux-commands.md
  ...（几十篇平铺）
```

---

## 顶级分区职责

| 分区 | 用途 | 典型嵌套示例 |
|------|------|--------------|
| `tech/` | 技术知识（Docker、Linux、Git、Rust、部署…） | `tech/docker/`、`tech/rust/`、`tech/linux/` |
| `ai/` | AI 工程（MCP、Prompt、工具对比…） | `ai/mcp/`、`ai/prompt/`、`ai/tools/` |
| `projects/` | 项目实践文档 | `projects/easytier/`、`projects/chatbot/` |
| `methods/` | 学习方法论、思维模型、工作流 | `methods/learning/`、`methods/workflow/` |
| `career/` | 简历、面试、心态 | `career/interview/`、`career/resume/` |
| `ideas/` | 项目构想和设计草案 | `ideas/license-platform/` |
| `cheatsheet/` | 速查（密钥、路径、脚本、片段） | `cheatsheet/shell/`、`cheatsheet/api/` |
| `journal/` | 工作日志 | `journal/2026-03/`、`journal/2026-04/` |

> 分区只是**入口**，不是「所有文件必须直接躺在这里」。  
> 进入分区后，**继续按主题建子目录**，直到单目录内文件数量可控（通常 ≤ 10 篇）。

---

## 何时建子目录

| 场景 | 做法 |
|------|------|
| 同一主题 ≥ 2 篇相关笔记 | 建子目录，如 `tech/docker/` |
| 系列教程 / 多章文档 | 子目录 + 编号文件，如 `01-overview.md` |
| 单个大型项目 | `projects/<name>/` + `README.md` 导航 |
| 按时间归档的日志 | `journal/YYYY-MM/` |
| 仅 1 篇且短期不会扩展 | 可暂放分区根或一层子目录，但文件名须语义化 |

**经验法则**：打开某目录，若一眼看不出 5 秒内该点哪篇 → 说明该拆子目录了。

---

## 文件组织规范

### 一般文档

- **语义化文件名**：`networking.md`、`toolchain.md`（子目录已提供上下文，文件名不必重复前缀）
- **完整路径示例**：`tech/docker/networking.md`、`ai/mcp/rmcp-guide.md`
- 若仍在分区根下（仅适合单篇孤立笔记）：`tech/wails-quickstart.md`

### 项目类文档（重点）

项目文档**必须目录化**，推荐 **README 导航 + 编号文档**：

```text
projects/
  easytier/
    README.md                  # 项目导航页
    01-project-overview.md
    02-network-protocols.md
    03-routing-system.md
    04-nat-traversal.md
```

#### 项目目录规则

1. **每个项目独占一个目录**，不同项目不混放
2. **必须有 `README.md`** 作为目录页 / 导航页
3. **系列文档带编号**：`01-`、`02-`… 建立阅读顺序
4. 需要更细粒度时：`01-01-goals.md`、`02-01-backend.md`
5. 项目内若模块再拆分，可在项目目录下继续建子目录（深度不限，按主题自然分层）

> ✅ `projects/easytier/02-architecture.md`  
> ❌ `projects/easytier-architecture.md` 与 `projects/easytier-overview.md` 平铺在 `projects/` 根下

### 日志类文档

- 路径：`journal/YYYY-MM/YYYY-MM-DD-主题.md`
- 示例：`journal/2026-03/2026-03-26-mcp-refactor.md`

---

## README / 目录页规范

**项目目录**以及**文件数 ≥ 5 的主题子目录**，推荐提供 `README.md` 导航页，至少包含：

1. **一句话摘要**
2. **核心目标 / 价值**
3. **文档索引**（Wikilinks 列表）
4. **推荐阅读顺序**（可选）

### 示例（项目 README）

```markdown
# EasyTier 项目文档

> [!abstract] 概述
> EasyTier 是一个 P2P VPN / 异地组网项目。

## 文档目录

- [[01-project-overview|01 项目概览]]
- [[02-network-protocols|02 网络协议]]
- [[03-routing-system|03 路由系统]]

## 推荐阅读顺序

1. 项目概览 → 2. 网络协议 → 3. 路由系统
```

### 示例（主题子目录 README）

```markdown
# Docker 笔记

> [!abstract] 概述
> Docker 安装、网络、Compose 与部署实践。

## 文档

- [[guide|安装与入门]]
- [[networking|容器网络]]
- [[compose|Compose 编排]]
```

---

## Obsidian 格式规范

### Callout 使用

```markdown
> [!abstract] 概述      # 每篇文件开头必须有
> [!tip] 最佳实践
> [!warning] 注意
> [!danger] 安全警告    # 密钥、破坏性操作
> [!example] 示例
> [!info] 补充信息
> [!quote] 引用
```

### 内部链接

- Wikilinks：`[[networking]]`（同目录）或 `[[tech/docker/networking|容器网络]]`（跨目录带显示名）
- 章节引用：`[[guide#容器网络]]`
- **禁止** Markdown 链接引用本库内文件：`[Docker](./guide.md)` ← 不要这样

### 文件末尾

每篇文件末尾必须有 `## 相关笔记` 章节，用 Wikilinks 列出关联文件（可跨子目录）。

---

## 查询操作规范（MCP v0.4 CLI 模型）

| 目的 | 命令 |
|------|------|
| 阅读本规范 | `obsidian.guide` |
| 查看文件树 + 标签统计 | `obsidian.index` |
| 按标签 / 关键词 / 文件名搜索 | `obsidian.search` |
| 语义加权搜索（标题/标签/正文） | `obsidian.semantic_search`（detail-only） |
| 读取全文 | `obsidian.read` |

### 查找文件推荐流程

1. `obsidian.index` — 看树状结构，定位分区与子目录
2. `obsidian.search` — 按 `tags` / `keyword` / `exact_name` 缩小范围
3. `obsidian.read` — 按相对路径读取，如 `tech/docker/networking.md`

### 搜索示例

```json
{ "tags": ["docker"] }
{ "keyword": "nginx" }
{ "exact_name": "networking" }
{ "tags": ["mcp"], "include_index": true }
```

---

## 写入操作规范

### 创建新文件

1. **确定分区** — 选择正确的顶级目录
2. **确定子目录** — 优先放入已有主题子目录；无则新建（见「何时建子目录」）
3. **查重** — `obsidian.search` 检查是否已有同主题文件；有则追加，禁止重复创建
4. **命名** — 语义化英文小写 + 短横线；子目录内文件名不必重复主题前缀
5. **格式** — 概述 Callout + `## 相关笔记`
6. **路径深度** — 支持多层嵌套（如 `projects/easytier/docs/api/endpoints.md` ✅）

### write 参数示例

```json
{
  "directory": "tech/docker",
  "filename": "networking",
  "tags": ["docker", "network"],
  "aliases": ["容器网络"],
  "status": "active",
  "content": "> [!abstract] 概述\n> ...\n\n## 相关笔记\n\n- [[guide]]",
  "append": true
}
```

### 更新现有文件

1. 追加内容放在合适章节下，不打乱结构
2. 保持 Callout 和 Wikilinks 风格一致
3. 不删除已有内容，除非用户明确要求

> `updated` 日期与 Frontmatter 由服务自动维护，content 中不要手写 frontmatter。

### 禁止事项

- ❌ 同主题重复建文件
- ❌ 在分区根下无节制堆文件（应建子目录）
- ❌ 把项目文档散落在 `projects/` 根目录
- ❌ 项目目录无 README / 无导航
- ❌ 删除或覆盖 `cheatsheet/api-keys.md` 中的密钥
- ❌ 在非 `journal/` 写日志类内容
- ❌ content 缺少概述 Callout 或 `## 相关笔记`
- ❌ 用 Markdown 链接替代 Wikilinks 引用本库内文件

---

## 内容精简原则

- **合并** — 同主题碎片合并到一篇或同一子目录系列
- **精简** — 官方文档只保留核心命令与个人注释
- **保留** — 踩坑经验、自写代码片段
- **表格** — 对比信息优先用表格
- **代码块** — 命令和配置必须包裹在代码块中

---

## 知识库结构参考（树状，非平铺索引）

以下为**推荐组织方式**示意。实际 vault 可以也应该比此更深、更细分。

```text
tech/
  docker/
    guide.md
    networking.md
  linux/
    commands.md
  git/
    and-gitea.md
  rust/
    toolchain.md
  wails/
    quickstart.md
  server/
    deploy.md

ai/
  agents/
    md-guide.md
  mcp/
    development.md
    protocol/
      rmcp.md
      overview.md
      guides/
        migration.md

projects/
  easytier/
    README.md
    01-project-overview.md
    docs/
      api/
        endpoints.md
        auth.md
  tools/
    coding-tools-comparison.md
  prompt/
    engineering.md

projects/
  easytier/
    README.md
    01-project-overview.md
    02-network-protocols.md
  chatbot/
    README.md
    01-architecture.md

methods/
  learning/
    system.md
  workflow/
    ai-notes.md
  thinking/
    problem-decomposition.md

career/
  resume.md
  interview/
    tips.md
  mindset.md

ideas/
  license-platform/
    design.md

cheatsheet/
  api-keys.md
  tool-paths.md
  shell/
    snippets.md
  proxy/
    toggle.md

journal/
  2026-03/
    2026-03-26-mcp-refactor.md
  README.md                  # 日志总索引（可选）
```

> 上树仅作结构示范。写入前务必 `obsidian.index` 或 `obsidian.search` 确认实际路径，**不要假设文件仍在分区根下**。

---

## 快速参考

| 操作 | 步骤 |
|------|------|
| 了解规范 | `obsidian.guide` |
| 看整体结构 | `obsidian.index` |
| 找 Docker 笔记 | `obsidian.search` `{ "tags": ["docker"] }` → `obsidian.read` |
| 新增 MCP 内容 | 查 `ai/mcp/` 是否已有 → 追加或写入 `ai/mcp/<topic>.md` |
| 记录新项目 | 建 `projects/<name>/` + `README.md` + 编号文档 |
| 添加 API 密钥 | 追加到 `cheatsheet/api-keys.md` |
| 写工作日志 | `journal/YYYY-MM/YYYY-MM-DD-主题.md` |

### 典型工作流

1. `obsidian.guide` — 确认规范
2. `obsidian.index` — 看树状结构，定位分区与子目录
3. `obsidian.search` — 查重、找已有笔记
4. `obsidian.read` — 阅读待更新文件
5. 确定 `directory`（含子目录路径）→ `obsidian.write`
6. 项目类：先建目录 + README，再写编号系列文档

### MCP 工具速记

- `help` — 命令手册
- `executeCommand` — 执行 `obsidian.*` 命令

```json
{ "command": "obsidian.search", "args": { "tags": ["docker"] } }
{ "command": "obsidian.read", "args": { "path": "tech/docker/networking.md" } }
{ "command": "obsidian.write", "args": { "directory": "tech/docker", "filename": "networking", "tags": ["docker"], "aliases": [], "status": "active", "content": "..." } }
```
