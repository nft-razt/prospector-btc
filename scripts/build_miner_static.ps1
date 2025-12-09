# scripts/build_miner_static.ps1
# =================================================================
# APARATO: STATIC MINER BUILDER (WINDOWS / POWERSHELL)
# OBJETIVO: Generar binario MUSL desde Windows usando Docker
# ESTADO: ASCII-SAFE (COMPATIBILIDAD UNIVERSAL)
# =================================================================

$ErrorActionPreference = "Stop"

Write-Host "[INFO] INICIANDO COMPILACION ESTATICA (MUSL) DESDE WINDOWS..." -ForegroundColor Cyan

# 1. Verificacion de Docker
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Error "[ERROR] Docker no esta instalado o no esta en el PATH."
}

# Redirigimos la salida al vacio ($null)
docker info > $null 2>&1

if ($LASTEXITCODE -ne 0) {
    Write-Error "[ERROR] Docker Desktop no esta corriendo. Por favor inicia Docker Desktop."
}

# 2. Definicion de Rutas
$TargetDir = Join-Path (Get-Location) "dist\target"
$OutputBin = Join-Path $TargetDir "x86_64-unknown-linux-musl\release\miner-worker"

# 3. Limpieza
if (Test-Path $OutputBin) {
    Write-Host "[LIMPIEZA] Eliminando binario anterior..." -ForegroundColor Yellow
    Remove-Item $OutputBin -Force
}

# 4. Ejecucion del Contenedor (Cross-Compilation)
Write-Host "[DOCKER] Lanzando contenedor de compilacion..." -ForegroundColor Green

# Usamos la ruta absoluta del directorio actual
$WorkDir = Get-Location

docker run --rm -it `
  -v "${WorkDir}:/home/rust/src" `
  -v cargo-cache:/root/.cargo/registry `
  -w /home/rust/src `
  -e RUSTFLAGS='-C target-feature=+crt-static' `
  messense/rust-musl-cross:x86_64-musl `
  cargo build --release --bin miner-worker --target x86_64-unknown-linux-musl

# 5. Verificacion
if (Test-Path $OutputBin) {
    $Size = (Get-Item $OutputBin).Length / 1MB
    Write-Host "[EXITO] COMPILACION COMPLETADA." -ForegroundColor Green
    Write-Host " -> Artefacto: $OutputBin"
    Write-Host " -> Tamano: $("{0:N2}" -f $Size) MB"
} else {
    Write-Error "[FATAL] El binario no fue generado."
}
