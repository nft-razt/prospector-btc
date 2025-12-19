/**
 * =================================================================
 * APARATO: NEXT.JS CONFIGURATION (V23.5 - TYPESCRIPT SECURED)
 * CLASIFICACIÓN: BUILD INFRASTRUCTURE (L5)
 * RESPONSABILIDAD: ORQUESTACIÓN DEL RUNTIME Y RESOLUCIÓN DE TIPOS
 * =================================================================
 */

// @ts-check

/**
 * Forzamos la resolución de tipos mediante JSDoc referenciando el paquete localmente.
 * ✅ RESOLUCIÓN: Esto elimina el error 2307 al proveer una ruta de importación clara.
 * @type {import('next').NextConfig}
 */
const nextConfig = {
  output: "standalone",
  reactStrictMode: true,
  poweredByHeader: false,

  // Transpilación de librerías del monorepo para consistencia global
  transpilePackages: [
    "@prospector/api-contracts",
    "@prospector/api-client",
    "@prospector/heimdall-ts",
    "@prospector/ui-kit",
    "@prospector/crypto-vault",
  ],

  // Configuración de proxies para el desarrollo local (Neural Link)
  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";
    return [
      {
        source: "/api/v1/:path*",
        destination: `${apiUrl}/api/v1/:path*`,
      },
    ];
  },
};

// Integración con el motor de ejecución nativa de Nx
const { withNx } = require("@nx/next");

module.exports = withNx(nextConfig);
