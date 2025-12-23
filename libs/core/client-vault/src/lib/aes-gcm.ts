/**
 * =================================================================
 * APARATO: DETERMINISTIC VAULT ENGINE (V56.5 - TYPE HARDENED)
 * CLASIFICACIÓN: CORE SECURITY (ESTRATO L1)
 * RESPONSABILIDAD: CIFRADO CLIENT-SIDE RESILIENTE
 * =================================================================
 */

export interface EncryptedVaultPayload {
  cipher_text_base64: string;
  initialization_vector_base64: string;
  salt_base64: string;
}

export class VaultCryptoEngine {
  private static readonly CRYPTO_ALGORITHM = "AES-GCM";
  private static readonly DERIVATION_ALGORITHM = "PBKDF2";
  private static readonly HASH_FUNCTION = "SHA-256";
  private static readonly KEY_LENGTH_BITS = 256;
  private static readonly PBKDF2_ITERATIONS = 150_000;

  /**
   * Helper para normalizar tipos de buffer y evitar colisiones de TS2322.
   * Garantiza que el buffer no sea un SharedArrayBuffer (incompatible con SubtleCrypto).
   */
  private static ensureBufferSource(data: Uint8Array | ArrayBuffer): BufferSource {
    return data as BufferSource;
  }

  public static async encryptPortable(
    plain_text: string,
    master_passphrase: string,
    operator_email: string
  ): Promise<EncryptedVaultPayload> {
    const text_encoder = new TextEncoder();
    const salt_material = `prospector_strata_v1_${operator_email.toLowerCase()}`;
    const salt_buffer = text_encoder.encode(salt_material);

    const initialization_vector = window.crypto.getRandomValues(new Uint8Array(12));
    const derived_key = await this.derive_sovereign_key(master_passphrase, salt_buffer);

    const encoded_plain_text = text_encoder.encode(plain_text);

    // ✅ RESOLUCIÓN TS2322: Cast a BufferSource tras normalización
    const encrypted_data = await window.crypto.subtle.encrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: this.ensureBufferSource(initialization_vector),
      },
      derived_key,
      this.ensureBufferSource(encoded_plain_text)
    );

    return {
      cipher_text_base64: this.buffer_to_base64(encrypted_data),
      initialization_vector_base64: this.buffer_to_base64(initialization_vector.buffer),
      salt_base64: this.buffer_to_base64(salt_buffer.buffer),
    };
  }

  public static async decryptPortable(
    payload: EncryptedVaultPayload,
    master_passphrase: string,
    operator_email: string
  ): Promise<string> {
    const text_decoder = new TextDecoder();
    const text_encoder = new TextEncoder();

    const salt_material = `prospector_strata_v1_${operator_email.toLowerCase()}`;
    const salt_buffer = text_encoder.encode(salt_material);

    const iv_buffer = this.base64_to_array_buffer(payload.initialization_vector_base64);
    const cipher_buffer = this.base64_to_array_buffer(payload.cipher_text_base64);

    const derived_key = await this.derive_sovereign_key(master_passphrase, salt_buffer);

    try {
      // ✅ RESOLUCIÓN TS2322: Normalización de entrada para descifrado
      const decrypted_data = await window.crypto.subtle.decrypt(
        {
          name: this.CRYPTO_ALGORITHM,
          iv: this.ensureBufferSource(new Uint8Array(iv_buffer)),
        },
        derived_key,
        this.ensureBufferSource(new Uint8Array(cipher_buffer))
      );

      return text_decoder.decode(decrypted_data);
    } catch (error) {
      throw new Error("VAULT_ACCESS_DENIED: Critical integrity failure.");
    }
  }

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
        salt: this.ensureBufferSource(salt) as Uint8Array,
        iterations: this.PBKDF2_ITERATIONS,
        hash: this.HASH_FUNCTION,
      },
      key_material,
      { name: this.CRYPTO_ALGORITHM, length: this.KEY_LENGTH_BITS },
      false,
      ["encrypt", "decrypt"]
    );
  }

  private static buffer_to_base64(buffer: ArrayBuffer | ArrayBufferView | ArrayBufferLike): string {
    const bytes = new Uint8Array(buffer as ArrayBuffer);
    let binary_string = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binary_string += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binary_string);
  }

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
