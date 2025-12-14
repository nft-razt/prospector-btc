SELECT
  -- La dirección (Toma la primera del array para simplificar P2PKH)
  addresses[OFFSET(0)] as address,

  -- Saldo actual (aproximado por salidas no gastadas es complejo en SQL puro,
  -- aquí sumamos valores de salida para simplificar la 'importancia' de la dirección)
  sum(value) as total_received,

  -- Actividad
  count(*) as tx_count,

  -- Arqueología
  min(block_timestamp) as first_seen,
  max(block_timestamp) as last_seen

FROM `bigquery-public-data.crypto_bitcoin.outputs`
WHERE
  -- Optimización: Solo direcciones estándar
  array_length(addresses) = 1
  AND value > 0

  -- Foco de Tesis: Rango histórico vulnerable (2009-2015 por ejemplo)
  AND block_timestamp < '2025-12-13'

GROUP BY address
-- Filtro de Calidad: Solo direcciones que recibieron más de 0.001 BTC en total
HAVING total_received > 100000
