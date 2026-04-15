# Obsidian 知识库操作规范

> 本文件面向 AI 代理（LLM），规范对此 Obsidian Vault 的所有读写操作。

## 目录结构与分区职责

| 目录 | 用途 |
|------|------|
| `tech/` | 技术知识（Docker、Linux、Git、SSH、VSCode、Zed、Rust、Wails、部署） |
| `ai/` | AI 工程（AGENTS.md 规范、MCP 开发、AI 工具对比、Kiro、Prompt） |
| `projects/` | 项目实践文档，**必须按项目名建立子目录**，如 `projects/easytier/`、`projects/chatbot/` |
| `methods/` | 学习方法论、思维模型、工作流 |
| `career/` | 简历、面试、心态管理 |
| `ideas/` | 项目构想和设计草案 |
| `cheatsheet/` | 速查手册（API 密钥、工具路径、代理脚本、代码片段） |
| `journal/` | 工作日志，按 `journal/YYYY-MM/` 组织，文件名格式：`YYYY-MM-DD-主题.md` |

> ✅ **明确允许分层存放**：支持子目录，如 `projects/easytier`、`journal/2026-03`、`ai/mcp/protocol`，最多 3 层。  
> ❌ **不要强行单层平铺**，特别是项目类内容，禁止把同一项目的多篇文档直接乱放在 `projects/` 根下。

---

## 文件组织规范

### 一般文档

- **语义化**：文件名应能直接反映内容主题
- 例如：`docker-guide`、`mcp-development`、`workflow-ai-notes`

### 项目类文档（重点）

项目文档**推荐采用目录化 + 编号化**组织，而不是随意堆叠。

#### 推荐结构

```text
projects/
  easytier/
    README.md                  # 项目目录说明 / 导航页
    01-project-overview.md     # 项目概览
    02-network-protocols.md    # 核心模块 1
    03-routing-system.md       # 核心模块 2
    04-nat-traversal.md        # 核心模块 3
```

#### 推荐规则

1. **每个项目一个目录**，不要把不同项目混在一起
2. **建议项目目录内提供 `README.md`**，作为该项目的目录页 / 导航页
3. **建议每篇项目文档带编号**，如：
   - `01-project-overview.md`
   - `02-architecture.md`
   - `03-api-design.md`
   - `04-deployment.md`
4. 如果某篇文档还要继续细分，可使用更细粒度编号，例如：
   - `01-01-overview.md`
   - `01-02-goals.md`
   - `02-01-backend-architecture.md`
5. 编号的目的不是形式化，而是为了：
   - 建立阅读顺序
   - 形成目录感
   - 避免项目文档无序堆放

> ✅ 推荐：`projects/xxx/01-xx-xxx.md`  
> ✅ 推荐：`projects/xxx/README.md` 作为导航页  
> ❌ 不推荐：把项目的所有分析、日志、设计稿都散乱放在 `projects/` 根目录

---

## README / 目录页规范（项目类强烈推荐）

对于项目目录，推荐提供一个 `README.md` 或等价目录页，至少包含：

1. **项目一句话摘要**
2. **项目核心目标 / 核心价值**
3. **目录索引**（每篇文档做什么）
4. **推荐阅读顺序**

### 示例

```markdown
# EasyTier 项目文档

> [!abstract] 概述
> EasyTier 是一个 P2P VPN / 异地组网项目，核心目标是实现稳定的内网穿透和虚拟组网。

## 文档目录

- [[01-project-overview|01 项目概览]] - 说明项目目标、定位、组件组成
- [[02-network-protocols|02 网络协议]] - 梳理协议层设计
- [[03-routing-system|03 路由系统]] - 解释路径选择与流量转发

## 推荐阅读顺序

1. 项目概览
2. 架构总览
3. 核心模块拆解
```

> 项目类知识如果没有目录感，后续检索和维护都会越来越乱。  
> 所以**优先建立 README 导航页 + 编号文档体系**。

---

## Obsidian 格式规范

### Callout 使用

```markdown
> [!abstract] 概述      # 每篇文件开头必须有
> [!tip] 最佳实践       # 推荐做法
> [!warning] 注意       # 易踩坑
> [!danger] 安全警告    # 涉及密钥、破坏性操作
> [!example] 示例       # 代码或使用场景
> [!info] 补充信息      # 背景资料
> [!quote] 引用         # 引用他人话语
```

### 内部链接

- 使用 Wikilinks：`[[docker-guide]]`
- 章节引用：`[[docker-guide#容器网络]]`
- 自定义显示：`[[docker-guide|Docker 指南]]`
- **禁止** Markdown 链接引用本库内文件：`[Docker](./docker-guide.md)` ← 不要这样

### 文件末尾

每篇文件末尾必须有 `## 相关笔记` 章节，使用 Wikilinks 列出关联文件。

---

## 查询操作规范

### 查找文件

1. 先调用 `note_index_tree` 查看文件树确定文件所属分区
2. 按分区目录查找目标文件
3. 使用文件名（英文）或 `aliases`（中文别名）定位

### 搜索内容

- 按 `tags` 搜索：如搜索所有 `docker` 相关 → 查 `tags` 含 `docker` 的文件
- 按分区浏览：直接列出对应目录下的文件
- 全文搜索：搜索关键词

---

## 写入操作规范

### 创建新文件

1. **确定分区** — 根据内容选择正确的目录
2. **检查是否已有同主题文件** — 有则追加/更新，禁止创建重复文件
3. **命名** — 语义化英文文件名
4. **格式** — 必须包含概述 Callout + 相关笔记章节
5. **路径** — 只在上述 8 个分区目录中创建
6. **项目类内容优先目录化** — 新项目先建子目录，再考虑写 `README.md` 和编号文档

### 更新现有文件

1. **追加内容放在合适的章节下**，不要打乱已有结构
2. **保持 Callout 和 Wikilinks 风格一致**
3. **不删除已有内容**，除非用户明确要求

> `updated` 日期由程序自动更新。Frontmatter 由服务自动生成，content 中不要包含。

### 禁止事项

- ❌ 创建与已有文件主题重复的新文件
- ❌ 删除或覆盖 `cheatsheet/api-keys.md` 中的密钥
- ❌ 在非 `journal/` 目录写日志类内容
- ❌ 把同一项目的大量文档无序散落到 `projects/` 根目录
- ❌ 项目类文档完全没有导航页、编号、摘要，导致目录失控
- ❌ content 中缺少概述 Callout（`> [!abstract] 概述`）
- ❌ content 中缺少 `## 相关笔记` 章节
- ❌ 使用 Markdown 链接替代 Wikilinks 引用本库内文件

---

## 内容精简原则

- **合并** — 同主题碎片笔记合并到一篇
- **精简** — 去除大段复制的官方文档，只保留核心命令和个人注释
- **保留** — 有实际踩坑经验的内容、自己写的代码片段
- **表格** — 对比信息优先用表格展示
- **代码块** — 命令和配置必须用代码块包裹

---

## 快速参考

| 操作 | 步骤 |
|------|------|
| 查找 Docker 笔记 | `note_index_tree` → tech/ → `[[docker-guide]]` |
| 新增 MCP 内容 | 更新 `ai/mcp-development.md`，不要新建文件 |
| 记录新项目 | `projects/<project-name>/` 新建子目录，并优先创建 `README.md` |
| 添加 API 密钥 | 追加到 `cheatsheet/api-keys.md` |
| 写工作日志 | `journal/YYYY-MM/YYYY-MM-DD-主题.md` |
| 记录新工具 | 判断属于 `tech/` 还是 `cheatsheet/`，追加或新建 |

----

# Fromsko 知识库

> [!abstract] 概述
> 个人技术笔记与知识管理系统，涵盖技术实践、AI 工程、项目经验、方法论和职业发展。

---

## tech/ — 技术知识

| 文件 | 内容 |
|------|------|
| [[docker-guide]] | Docker 安装、配置、网络、服务部署 |
| [[linux-commands]] | Linux 命令、fd/rg 工具、压缩、Termux |
| [[git-and-gitea]] | Git 版本管理、Gitea 平台、Actions Runner |
| [[ssh-and-network]] | SSH 公钥、WSL2 网络、端口映射 |
| [[vscode-extension-dev]] | VSCode 插件开发核心 API |
| [[zed-extension-dev]] | Zed 编辑器 Rust+WASM 插件开发 |
| [[rust-toolchain]] | rust-analyzer 配置与故障排查 |
| [[wails-quickstart]] | Wails 桌面应用框架入门 |
| [[server-deploy]] | Vercel Serverless 部署与 MCP 集成 |

---

## ai/ — AI 工程

| 文件 | 内容 |
|------|------|
| [[agents-md-guide]] | AGENTS.md 跨工具 AI 指令标准 |
| [[mcp-development]] | MCP SDK 开发（Python/TS/Go） |
| [[ai-coding-tools]] | Cline/Codex/Claude Code/Crush 对比 |
| [[kiro-steering]] | Kiro IDE Steering 三层架构设计 |
| [[prompt-engineering]] | 提示词模板与 Shell 助手 Prompt |

---

## projects/ — 项目实践

| 文件 | 内容 |
|------|------|
| `easytier/` | EasyTier P2P VPN 项目深度学习（目录化、编号化组织） |
| [[release-troubleshoot]] | CMP GitHub Actions 发布排障 |

---

## methods/ — 方法论

| 文件 | 内容 |
|------|------|
| [[learning-system]] | 学习体系：阅读、笔记、知识构建 |
| [[problem-decomposition]] | 问题拆解六步法与思考模型 |
| [[workflow-ai-notes]] | AI 辅助笔记工作流 |

---

## career/ — 职业发展

| 文件 | 内容 |
|------|------|
| [[resume]] | 个人简历与技术栈 |
| [[interview-tips]] | 面试心得与项目经验展示 |
| [[mindset]] | 心态管理与成长思维 |

---

## ideas/ — 项目构想

| 文件 | 内容 |
|------|------|
| [[license-platform]] | 授权码平台架构设计 |

---

## cheatsheet/ — 速查手册

| 文件 | 内容 |
|------|------|
| [[api-keys]] | 各平台 API 密钥汇总 |
| [[tool-paths]] | 工具配置路径速查 |
| [[proxy-toggle]] | 代理开关脚本 |
| [[shell-snippets]] | 常用代码片段 |

---

## journal/ — 工作日志

| 文件 | 内容 |
|------|------|
| [[journal/README\|日志索引]] | 按月份组织的工作记录 |
| [[personality]] | ENFP-T 人格分析 |

---

## MCP 工具调用速记

- `note_index_tree`：查看文件树与标签
- `query_note`：按 `tags` / `exact_name` / `keyword` 搜索
- `read_note`：按相对路径读取全文
- `write_note`：写入或更新笔记
- `delete_note`：删除笔记

### 最小示例

```json
{"tags": ["docker"]}
{"path": "tech/docker-guide.md"}
{"directory": "projects/easytier", "filename": "01-project-overview", "tags": ["project"], "aliases": ["项目概览"], "status": "active", "content": "> [!abstract] 概述\n> 内容\n\n## 相关笔记\n\n- [[README]]"}
```

### 典型工作流

1. 先看 `write_note_tips`
2. 再用 `note_index_tree` 看结构
3. 用 `query_note` 查重
4. 用 `read_note` 阅读已有内容
5. 项目类优先创建项目目录、README 和编号文档
6. 最后用 `write_note` 写入
