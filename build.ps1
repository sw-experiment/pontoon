# Windows-only build script for Pontoon (Phase 1)
# Phase 2 TODO: Add cross-platform builds via CI/CD

Write-Host "=== Pontoon Build Script (Windows) ===" -ForegroundColor Cyan
Write-Host ""

$Target = "x86_64-pc-windows-gnu"

# Ensure Rust toolchain is installed
Write-Host "=== Checking Rust Toolchain ===" -ForegroundColor Cyan
rustup target add $Target
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to install target $Target" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Build for Windows
Write-Host "=== Building for Windows ===" -ForegroundColor Cyan
Write-Host "Target: $Target" -ForegroundColor Yellow
Write-Host ""

cargo build --release --target $Target

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "OK Build successful" -ForegroundColor Green
    
    # Show binary location
    $BinaryPath = "target/$Target/release/pontoon.exe"
    if (Test-Path $BinaryPath) {
        $Size = (Get-Item $BinaryPath).Length / 1MB
        $SizeRounded = [math]::Round($Size, 2)
        Write-Host "Binary: $BinaryPath ($SizeRounded MB)" -ForegroundColor Gray
    }
} else {
    Write-Host ""
    Write-Host "X Build failed" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Run tests
Write-Host "=== Running Tests ===" -ForegroundColor Cyan
Write-Host ""

cargo test --target $Target

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "OK All tests passed" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "X Tests failed" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "=== Build Complete ===" -ForegroundColor Cyan
Write-Host "Executable: target\$Target\release\pontoon.exe" -ForegroundColor Green
Write-Host ""
Write-Host "Phase 2 TODO: Add Linux and macOS builds" -ForegroundColor Yellow
