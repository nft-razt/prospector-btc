/**
 * =================================================================
 * APARATO: SYSTEM CONTENT (ES)
 * RESPONSABILIDAD: PÁGINAS DE ERROR Y ESTADOS DE MANTENIMIENTO
 * =================================================================
 */

import { type SystemParams } from "../../../schemas/system/system.schema";

export const systemContent: SystemParams = {
  not_found: {
    title: "SEÑAL PERDIDA",
    description: "Las coordenadas solicitadas no corresponden a ningún sector conocido en la red Prospector.",
    error_code: "ERROR_404_VACÍO",
    cta_return: "Regresar al Centro de Mando",
  },
  maintenance: {
    title: "SISTEMA EN ACTUALIZACIÓN",
    message: "El protocolo Hydra-Zero está bajo mantenimiento crítico de infraestructura.",
  },
};
