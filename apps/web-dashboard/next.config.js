//@ts-check
const { composePlugins, withNx } = require('@nx/next');

/**
 * CONFIGURACIÓN MAESTRA NEXT.JS // PROSPECTOR BTC
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  nx: {
    // La opción svgr se gestiona ahora via plugins externos si fuera necesaria.
    // Mantenemos el objeto limpio para futuras configuraciones de Nx.
  },

  // 🔥 CRÍTICO PARA PRODUCCIÓN (VERCEL/DOCKER)
  // Genera una carpeta 'standalone' que incluye solo los node_modules necesarios.
  // Esto reduce drásticamente el tamaño de la imagen y acelera el arranque.
  output: 'standalone',

  // Inyección de variables estáticas en tiempo de compilación
  env: {
    NEXT_PUBLIC_APP_VERSION: process.env.npm_package_version || '1.0.0-snapshot',
  },

  // Configuración de Imágenes
  // 'unoptimized: true' es vital para despliegues estáticos o contenedores
  // donde no queremos depender del servicio de optimización de imágenes de Vercel (límites).
  images: {
    unoptimized: true,
  },

  // 🔌 TUNEL DE CONEXIÓN (PROXY INVERSO)
  // Permite que el Frontend hable con el Backend como si fueran el mismo dominio.
  // Evita preflight requests (OPTIONS) y problemas de CORS en navegadores estrictos.
  async rewrites() {
    // Detección inteligente del destino:
    // 1. Producción: Usa la variable de entorno inyectada en Vercel.
    // 2. Local: Usa localhost:3000 por defecto.
    // Nota: Eliminamos '/api/v1' del destino base para mapearlo dinámicamente en el return.
    const rawUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api/v1';

    // Limpieza: Aseguramos que la URL base no tenga trailing slash para evitar dobles barras
    const targetUrl = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;

    console.log(` [Next.js] Proxy Tunnel activo hacia: ${targetUrl}`);

    return [
      {
        // Captura cualquier llamada a /api/v1 en el frontend...
        source: '/api/v1/:path*',
        // ...y la redirige transparentemente al Backend en Render.
        destination: `${targetUrl}/:path*`,
      },
    ];
  },

  // Headers de Seguridad y Rendimiento
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'X-DNS-Prefetch-Control', value: 'on' },
          { key: 'X-Frame-Options', value: 'DENY' }, // Previene Clickjacking
          { key: 'X-Content-Type-Options', value: 'nosniff' },
          { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
        ],
      },
    ];
  },
};

// Composición de Plugins de Nx
// Si en el futuro añadimos 'next-intl' o 'bundle-analyzer', se apilan aquí.
const plugins = [withNx];

module.exports = composePlugins(...plugins)(nextConfig);
