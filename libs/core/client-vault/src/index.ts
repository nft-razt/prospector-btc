// libs/core/client-vault/src/index.ts
/**
 * =================================================================
 * APARATO: CLIENT-SIDE VAULT (ZERO-DEPENDENCY)
 * RESPONSABILIDAD: CIFRADO AES-GCM NATIVO DEL NAVEGADOR
 * ESTÁNDAR: WEB CRYPTO API (NO REQUIERE NODE.JS POLYFILLS)
 * =================================================================
 */

// Configuración criptográfica (Estándares OWASP)
const ALGO_NAME = "AES-GCM";
const PBKDF2_ITERATIONS = 100_000;
const SALT_LEN = 16;
const IV_LEN = 12; // 96 bits para GCM

// --- UTILIDADES DE ALTO RENDIMIENTO (Reemplazo de Buffer) ---

/**
 * Convierte Uint8Array a Base64 String (Nativo Browser).
 */
function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

/**
 * Convierte Base64 String a Uint8Array (Nativo Browser).
 */
function base64ToBytes(base64: string): Uint8Array {
  const binaryString = window.atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}

// --- NÚCLEO CRIPTOGRÁFICO ---

/**
 * Deriva una clave criptográfica (CryptoKey) a partir de una contraseña.
 * Utiliza PBKDF2 para resistencia contra fuerza bruta.
 */
async function deriveKey(
  password: string,
  salt: Uint8Array
): Promise<CryptoKey> {
  const enc = new TextEncoder();

  // 1. Importar la contraseña como material de clave "raw"
  const keyMaterial = await window.crypto.subtle.importKey(
    "raw",
    enc.encode(password),
    { name: "PBKDF2" },
    false,
    ["deriveKey"]
  );

  // 2. Derivar la clave AES-GCM
  return window.crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: salt, // TypeScript ahora acepta Uint8Array aquí correctamente
      iterations: PBKDF2_ITERATIONS,
      hash: "SHA-256",
    },
    keyMaterial,
    { name: ALGO_NAME, length: 256 },
    false,
    ["encrypt", "decrypt"]
  );
}

/**
 * Cifra un texto plano (JSON string) usando una contraseña.
 * Retorna un objeto con el blob cifrado y los parámetros necesarios (IV, Salt).
 */
export async function encryptVaultItem(password: string, data: string) {
  const enc = new TextEncoder();
  const salt = window.crypto.getRandomValues(new Uint8Array(SALT_LEN));
  const iv = window.crypto.getRandomValues(new Uint8Array(IV_LEN));

  const key = await deriveKey(password, salt);

  const encryptedContent = await window.crypto.subtle.encrypt(
    { name: ALGO_NAME, iv: iv },
    key,
    enc.encode(data)
  );

  return {
    encrypted_blob: bytesToBase64(new Uint8Array(encryptedContent)),
    iv: bytesToBase64(iv),
    salt: bytesToBase64(salt),
  };
}

/**
 * Descifra un blob usando la contraseña y los parámetros originales.
 */
export async function decryptVaultItem(
  password: string,
  encryptedBlobB64: string,
  ivB64: string,
  saltB64: string
) {
  try {
    const salt = base64ToBytes(saltB64);
    const iv = base64ToBytes(ivB64);
    const encryptedData = base64ToBytes(encryptedBlobB64);

    const key = await deriveKey(password, salt);

    const decryptedContent = await window.crypto.subtle.decrypt(
      { name: ALGO_NAME, iv: iv },
      key,
      encryptedData
    );

    const dec = new TextDecoder();
    return dec.decode(decryptedContent);
  } catch (e) {
    // Seguridad: No revelar si falló la clave o la integridad (padding)
    throw new Error("Decryption failed: Invalid credentials or corrupted data.");
  }
}
