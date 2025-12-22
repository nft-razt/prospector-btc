/**
 * =================================================================
 * APARATO: NEURAL LINK CODEC (V62.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: SERIALIZACIÓN Y DESERIALIZACIÓN BINARIA
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el motor de MessagePack para el Dashboard. Transforma
 * ráfagas Base64 recibidas por SSE en objetos de dominio tipados,
 * optimizando el uso de CPU y reduciendo el consumo de ancho de banda.
 * =================================================================
 */

import { decode } from "@msgpack/msgpack";
import { type RealTimeEvent } from "@prospector/api-contracts";

export class NeuralCodec {
  /**
   * Decodifica un string Base64-MessagePack en un evento de dominio.
   *
   * @param encodedData - El payload binario codificado en Base64.
   * @returns El evento decodificado o null ante fallo de integridad.
   */
  public static decodeEvent(encodedData: string): RealTimeEvent | null {
    if (!encodedData || encodedData.length < 4) return null;

    try {
      // 1. Reconstrucción del Buffer (Base64 to Uint8Array)
      const binaryString = window.atob(encodedData);
      const buffer = new Uint8Array(binaryString.length);

      for (let i = 0; i < binaryString.length; i++) {
        buffer[i] = binaryString.charCodeAt(i);
      }

      // 2. Deserialización MessagePack (Zero-Copy)
      // ✅ RESOLUCIÓN: Requiere @msgpack/msgpack instalado en el estrato L4.
      return decode(buffer) as RealTimeEvent;
    } catch (error) {
      console.error("🔥 [NEURAL_CODEC_CRITICAL]: Signal corruption detected.", error);
      return null;
    }
  }
}
