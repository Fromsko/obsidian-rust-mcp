#Requires -Version 5.0
param(
    [string]$InstallDir = "$env:LOCALAPPDATA\obsidian-mcp",
    [bool]$Build = $true,
    [bool]$Force = $false
)

$ErrorActionPreference = "Stop"

function Write-Success { param([string]$Message) Write-Host "[OK] $Message" -ForegroundColor Green }
function Write-Info { param([string]$Message) Write-Host "[INFO] $Message" -ForegroundColor Cyan }
function Write-Warn { param([string]$Message) Write-Host "[WARN] $Message" -ForegroundColor Yellow }
function Write-Fail { param([string]$Message) Write-Host "[FAIL] $Message" -ForegroundColor Red }

Write-Host ""
Write-Host "==============================================================" -ForegroundColor Magenta
Write-Host "     Obsidian Rust MCP 安装脚本 v0.1.4" -ForegroundColor Magenta
Write-Host "==============================================================" -ForegroundColor Magenta
Write-Host ""
Write-Host "配置信息：" -ForegroundColor Yellow
Write-Host "   安装目录: $InstallDir"
Write-Host "   本地构建: $Build"
Write-Host "   强制覆盖: $Force"
Write-Host ""

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$binaryPath = Join-Path $projectRoot "target\release\obsidian-mcp.exe"
$targetPath = Join-Path $InstallDir "obsidian-mcp.exe"

if ($Force) {
    Write-Host "正在检查旧进程..." -ForegroundColor Cyan
    $runningProcesses = Get-Process -Name "obsidian-mcp" -ErrorAction SilentlyContinue

    if ($runningProcesses) {
        Write-Warn "发现正在运行的 obsidian-mcp 进程，准备停止..."
        foreach ($proc in $runningProcesses) {
            Write-Info "正在停止进程 PID: $($proc.Id)"
            try {
                Stop-Process -Id $proc.Id -Force -ErrorAction Stop
            }
            catch {
                Write-Warn "停止进程失败: $($_.Exception.Message)"
            }
        }
        Start-Sleep -Seconds 1
        Write-Success "旧进程已停止"
    }
    else {
        Write-Info "没有发现运行中的进程"
    }
    Write-Host ""
}

if ($Build) {
    Write-Host "正在本地构建项目..." -ForegroundColor Cyan

    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    if (-not $rustc) {
        Write-Fail "未找到 Rust 环境，请先安装 Rust: https://rustup.rs"
        exit 1
    }

    Push-Location $projectRoot
    try {
        Write-Info "执行 cargo build --release..."
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            Write-Fail "构建失败"
            exit 1
        }
        Write-Success "构建完成"
    }
    finally {
        Pop-Location
    }
    Write-Host ""
}

if (-not (Test-Path $binaryPath)) {
    Write-Fail "未找到构建产物: $binaryPath"
    Write-Info "请先执行 cargo build --release，或使用 -Build `$true"
    exit 1
}

Write-Host "正在创建安装目录..." -ForegroundColor Cyan
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Success "创建目录: $InstallDir"
}
else {
    Write-Info "安装目录已存在"
}
Write-Host ""

Write-Host "正在覆盖复制二进制文件..." -ForegroundColor Cyan
try {
    Copy-Item $binaryPath $targetPath -Force
    Write-Success "已复制到: $targetPath"
}
catch {
    Write-Fail "复制失败: $($_.Exception.Message)"
    Write-Info "如果目标文件正在被占用，请使用 -Force `$true 再试一次"
    exit 1
}

Write-Host ""
Write-Host "==============================================================" -ForegroundColor Green
Write-Success "安装完成！"
Write-Host "==============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "源文件: $binaryPath"
Write-Host "目标文件: $targetPath"
Write-Host ""
Write-Host "说明：本脚本仅负责本地构建和覆盖安装，不修改 MCP 客户端配置。"
Write-Host ""
