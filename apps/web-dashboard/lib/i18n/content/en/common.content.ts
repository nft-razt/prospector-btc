// apps/web-dashboard/lib/i18n/content/en/common.content.ts
/**
 * =================================================================
 * APARATO: COMMON CONTENT (EN)
 * ESTADO: FIXED (PATH RESOLUTION ADJUSTED)
 * =================================================================
 */

// CORRECCIÓN: Ajuste de profundidad de ruta relativa (../../)
import { type CommonParams } from "../../schemas/common.schema";

export const commonContent = {
  loading: "Initializing Systems...",
  error: "System Fault",
  copy: "Copy",
  success: "Operation Successful",
  actions: {
    confirm: "Confirm",
    cancel: "Cancel",
    back: "Back",
  },
} satisfies CommonParams;
