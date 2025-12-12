//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * Nivel: Elite Production
 *
 * Corrección de Tipado: Extendemos NextConfig para admitir la propiedad 'nx'.
 * @type {import('next').NextConfig & { nx?: { svgr?: boolean } }}
 */
const nextConfig = {
  nx: {
    svgr: false,
  },

  // TURBOPACK & WEBPACK SHIELD
  serverExternalPackages: [
    'nx',
    '@nx/devkit',
    '@nx/js',
    'typescript',
    'prettier',
    '@swc/core'
  ],

  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry'
  ],

  output: 'standalone',
  reactStrictMode: true,
  poweredByHeader: false,

  images: {
    unoptimized: true,
  },

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
