// apps/web-dashboard/next.config.js
//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * Objetivo: Despliegue en Vercel (Webpack Mode)
 *
 * @type {import('next').NextConfig}
 */
const nextConfig = {
  nx: {
    // Desactiva SVGR para evitar conflictos de compilación
    svgr: false,
  },

  // Transpilación de librerías internas del monorepo
  transpilePackages: [
    '@prospector/api-client',
    '@prospector/heimdall-ts',
    '@prospector/feat-telemetry'
  ],

  // Excluir herramientas de build del bundle del servidor
  serverExternalPackages: [
    'nx',
    '@nx/devkit',
    'typescript',
    'prettier',
    '@swc/core'
  ],

  reactStrictMode: true,
  poweredByHeader: false,
  compress: true,

  // Gestión de imágenes
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'lh3.googleusercontent.com',
      },
    ],
    unoptimized: true,
  },

  // Rewrites para conectar con el Backend (Render)
  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';
    return [
      {
        source: '/api/v1/:path*',
        destination: `${apiUrl}/:path*`,
      },
    ];
  },

  // 🛡️ ESCUDO ANTI-ANGULAR (CRÍTICO PARA NX + VERCEL)
  webpack: (config) => {
    config.resolve.alias = {
      ...config.resolve.alias,
      // Neutralizar dependencias de Angular que Nx intenta cargar
      '@angular-devkit/architect': false,
      '@angular-devkit/core': false,
      '@angular-devkit/schematics': false,
      '@angular-devkit/schematics/tools': false,
      '@angular-devkit/core/node': false,

      // Neutralizar herramientas internas de Nx no requeridas en runtime
      '@nx/key': false,
      '@swc-node/register': false,
    };
    return config;
  },
};

const plugins = [withNx];
module.exports = composePlugins(...plugins)(nextConfig);
