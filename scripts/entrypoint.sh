#!/bin/bash

# =================================================================
# APARATO: SOVEREIGN IGNITION ENTRYPOINT (V18.5 - RENDER HARDENED)
# CLASIFICACIÓN: OPS INFRASTRUCTURE (ESTRATO L6)
# RESPONSABILIDAD: AUDITORÍA DE ENTORNO Y CONECTIVIDAD PRE-IGNICIÓN
# =================================================================

set -e # Abortar ante fallo

echo " [IGNITION]: Starting Prospector BTC Orchestrator... "
echo " [VERSION]: V10.8 Strategic Audit Era "
echo " -------------------------------------------------- "

# 1. FUNCIÓN DE AUDITORÍA DE SECRETOS
check_env_var() {
    if [ -z "${!1}" ]; then
        echo "❌ [CRITICAL_FAULT]: Variable '$1' is UNDEFINED."
        return 1
    else
        # Ofuscamos el valor para el log pero confirmamos su presencia
        local length=${#!1}
        echo "✅ [SECURITY]: '$1' is set (Length: $length chars)."
        return 0
    fi
}

# 2. VALIDACIÓN DE ESTRATOS DE DATOS
echo "[🛰️ ] Stage 1: Auditing Environment Variables..."
ERRORS=0
check_env_var "DATABASE_URL" || ERRORS=$((ERRORS+1))
check_env_var "TURSO_AUTH_TOKEN" || ERRORS=$((ERRORS+1))
check_env_var "SUPABASE_URL" || ERRORS=$((ERRORS+1))
check_env_var "SUPABASE_SERVICE_ROLE_KEY" || ERRORS=$((ERRORS+1))
check_env_var "WORKER_AUTH_TOKEN" || ERRORS=$((ERRORS+1))

if [ $ERRORS -gt 0 ]; then
    echo " "
    echo "🛑 [FATAL]: $ERRORS critical environment variables are missing."
    echo "    Please inject them in the Render Dashboard -> Environment section."
    exit 1
fi

# 3. DIAGNÓSTICO DE CONECTIVIDAD (DNS & HTTP)
echo " "
echo "[🌐] Stage 2: Connectivity Diagnostics..."

# Extraer hosts de las URLs para testeo rápido
TURSO_HOST=$(echo $DATABASE_URL | sed -e 's|^[^/]*//||' -e 's|/.*$||' -e 's|:.*$||')
SUPABASE_HOST=$(echo $SUPABASE_URL | sed -e 's|^[^/]*//||' -e 's|/.*$||' -e 's|:.*$||')

test_host() {
    echo -n "  📡 Testing link to $1... "
    if getent hosts $1 > /dev/null; then
        echo "RESOLVED"
    else
        echo "DNS_FAILURE"
        # No salimos aquí, dejamos que el binario intente reconectar por si es un glitch de Render
    fi
}

test_host "$TURSO_HOST"
test_host "$SUPABASE_HOST"

# 4. LANZAMIENTO DEL KERNEL SOBERANO
echo " "
echo "[🚀] Stage 3: Transferring control to Orchestrator Kernel..."
echo " -------------------------------------------------- "
exec ./prospector-orchestrator
