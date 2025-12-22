/**
 * =================================================================
 * APARATO: CRYPTO VAULT MASTER BARREL (V55.0)
 * CLASIFICACIÓN: CORE SECURITY (ESTRATO L1)
 * RESPONSABILIDAD: EXPOSICIÓN DE LA INTERFAZ DE CIFRADO SOBERANO
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el único punto de entrada para la lógica de cifrado.
 * Resuelve la circularidad de tipos al delegar la definición de
 * interfaces al motor de lógica 'aes-gcm.ts'.
 * =================================================================
 */

export {
  VaultCryptoEngine,
  type EncryptedVaultPayload
} from "./lib/aes-gcm";

/**
 * CERTIFICACIÓN DE CALIDAD:
 * Los tipos exportados aquí son consumidos por 'IdentityInjector' y
 * 'api-contracts', garantizando que el material cifrado en el navegador
 * sea reconstruible de forma determinista.
 */
