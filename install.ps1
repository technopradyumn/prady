# Prady Compiler Installer for Windows
# Usage:
#   PowerShell:     irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex
#   Command Prompt: powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex"

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Installing Prady Toolchain (v1.0.0 GA)" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

$PradyHome = Join-Path $HOME ".prady"
$PradyBin  = Join-Path $PradyHome "bin"

# 1. Create target directory
if (-not (Test-Path $PradyBin)) {
    New-Item -ItemType Directory -Path $PradyBin -Force | Out-Null
}

$Installed = $false

# 2. Check if running in a cloned local repo with pre-built release binaries
$LocalPrady = Join-Path $PSScriptRoot "target\release\prady.exe"
$LocalLsp   = Join-Path $PSScriptRoot "target\release\prady-lsp.exe"

if ((Test-Path $LocalPrady) -and (Test-Path $LocalLsp)) {
    Write-Host "Installing from local release build..." -ForegroundColor Green
    Copy-Item $LocalPrady -Destination $PradyBin -Force
    Copy-Item $LocalLsp   -Destination $PradyBin -Force
    $Installed = $true
} else {
    # 3. Direct automated download from official GitHub release
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $ZipPath = Join-Path $env:TEMP "prady-v1.0.0-windows-x64.zip"

    # Try downloading official release asset
    $DownloadUrls = @(
        "https://github.com/technopradyumn/prady/releases/download/v1.0.0/prady-v1.0.0-x86_64-pc-windows-msvc.zip",
        "https://github.com/technopradyumn/prady/releases/latest/download/prady-v1.0.0-x86_64-pc-windows-msvc.zip"
    )

    $DownloadSuccess = $false
    foreach ($Url in $DownloadUrls) {
        try {
            Write-Host "Downloading Prady binary package..." -ForegroundColor Yellow
            Invoke-WebRequest -Uri $Url -OutFile $ZipPath -UseBasicParsing
            $DownloadSuccess = $true
            break
        } catch {
            continue
        }
    }

    if ($DownloadSuccess) {
        Write-Host "Extracting binaries to $PradyBin..." -ForegroundColor Green
        Expand-Archive -Path $ZipPath -DestinationPath $PradyBin -Force
        Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue

        # Flatten in case archive contains a subfolder
        Get-ChildItem -Path $PradyBin -Filter "*.exe" -Recurse | ForEach-Object {
            if ($_.DirectoryName -ne $PradyBin) {
                Move-Item -Path $_.FullName -Destination $PradyBin -Force
            }
        }
        $Installed = $true
    } else {
        Write-Host "Direct download failed. Please ensure a GitHub Release v1.0.0 exists." -ForegroundColor Red
        Write-Host "Fallback: Build from source using 'cargo build --release'." -ForegroundColor Yellow
        exit 1
    }
}

# 4. Permanently register in User PATH Environment Variable
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$PathEntries = $UserPath -split ';' | Where-Object { $_ -ne "" }

if ($PathEntries -notcontains $PradyBin) {
    Write-Host "Registering $PradyBin in User PATH environment variable..." -ForegroundColor Green
    $NewPath = ($PathEntries + $PradyBin) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
} else {
    Write-Host "$PradyBin is already registered in PATH." -ForegroundColor Gray
}

# Update current session environment so prady is immediately available
$env:PATH = "$PradyBin;$env:PATH"

# Broadcast WM_SETTINGCHANGE so all Windows processes recognize the new PATH
try {
    if (-not ("Win32.NativeMethods" -as [type])) {
        Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
"@
    }
    $HWND_BROADCAST = [IntPtr]0xffff
    $WM_SETTINGCHANGE = 0x001a
    $res = [UIntPtr]::Zero
    [Win32.NativeMethods]::SendMessageTimeout($HWND_BROADCAST, $WM_SETTINGCHANGE, [UIntPtr]::Zero, "Environment", 2, 5000, [ref]$res) | Out-Null
} catch {
    # Non-fatal if broadcast fails
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "  Prady is installed system-wide!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host "Prady CLI: $(Join-Path $PradyBin 'prady.exe')"
Write-Host "Prady LSP: $(Join-Path $PradyBin 'prady-lsp.exe')"
Write-Host ""
Write-Host "You can now use 'prady' anywhere across your system:" -ForegroundColor Cyan
Write-Host "    prady version" -ForegroundColor White
Write-Host "    prady run main.pr" -ForegroundColor White
Write-Host "    prady new my-project" -ForegroundColor White
