# Prady Compiler Installer for Windows
# Usage: irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Installing Prady Toolchain (v1.0.0)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

$PradyHome = Join-Path $HOME ".prady"
$PradyBin = Join-Path $PradyHome "bin"

# 1. Create directory structure
if (-not (Test-Path $PradyBin)) {
    New-Item -ItemType Directory -Path $PradyBin -Force | Out-Null
}

$Installed = $false

# 2. Check if local release binaries exist in current directory or repo
$LocalPrady = Join-Path $PSScriptRoot "target\release\prady.exe"
$LocalLsp = Join-Path $PSScriptRoot "target\release\prady-lsp.exe"

if ((Test-Path $LocalPrady) -and (Test-Path $LocalLsp)) {
    Write-Host "Copying locally built binaries to $PradyBin..." -ForegroundColor Green
    Copy-Item $LocalPrady -Destination $PradyBin -Force
    Copy-Item $LocalLsp -Destination $PradyBin -Force
    $Installed = $true
} else {
    # 3. Download from GitHub Release
    $ZipUrl = "https://github.com/technopradyumn/prady/releases/download/v1.0.0/prady-v1.0.0-x86_64-pc-windows-msvc.zip"
    $ZipPath = Join-Path $env:TEMP "prady-v1.0.0.zip"
    
    Write-Host "Downloading Prady binary release from GitHub..." -ForegroundColor Yellow
    try {
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Invoke-WebRequest -Uri $ZipUrl -OutFile $ZipPath -UseBasicParsing
        Expand-Archive -Path $ZipPath -DestinationPath $PradyBin -Force
        Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue
        $Installed = $true
    } catch {
        Write-Host "Release download failed: $_" -ForegroundColor Red
        Write-Host "If running from source, run: cargo build --release" -ForegroundColor Yellow
        exit 1
    }
}

# 4. Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$PathEntries = $UserPath -split ';' | Where-Object { $_ -ne "" }

if ($PathEntries -notcontains $PradyBin) {
    Write-Host "Adding $PradyBin to User PATH..." -ForegroundColor Green
    $NewPath = ($PathEntries + $PradyBin) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    $env:PATH = "$PradyBin;$env:PATH"
} else {
    Write-Host "$PradyBin is already in your PATH." -ForegroundColor Gray
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "  Prady installed successfully!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host "Compiler location: $(Join-Path $PradyBin 'prady.exe')"
Write-Host "LSP location:      $(Join-Path $PradyBin 'prady-lsp.exe')"
Write-Host ""
Write-Host "Restart your terminal, then verify with:" -ForegroundColor Cyan
Write-Host "    prady version" -ForegroundColor White
Write-Host "    prady run hello.pr" -ForegroundColor White
