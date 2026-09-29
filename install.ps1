# Installs the latest Prady CLI and language server for the current Windows user.
# Usage: irm https://raw.githubusercontent.com/technopradyumn/prady/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

$Repository = "technopradyumn/prady"
$Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repository/releases/latest" -Headers @{ "User-Agent" = "Prady-Installer" }
$AssetName = "prady-$($Release.tag_name)-x86_64-pc-windows-msvc.zip"
$Asset = $Release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1

if (-not $Asset) {
    throw "The latest Prady release ($($Release.tag_name)) does not contain the Windows x64 package '$AssetName'. See https://github.com/$Repository/releases."
}

if (-not [Environment]::Is64BitOperatingSystem) {
    throw "Prady's Windows release requires 64-bit Windows."
}

$InstallDirectory = Join-Path $env:USERPROFILE ".prady\bin"
$TemporaryDirectory = Join-Path ([IO.Path]::GetTempPath()) ("prady-install-" + [guid]::NewGuid().ToString("N"))
$ArchivePath = Join-Path $TemporaryDirectory $AssetName
$ExtractDirectory = Join-Path $TemporaryDirectory "extracted"

try {
    New-Item -ItemType Directory -Path $TemporaryDirectory, $InstallDirectory -Force | Out-Null
    Write-Host "Downloading Prady $($Release.tag_name)..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $ArchivePath
    Expand-Archive -LiteralPath $ArchivePath -DestinationPath $ExtractDirectory

    $Cli = Get-ChildItem -LiteralPath $ExtractDirectory -Filter "prady.exe" -File -Recurse | Select-Object -First 1
    $Lsp = Get-ChildItem -LiteralPath $ExtractDirectory -Filter "prady-lsp.exe" -File -Recurse | Select-Object -First 1
    if (-not $Cli -or -not $Lsp) {
        throw "The downloaded package is missing prady.exe or prady-lsp.exe. Please report this at https://github.com/$Repository/issues."
    }

    Copy-Item -LiteralPath $Cli.FullName -Destination (Join-Path $InstallDirectory "prady.exe") -Force
    Copy-Item -LiteralPath $Lsp.FullName -Destination (Join-Path $InstallDirectory "prady-lsp.exe") -Force

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathEntries = @($UserPath -split ";" | Where-Object {
        -not [string]::IsNullOrWhiteSpace($_) -and $_.TrimEnd("\") -ine $InstallDirectory.TrimEnd("\")
    })
    $PathEntries += $InstallDirectory
    [Environment]::SetEnvironmentVariable("Path", ($PathEntries -join ";"), "User")

    if (($env:Path -split ";" | Where-Object { $_.TrimEnd("\") -ieq $InstallDirectory.TrimEnd("\") }).Count -eq 0) {
        $env:Path = "$InstallDirectory;$env:Path"
    }

    & (Join-Path $InstallDirectory "prady.exe") version
    if ($LASTEXITCODE -ne 0) {
        throw "Prady was installed, but its version check failed. See https://github.com/$Repository/issues."
    }

    Write-Host ""
    Write-Host "Prady CLI and VS Code language server installed to $InstallDirectory" -ForegroundColor Green
    Write-Host "Open a new terminal (and restart VS Code), then try: prady run path\to\hello.pr"
}
finally {
    if (Test-Path -LiteralPath $TemporaryDirectory) {
        Remove-Item -LiteralPath $TemporaryDirectory -Recurse -Force
    }
}
