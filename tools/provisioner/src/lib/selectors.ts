/**
 * =================================================================
 * APARATO: UI SELECTORS REPOSITORY (HARDENED)
 * RESPONSABILIDAD: Single Source of Truth (SSoT) para el DOM de Colab.
 * ESTRATEGIA: Selectores resilientes con fallbacks de texto difuso.
 * =================================================================
 */

export const SELECTORS = {
  /**
   * Detección de Muros de Autenticación.
   * Se buscan botones de "Sign in" o redirecciones a cuentas de Google.
   */
  AUTH: {
    SIGN_IN_BTN: [
      "text=Sign in",
      'a[href*="accounts.google.com"]',
      "#gb > div > div > a",
      // Fallback XPath para texto difuso insensible a mayúsculas/minúsculas
      'xpath=//a[contains(translate(text(), "SIGN", "sign"), "sign in")]',
    ],
  },

  /**
   * Interacción con el Runtime (Conexión/Reconexión).
   */
  RUNTIME: {
    CONNECT_BTN: [
      "colab-connect-button", // Shadow DOM host (Ideal)
      "#connect", // ID Legacy
      "text=Connect", // Texto explícito
      "text=Reconnect", // Estado de error previo
      'button:has-text("Connect")', // Playwright selector
      'xpath=//div[contains(text(), "Connect")]', // XPath robusto
    ],
    // Elementos que confirman que la VM está asignada (RAM/Disk bars)
    RESOURCE_MONITOR: "colab-memory-usage-sparkline",
    STATUS_BAR_BUSY: 'colab-status-bar[status="busy"]',
  },

  /**
   * Interacción con el Editor de Código (Monaco).
   */
  EDITOR: {
    LINE: ".view-lines",
    FOCUSED: ".monaco-editor.focused",
    CELL_RUNNING: ".code-cell.running",
  },

  /**
   * Diálogos de Error y Captchas.
   */
  ALERTS: {
    CAPTCHA_IFRAME: 'iframe[src*="recaptcha"]',
    ERROR_DIALOG: "colab-dialog",
    DISCONNECTED: "text=Runtime disconnected",
    RUNTIME_ERROR: "text=Runtime error",
  },
} as const;
