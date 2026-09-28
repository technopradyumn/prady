# Build script for Prady VS Code Extension
# Run from the repository root: .\build-extension.ps1

param(
    [switch]$Release = $false
)

$profile = if ($Release) { "release" } else { "debug" }
$flag    = if ($Release) { "--release" } else { "" }

Write-Host "==> Building Prady compiler & LSP server ($profile)..." -ForegroundColor Cyan
cargo build --bin prady --bin prady-lsp $flag
if ($LASTEXITCODE -ne 0) { Write-Error "Cargo build failed"; exit 1 }

$ext     = "editors\vscode-prady"
$binExt  = ".exe"
$lspSrc  = "target\$profile\prady-lsp$binExt"
$cliSrc  = "target\$profile\prady$binExt"
$lspDst  = "$ext\prady-lsp$binExt"
$cliDst  = "$ext\prady$binExt"

Write-Host "==> Copying binaries to extension folder..." -ForegroundColor Cyan
Copy-Item $lspSrc $lspDst -Force
Copy-Item $cliSrc $cliDst -Force

Write-Host "==> Installing npm dependencies..." -ForegroundColor Cyan
Push-Location $ext
npm install
if ($LASTEXITCODE -ne 0) { Pop-Location; Write-Error "npm install failed"; exit 1 }

Write-Host "==> Packaging extension (.vsix)..." -ForegroundColor Cyan
npx @vscode/vsce package --no-dependencies 2>&1
Pop-Location

$vsix = Get-ChildItem "$ext\*.vsix" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
if ($vsix) {
    Write-Host ""
    Write-Host "Extension built successfully!" -ForegroundColor Green
    Write-Host "  VSIX:     $($vsix.FullName)" -ForegroundColor White
    Write-Host "  Install:  code --install-extension `"$($vsix.FullName)`"" -ForegroundColor White
    Write-Host ""
    $install = Read-Host "Install extension now? [y/N]"
    if ($install -eq 'y' -or $install -eq 'Y') {
        code --install-extension $vsix.FullName
    }
} else {
    Write-Error "VSIX packaging failed"
    exit 1
}
