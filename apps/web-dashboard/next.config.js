// =================================================================
// APARATO: NEXT.JS CONFIGURATION (BLINDADO)
// OBJETIVO: EVASIÓN DE ERRORES DE ADAPTADORES ANGULAR/NX EN VERCEL
// =================================================================

//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  // 1. BUILD OUTPUT
  output: 'standalone',

  // 2. OPTIMIZACIONES
  reactStrictMode: true,
  poweredByHeader: false,
  compress: true,

  // 3. IMÁGENES
  images: {
    remotePatterns: [
      { protocol: 'https', hostname: 'lh3.googleusercontent.com' },
    ],
    unoptimized: true,
  },

  // 4. TRANSPILACIÓN MONOREPO
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry' // Asegurar que todas las libs UI se transpilen
  ],

  // 5. PROXY (SOLO DEV/PREVIEW)
  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';
    return [
      {
        source: '/api/v1/:path*',
        destination: `${apiUrl}/api/v1/:path*`,
      },
    ];
  },

  // 6. 🛡️ ESCUDO WEBPACK (CRÍTICO PARA VERCEL/NX)
  // Ignora módulos que Nx intenta cargar dinámicamente pero no existen
  webpack: (config, { isServer }) => {
    config.resolve.alias = {
      ...config.resolve.alias,
      // Neutralizar adaptadores de Angular que causan 'Module not found'
      '@angular-devkit/architect': false,
      '@angular-devkit/core': false,
      '@angular-devkit/schematics': false,
      '@angular-devkit/schematics/tools': false,
      '@angular-devkit/core/node': false,
      '@angular-devkit/architect/node': false,

      // Neutralizar herramientas internas de Nx no requeridas en runtime
      '@nx/key': false,
      '@nx/powerpack-license': false,
    };

    return config;
  },
};

const plugins = [withNx];
module.exports = composePlugins(...plugins)(nextConfig);
