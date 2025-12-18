/**
 * =================================================================
 * APARATO: NEXT.JS CONFIG (V22.0)
 * NIVELACIÓN: RECHARTS TRANSPILATION SUPPORT
 * =================================================================
 */

//@ts-check
const { composePlugins, withNx } = require("@nx/next");

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  output: "standalone",
  reactStrictMode: true,
  poweredByHeader: false,
  compress: true,

  images: {
    remotePatterns: [
      { protocol: "https", hostname: "lh3.googleusercontent.com" },
    ],
    unoptimized: true,
  },

  // ✅ NIVELACIÓN: Añadimos 'recharts' para asegurar compatibilidad con el servidor
  transpilePackages: [
    "@prospector/api-contracts",
    "@prospector/api-client",
    "@prospector/heimdall-ts",
    "@prospector/feat-telemetry",
    "@prospector/ui-kit",
    "recharts"
  ],

  async rewrites() {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";
    return [
      {
        source: "/api/v1/:path*",
        destination: `${apiUrl}/api/v1/:path*`,
      },
    ];
  },

  webpack: (config) => {
    config.resolve.alias = {
      ...config.resolve.alias,
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
  /** @param {import('next').NextConfig} config */
  (config) => withNx(config),
];

module.exports = composePlugins(...plugins)(nextConfig);
