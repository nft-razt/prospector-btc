#!/bin/bash

/**
 * =================================================================
 * APARATO: HYDRA IGNITION ORCHESTRATOR (V15.0)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE
 * RESPONSABILIDAD: ARRANQUE VERBOSO Y VALIDACIÓN DE ESTRATOS
 *
 * ESTRATEGIA DE ÉLITE:
 * - Deterministic Timers: Mide el tiempo exacto de cada fase.
 * - Fault Isolation: Detiene el proceso si un estrato crítico falla.
 * - Verbose Logging: Salida estructurada para monitoreo en Render.
 * =================================================================
 */

set -e # Abortar ante cualquier error

# Constantes de Identidad
ORCHESTRATOR_ASCII="PROSPECTOR BTC // HYDRA-ZERO PROTOCOL"
TIMESTAMP_START=$(date +%s)

log_step() {
    echo -e "\n[$(date +'%H:%M:%S')] 🛰️  $1..."
}

log_success() {
    echo -e "[$(date +'%H:%M:%S')] ✅ $1 completed in $2 seconds."
}

log_error() {
    echo -e "[$(date +'%H:%M:%S')] ❌ FATAL ERROR: $1"
    exit 1
}

echo "$ORCHESTRATOR_ASCII"
echo "--------------------------------------------------"

# --- FASE 1: AUDITORÍA DE ENTORNO ---
log_step "AUDITING ENVIRONMENT VARIABLES"
T1=$(date +%s)

if [ -z "$DATABASE_URL" ]; then log_error "DATABASE_URL is not defined in Render environment."; fi
if [ -z "$TURSO_AUTH_TOKEN" ]; then echo "⚠️  Warning: TURSO_AUTH_TOKEN is empty. Assuming local or unsecured DB."; fi
if [ -z "$WORKER_AUTH_TOKEN" ]; then log_error "WORKER_AUTH_TOKEN is missing. Nodes will be unable to handshake."; fi

T2=$(date +%s); log_success "Environment validation" $((T2 - T1))

# --- FASE 2: VALIDACIÓN DE ARTEFACTOS CRÍTICOS ---
log_step "CHECKING CRYPTOGRAPHIC ARTEFACTS"
T1=$(date +%s)

if [ ! -f "utxo_filter.bin" ]; then
    log_error "utxo_filter.bin NOT FOUND. The swarm cannot audit without the target list."
else
    FILE_SIZE=$(du -h "utxo_filter.bin" | cut -f1)
    echo "   -> Artifact: utxo_filter.bin ($FILE_SIZE)"
fi

T2=$(date +%s); log_success "Artefact verification" $((T2 - T1))

# --- FASE 3: MIGRACIÓN DEL LEDGER TÁCTICO ---
log_step "INITIATING DATABASE MIGRATION (STRATUM L3)"
T1=$(date +%s)

# Ejecutamos el binario independiente de migración creado en la Fase V7.0
if ./prospector-migrator; then
    T2=$(date +%s); log_success "Database migration" $((T2 - T1))
else
    log_error "Migration failed. Tactical ledger is out of sync."
fi

# --- FASE 4: IGNICIÓN DE LA API ---
log_step "LAUNCHING ORCHESTRATOR KERNEL"
TIMESTAMP_END=$(date +%s)
echo "🚀 Total Bootstrapping time: $((TIMESTAMP_END - TIMESTAMP_START)) seconds."
echo "--------------------------------------------------"

# Ejecución final del servidor (reemplaza el proceso actual)
exec ./orchestrator
