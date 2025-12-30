param(
    [ValidateSet("release", "debug")]
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"

$target = "wasm32-unknown-unknown"
$crate = "blackhole_web"
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

Push-Location $repoRoot
try {
    if ($Profile -eq "release") {
        cargo build --target $target --release --lib
    } else {
        cargo build --target $target --lib
    }

$wasmPath = Join-Path $repoRoot "target/$target/$Profile/$crate.wasm"
if (-not (Test-Path $wasmPath)) {
    throw "Wasm output not found at $wasmPath"
}

wasm-bindgen --out-dir (Join-Path $repoRoot "web/pkg") --target web $wasmPath
} finally {
    Pop-Location
}
