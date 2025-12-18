-- =================================================================
-- APARATO: CENSUS ANALYTICS ENGINE (SQL)
-- CLASIFICACIÓN: DATA STRATIFICATION (L4)
-- RESPONSABILIDAD: AGREGACIÓN DE UTXOS PARA EL DASHBOARD
-- =================================================================

-- 1. Definición del tipo enumerado para consistencia de Dominio
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'wealth_category') THEN
        CREATE TYPE wealth_category AS ENUM (
            'Satoshi_Era',
            'Lost_Coins',
            'Exchanges',
            'Whales',
            'Retail'
        );
    END IF;
END $$;

-- 2. Vista de Distribución de Riqueza (The Bubble Chart Source)
-- Nota: Esta vista asume una tabla base 'utxo_census' poblada por el census-taker
CREATE OR REPLACE VIEW public.wealth_distribution_view AS
SELECT
    id AS cluster_identifier,
    label AS display_label,
    last_active_year AS last_activity_year,
    address_count AS wallet_count,
    total_btc AS balance_bitcoin,
    category::wealth_category AS wealth_category,
    (last_active_year < 2012 AND total_btc > 50) AS is_zombie_target
FROM public.census_aggregates
WHERE total_btc > 0.1
ORDER BY total_btc DESC;

-- 3. Resumen Global (The HUD Source)
CREATE OR REPLACE VIEW public.census_summary AS
SELECT
    COUNT(id) AS total_indexed_addresses,
    SUM(total_btc) FILTER (WHERE last_active_year < 2013) AS zombie_btc_estimate,
    MAX(last_block) AS last_block_synced,
    COUNT(id) FILTER (WHERE last_active_year < 2011) AS high_entropy_risk_count,
    NOW() AS updated_at
FROM public.census_aggregates;

-- Permisos de lectura para el rol anónimo (autenticado por API Key)
GRANT SELECT ON public.wealth_distribution_view TO anon;
GRANT SELECT ON public.census_summary TO anon;
