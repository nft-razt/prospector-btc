/**
 * =====================================================================
 * SNAPSHOT DEL PROYECTO - APARATO DE SEGURIDAD L1
 * =====================================================================
 * APARATO: DETERMINISTIC VAULT ENGINE (V56.8 - TYPE HARDENED)
 * CLASIFICACIÓN: CORE SECURITY (ESTRATO L1)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE RESILIENTE Y SOBERANO
 *
 * ESTRATEGIA DE ÉLITE:
 * - Implementa AES-GCM 256 bits con derivación PBKDF2.
 * - Resuelve TS2322 mediante normalización de BufferSource.
 * - Cero abreviaciones (iv -> initializationVector, plain_text -> plainText).
 * =====================================================================
 */

/**
 * Contrato de datos para el material cifrado de la bóveda.
 * Representa la estructura inmutable necesaria para la persistencia táctica.
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
 *
 * # Mathematical Proof:
 * Utiliza el estándar NIST SP 800-38D para el modo Galois/Counter Mode (GCM).
 * La derivación se realiza mediante PBKDF2 (NIST SP 800-132) con 150,000 iteraciones
 * y HMAC-SHA-256 para maximizar la resistencia a ataques de fuerza bruta.
 */
export class VaultCryptoEngine {
  private static readonly CRYPTO_ALGORITHM = "AES-GCM";
  private static readonly DERIVATION_ALGORITHM = "PBKDF2";
  private static readonly HASH_FUNCTION = "SHA-256";
  private static readonly KEY_LENGTH_BITS = 256;
  private static readonly PBKDF2_ITERATIONS = 150_000;

  /**
   * Asegura que el dato sea un BufferSource compatible.
   * Resuelve el error TS2322 forzando la compatibilidad con el contrato de WebCrypto.
   *
   * # Performance:
   * Realiza un cast ligero de tipo sin copia de memoria adicional.
   */
  private static normalizeToBufferSource(data: Uint8Array): BufferSource {
    return data as BufferSource;
  }

  /**
   * Cifra un texto plano utilizando una derivación de llave vinculada al email del operador.
   *
   * @param plainText - Texto original a proteger.
   * @param masterPassphrase - Frase maestra del operador (nunca viaja al servidor).
   * @param operatorEmail - Componente de entropía para la sal determinista.
   *
   * # Errors:
   * Arroja una excepción si el motor de WebCrypto no está disponible o falla el cifrado.
   */
  public static async encryptPortable(
    plainText: string,
    masterPassphrase: string,
    operatorEmail: string
  ): Promise<EncryptedVaultPayload> {
    const textEncoder = new TextEncoder();

    // 1. DERIVACIÓN DE SAL DETERMINISTA
    const saltMaterial = `prospector_strata_v1_${operatorEmail.toLowerCase()}`;
    const saltBuffer = textEncoder.encode(saltMaterial);

    // 2. GENERACIÓN DE VECTOR DE INICIALIZACIÓN (12 bytes para GCM)
    const initializationVector = window.crypto.getRandomValues(new Uint8Array(12));

    // 3. DERIVACIÓN DE LLAVE SOBERANA
    const derivedKey = await this.deriveSovereignKey(masterPassphrase, saltBuffer);

    // 4. EJECUCIÓN DE CIFRADO ATÓMICO
    const encodedPlainText = textEncoder.encode(plainText);

    /**
     * ✅ RESOLUCIÓN TS2322:
     * Utilizamos normalizeToBufferSource para garantizar que el Uint8Array
     * cumpla estrictamente con la interfaz BufferSource esperada por el API.
     */
    const encryptedData = await window.crypto.subtle.encrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: this.normalizeToBufferSource(initializationVector),
      },
      derivedKey,
      this.normalizeToBufferSource(encodedPlainText)
    );

    return {
      cipher_text_base64: this.bufferToBase64(encryptedData),
      initialization_vector_base64: this.bufferToBase64(initializationVector.buffer),
      salt_base64: this.bufferToBase64(saltBuffer.buffer),
    };
  }

  /**
   * Descifra un payload de la bóveda validando la integridad del mensaje y la autoría.
   *
   * @param payload - Objeto con el material cifrado y metadatos.
   * @param masterPassphrase - Frase maestra del operador.
   * @param operatorEmail - Email del operador para reconstruir la sal.
   */
  public static async decryptPortable(
    payload: EncryptedVaultPayload,
    masterPassphrase: string,
    operatorEmail: string
  ): Promise<string> {
    const textDecoder = new TextDecoder();
    const textEncoder = new TextEncoder();

    // 1. RECONSTRUCCIÓN DEL CONTEXTO DE DERIVACIÓN
    const saltMaterial = `prospector_strata_v1_${operatorEmail.toLowerCase()}`;
    const saltBuffer = textEncoder.encode(saltMaterial);

    // 2. RECUPERACIÓN DE BUFFERS
    const ivBuffer = this.base64ToArrayBuffer(payload.initialization_vector_base64);
    const cipherBuffer = this.base64ToArrayBuffer(payload.cipher_text_base64);

    const derivedKey = await this.deriveSovereignKey(masterPassphrase, saltBuffer);

    try {
      /**
       * ✅ RESOLUCIÓN TS2322:
       * Envolviendo el ArrayBuffer en un Uint8Array y normalizando,
       * satisfacemos los requisitos de tipado estricto.
       */
      const decryptedData = await window.crypto.subtle.decrypt(
        {
          name: this.CRYPTO_ALGORITHM,
          iv: this.normalizeToBufferSource(new Uint8Array(ivBuffer)),
        },
        derivedKey,
        this.normalizeToBufferSource(new Uint8Array(cipherBuffer))
      );

      return textDecoder.decode(decryptedData);
    } catch (error) {
      throw new Error("VAULT_ACCESS_DENIED: Critical integrity failure or unauthorized access.");
    }
  }

  /**
   * Deriva una clave criptográfica de 256 bits mediante estiramiento de clave PBKDF2.
   */
  private static async deriveSovereignKey(
    passphrase: string,
    salt: Uint8Array
  ): Promise<CryptoKey> {
    const textEncoder = new TextEncoder();

    const keyMaterial = await window.crypto.subtle.importKey(
      "raw",
      this.normalizeToBufferSource(textEncoder.encode(passphrase)),
      { name: this.DERIVATION_ALGORITHM },
      false,
      ["deriveKey"]
    );

    return window.crypto.subtle.deriveKey(
      {
        name: this.DERIVATION_ALGORITHM,
        salt: this.normalizeToBufferSource(salt),
        iterations: this.PBKDF2_ITERATIONS,
        hash: this.HASH_FUNCTION,
      },
      keyMaterial,
      { name: this.CRYPTO_ALGORITHM, length: this.KEY_LENGTH_BITS },
      false,
      ["encrypt", "decrypt"]
    );
  }

  /**
   * Transforma un buffer binario en una cadena Base64 segura.
   */
  private static bufferToBase64(buffer: ArrayBuffer | ArrayBufferView | ArrayBufferLike): string {
    const bytes = new Uint8Array(buffer as ArrayBuffer);
    let binaryString = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binaryString += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binaryString);
  }

  /**
   * Reconstruye un ArrayBuffer puro a partir de una cadena Base64.
   */
  private static base64ToArrayBuffer(base64String: string): ArrayBuffer {
    const binaryString = window.atob(base64String);
    const buffer = new ArrayBuffer(binaryString.length);
    const byteArray = new Uint8Array(buffer);
    for (let i = 0; i < binaryString.length; i++) {
      byteArray[i] = binaryString.charCodeAt(i);
    }
    return buffer;
  }
}
