# Obsidian 知识库操作规范

> 本文件面向 AI 代理（LLM），规范对此 Obsidian Vault 的所有读写操作。

## 知识库根目录

```
D:\notes\Fromsko
```

---

## 目录结构与分区职责

| 目录 | 用途 | 写入规则 |
|------|------|---------|
| `tech/` | 技术知识（Docker、Linux、Git、SSH、VSCode、Zed、Rust、Wails、部署） | 按主题合并，一个主题一篇，禁止拆散 |
| `ai/` | AI 工程（AGENTS.md 规范、MCP 开发、AI 工具对比、Kiro、Prompt） | 同上 |
| `projects/` | 项目实践文档 | 按项目名建子目录，如 `projects/easytier/` |
| `methods/` | 学习方法论、思维模型、工作流 | 只存方法论，不存技术细节 |
| `career/` | 简历、面试、心态管理 | 个人职业相关 |
| `ideas/` | 项目构想和设计草案 | 未实现的想法 |
| `cheatsheet/` | 速查手册（API 密钥、工具路径、代理脚本、代码片段） | 短小精悍，速查为主 |
| `journal/` | 工作日志，按 `journal/YYYY-MM/` 组织 | 日志文件名格式：`YYYY-MM-DD-主题.md` |

---

## 文件命名规范

- **全部使用英文小写 + 短横线**：`docker-guide.md`、`mcp-development.md`
- **禁止**：中文文件名、数字编号前缀（如 `06-01-001-xxx.md`）、空格
- **语义化**：文件名应能直接反映内容主题

---

## Frontmatter 格式（必须）

每个 `.md` 文件必须包含以下 YAML Frontmatter：

```yaml
---
tags:
  - tag1
  - tag2
aliases:
  - 中文别名
created: YYYY-MM-DD
updated: YYYY-MM-DD
status: active  # active | archived | draft
---
```

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

1. 先查阅 `README.md` 总索引确定文件所属分区
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
3. **命名** — 英文小写短横线，语义化
4. **格式** — 必须包含 Frontmatter + 概述 Callout + 相关笔记
5. **路径** — 只在上述 8 个分区目录中创建，禁止在根目录或其他位置创建

### 更新现有文件

1. **更新 `updated` 日期** 为当天
2. **追加内容放在合适的章节下**，不要打乱已有结构
3. **保持 Callout 和 Wikilinks 风格一致**
4. **不删除已有内容**，除非用户明确要求

### 禁止事项

- ❌ 在根目录创建随意文件
- ❌ 使用中文文件名或带空格的文件名
- ❌ 创建与已有文件主题重复的新文件
- ❌ 使用数字编号前缀命名（如 `01-xxx.md`）
- ❌ 删除或覆盖 `cheatsheet/api-keys.md` 中的密钥
- ❌ 在非 `journal/` 目录写日志类内容
- ❌ 输出时省略 Frontmatter
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
| 查找 Docker 笔记 | `README.md` → tech/ → `[[docker-guide]]` |
| 新增 MCP 内容 | 更新 `ai/mcp-development.md`，不要新建文件 |
| 记录新项目 | `projects/<project-name>/` 新建子目录 |
| 添加 API 密钥 | 追加到 `cheatsheet/api-keys.md` |
| 写工作日志 | `journal/YYYY-MM/YYYY-MM-DD-主题.md` |
| 记录新工具 | 判断属于 `tech/` 还是 `cheatsheet/`，追加或新建 |

----



# Fromsko 知识库

> [!abstract] 概述
> 个人技术笔记与知识管理系统，涵盖技术实践、AI 工程、项目经验、方法论和职业发展。

---

## 目录结构

```
├── tech/          技术知识（8篇）
├── ai/            AI 工程（5篇）
├── projects/      项目实践
│   ├── easytier/  EasyTier P2P VPN（20篇）
│   └── cmp/       CMP 排障（1篇）
├── methods/       方法论（3篇）
├── career/        职业发展（3篇）
├── ideas/         项目构想（1篇）
├── cheatsheet/    速查手册（4篇）
└── journal/       工作日志
```

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
| `easytier/` | EasyTier P2P VPN 项目深度学习（20篇） |
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

## MCP 工具调用示例

> 本节面向 LLM，提供四个工具的正确和错误调用示例。严格按照示例格式传参。

### note_index_tree — 获取文件树和标签索引

无需任何参数，直接调用即可。

```json
✅ 正确：
{}

❌ 错误：
{"path": "tech/"}          // 不接受任何参数
{"directory": "ai"}        // 不接受任何参数
```

### write_note_tips — 查阅写入规范

无需任何参数，直接调用即可。首次操作知识库前**必须**调用此工具。

```json
✅ 正确：
{}

❌ 错误：
{"topic": "frontmatter"}   // 不接受任何参数
```

### query_note — 搜索笔记

支持三种查询模式混合使用，**至少提供一个参数**。

```json
✅ 按标签搜索（多标签取交集）：
{"tags": ["docker", "linux"]}

✅ 精确文件名匹配（不含 .md 后缀）：
{"exact_name": "docker-guide"}

✅ 模糊关键词搜索（匹配文件名、别名、标签、路径）：
{"keyword": "Docker"}

✅ 混合查询（标签 + 关键词）：
{"tags": ["rust"], "keyword": "mcp"}

✅ 三种同时使用：
{"tags": ["ai"], "exact_name": "mcp-development", "keyword": "SDK"}

❌ 不传任何参数：
{}
// 错误：至少提供 tags、exact_name 或 keyword 中的一个

❌ 参数名拼写错误：
{"tag": ["docker"]}        // 应该是 "tags"（复数）
{"name": "docker-guide"}   // 应该是 "exact_name"
{"search": "Docker"}       // 应该是 "keyword"

❌ 类型错误：
{"tags": "docker"}         // tags 必须是数组，应该是 ["docker"]
{"exact_name": ["a","b"]}  // exact_name 必须是字符串
```

### read_note — 读取笔记完整内容

传入笔记的相对路径（从 `query_note` 或 `note_index_tree` 的返回结果中获取）。

```json
✅ 读取指定笔记：
{"path": "tech/docker-guide.md"}

✅ 读取子目录下的笔记：
{"path": "projects/easytier/01-project-overview.md"}

✅ 读取 cheatsheet：
{"path": "cheatsheet/api-keys.md"}

❌ 路径为空：
{}
// 错误：缺少 path 参数

❌ 使用绝对路径：
{"path": "D:\\notes\\Fromsko\\tech\\docker-guide.md"}
// 应该用相对路径 "tech/docker-guide.md"

❌ 路径穿越：
{"path": "../secret.md"}
// 错误：路径不能包含 ..

❌ 文件不存在：
{"path": "tech/not-exist.md"}
// 错误：文件不存在
```

### write_note — 写入笔记

所有 6 个参数**全部必填**。Frontmatter 由服务自动生成，content 中不要包含 frontmatter。

```json
✅ 创建新笔记：
{
  "directory": "tech",
  "filename": "nginx-guide",
  "tags": ["nginx", "deploy", "linux"],
  "aliases": ["Nginx 配置指南"],
  "status": "active",
  "content": "> [!abstract] 概述\n> Nginx 反向代理与负载均衡配置指南。\n\n## 基础配置\n\n```nginx\nserver {\n    listen 80;\n    server_name example.com;\n}\n```\n\n## 相关笔记\n\n- [[docker-guide]]\n- [[server-deploy]]"
}

✅ 追加到已有笔记（文件已存在时自动追加，updated 日期自动更新）：
{
  "directory": "ai",
  "filename": "mcp-development",
  "tags": ["mcp", "rust"],
  "aliases": ["MCP 开发指南"],
  "status": "active",
  "content": "## 新增章节\n\n这里是追加的内容。"
}

✅ 创建工作日志：
{
  "directory": "journal",
  "filename": "2026-02-12-obsidian-mcp",
  "tags": ["journal", "mcp", "rust"],
  "aliases": ["Obsidian MCP 开发日志"],
  "status": "active",
  "content": "> [!abstract] 概述\n> 今天完成了 Obsidian MCP 服务的开发。\n\n## 完成事项\n\n- 实现了四个工具\n- 编译通过\n\n## 相关笔记\n\n- [[mcp-development]]"
}

❌ 缺少必填参数：
{"directory": "tech", "filename": "test"}
// 错误：缺少 tags, aliases, status, content

❌ 无效的目录名：
{"directory": "docs", ...}
// 错误：目录必须是 tech/ai/projects/methods/career/ideas/cheatsheet/journal 之一

❌ 文件名包含中文或大写：
{"filename": "Docker指南", ...}    // 禁止中文
{"filename": "Docker-Guide", ...}  // 必须全小写
{"filename": "my guide", ...}      // 禁止空格

❌ 无效的状态值：
{"status": "published", ...}       // 必须是 active/archived/draft 之一

❌ content 中包含 frontmatter：
{"content": "---\ntags:\n  - test\n---\n正文", ...}
// 错误：frontmatter 由服务自动生成，content 只写正文

❌ content 中缺少概述 Callout：
{"content": "## 标题\n\n直接写内容", ...}
// 应该以 > [!abstract] 概述 开头

❌ content 中缺少相关笔记章节：
{"content": "> [!abstract] 概述\n> 内容\n\n## 正文\n\n...", ...}
// 末尾必须有 ## 相关笔记 章节
```

### 典型工作流

1. 首次使用 → 调用 `write_note_tips` 查阅规范
2. 了解库结构 → 调用 `note_index_tree` 查看文件树和标签
3. 查找笔记 → 调用 `query_note` 搜索是否已有同主题文件
4. 阅读笔记 → 调用 `read_note` 读取笔记完整内容
5. 写入笔记 → 调用 `write_note` 创建或追加内容
