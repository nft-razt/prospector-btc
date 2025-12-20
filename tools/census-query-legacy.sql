/**
 * =================================================================
 * APARATO: GLOBAL LEGACY UTXO EXTRACTOR (V20.12.2025)
 * RESPONSABILIDAD: GENERACIÓN DEL CENSO PARA VALIDACIÓN DE TESIS
 * OBJETIVO: Direcciones P2PKH (Prefijo 1) con Balance > 0
 * =================================================================
 */

SELECT
    address,
    SUM(value) AS total_balance_satoshis
FROM (
    -- Entradas de saldo (Outputs generados)
    SELECT
        array_to_string(addresses, ",") AS address,
        value
    FROM `bigquery-public-data.crypto_bitcoin.outputs`
    WHERE block_timestamp <= '2025-12-20'

    UNION ALL

    -- Salidas de saldo (Inputs consumidos)
    SELECT
        array_to_string(addresses, ",") AS address,
        -value
    FROM `bigquery-public-data.crypto_bitcoin.inputs`
    WHERE block_timestamp <= '2025-12-20'
)
WHERE address LIKE '1%' -- Filtro estricto: Solo Direcciones Legacy
GROUP BY address
HAVING total_balance_satoshis > 0;
