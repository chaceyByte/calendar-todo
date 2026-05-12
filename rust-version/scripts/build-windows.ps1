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
    Write-ColorOutput "Checking environment..." Yellow
    
    # 检查 Node.js
    try {
        $nodeVersion = node --version
        Write-ColorOutput "  [OK] Node.js: $nodeVersion" Green
    }
    catch {
        Write-ColorOutput "  [ERROR] Node.js not installed or not in PATH" Red
        exit 1
    }
    
    # 检查 npm
    try {
        $npmVersion = npm --version
        Write-ColorOutput "  [OK] npm: $npmVersion" Green
    }
    catch {
        Write-ColorOutput "  [ERROR] npm not found" Red
        exit 1
    }
    
    # 检查 Rust
    try {
        $rustVersion = rustc --version
        Write-ColorOutput "  [OK] Rust: $rustVersion" Green
    }
    catch {
        Write-ColorOutput "  [ERROR] Rust not installed or not in PATH" Red
        Write-ColorOutput "  Please install Rust from https://rustup.rs/" Yellow
        exit 1
    }
    
    # 检查 Cargo
    try {
        $cargoVersion = cargo --version
        Write-ColorOutput "  [OK] Cargo: $cargoVersion" Green
    }
    catch {
        Write-ColorOutput "  [ERROR] Cargo not found" Red
        exit 1
    }
    
    Write-ColorOutput "Environment check passed" Green
    Write-Host ""
}

function Install-Dependencies {
    Write-ColorOutput "Installing dependencies..." Yellow
    try {
        npm install
        Write-ColorOutput "Dependencies installed" Green
    }
    catch {
        Write-ColorOutput "Failed to install dependencies" Red
        exit 1
    }
    Write-Host ""
}

function Build-Frontend {
    Write-ColorOutput "Building frontend..." Yellow
    try {
        npm run build
        Write-ColorOutput "Frontend build completed" Green
    }
    catch {
        Write-ColorOutput "Frontend build failed" Red
        exit 1
    }
    Write-Host ""
}

function Build-WindowsApp {
    Write-ColorOutput "Building Windows application..." Yellow
    Write-ColorOutput "   Target: x86_64-pc-windows-msvc" Cyan
    Write-Host ""
    
    try {
        npm run tauri build -- --target x86_64-pc-windows-msvc
        Write-ColorOutput "Windows application build completed" Green
    }
    catch {
        Write-ColorOutput "Windows application build failed" Red
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
    Write-ColorOutput "Looking for generated installer files..." Yellow
    
    $projectRoot = Split-Path -Parent $PSScriptRoot
    $targetDir = Join-Path $projectRoot "src-tauri\target\x86_64-pc-windows-msvc\release\bundle"
    
    $foundFiles = @()
    
    # 查找 MSI 安装包
    $msiDir = Join-Path $targetDir "msi"
    if (Test-Path $msiDir) {
        $msiFile = Get-LatestFile -Directory $msiDir -Filter "*.msi"
        if ($msiFile) {
            $sizeMB = [math]::Round($msiFile.Length / 1MB, 2)
            $sizeStr = "$sizeMB MB"
            $fileInfo = New-Object PSObject
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Type -Value "MSI Installer"
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Name -Value $msiFile.Name
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Path -Value $msiFile.FullName
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Size -Value $sizeStr
            $foundFiles += $fileInfo
        }
    }
    
    # 查找 NSIS 安装包
    $nsisDir = Join-Path $targetDir "nsis"
    if (Test-Path $nsisDir) {
        $exeFile = Get-LatestFile -Directory $nsisDir -Filter "*.exe"
        if ($exeFile) {
            $sizeMB = [math]::Round($exeFile.Length / 1MB, 2)
            $sizeStr = "$sizeMB MB"
            $fileInfo = New-Object PSObject
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Type -Value "EXE Installer (NSIS)"
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Name -Value $exeFile.Name
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Path -Value $exeFile.FullName
            Add-Member -InputObject $fileInfo -MemberType NoteProperty -Name Size -Value $sizeStr
            $foundFiles += $fileInfo
        }
    }
    
    if ($foundFiles.Count -gt 0) {
        Write-ColorOutput "Found generated files:" Green
        Write-Host ""
        foreach ($file in $foundFiles) {
            Write-ColorOutput "  [$($file.Type)]" Cyan
            Write-ColorOutput "    File: $($file.Name)" White
            Write-ColorOutput "    Path: $($file.Path)" Gray
            Write-ColorOutput "    Size: $($file.Size)" Gray
            Write-Host ""
        }
    }
    else {
        Write-ColorOutput "Warning: No installer files found" Yellow
    }
}

function Show-Footer {
    Write-ColorOutput "========================================" Cyan
    Write-ColorOutput "  Windows build completed!" White
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
