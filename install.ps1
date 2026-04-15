#Requires -Version 5.0
<#
.SYNOPSIS
    Obsidian Rust MCP 一键安装脚本

.DESCRIPTION
    在 Windows 上安装 Obsidian Rust MCP 服务器到指定目录

.PARAMETER InstallDir
    安装目录，默认为 "$env:LOCALAPPDATA\obsidian-mcp"

.PARAMETER McpName
    MCP 服务名称，默认为 "obsidian-mcp"

.PARAMETER VaultRoot
    Obsidian 知识库根目录，用于生成 MCP 配置

.PARAMETER Build
    是否先构建项目，默认为 true

.EXAMPLE
    # 默认安装
    .\install.ps1

.EXAMPLE
    # 指定安装目录和名称
    .\install.ps1 -InstallDir "D:\Tools\obsidian-mcp" -McpName "my-obsidian"

.EXAMPLE
    # 指定知识库路径
    .\install.ps1 -VaultRoot "D:\notes\MyVault"
#>

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\obsidian-mcp",
    [string]$McpName = "obsidian-mcp",
    [string]$VaultRoot = "",
    [bool]$Build = $true
)

$ErrorActionPreference = "Stop"

# 颜色定义
function Write-Success { param([string]$Message) Write-Host "[OK] $Message" -ForegroundColor Green }
function Write-Info { param([string]$Message) Write-Host "[INFO] $Message" -ForegroundColor Cyan }
function Write-Warn { param([string]$Message) Write-Host "[WARN] $Message" -ForegroundColor Yellow }
function Write-Fail { param([string]$Message) Write-Host "[FAIL] $Message" -ForegroundColor Red }

Write-Host ""
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Magenta
Write-Host "     Obsidian Rust MCP 安装脚本 v0.1.4" -ForegroundColor Magenta
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Magenta
Write-Host ""

# 显示配置
Write-Host "📋 安装配置：" -ForegroundColor Yellow
Write-Host "   安装目录: $InstallDir"
Write-Host "   MCP 名称: $McpName"
if ($VaultRoot) {
    Write-Host "   知识库路径: $VaultRoot"
}
Write-Host ""

# 1. 构建项目
if ($Build) {
    Write-Host "🔨 正在构建项目..." -ForegroundColor Cyan

    # 检查 Rust 环境
    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    if (-not $rustc) {
        Write-Fail "未找到 Rust 环境，请先安装 Rust: https://rustup.rs"
        exit 1
    }

    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $projectDir = $scriptDir

    Write-Info "项目目录: $projectDir"

    # 构建 Release 版本
    Push-Location $projectDir
    try {
        Write-Info "执行 cargo build --release..."
        cargo build --release 2>&1 | ForEach-Object { Write-Host $_ }

        if ($LASTEXITCODE -ne 0) {
            Write-Fail "构建失败"
            exit 1
        }

        Write-Success "构建完成"
    }
    finally {
        Pop-Location
    }
}

# 2. 创建安装目录
Write-Host ""
Write-Host "📁 正在创建安装目录..." -ForegroundColor Cyan

if (Test-Path $InstallDir) {
    Write-Warn "安装目录已存在，将覆盖现有文件"
} else {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Success "创建目录: $InstallDir"
}

# 3. 复制二进制文件
Write-Host ""
Write-Host "📦 正在复制文件..." -ForegroundColor Cyan

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectDir = $scriptDir
$binaryPath = "$projectDir\target\release\obsidian-mcp.exe"
$targetPath = "$InstallDir\obsidian-mcp.exe"

if (-not (Test-Path $binaryPath)) {
    Write-Fail "未找到构建产物: $binaryPath"
    Write-Info "请确保已执行构建，或使用 -Build:$false 并确保二进制文件存在"
    exit 1
}

Copy-Item $binaryPath $targetPath -Force
Write-Success "复制 obsidian-mcp.exe"

# 4. 复制配置文件示例
$configExample = "$InstallDir\config.example.txt"
$configContent = @"
# Obsidian Rust MCP 配置文件
# 使用前请复制到对应位置并修改配置

# 安装信息
INSTALL_DIR=$InstallDir
MCP_NAME=$McpName

# 知识库路径（可选，覆盖环境变量）
"@

if ($VaultRoot) {
    $configContent += "`nOBSIDIAN_VAULT_ROOT=$VaultRoot"
}

$configContent | Out-File $configExample -Encoding UTF8
Write-Success "生成配置示例: $configExample"

# 5. 生成 MCP 配置文件
Write-Host ""
Write-Host "🔧 生成 MCP 配置..." -ForegroundColor Cyan

# 检测 Claude Desktop 配置路径
$claudeDir = "$env:APPDATA\Claude"
$claudeConfig = "$claudeDir\claude_desktop_config.json"

if (Test-Path $claudeConfig) {
    Write-Info "找到 Claude Desktop 配置: $claudeConfig"

    # 读取现有配置
    $config = Get-Content $claudeConfig -Raw | ConvertFrom-Json

    # 检查是否已存在该 MCP
    $existingMcp = $config.mcpServers.$McpName
    if ($existingMcp) {
        Write-Warn "MCP '$McpName' 已存在，将更新配置"
    }

    # 构建 env 对象
    $envObj = @{}
    if ($VaultRoot) {
        $envObj["OBSIDIAN_VAULT_ROOT"] = $VaultRoot
    }

    # 更新配置
    if ($config.mcpServers) {
        $config.mcpServers | Add-Member -NotePropertyName $McpName -NotePropertyValue @{
            command = $targetPath
        } -Force
        if ($envObj.Count -gt 0) {
            $config.mcpServers.$McpName | Add-Member -NotePropertyName "env" -NotePropertyValue $envObj -Force
        }
    } else {
        $mcpEntry = @{
            command = $targetPath
        }
        if ($envObj.Count -gt 0) {
            $mcpEntry["env"] = $envObj
        }
        $config | Add-Member -NotePropertyName "mcpServers" -NotePropertyValue @{ $McpName = $mcpEntry } -Force
    }

    # 保存配置
    $config | ConvertTo-Json -Depth 10 | Out-File $claudeConfig -Encoding UTF8
    Write-Success "已更新 Claude Desktop 配置"

    Write-Host ""
    Write-Host "💡 请重启 Claude Desktop 以使配置生效" -ForegroundColor Yellow
} else {
    Write-Warn "未找到 Claude Desktop 配置: $claudeConfig"
    Write-Info "请手动添加以下配置到 Claude Desktop 的 mcpServers:"

    $manualConfig = @"
  "$McpName": {
    "command": "$targetPath"$($VaultRoot ? ",`n    `"env`": { `"OBSIDIAN_VAULT_ROOT`": `"$VaultRoot`" }" : "")
  }
"@
    Write-Host $manualConfig -ForegroundColor Gray
}

# 6. 输出完成信息
Write-Host ""
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Green
Write-Success "安装完成!"
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "📍 安装目录: $InstallDir"
Write-Host "📦 二进制文件: $targetPath"

if ($VaultRoot) {
    Write-Host "📚 知识库路径: $VaultRoot"
}

Write-Host ""
Write-Host "🔧 使用方法：" -ForegroundColor Yellow
Write-Host "   1. 直接运行: $targetPath"
Write-Host "   2. 或添加到系统 PATH"
Write-Host ""
Write-Host "📖 更多信息: https://github.com/Fromsko/obsidian-rust-mcp"
Write-Host ""
