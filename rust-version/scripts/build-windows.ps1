#Requires -Version 5.1
<#
.SYNOPSIS
    Windows 平台打包脚本
.DESCRIPTION
    用于在 Windows 本地构建 Tauri 应用的 PowerShell 脚本
.EXAMPLE
    .\scripts\build-windows.ps1
#>

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

# 颜色输出函数
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Show-Header {
    Write-ColorOutput "========================================" Cyan
    Write-ColorOutput "  Windows 平台打包脚本" White
    Write-ColorOutput "========================================" Cyan
    Write-Host ""
}

function Test-Environment {
    Write-ColorOutput "📋 检查打包环境..." Yellow
    
    # 检查 Node.js
    try {
        $nodeVersion = node --version
        Write-ColorOutput "  ✓ Node.js: $nodeVersion" Green
    }
    catch {
        Write-ColorOutput "  ✗ Node.js 未安装或未添加到 PATH" Red
        exit 1
    }
    
    # 检查 npm
    try {
        $npmVersion = npm --version
        Write-ColorOutput "  ✓ npm: $npmVersion" Green
    }
    catch {
        Write-ColorOutput "  ✗ npm 未找到" Red
        exit 1
    }
    
    # 检查 Rust
    try {
        $rustVersion = rustc --version
        Write-ColorOutput "  ✓ Rust: $rustVersion" Green
    }
    catch {
        Write-ColorOutput "  ✗ Rust 未安装或未添加到 PATH" Red
        Write-ColorOutput "    请访问 https://rustup.rs/ 安装 Rust" Yellow
        exit 1
    }
    
    # 检查 Cargo
    try {
        $cargoVersion = cargo --version
        Write-ColorOutput "  ✓ Cargo: $cargoVersion" Green
    }
    catch {
        Write-ColorOutput "  ✗ Cargo 未找到" Red
        exit 1
    }
    
    Write-ColorOutput "✓ 环境检查通过" Green
    Write-Host ""
}

function Install-Dependencies {
    Write-ColorOutput "📦 安装依赖..." Yellow
    try {
        npm install
        Write-ColorOutput "✓ 依赖安装完成" Green
    }
    catch {
        Write-ColorOutput "✗ 依赖安装失败" Red
        exit 1
    }
    Write-Host ""
}

function Build-Frontend {
    Write-ColorOutput "🔨 构建前端项目..." Yellow
    try {
        npm run build
        Write-ColorOutput "✓ 前端构建完成" Green
    }
    catch {
        Write-ColorOutput "✗ 前端构建失败" Red
        exit 1
    }
    Write-Host ""
}

function Build-WindowsApp {
    Write-ColorOutput "🪟 开始构建 Windows 应用..." Yellow
    Write-ColorOutput "   目标平台: x86_64-pc-windows-msvc" Cyan
    Write-Host ""
    
    try {
        npm run tauri build -- --target x86_64-pc-windows-msvc
        Write-ColorOutput "✓ Windows 应用构建完成" Green
    }
    catch {
        Write-ColorOutput "✗ Windows 应用构建失败" Red
        exit 1
    }
    Write-Host ""
}

function Get-LatestFile {
    param(
        [string]$Directory,
        [string]$Filter
    )
    
    if (-not (Test-Path $Directory)) {
        return $null
    }
    
    $files = Get-ChildItem -Path $Directory -Filter $Filter -File | 
        Sort-Object LastWriteTime -Descending
    
    return $files | Select-Object -First 1
}

function Show-BuildOutputs {
    Write-ColorOutput "📂 查找生成的安装包..." Yellow
    
    $projectRoot = Split-Path -Parent $PSScriptRoot
    $targetDir = Join-Path $projectRoot "src-tauri\target\x86_64-pc-windows-msvc\release\bundle"
    
    $foundFiles = @()
    
    # 查找 MSI 安装包
    $msiDir = Join-Path $targetDir "msi"
    if (Test-Path $msiDir) {
        $msiFile = Get-LatestFile -Directory $msiDir -Filter "*.msi"
        if ($msiFile) {
            $foundFiles += @{
                Type = "MSI 安装包"
                Name = $msiFile.Name
                Path = $msiFile.FullName
                Size = "{0:N2} MB" -f ($msiFile.Length / 1MB)
            }
        }
    }
    
    # 查找 NSIS 安装包
    $nsisDir = Join-Path $targetDir "nsis"
    if (Test-Path $nsisDir) {
        $exeFile = Get-LatestFile -Directory $nsisDir -Filter "*.exe"
        if ($exeFile) {
            $foundFiles += @{
                Type = "EXE 安装包 (NSIS)"
                Name = $exeFile.Name
                Path = $exeFile.FullName
                Size = "{0:N2} MB" -f ($exeFile.Length / 1MB)
            }
        }
    }
    
    if ($foundFiles.Count -gt 0) {
        Write-ColorOutput "✓ 找到以下生成的文件:" Green
        Write-Host ""
        foreach ($file in $foundFiles) {
            Write-ColorOutput "  [$($file.Type)]" Cyan
            Write-ColorOutput "    文件: $($file.Name)" White
            Write-ColorOutput "    路径: $($file.Path)" Gray
            Write-ColorOutput "    大小: $($file.Size)" Gray
            Write-Host ""
        }
    }
    else {
        Write-ColorOutput "⚠ 未找到生成的安装包文件" Yellow
    }
}

function Show-Footer {
    Write-ColorOutput "========================================" Cyan
    Write-ColorOutput "  Windows 打包流程完成!" White
    Write-ColorOutput "========================================" Cyan
}

# 主执行流程
Show-Header
Test-Environment
Install-Dependencies
Build-Frontend
Build-WindowsApp
Show-BuildOutputs
Show-Footer
