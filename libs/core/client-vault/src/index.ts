/**
 * =================================================================
 * APARATO: CRYPTO VAULT BARREL (V36.0)
 * CLASIFICACIÓN: CORE SECURITY (L1)
 * RESPONSABILIDAD: EXPOSICIÓN DE MOTORES DE CIFRADO SIMÉTRICO
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS
 * =================================================================
 */

// Exportación del motor de clase para cifrado AES-GCM
export { VaultCryptoEngine } from "./lib/aes-gcm";

// Exportación de tipos de payload para contratos de red
export type { EncryptedVaultPayload } from "./lib/aes-gcm";

/**
 * Nota de Arquitectura:
 * Se mantienen las funciones utilitarias para compatibilidad legacy
 * pero se recomienda el uso de VaultCryptoEngine para operaciones ZK.
 */
export * from "./lib/aes-gcm";
