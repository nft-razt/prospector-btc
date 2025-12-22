/**
 * =================================================================
 * APARATO: TELEMETRY LOGIC FORMATTERS (V20.0 - SOBERANO)
 * CLASIFICACIÓN: UTILITIES (ESTRATO L5)
 * RESPONSABILIDAD: TRANSFORMACIÓN DE MÉTRICAS COMPUTACIONALES
 * =================================================================
 */

/**
 * Transforma un volumen de hashes (BigInt string) en una magnitud legible.
 * Soporta escalas hasta ExaHashes para el rigor de la Tesis Doctoral.
 *
 * @param volume_string Representación en string del conteo de hashes.
 */
export function formatComputationalEffort(volume_string: string): string {
  const numeric_volume = BigInt(volume_string || "0");

  const terra_hash = BigInt(1_000_000_000_000);
  const giga_hash = BigInt(1_000_000_000);
  const mega_hash = BigInt(1_000_000);

  if (numeric_volume >= terra_hash) {
    return `${(Number(numeric_volume) / 1e12).toFixed(3)} TH`;
  }
  if (numeric_volume >= giga_hash) {
    return `${(Number(numeric_volume) / 1e9).toFixed(2)} GH`;
  }
  if (numeric_volume >= mega_hash) {
    return `${(Number(numeric_volume) / 1e6).toFixed(1)} MH`;
  }

  return `${numeric_volume.toLocaleString()} H`;
}

/**
 * Formatea la duración de ejecución de milisegundos a tiempo táctico.
 */
export function formatExecutionTime(milliseconds: number): string {
  const seconds = Math.floor(milliseconds / 1000);
  const minutes = Math.floor(seconds / 60);
  const remaining_seconds = seconds % 60;

  return `${minutes}m ${remaining_seconds}s`;
}
