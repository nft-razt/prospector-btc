import NextAuth from "next-auth"
import Google from "next-auth/providers/google"

export const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    Google({
      clientId: process.env.AUTH_GOOGLE_ID,
      clientSecret: process.env.AUTH_GOOGLE_SECRET,
      authorization: {
        params: {
          prompt: "consent",
          access_type: "offline",
          response_type: "code"
        }
      }
    }),
  ],
  pages: {
    signIn: "/login", // Página de login personalizada (la haremos luego)
  },
  callbacks: {
    authorized: async ({ auth }) => {
      // Logged in return true, else false
      return !!auth
    },
    session: async ({ session, token }) => {
      // Aquí podemos inyectar el rol del usuario desde nuestra DB (Turso) en el futuro
      return session;
    }
  },
  secret: process.env.AUTH_SECRET, // Obligatorio en producción
})
