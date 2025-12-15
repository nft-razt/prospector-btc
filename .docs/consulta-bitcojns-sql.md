SELECT
addresses[OFFSET(0)] as address,
value as balance
FROM `bigquery-public-data.crypto_bitcoin.outputs`
WHERE
-- Filtramos para obtener transacciones antiguas (2009-2011)
block_timestamp < '2011-01-01'
-- Solo salidas estándar a una sola dirección
AND array_length(addresses) = 1
-- Que tengan saldo
AND value > 0
-- Limitamos a 100,000 para que sea rápido y gratis en el Sandbox
LIMIT 100000
