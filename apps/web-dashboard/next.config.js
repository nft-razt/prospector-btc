//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  nx: {
    // Svgr eliminado por obsolescencia
  },
  output: 'standalone',
  env: {
    NEXT_PUBLIC_APP_VERSION: process.env.npm_package_version,
  },
  images: {
    unoptimized: true,
  },

  // 🔥 LA MAGIA DEL PROXY 🔥
  // Esto permite que el navegador hable con el backend sin saber su URL real
  async rewrites() {
    // En producción (Render), usamos la variable interna.
    // En local, usamos localhost:3000.
    const apiUrl = process.env.INTERNAL_API_HOST
      ? `http://${process.env.INTERNAL_API_HOST}`
      : 'http://localhost:3000';

    console.log(`🔌 Next.js Proxy Tunnel configurado hacia: ${apiUrl}`);

    return [
      {
        source: '/api/v1/:path*',
        destination: `${apiUrl}/api/v1/:path*`,
      },
    ];
  },
};

const plugins = [withNx];

module.exports = composePlugins(...plugins)(nextConfig);
