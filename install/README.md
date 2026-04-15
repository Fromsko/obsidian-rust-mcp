# Windows 一键安装

本目录包含 Windows PowerShell 一键安装脚本，可快速安装 Obsidian Rust MCP。

## 安装脚本

| 文件 | 说明 |
|------|------|
| `install.ps1` | PowerShell 安装脚本 |

## 快速开始

### 方式一：克隆后直接运行

```powershell
git clone https://github.com/Fromsko/obsidian-rust-mcp.git
cd obsidian-rust-mcp
.\install.ps1
```

### 方式二：指定参数安装

```powershell
# 指定安装目录和 MCP 名称
.\install.ps1 -InstallDir "D:\Tools\obsidian-mcp" -McpName "my-obsidian"

# 指定知识库路径
.\install.ps1 -VaultRoot "D:\notes\MyVault"

# 组合使用
.\install.ps1 -InstallDir "D:\Tools\mcp" -McpName "vault" -VaultRoot "D:\notes\vault"
```

## 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `InstallDir` | string | `$env:LOCALAPPDATA\obsidian-mcp` | 安装目录 |
| `McpName` | string | `obsidian-mcp` | MCP 服务名称 |
| `VaultRoot` | string | (空) | Obsidian 知识库路径 |
| `Build` | bool | `$true` | 是否先构建项目 |

## 使用示例

### 1. 基础安装（自动构建）

```powershell
.\install.ps1
```

### 2. 仅安装不构建（需要已有 Release 二进制）

```powershell
.\install.ps1 -Build $false
```

### 3. 自定义安装

```powershell
# 安装到 D:\Tools\mcp，命名为 "my-vault"
.\install.ps1 -InstallDir "D:\Tools\mcp" -McpName "my-vault"
```

### 4. 指定知识库路径

```powershell
.\install.ps1 -VaultRoot "D:\notes\MyObsidianVault"
```

### 5. 完整参数

```powershell
.\install.ps1 -InstallDir "D:\Apps\obsidian-mcp" -McpName "notes" -VaultRoot "D:\notes\vault" -Build $true
```

## 安装后

1. **Claude Desktop 用户**：重启 Claude Desktop 即可使用
2. **其他 MCP 客户端**：将安装目录添加到 PATH，或使用完整路径

## 卸载

```powershell
# 删除安装目录
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\obsidian-mcp"

# 从 Claude Desktop 移除配置（需手动编辑 claude_desktop_config.json）
```

## 故障排除

### Execution Policy 错误

```powershell
# 如果遇到脚本执行策略错误
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\install.ps1
```

### 未找到 Rust 环境

确保已安装 Rust：https://rustup.rs

### 构建失败

```powershell
# 手动构建
cargo build --release

# 然后跳过构建步骤
.\install.ps1 -Build $false
```
