/**
 * =================================================================
 * APARATO: COMMON CONTENT (ES)
 * CLASIFICACIÓN: SHARED CONTENT (ESTRATO L1-UI)
 * RESPONSABILIDAD: TEXTOS TRANSVERSALES DEL SISTEMA
 * =================================================================
 */

import { type CommonParams } from "../../schemas/common.schema";

export const commonContent: CommonParams = {
  loading: "Inicializando Sistemas...",
  error: "Fallo General del Sistema",
  copy: "Copiar al Portapapeles",
  success: "Operación Completada con Éxito",
  actions: {
    confirm: "Confirmar Acción",
    cancel: "Abortar Operación",
    back: "Regresar al Nivel Anterior",
  },
};
