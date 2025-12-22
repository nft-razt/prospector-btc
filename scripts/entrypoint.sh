#!/bin/bash

# =================================================================
# APARATO: HYDRA IGNITION ORCHESTRATOR (V16.0 - SOBERANO)
# RESPONSABILIDAD: BOOTSTRAP DE ENTORNOS CLOUD
# =================================================================

set -e # Abortar ante fallo de cualquier estrato

echo "      ___           ___           ___           ___     "
echo "     /  /\         /  /\         /  /\         /  /\    "
echo "    /  /::\       /  /::\       /  /::\       /  /::\   "
echo "   /  /:/\:\     /  /:/\:\     /  /:/\:\     /  /:/\:\  "
echo "  /  /::\ \:\   /  /::\ \:\   /  /:/  \:\   /  /::\ \:\ "
echo " /__/:/\:\ \:\ /__/:/\:\ \:\ /__/:/ \  \:\ /__/:/\:\ \:\\"
echo " \  \:\ \:\_\/ \  \:\ \:\_\/ \  \:\  \__\/ \  \:\ \:\_\/"
echo "  \  \:\ \:\    \  \:\ \:\    \  \:\        \  \:\ \:\  "
echo "   \  \:\_\/     \  \:\_\/     \  \:\        \  \:\_\/  "
echo "    \  \:\        \  \:\        \  \:\        \  \:\    "
echo "     \__\/         \__\/         \__\/         \__\/    "
echo " "
echo " [IGNITION]: Starting Prospector BTC Orchestrator... "
echo " [VERSION]: V10.8 Strategic Audit Era "
echo " -------------------------------------------------- "

# 1. AUDITORÍA DE VARIABLES DE ENTORNO CRÍTICAS
echo "[🛰️] Auditing Strategic Handshake Environment..."

if [ -z "$DATABASE_URL" ]; then echo "❌ ERROR: DATABASE_URL not set."; exit 1; fi
if [ -z "$TURSO_AUTH_TOKEN" ]; then echo "⚠️ WARNING: TURSO_AUTH_TOKEN missing. Proceeding in unauthenticated mode."; fi
if [ -z "$SUPABASE_URL" ]; then echo "❌ ERROR: SUPABASE_URL (Engine B) not set."; exit 1; fi
if [ -z "$WORKER_AUTH_TOKEN" ]; then echo "❌ ERROR: WORKER_AUTH_TOKEN missing. Nodes will fail handshake."; exit 1; fi

echo "✅ Environment integrity verified."

# 2. SINAPSIS CON EL FILTRO UTXO
echo "[🧊] Checking Cryptographic Census Artifacts..."
# Nota: En Render, el Dockerfile debería descargar esto o montarse vía volumen.
if [ ! -f "utxo_filter.bin" ]; then
    echo "⚠️  WARNING: utxo_filter.bin missing. System will start in Maintenance Mode."
fi

# 3. LANZAMIENTO DEL KERNEL
echo "[🚀] Launching Sovereign Kernel..."
exec ./prospector-orchestrator
