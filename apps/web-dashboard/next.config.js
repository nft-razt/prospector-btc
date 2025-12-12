//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * Nivel: Elite Production
 */
const nextConfig = {
  nx: {
    svgr: false,
  },

  // TURBOPACK & WEBPACK SHIELD
  // Evita conflictos de resolución de dependencias de build tools
  serverExternalPackages: [
    'nx',
    '@nx/devkit',
    '@nx/js',
    'typescript',
    'prettier',
    '@swc/core'
  ],

  // Inclusión explícita de librerías internas del monorepo
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry'
  ],

  output: 'standalone',
  reactStrictMode: true,
  poweredByHeader: false, // Seguridad por oscuridad (oculta X-Powered-By)

  // Optimización de Imágenes para Contenedores
  images: {
    unoptimized: true,
  },

  // Proxy Reverso Interno
  async rewrites() {
    const rawUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
    const targetUrl = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;

    return [
      {
        source: '/api/v1/:path*',
        destination: `${targetUrl}/:path*`,
      },
    ];
  },

  // Headers de Seguridad HTTP
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-DNS-Prefetch-Control', value: 'on' },
          { key: 'X-Frame-Options', value: 'DENY' },
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
          { key: 'Permissions-Policy', value: "camera=(), microphone=(), geolocation=()" }
        ],
      },
    ];
  },
};

const plugins = [withNx];
module.exports = composePlugins(...plugins)(nextConfig);
