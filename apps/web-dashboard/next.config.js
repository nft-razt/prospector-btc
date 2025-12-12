// =================================================================
// APARATO: NEXT.JS CONFIGURATION
// ESTADO: BLINDADO (DEPENDENCY EXCLUSION & WEBPACK FALLBACK)
// =================================================================

//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * Objetivo: Despliegue en Vercel Edge Network
 *
 * @type {import('next').NextConfig & { nx?: { svgr?: boolean } }}
 */
const nextConfig = {
  nx: {
    svgr: false,
  },

  // 1. COMPILACIÓN DE MONOREPO
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry'
  ],

  // 2. EXCLUSIONES DE SERVIDOR
  serverExternalPackages: [
    'nx',
    '@nx/devkit',
    '@nx/js',
    'typescript',
    'prettier',
    '@swc/core',
    'esbuild'
  ],

  // 3. OPTIMIZACIÓN VERCEL
  reactStrictMode: true,
  poweredByHeader: false,
  compress: true,

  // 4. GESTIÓN DE IMÁGENES (Con tipado estricto)
  images: {
    remotePatterns: [
      {
        protocol: /** @type {'https'} */ ('https'),
        hostname: 'lh3.googleusercontent.com',
      },
    ],
    unoptimized: true,
  },

  // 5. REWRITES (Proxy reverso al Backend)
  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
    return [
      {
        source: '/api/v1/:path*',
        destination: `${apiUrl}/:path*`,
      },
    ];
  },

  // 6. 🛡️ ESCUDO CONTRA ERRORES DE BUILD (CRÍTICO)
  // Interceptamos los 'require' de Nx que buscan Angular y los enviamos al vacío.
  webpack: (config, { isServer }) => {
    config.resolve.alias = {
      ...config.resolve.alias,
      // Neutralizar adaptadores de Angular
      '@angular-devkit/architect': false,
      '@angular-devkit/core': false,
      '@angular-devkit/schematics': false,
      '@angular-devkit/schematics/tools': false,
      '@angular-devkit/core/node': false,
      '@angular-devkit/architect/node': false,

      // Neutralizar herramientas internas de Nx no requeridas en runtime
      '@nx/key': false,
      '@nx/powerpack-license': false,
      '@swc-node/register': false,
      '@swc-node/register/read-default-tsconfig': false,
      '@swc-node/register/register': false,

      // Neutralizar prettier
      'prettier': false,
    };

    return config;
  },
};

const plugins = [withNx];
module.exports = composePlugins(...plugins)(nextConfig);
