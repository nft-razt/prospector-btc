🛡️ MANIFIESTO DE CALIDAD Y PRUEBAS: PROSPECTOR SYSTEM
Clasificación: PROTOCOLO DE VALIDACIÓN CIENTÍFICA
Estado: VIVO
1. FILOSOFÍA DE PRUEBAS (The Testing Pyramid)
En Prospector, no probamos "por cumplir". Probamos para demostrar invariantes matemáticas.
Unit Tests (Nivel Atómico): Viven junto al código (src/). Prueban la lógica interna (ej: suma de puntos en curva, serialización).
Property-Based Tests (Fuzzing): Usamos proptest. No probamos con un valor, probamos con millones de valores aleatorios para encontrar "Cisnes Negros".
Integration Tests (Nivel Aparato): Viven en tests/. Verifican que los módulos hablen entre sí (ej: Generador -> Estrategia).
End-to-End (Nivel Sistema): Prueban el flujo completo (CSV -> Filtro -> Minero -> Hallazgo).
2. MAPA DE RUTAS DE PRUEBA (Inventory)
A continuación, se detallan los archivos que DEBEN SER CREADOS para validar la arquitectura.
A. ESTRATO CORE (Matemáticas y Criptografía)
Aquí la tolerancia a fallos es CERO.
libs/core/math-engine
Unitario: src/hashing.rs (al final del archivo)
Lógica: Verificar vectores de prueba oficiales de NIST para SHA256 y RIPEMD160.
Integración: tests/secp256k1_vectors.rs (CREAR)
Lógica: Cargar vectores de prueba oficiales de Bitcoin Core. Verificar que privkey -> pubkey coincida con lo que dice Satoshi.
Fuzzing: tests/fuzz_keys.rs (CREAR)
Herramienta: proptest
Lógica: Generar bytes aleatorios masivos y asegurar que el motor nunca entre en pánico (Panic Freedom).
libs/core/generators
Unitario: src/address_legacy.rs
Lógica: Testear conversión de PubKey a Dirección usando la dirección del Bloque Génesis.
Unitario: src/wif.rs
Lógica: Roundtrip PrivKey -> WIF -> PrivKey. Lo que entra debe salir igual.
libs/core/probabilistic
Integración: tests/bloom_reliability.rs (CREAR)
Lógica: Insertar 1 millón de elementos aleatorios. Verificar la tasa de falsos positivos real vs la teórica. Demostrar la fórmula de la tesis.
B. ESTRATO DOMAIN (Cerebro y Estrategia)
libs/domain/mining-strategy
Unitario: src/brainwallet.rs
Lógica: Verificar frases conocidas. SHA256("correct horse battery staple") debe dar una dirección específica.
Integración: tests/strategy_execution.rs (CREAR)
Lógica: Instanciar un iterador de estrategia y correr 1000 ciclos. Asegurar que no hay fugas de memoria.
libs/domain/models-rs
Unitario: src/work.rs
Lógica: Serialización JSON. Crear un objeto WorkOrder, convertirlo a JSON string y volver a objeto. Deben ser idénticos.
C. ESTRATO INFRA (El Mundo Real)
libs/infra/transport
Unitario: src/lib.rs
Lógica: Serialización Binaria. Asegurar que los datos respetan Little Endian y que rechaza payloads gigantes (Protección DoS).
libs/infra/db-turso
Mocking: tests/mock_db.rs (CREAR)
Lógica: Como no podemos conectar a Turso en CI, creamos un Mock que simula la respuesta de la base de datos para probar el repositorio.
D. ESTRATO APPS (Robots)
apps/census-taker
E2E: tests/cli_flow.rs (CREAR)
Lógica: Invocar el binario con un CSV de prueba de 5 líneas. Verificar que crea el archivo utxo_filter.bin y que el binario es válido.
3. HERRAMIENTAS Y CONFIGURACIÓN (Best Practices)
Para elevar el nivel a "Elite", debemos instalar dependencias de desarrollo (dev-dependencies) en el Cargo.toml raíz o en cada librería.
Las Armas del Ingeniero:
proptest: Para Property-Based Testing (Fuzzing lógico).
tokio-test: Para probar funciones asíncronas (async fn).
criterion: Para Benchmarks (Medir nanosegundos). Vital para la tesis para demostrar velocidad.
Comandos de Calidad (Tu CI Pipeline Manual)
Cada vez que termines un aparato, ejecuta esta "Trinidad":
Formato: cargo fmt
Linting Estricto: cargo clippy -- -D warnings (Falla si hay una sola advertencia).
Pruebas: cargo test
4. IMPLEMENTACIÓN INMEDIATA (Proactiva)
Para cumplir con este manifiesto, ejecuta ahora mismo la instalación de las herramientas de prueba en el workspace.
Editar Cargo.toml (Raíz) y agregar a [workspace.dependencies]:

Toml
[workspace.dependencies]
# ... las que ya estaban ...
proptest = "1.4"
criterion = "0.5"
tokio-test = "0.4"
mockall = "0.12" # Para crear Mocks de la DB

---

