#!/usr/bin/env pwsh
# Windows 安装脚本

$ErrorActionPreference = "Stop"

# 获取最新版本
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/fagao-ai/curl-transformer/releases/latest"
$version = $latestRelease.tag_name
Write-Host "Installing ct $version for Windows..."

# 下载
$downloadUrl = "https://github.com/fagao-ai/curl-transformer/releases/download/$version/ct-windows-amd64.exe"
$exePath = "$env:TEMP\ct.exe"

Write-Host "Downloading from $downloadUrl..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $exePath

# 安装
$binDir = "$env:USERPROFILE\.local\bin"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

Write-Host "Installing to $binDir..."
Move-Item -Path $exePath -Destination "$binDir\ct.exe" -Force

# 清理
Remove-Item -Path $exePath -Force -ErrorAction SilentlyContinue

Write-Host "✓ ct has been installed to $binDir\ct.exe"
Write-Host ""
Write-Host "Add to PATH if needed:"
Write-Host "  [Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';$binDir', 'User')"
Write-Host ""
Write-Host "Then restart your terminal and verify installation:"
Write-Host "  ct --version"
