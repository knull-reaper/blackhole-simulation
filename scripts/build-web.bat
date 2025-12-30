@echo off
setlocal enabledelayedexpansion

set "PROFILE=release"
if /I "%~1"=="debug" set "PROFILE=debug"

set "TARGET=wasm32-unknown-unknown"
set "CRATE=blackhole_web"

set "SCRIPT_DIR=%~dp0"
pushd "%SCRIPT_DIR%.."

if /I "%PROFILE%"=="release" (
    cargo build --target %TARGET% --release --lib || goto :error
) else (
    cargo build --target %TARGET% --lib || goto :error
)

set "WASM_PATH=target\%TARGET%\%PROFILE%\%CRATE%.wasm"
if not exist "%WASM_PATH%" (
    echo Wasm output not found at %WASM_PATH%
    goto :error
)

wasm-bindgen --out-dir web\pkg --target web "%WASM_PATH%" || goto :error
popd
exit /b 0

:error
popd
exit /b 1
