/**
 * =================================================================
 * APARATO: CRYPTOGRAPHIC VAULT ENGINE (AES-GCM)
 * CLASIFICACIÓN: CORE SECURITY LAYER (L1)
 * RESPONSABILIDAD: CIFRADO SIMÉTRICO ZERO-KNOWLEDGE EN EL CLIENTE
 *
 * ESTRATEGIA DE ÉLITE:
 * - Algoritmo: AES-256-GCM (Autenticación y Cifrado combinados).
 * - Derivación: PBKDF2 con 100,000 iteraciones de SHA-256.
 * - Seguridad: Los buffers se tratan como BufferSource inmutables.
 * - Zero-Knowledge: El servidor nunca recibe el material de clave original.
 * =================================================================
 */

/**
 * Representación atómica del payload cifrado listo para persistencia.
 */
export interface EncryptedVaultPayload {
  readonly cipher_text_base64: string;
  readonly initialization_vector_base64: string;
  readonly salt_base64: string;
}

/**
 * Motor de cifrado de alta precisión.
 * Implementa el estándar de seguridad para la Bóveda de Prospector.
 */
export class VaultCryptoEngine {
  private static readonly CRYPTO_ALGORITHM = 'AES-GCM';
  private static readonly DERIVATION_ALGORITHM = 'PBKDF2';
  private static readonly HASH_FUNCTION = 'SHA-256';
  private static readonly KEY_LENGTH_BITS = 256;
  private static readonly PBKDF2_ITERATIONS = 100_000;
  private static readonly SALT_LENGTH_BYTES = 16;
  private static readonly IV_LENGTH_BYTES = 12;

  /**
   * Cifra una cadena de texto plano utilizando una clave maestra de usuario.
   *
   * @param plainText - El contenido sensible (ej: Clave Privada WIF).
   * @param masterKey - La contraseña maestra del operador.
   * @returns Una promesa con el payload cifrado y sus metadatos de reconstrucción.
   */
  public static async encrypt(plainText: string, masterKey: string): Promise<EncryptedVaultPayload> {
    const textEncoder = new TextEncoder();
    const saltBuffer = window.crypto.getRandomValues(new Uint8Array(this.SALT_LENGTH_BYTES));
    const initializationVector = window.crypto.getRandomValues(new Uint8Array(this.IV_LENGTH_BYTES));

    // 1. Derivación de la clave criptográfica desde la contraseña
    const derivedKey = await this.deriveCryptographicKey(masterKey, saltBuffer);

    // 2. Ejecución del proceso de cifrado
    // ✅ RESOLUCIÓN: Cast explícito a BufferSource para satisfacer el contrato de subtle.encrypt
    const encryptedData = await window.crypto.subtle.encrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: initializationVector as BufferSource
      },
      derivedKey,
      textEncoder.encode(plainText) as BufferSource
    );

    return {
      cipher_text_base64: this.convertBufferToBase64(encryptedData),
      initialization_vector_base64: this.convertBufferToBase64(initializationVector.buffer),
      salt_base64: this.convertBufferToBase64(saltBuffer.buffer),
    };
  }

  /**
   * Descifra un payload cifrado utilizando la clave maestra original.
   *
   * @throws Error si la integridad del mensaje ha sido comprometida o la clave es incorrecta.
   */
  public static async decrypt(payload: EncryptedVaultPayload, masterKey: string): Promise<string> {
    const textDecoder = new TextDecoder();
    const saltBuffer = this.convertBase64ToBuffer(payload.salt_base64);
    const initializationVector = this.convertBase64ToBuffer(payload.initialization_vector_base64);
    const encryptedData = this.convertBase64ToBuffer(payload.cipher_text_base64);

    const derivedKey = await this.deriveCryptographicKey(masterKey, new Uint8Array(saltBuffer));

    // ✅ RESOLUCIÓN: Alineación de tipos para subtle.decrypt
    const decryptedData = await window.crypto.subtle.decrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: initializationVector as BufferSource
      },
      derivedKey,
      encryptedData as BufferSource
    );

    return textDecoder.decode(decryptedData);
  }

  /**
   * Deriva una CryptoKey segura a partir de una contraseña utilizando PBKDF2.
   * ✅ RESOLUCIÓN: Manejo estricto de Pbkdf2Params para evitar Error 2769.
   */
  private static async deriveCryptographicKey(password: string, salt: Uint8Array): Promise<CryptoKey> {
    const textEncoder = new TextEncoder();

    const baseKeyMaterial = await window.crypto.subtle.importKey(
      'raw',
      textEncoder.encode(password) as BufferSource,
      { name: this.DERIVATION_ALGORITHM },
      false,
      ['deriveKey']
    );

    return window.crypto.subtle.deriveKey(
      {
        name: this.DERIVATION_ALGORITHM,
        salt: salt as BufferSource,
        iterations: this.PBKDF2_ITERATIONS,
        hash: this.HASH_FUNCTION,
      },
      baseKeyMaterial,
      { name: this.CRYPTO_ALGORITHM, length: this.KEY_LENGTH_BITS },
      false,
      ['encrypt', 'decrypt']
    );
  }

  // --- UTILIDADES DE TRANSFORMACIÓN DE BAJO NIVEL ---

  private static convertBufferToBase64(buffer: ArrayBuffer | ArrayBufferView): string {
    const bytes = buffer instanceof ArrayBuffer ? new Uint8Array(buffer) : new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    let binaryString = '';
    for (let i = 0; i < bytes.byteLength; i++) {
      binaryString += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binaryString);
  }

  private static convertBase64ToBuffer(base64String: string): ArrayBuffer {
    const binaryString = window.atob(base64String);
    const buffer = new ArrayBuffer(binaryString.length);
    const bytes = new Uint8Array(buffer);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return buffer;
  }
}
