// =================================================================
// APARATO: NEXT.JS CONFIGURATION (PURE)
// OBJETIVO: DESPLIEGUE SERVERLESS / EDGE OPTIMIZADO
// =================================================================

//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  // 1. GENERACIÓN DE SALIDA (Vital para Docker/Vercel)
  output: 'standalone',

  // 2. OPTIMIZACIONES
  reactStrictMode: true,
  poweredByHeader: false, // Seguridad por oscuridad (oculta "X-Powered-By: Next.js")
  compress: true,         // Gzip/Brotli automático

  // 3. IMÁGENES REMOTAS (Google Auth Avatars)
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'lh3.googleusercontent.com',
      },
    ],
    unoptimized: true, // Ahorra costos de procesamiento de imagen en Vercel
  },

  // 4. TRANSPILE (Monorepo Link)
  // Asegura que las librerías locales se compilen correctamente
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts'
  ],

  // 5. PROXY REVERSO (Desarrollo Local)
  // Solo aplica en 'next dev'. En prod, usas variables de entorno completas.
  async rewrites() {
    return [
      {
        source: '/api/v1/:path*',
        destination: process.env.NEXT_PUBLIC_API_URL
          ? `${process.env.NEXT_PUBLIC_API_URL}/:path*`
          : 'http://localhost:3000/api/v1/:path*',
      },
    ];
  },
};

const plugins = [
  // Inyecta automáticamente la inteligencia de Nx
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);
