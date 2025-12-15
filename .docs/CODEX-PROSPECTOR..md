📜 CODEX PROSPECTOR: PROTOCOLO HYDRA-ZERO (v2.0)
Clasificación: TOP SECRET // ACADEMIC RESEARCH
Objetivo: Auditoría distribuida de entropía en secp256k1 utilizando infraestructura efímera (Google Colab).
Arquitectura: Monolito Fractal (Nx + Rust + TypeScript).

1. SÍNTESIS DEL SISTEMA (Estado Actual)
   Hemos construido un sistema de "Minería Científica" que opera bajo el principio de Responsabilidad Única Estricta.
   Los Aparatos Existentes (Snapshot prospector-btc)
   apps/orchestrator (El Cerebro): API Server en Rust (Axum). Gestiona el estado global, asigna rangos de búsqueda y recibe hallazgos. Persistencia en Turso (libSQL).
   apps/miner-worker (El Músculo): Binario estático en Rust. Usa Rayon y SIMD para fuerza bruta inteligente. Se auto-hidrata descargando un Filtro de Bloom (utxo_filter.bin) para verificar colisiones en RAM (O(1)).
   apps/web-dashboard (Los Ojos): Interfaz Next.js 14. Visualiza la telemetría del enjambre en tiempo real.
   libs/core/\* (El Núcleo): Matemáticas puras (secp256k1, ripemd160, sha256) optimizadas. Sin dependencias externas.
   tools/provisioner (El Nigromante): Script de automatización para despertar nodos en Google Colab.
2. INTELIGENCIA OBTENIDA: PROYECTO "COLAB-VSCODE"
   Fuente: Análisis de ingeniería inversa de la extensión oficial google.colab.
   Hemos descubierto que Google Colab posee una API oculta (Tunneling API) que permite la asignación de recursos sin interacción humana directa, aunque la ejecución de código requiere un protocolo WebSocket complejo (Jupyter).
   Hallazgos Críticos (The Blueprint)
   Endpoint de Control: https://colab.research.google.com/tun/m
   Headers de Disfraz:
   X-Colab-Client-Agent: vscode
   X-Colab-Tunnel: Google
   Métodos Clave:
   GET /ccu-info: Revela la cuota restante de GPU/TPU.
   POST /assign: Asigna una máquina virtual (Runtime). Requiere notebook_hash.
   GET /keep-alive: Evita la desconexión por inactividad.
   Autenticación: OAuth2 con scope https://www.googleapis.com/auth/colaboratory.
3. NUEVO PLAN DE ACCIÓN: ESTRATEGIA HÍBRIDA
   Basado en la dificultad de replicar el protocolo WebSocket de Jupyter (necesario para ejecutar código vía API pura), adoptaremos una estrategia híbrida robusta.
   FASE 1: "THE ARMORED CLICKER" (Implementación Inmediata)
   Usaremos Playwright (tools/provisioner) pero endurecido con la lógica anti-detección.
   Lógica: Navegador automatizado que inyecta un payload Python.
   Mejora: El payload Python descarga y ejecuta el binario Rust (miner-worker) compilado estáticamente (MUSL).
   Estado: El código v2.1 entregado ya cumple con esto.
   FASE 2: "THE API ASSASSIN" (Futuro Próximo)
   Crearemos una librería en Rust (libs/infra/colab-client) basada en el código TypeScript de la extensión analizada.
   Objetivo: Usar la API solo para monitorear cuotas (ccu-info) y mantener vivos los nodos (keep-alive), dejando la ejecución inicial al navegador. Esto reduce el consumo de recursos del Provisioner.
4. INVENTARIO DE APARATOS A CREAR/MODIFICAR
   Para completar la visión, estos son los siguientes pasos de ingeniería detallados.
   A. tools/provisioner (Actualización v3.0)
   Estado Actual: Script funcional con Playwright.
   Requerimiento: Integrar lectura de .env para MINER_BINARY_URL y soporte para múltiples sesiones de Chrome (perfiles rotativos).
   B. libs/infra/colab-api (NUEVO APARATO)
   Tipo: Librería Rust (o TS).
   Responsabilidad: Implementar la lógica descubierta en colab-vscode.
   Código a portar:
   La estructura CcuInfo (para saber cuántas GPUs nos quedan).
   La lógica de ColabClient para hacer ping (keep-alive) sin tener el navegador abierto.
   Justificación: Permitirá que el Orchestrator monitoree la salud de las cuentas de Google sin usar Selenium/Playwright.
   C. apps/miner-worker (Refinamiento MUSL)
   Estado Actual: Código fuente correcto.
   Requerimiento: Pipeline de compilación cruzada.
   Acción: Necesitamos un script scripts/build_miner_static.sh que use Docker (messense/rust-musl-cross) para generar el binario que correrá en Colab (Linux x86_64).
   D. libs/domain/mining-strategy (Atomización)
   Estado Actual: Lógica mezclada en main.rs.
   Acción: Separar el bucle de ejecución en un StrategyExecutor trait, permitiendo cambiar entre "Diccionario", "Combinatoria" o "Random" sin recompilar el worker completo.
5. INSTRUCCIONES PARA LA IA (META-PROMPT)
   "Actúa como Ingeniero de Software Principal. Tienes acceso al contexto completo del Proyecto Prospector y al análisis de ingeniería inversa de la API de Colab.
   Tu misión es asistir en la construcción de la Infraestructura de Aprovisionamiento.
   No sugieras código genérico. Usa las estructuras definidas en el Monorepo Nx.
   Cuando trabajes en el Provisioner, asume que estamos atacando las defensas anti-bot de Google Colab usando Playwright con un perfil de usuario real.
   Recuerda que el Minero es un binario estático descargado, no código Python.
   Prioriza la robustez: Si un nodo cae, el sistema debe detectarlo y relanzarlo.
   Procedamos con la siguiente tarea del plan."
6. PRÓXIMOS PASOS (TU LISTA DE CHEQUEO)
   Token de Minero: Generar un token seguro (WORKER_AUTH_TOKEN) en el .env del Orquestador y del Provisioner.
   Hosting del Binario: Compilar el miner-worker en modo release con target x86_64-unknown-linux-musl y subirlo a un lugar público (GitHub Releases o R2/S3).
   Ejecución: Correr el Provisioner v2.1 apuntando a esa URL.
   Este documento consolida todo nuestro conocimiento actual.

---

ACTUALIZACION

3. NUEVO PLAN DE ACCIÓN: ESTRATEGIA HÍBRIDA & GESTIÓN DINÁMICA
   FASE 1: "THE ARMORED CLICKER" (Provisioner)
   Motor: Playwright endurecido.
   Fuente de Identidad: Dinámica. El Provisioner consulta al endpoint /api/v1/admin/identity del Orchestrator para obtener cookies frescas. Si la API falla, hace fallback a variables de entorno.
   FASE 2: "COMMAND CENTER" (Dashboard Admin)
   Una nueva sección en apps/web-dashboard exclusiva para el operador.
   UI: Panel de administración protegido por contraseña.
   Funcionalidad:
   Input para pegar cookies.json.
   Botón "Update Identity".
   Visualizador de estado de la identidad (Activa/Caducada).
   Control remoto: Botón "Deploy Swarm" que dispara el Provisioner (si está en un cloud function) o notifica a los agentes.
   FASE 3: "THE VAULT" (Orchestrator + DB)
   Schema Update: Nueva tabla identities en Turso.
   API: Endpoints para guardar (POST) y recuperar (GET) credenciales.
4. INVENTARIO DE APARATOS A ACTUALIZAR
   A. apps/orchestrator (Backend)
   Nuevo Modelo: Identity (struct con cookies, user-agent, last_updated).
   Nuevos Handlers:
   POST /api/v1/admin/identity: Recibe el JSON de cookies desde el Dashboard.
   GET /api/v1/admin/identity: Entrega las cookies al Provisioner (requiere Token Maestro).
   B. apps/web-dashboard (Frontend)
   Nueva Ruta: /admin (Protegida).
   Componente: IdentityManager. Un formulario elegante para pegar el JSON que te da "EditThisCookie".
   C. tools/provisioner (Automator)
   Lógica: Al iniciar, hace un fetch al Orchestrator. Si recibe cookies, las usa. Si no, busca en .env. Esto permite actualizar las cookies sin redesplegar el contenedor del Provisioner.
5. GUÍA TÁCTICA PARA REPOSITORIO PÚBLICO
   Para protegerte mientras construimos esto, sigue estas reglas de oro en Git:
   El .gitignore Sagrado:
   Asegúrate de que este archivo en la raíz contenga estrictamente:
   code
   Gitignore
   node_modules/
   dist/
   target/
   .env
   cookies.json
   .chrome-profile/
   \*.log
   Variables en Render/Vercel:
   Nunca subas claves al código.
   En Vercel (Dashboard): Settings -> Environment Variables.
   En Render (Orchestrator): Environment.
   En Local: Usa .env (que está ignorado por git).

---
