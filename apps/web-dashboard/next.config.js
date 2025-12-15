// apps/web-dashboard/next.config.js
//@ts-check
const { composePlugins, withNx } = require("@nx/next");

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  output: "standalone",
  reactStrictMode: true,
  poweredByHeader: false,
  compress: true, // Gzip nativo para reducir I/O

  // Optimización de imágenes para entornos restringidos
  images: {
    remotePatterns: [
      { protocol: "https", hostname: "lh3.googleusercontent.com" },
    ],
    unoptimized: true, // Ahorra CPU en el servidor al no procesar imágenes al vuelo
  },

  // Transpilación de librerías internas del Monorepo
  transpilePackages: [
    "@prospector/api-contracts",
    "@prospector/api-client",
    "@prospector/heimdall-ts",
    "@prospector/feat-telemetry",
    "@prospector/ui-kit",
  ],

  // Rewrites para desarrollo local (Proxy al Backend)
  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";
    return [
      {
        source: "/api/v1/:path*",
        destination: `${apiUrl}/api/v1/:path*`,
      },
    ];
  },

  // Reducción de ruido en Webpack
  webpack: (config) => {
    config.resolve.alias = {
      ...config.resolve.alias,
      // Eliminación de dependencias de Angular innecesarias que a veces Nx inyecta
      "@angular-devkit/architect": false,
      "@angular-devkit/core": false,
      "@angular-devkit/schematics": false,
      "@angular-devkit/schematics/tools": false,
      "@angular-devkit/core/node": false,
      "@angular-devkit/architect/node": false,
      "@nx/key": false,
      "@nx/powerpack-license": false,
    };
    return config;
  },
};

const plugins = [
  // 🔥 CORRECCIÓN TS7006: Tipado explícito del parámetro config vía JSDoc
  // 🔥 CORRECCIÓN TS2353: Eliminación de opción 'svgr' no existente en WithNxContext v20+
  /** @param {import('next').NextConfig} config */
  (config) => withNx(config),
];

module.exports = composePlugins(...plugins)(nextConfig);
