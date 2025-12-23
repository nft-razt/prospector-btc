/**
 * =================================================================
 * APARATO: AUTHENTICATION CONFIGURATION (V4.24 - SOBERANO)
 * CLASIFICACIÓN: SECURITY INFRASTRUCTURE (ESTRATO L4)
 * RESPONSABILIDAD: DEFINICIÓN DE OPCIONES Y SESIÓN DE OPERADOR
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el protocolo de identidad NextAuth v4. Provee la
 * configuración maestra y un ayudante de sesión 'auth' que emula
 * el comportamiento de v5, facilitando una futura migración sin
 * romper los componentes de servidor actuales.
 * =================================================================
 */

import { NextAuthOptions, getServerSession } from "next-auth";
import GoogleProvider from "next-auth/providers/google";

/**
 * Opciones maestras de configuración para el apretón de manos con Google.
 */
export const auth_options: NextAuthOptions = {
  providers: [
    GoogleProvider({
      clientId: process.env.AUTH_GOOGLE_ID as string,
      clientSecret: process.env.AUTH_GOOGLE_SECRET as string,
    }),
  ],
  session: {
    strategy: "jwt", // Crítico para operación en el Edge / Middleware
  },
  pages: {
    signIn: "/login",
    error: "/auth/error",
  },
  callbacks: {
    /**
     * Sincroniza el ID del operador desde el proveedor al token JWT.
     */
    async jwt({ token, user }) {
      if (user) {
        token.id = user.id;
      }
      return token;
    },
    /**
     * Inyecta los metadatos del token en la sesión del cliente.
     */
    async session({ session, token }) {
      if (session.user && token.id) {
        (session.user as any).id = token.id;
      }
      return session;
    },
  },
  secret: process.env.AUTH_SECRET,
};

/**
 * AYUDANTE SOBERANO: auth()
 * Emula la API de NextAuth v5 en un entorno v4.
 *
 * @returns {Promise<Session | null>} La sesión activa del operador.
 */
export const auth = () => getServerSession(auth_options);

export default auth_options;
