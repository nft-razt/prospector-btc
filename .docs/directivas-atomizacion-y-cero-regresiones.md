# 📜 CONSTITUCIÓN DE INGENIERÍA: CERO REGRESIONES Y ATOMIZACIÓN

## 1. EL MANDAMIENTO DE "CERO REGRESIONES"
- **Definición:** Ninguna refactorización puede eliminar lógica funcional previa bajo el pretexto de "limpieza".
- **Persistencia de Lógica:** Si un aparato es reemplazado por uno más rápido (ej: ASM), debe mantener el 100% de la cobertura algorítmica anterior (ECC, Modulo, Hashing).
- **Validación Cruzada:** Cada cambio en el núcleo matemático (L1) debe ser validado contra los vectores de prueba de Satoshi (Genesis Block).

## 2. ARQUITECTURA DE APARATOS ATÓMICOS (L1 - MATH)
Para evitar archivos "monstruo", la lógica matemática se divide en 4 componentes puros:

1.  **`arithmetic.rs` (Capa de Bits):** Operaciones U256 crudas (Suma, Resta, Carry) usando ASM. No sabe nada de Bitcoin ni de Curvas.
2.  **`field.rs` (Capa de Campo):** Aritmética Modular (mod p). Inversos modulares y multiplicaciones de campo.
3.  **`curve.rs` (Capa Geométrica):** Suma de puntos, duplicación y Coordenadas Jacobianas.
4.  **`constants.rs` (Capa de Identidad):** Parámetros fijos (G, n, p, a, b) de secp256k1.

## 3. ESTÁNDAR DE COMPLETITUD
- **Nomenclatura:** Prohibidas las abreviaciones (`pk` -> `public_key_point`, `sk` -> `private_scalar`).
- **TSDoc/RustDoc:** Cada función debe incluir secciones `# Errors`, `# Performance` y `# Mathematical Proof`.

---


