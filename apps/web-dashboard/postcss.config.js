# Crear el archivo de configuración de PostCSS
Set-Content -Path "apps/web-dashboard/postcss.config.js" -Value "module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};"
