//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * Nivel: Elite Production (Turbopack Compatible)
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  nx: {
    // Svgr se gestiona via plugins externos si es necesario.
    svgr: false,
  },

  // 🔥 CRÍTICO: OPTIMIZACIÓN DE DEPENDENCIAS (FIX TURBOPACK)
  // Evita que Turbopack intente empaquetar herramientas de build (Nx)
  // que contienen referencias opcionales a Angular, rompiendo el build.
  serverExternalPackages: [
    'nx',
    '@nx/devkit',
    '@nx/js',
    'prettier',
    'typescript',
    '@swc/core'
  ],

  // Aseguramos que nuestras librerías internas sí se transpilen
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry'
  ],

  // 🔥 CRÍTICO PARA PRODUCCIÓN (VERCEL/DOCKER)
  // Genera un standalone folder reducido para despliegues ligeros.
  output: 'standalone',

  // Inyección de variables estáticas
  env: {
    NEXT_PUBLIC_APP_VERSION: process.env.npm_package_version || '1.0.0-snapshot',
  },

  images: {
    unoptimized: true, // Vital para contenedores sin procesador de imágenes externo
  },

  // 🔌 TUNEL DE CONEXIÓN (PROXY INVERSO)
  async rewrites() {
    const rawUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
    const targetUrl = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;

    console.log(` [Next.js] Proxy Tunnel activo hacia: ${targetUrl}`);

    return [
      {
        source: '/api/v1/:path*',
        destination: `${targetUrl}/:path*`,
      },
    ];
  },

  // Headers de Seguridad
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-DNS-Prefetch-Control', value: 'on' },
          { key: 'X-Frame-Options', value: 'DENY' },
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
        ],
      },
    ];
  },

  // Red de seguridad para Webpack (si Turbopack se deshabilita)
  webpack: (config) => {
    config.externals.push('pino-pretty', 'lokijs', 'encoding');
    return config;
  },
};

const plugins = [withNx];

module.exports = composePlugins(...plugins)(nextConfig);
