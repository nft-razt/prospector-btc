# 🧪 PROSPECTOR PROVER (Apparatus)

**Herramienta de Certificación de Integridad del Sistema.**

Este componente genera "Golden Tickets": escenarios de prueba donde conocemos de antemano la clave privada y nos aseguramos de que está incluida en el Filtro de Bloom y dentro de un rango de trabajo válido.

## 🎯 Objetivo

Validar que todo el pipeline (Minería -> Filtrado -> Reporte) funciona correctamente antes de gastar recursos en buscar claves desconocidas.

## 🚀 Uso

Generar un escenario de prueba en la carpeta `dist/proof`:

```bash
nx run prospector-prover:run --args="--output dist/proof --target 123"
```
