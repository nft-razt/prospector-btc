/**
 * =================================================================
 * APARATO: DETERMINISTIC VAULT ENGINE (V56.2 - BUFFER SOBERANO)
 * CLASIFICACIÓN: CORE SECURITY (ESTRATO L1)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE RESILIENTE Y SOBERANO
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo AES-GCM 256 con derivación PBKDF2.
 * Esta versión resuelve el error TS2322 forzando la normalización
 * de BufferSource a través de Uint8Array, garantizando que el
 * compilador reconozca el material como compatible con SubtleCrypto
 * y libre de SharedArrayBuffers.
 * =================================================================
 */

/**
 * Contrato de datos para el material cifrado de la bóveda.
 */
export interface EncryptedVaultPayload {
  /** El contenido cifrado codificado en formato Base64. */
  cipher_text_base64: string;
  /** Vector de inicialización único para garantizar la unicidad del cifrado. */
  initialization_vector_base64: string;
  /** Sal determinista utilizada para la derivación de la llave maestra. */
  salt_base64: string;
}

/**
 * Motor criptográfico soberano para la gestión de la Bóveda Zero-Knowledge.
 */
export class VaultCryptoEngine {
  private static readonly CRYPTO_ALGORITHM = "AES-GCM";
  private static readonly DERIVATION_ALGORITHM = "PBKDF2";
  private static readonly HASH_FUNCTION = "SHA-256";
  private static readonly KEY_LENGTH_BITS = 256;
  private static readonly PBKDF2_ITERATIONS = 150_000;

  /**
   * Cifra un texto plano utilizando una derivación de llave vinculada al email del operador.
   */
  public static async encryptPortable(
    plain_text: string,
    master_passphrase: string,
    operator_email: string
  ): Promise<EncryptedVaultPayload> {
    const text_encoder = new TextEncoder();

    // 1. DERIVACIÓN DE SAL DETERMINISTA
    const salt_material = `prospector_strata_v1_${operator_email.toLowerCase()}`;
    const salt_buffer = text_encoder.encode(salt_material);

    // 2. GENERACIÓN DE VECTOR DE INICIALIZACIÓN (12 bytes para GCM)
    const initialization_vector = window.crypto.getRandomValues(new Uint8Array(12));

    // 3. DERIVACIÓN DE LLAVE SOBERANA
    const derived_key = await this.derive_sovereign_key(master_passphrase, salt_buffer);

    // 4. EJECUCIÓN DE CIFRADO ATÓMICO
    const encoded_plain_text = text_encoder.encode(plain_text);

    /**
     * ✅ RESOLUCIÓN TS2322:
     * Se realiza un cast explícito a Uint8Array para satisfacer la interfaz BufferSource.
     */
    const encrypted_data = await window.crypto.subtle.encrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: initialization_vector as Uint8Array,
      },
      derived_key,
      encoded_plain_text
    );

    return {
      cipher_text_base64: this.buffer_to_base64(encrypted_data),
      initialization_vector_base64: this.buffer_to_base64(initialization_vector.buffer),
      salt_base64: this.buffer_to_base64(salt_buffer.buffer),
    };
  }

  /**
   * Descifra un payload de la bóveda validando la integridad del mensaje y la autoría.
   */
  public static async decryptPortable(
    payload: EncryptedVaultPayload,
    master_passphrase: string,
    operator_email: string
  ): Promise<string> {
    const text_decoder = new TextDecoder();
    const text_encoder = new TextEncoder();

    // 1. RECONSTRUCCIÓN DEL CONTEXTO DE DERIVACIÓN
    const salt_material = `prospector_strata_v1_${operator_email.toLowerCase()}`;
    const salt_buffer = text_encoder.encode(salt_material);

    // 2. RECUPERACIÓN DE BUFFERS
    const iv_buffer = this.base64_to_array_buffer(payload.initialization_vector_base64);
    const cipher_buffer = this.base64_to_array_buffer(payload.cipher_text_base64);

    const derived_key = await this.derive_sovereign_key(master_passphrase, salt_buffer);

    try {
      /**
       * ✅ RESOLUCIÓN TS2322 (Línea 101):
       * Envolviendo el ArrayBuffer en un Uint8Array garantizamos que cumpla
       * con la definición de 'BufferSource' requerida por SubtleCrypto.
       */
      const decrypted_data = await window.crypto.subtle.decrypt(
        {
          name: this.CRYPTO_ALGORITHM,
          iv: new Uint8Array(iv_buffer),
        },
        derived_key,
        new Uint8Array(cipher_buffer)
      );

      return text_decoder.decode(decrypted_data);
    } catch (error) {
      throw new Error("VAULT_ACCESS_DENIED: Critical integrity failure or unauthorized access.");
    }
  }

  /**
   * Deriva una clave criptográfica de 256 bits mediante estiramiento de clave PBKDF2.
   */
  private static async derive_sovereign_key(passphrase: string, salt: Uint8Array): Promise<CryptoKey> {
    const text_encoder = new TextEncoder();

    const key_material = await window.crypto.subtle.importKey(
      "raw",
      text_encoder.encode(passphrase),
      { name: this.DERIVATION_ALGORITHM },
      false,
      ["deriveKey"]
    );

    return window.crypto.subtle.deriveKey(
      {
        name: this.DERIVATION_ALGORITHM,
        salt: salt as Uint8Array,
        iterations: this.PBKDF2_ITERATIONS,
        hash: this.HASH_FUNCTION,
      },
      key_material,
      { name: this.CRYPTO_ALGORITHM, length: this.KEY_LENGTH_BITS },
      false,
      ["encrypt", "decrypt"]
    );
  }

  /**
   * Transforma un buffer binario en una cadena Base64 segura.
   */
  private static buffer_to_base64(buffer: ArrayBuffer | ArrayBufferView | ArrayBufferLike): string {
    const bytes = new Uint8Array(buffer as ArrayBuffer);
    let binary_string = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binary_string += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binary_string);
  }

  /**
   * Reconstruye un ArrayBuffer puro a partir de una cadena Base64.
   */
  private static base64_to_array_buffer(base64_string: string): ArrayBuffer {
    const binary_string = window.atob(base64_string);
    const buffer = new ArrayBuffer(binary_string.length);
    const byte_array = new Uint8Array(buffer);
    for (let i = 0; i < binary_string.length; i++) {
      byte_array[i] = binary_string.charCodeAt(i);
    }
    return buffer;
  }
}
