🕵️ MANUAL DE EXTRACCIÓN DE IDENTIDAD (COOKIES & TOKENS)
Objetivo: Obtener el archivo cookies.json de una sesión autenticada de Google.
Por qué: Este archivo es el "Pasaporte". Al inyectarlo en el Provisioner remoto, Google creerá que es tu navegador de confianza, evitando CAPTCHAs y 2FA.
OPCIÓN A: LA VÍA RÁPIDA (Extensión de Navegador)
Ideal si tu notebook es muy lento para correr scripts de Node.js.
Instalar Extensión:
Instala "EditThisCookie" o "Cookie-Editor" en tu Chrome/Brave local.
Chrome Web Store Link
Login Limpio:
Abre una pestaña nueva.
Ve a https://colab.research.google.com.
Asegúrate de estar logueado con la cuenta que usarás para minar.
Extracción:
Haz clic en el icono de la extensión (la galleta).
Busca la opción "Export" o "Export as JSON".
Se copiará un texto largo en tu portapapeles.
Guardado:
Crea un archivo llamado cookies.json en la carpeta tools/provisioner/ de tu proyecto.
Pega el contenido.
OPCIÓN B: "THE HARVESTER" (Script Automatizado)
Si prefieres hacerlo vía código para integrarlo en el flujo.
He creado un pequeño script dentro de tools/provisioner que abrirá un navegador ligero, esperará a que te loguees manualmente, y guardará las cookies automáticamente al cerrar.
Script tools/provisioner/src/harvester.ts

---
