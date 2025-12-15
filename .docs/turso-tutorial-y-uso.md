# 🗄️ MANUAL DE INFRAESTRUCTURA: TURSO (libSQL)

**Clasificación:** INFRAESTRUCTURA DE PERSISTENCIA
**Tecnología:** libSQL (Fork de SQLite optimizado para el Edge)
**Estrategia:** Distributed Sharding (Capa Gratuita Masiva)

---

## 1. ¿QUÉ ES Y POR QUÉ LO USAMOS?

**Turso** es la plataforma comercial construida sobre **libSQL**.
A diferencia de PostgreSQL o MySQL, Turso no requiere un servidor pesado corriendo. Funciona bajo el protocolo HTTP/WebSocket, lo que lo hace perfecto para entornos "Serverless" como Google Colab o Cloud Functions.

### Ventajas Estratégicas para Prospector (Tesis):

1.  **Costo Cero (Hydra-Zero):** El plan gratuito permite **9GB de almacenamiento** y **1 Billón de lecturas** al mes.
2.  **Replicación Edge:** Los datos viven cerca del usuario (baja latencia para el Dashboard).
3.  **Portabilidad Extrema:**
    - **En Local:** Es un simple archivo (`prospector.db`).
    - **En Nube:** Es una URL (`libsql://...`).
    - **El código Rust NO CAMBIA** entre uno y otro.

---

## 2. INSTALACIÓN EN WINDOWS (Referencia)

Dado que en Windows la instalación nativa a veces falla, utilizamos dos métodos.

### Método A: Inyección Quirúrgica (El que usamos)

1.  Descargar el binario `turso.exe` desde [GitHub Releases](https://github.com/tursodatabase/turso-cli/releases).
2.  Moverlo a `%USERPROFILE%\.cargo\bin` (ya que esta ruta está en el PATH de Rust).
3.  Verificar: `turso --version`.

### Método B: Vía WSL (Subsistema Linux)

Si usas la terminal de Ubuntu en Windows:

```bash
curl -sSfL https://get.tur.so/install.sh | bash
Si usas la terminal de Ubuntu en Windows:
```

3. AUTENTICACIÓN (LOGIN)
   El CLI de Turso te pide loguearte para poder crear bases de datos en la nube.
   Si solo vas a trabajar en local con archivos, este paso es opcional, pero RECOMENDADO para el despliegue final.

Pasos para Loguearse:
Ejecuta en la terminal:

Si usas la terminal de Ubuntu en Windows:

```Cmd
turso auth signup
Si usas la terminal de Ubuntu en Windows:
```

# O si ya tienes cuenta:

Si usas la terminal de Ubuntu en Windows:

```Cmd
turso auth login
Si usas la terminal de Ubuntu en Windows:
```

Esto abrirá tu navegador. Autoriza con GitHub.
Al volver a la terminal, dirá: ✔ Success! Logged in as [tu_usuario].

4. MANUAL DE USO (CHEATSHEET)
   Estos son los comandos que usarás durante el desarrollo.
   A. Gestión de Bases de Datos

```Cmd
# Crear una nueva base de datos en la nube
turso db create prospector-db
```

# Ver la lista de tus bases de datos

```Cmd
turso db list
```

# Ver la información de conexión (URL)

```Cmd
turso db show prospector-db
```

B. Conexión y Tokens (Para el código Rust)
Para que el Orquestador se conecte, necesita URL y Token.

```Cmd
# Obtener la URL (empieza con libsql://)
turso db show prospector-db --url
```

# Generar un token de seguridad (Password)

```Cmd
turso db tokens create prospector-db
```

Estos dos valores son los que pondremos en el archivo .env en producción.

C. Consola SQL (Shell)
Para inspeccionar datos manualmente o crear tablas a mano:

```Cmd
turso db shell prospector-db
Dentro puedes escribir SQL normal: SELECT * FROM findings;
```

5. ESTRATEGIA DE INTEGRACIÓN (RUST & DEPLOY)
   IMPORTANTE: No instalaremos Turso en Render ni en Koyeb. Turso ES el servicio de hosting de la base de datos.
   Nuestra aplicación (Orquestador) solo actúa como Cliente.

Escenario 1: Desarrollo Local (Tu PC)
Modo: Archivo Local (Embedded).
Configuración .env:

```Ini
DATABASE_URL=file:prospector_local.db
```

Comportamiento: La librería de Rust crea un archivo en tu disco duro. No necesita internet. Más rápido para pruebas.

Escenario 2: Producción (Render / Koyeb)
Modo: Remoto (Cloud).
Configuración (Variables de entorno en el host):

```Ini
DATABASE_URL=libsql://prospector-db-[tu-usuario].turso.io
TURSO_AUTH_TOKEN=eyJh... (el token largo)
```

Comportamiento: El Orquestador se conecta vía HTTP seguro a la nube de Turso.

Escenario 3: "Hydra Sharding" (Futuro Tesis)
Si llenamos los 9GB gratis, el script tools/provisioner puede usar el CLI de Turso para crear automáticamente prospector-shard-02, prospector-shard-03, etc., distribuyendo los datos horizontalmente sin pagar un centavo.

6. RESUMEN DE FLUJO
   Tú (Dev): Usas turso db shell para ver si los mineros están encontrando algo.

Orquestador (App): Usa la librería libsql-client de Rust para guardar los hallazgos.
Turso (Cloud): Guarda los datos y los replica globalmente.
Dashboard (Web): Lee de Turso para mostrar las gráficas bonitas.

---
