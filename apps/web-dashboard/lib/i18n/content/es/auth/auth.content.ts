/**
 * =================================================================
 * APARATO: AUTH CONTENT (ES)
 * RESPONSABILIDAD: INTERFAZ DE IDENTIFICACIÓN Y ACCESO
 * =================================================================
 */

import { type AuthParams } from "../../../schemas/auth/auth.schema";

export const authContent: AuthParams = {
  login: {
    title: "Identificación de Operador Requerida",
    google_btn: "Autenticar mediante Google",
    footer_text: "Conexión Segura Cifrada // TLS 1.3",
  },
  logout: {
    label: "Cerrando sesión...",
    confirm_msg: "Sesión de Operador Terminada",
  },
  errors: {
    signin_failed: "Fallo en el Apretón de Manos de Autenticación",
    access_denied: "Acceso Denegado: Nivel de Autorización Insuficiente",
  },
};
