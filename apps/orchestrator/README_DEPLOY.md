# 🚀 PROTOCOLO DE DESPLIEGUE: ORCHESTRATOR & FILTRO

Este documento detalla cómo desplegar el Orquestador en **Render** asegurando que el archivo crítico `utxo_filter.bin` esté disponible para los Mineros.

## 1. El Problema del Artefacto Gigante
El archivo `utxo_filter.bin` pesa ~200MB.
- ❌ No se puede subir a GitHub (Límite 100MB).
- ❌ No se debe generar en Render (Consume demasiada RAM/CPU en el build).
- ✅ Se debe descargar de una fuente externa durante la construcción de la imagen Docker.

## 2. Pasos de Preparación (Solo una vez por actualización del UTXO Set)

### A. Generar el Filtro Localmente
Ejecuta el Census Taker en tu máquina local (necesitas el CSV de BigQuery):
```bash
# Desde la raíz del proyecto
cargo run --bin census-taker -- --input ruta/a/tu/bigquery.csv --output utxo_filter.bin
B. Alojar el Artefacto (Estrategia GitHub Releases)
Ve a tu repositorio en GitHub.
Crea un nuevo Release (ej: v0.0.1-alpha).
Sube el archivo utxo_filter.bin generado como un Asset adjunto al release.
Copia el enlace de descarga directa del archivo.
Debe verse algo así: https://github.com/Usuario/repo/releases/download/v0.0.1/utxo_filter.bin
C. Configurar el Dockerfile
Edita apps/orchestrator/Dockerfile y actualiza la variable FILTER_URL con tu enlace copiado:
code
Dockerfile
ARG FILTER_URL="https://github.com/TU_USUARIO/..."
3. Configuración en Render.com
Crear Web Service:
Conecta tu repositorio de GitHub.
Selecciona el directorio raíz (defaults).
Runtime: Docker.
Variables de Entorno (Environment Variables):
Render necesita saber dónde está la base de datos y el token maestro.
Clave	Valor	Descripción
DATABASE_URL	libsql://tu-db.turso.io	URL de conexión a Turso
TURSO_AUTH_TOKEN	...	Token de Turso
WORKER_AUTH_TOKEN	secreto-super-seguro	Token que usarán los mineros
RUST_LOG	info	Nivel de logs
Deploy:
Render detectará el Dockerfile.
Durante el build, verás en los logs: ⬇️ Descargando Filtro UTXO....
Si la URL es incorrecta, el despliegue fallará (esto es bueno, evita desplegar un sistema roto).
4. Verificación
Una vez desplegado, verifica que el filtro es accesible públicamente. Los mineros usarán esta URL para auto-hidratarse:
https://tu-app-en-render.com/resources/utxo_filter.bin

---


