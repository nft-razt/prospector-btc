// tools/provisioner/src/lib/selectors.ts
/**
 * =================================================================
 * APARATO: UI SELECTORS REPOSITORY
 * RESPONSABILIDAD: Single Source of Truth (SSoT) para elementos del DOM.
 * ESTRATEGIA: Arrays de prioridad para selectores de respaldo.
 * =================================================================
 */

export const SELECTORS = {
  /** Botones de Inicio de Sesión (Detección de Auth Wall) */
  SIGN_IN_BTN: [
    'text=Sign in',
    'a[href*="accounts.google.com"]',
    '#gb > div > div > a'
  ],

  /** Botones de Conexión al Runtime */
  CONNECT_BTN: [
    'colab-connect-button',           // Shadow DOM host usual
    '#connect',                       // ID Legacy
    'text=Connect',                   // Texto explícito
    'text=Reconnect',                 // Estado de error previo
    'button:has-text("Connect")'      // Fallback genérico
  ],

  /** Menú de configuración de Runtime */
  RUNTIME_MENU: 'colab-runtime-menu-button',
  CHANGE_RUNTIME_ITEM: 'text=Change runtime type',

  /** Configuración de GPU/TPU */
  ACCELERATOR_RADIO: {
    GPU: 'paper-radio-button[name="accelerator"][label="T4 GPU"]',
    TPU: 'paper-radio-button[name="accelerator"][label="TPU v2"]',
    NONE: 'paper-radio-button[name="accelerator"][label="None"]'
  },
  SAVE_RUNTIME_BTN: 'text=Save',

  /** Editor de Código (Monaco/CodeMirror) */
  EDITOR_LINE: '.view-lines',
  EDITOR_FOCUSED: '.monaco-editor.focused',

  /** Indicadores de Estado (Observabilidad Visual) */
  RAM_DISK_BAR: 'colab-memory-usage-sparkline', // Aparece solo cuando hay VM asignada
  BUSY_INDICATOR: 'colab-status-bar[status="busy"]',
  EXECUTING_CELL: '.code-cell.running',

  /** Diálogos de Error/Captcha */
  CAPTCHA_IFRAME: 'iframe[src*="recaptcha"]',
  ERROR_DIALOG: 'colab-dialog',
  DISCONNECTED_MSG: 'text=Runtime disconnected'
};
