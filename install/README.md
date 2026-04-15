# Windows 一键安装

本目录包含 Windows PowerShell 一键安装脚本，用于**本地构建并覆盖安装** Obsidian Rust MCP。

## 文件

| 文件 | 说明 |
|------|------|
| `install.ps1` | PowerShell 安装脚本 |

## 设计原则

- **仅做安装**：脚本只负责本地构建、停止旧进程、覆盖复制二进制文件
- **不改客户端配置**：不会自动修改 Claude Desktop 或其他 MCP 客户端配置
- **支持强制覆盖**：可选 `-Force $true`，先停止旧进程再替换

## 快速开始

### 默认安装

```powershell
.\install\install.ps1
```

### 指定安装目录

```powershell
.\install\install.ps1 -InstallDir "D:\Tools\obsidian-mcp"
```

### 覆盖安装到指定目录

```powershell
.\install\install.ps1 -InstallDir "C:\Users\Administrator\go\bin" -Force $true
```

## 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `InstallDir` | string | `$env:LOCALAPPDATA\obsidian-mcp` | 安装目录 |
| `Build` | bool | `$true` | 是否先本地构建项目 |
| `Force` | bool | `$false` | 是否停止旧进程并强制覆盖 |

## 使用示例

### 1. 本地构建并安装

```powershell
.\install\install.ps1 -Build $true
```

### 2. 不构建，直接安装已有二进制

```powershell
.\install\install.ps1 -Build $false
```

### 3. 停止旧进程并覆盖安装

```powershell
.\install\install.ps1 -InstallDir "C:\Users\Administrator\go\bin" -Force $true -Build $false
```

### 4. 完整示例

```powershell
.\install\install.ps1 -InstallDir "D:\Apps\obsidian-mcp" -Build $true -Force $true
```

## 安装逻辑

1. 可选：检测并停止正在运行的 `obsidian-mcp` 进程（`-Force $true`）
2. 可选：执行本地构建 `cargo build --release`
3. 检查本地构建产物 `target\release\obsidian-mcp.exe`
4. 创建安装目录
5. 覆盖复制到目标路径

## 注意事项

### 1. 需要 Rust 环境

如果启用 `-Build $true`，请确保已安装 Rust：

https://rustup.rs

### 2. 目标文件被占用

如果目标文件正在运行或被占用，请使用：

```powershell
.\install\install.ps1 -InstallDir "C:\Users\Administrator\go\bin" -Force $true
```

### 3. PowerShell 执行策略

如果提示脚本执行被禁止：

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## 说明

该脚本**不会自动写入 MCP 客户端配置**。安装完成后，请自行在 Claude Desktop、Cursor 或其他 MCP 客户端中配置可执行文件路径。
