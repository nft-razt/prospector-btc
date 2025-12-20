# =================================================================
# APARATO: STATIC MINER BUILDER (V21.0 - REMOTE DELEGATION)
# RESPONSABILIDAD: Guía para la generación del binario en la nube
# =================================================================

Write-Host "`n🛡️ [ARCHITECTURE_NOTE]: Local Docker infrastructure not detected." -ForegroundColor Yellow
Write-Host "Pivotando hacia el Protocolo de Compilación Delegada (GitHub Forge).`n" -ForegroundColor Cyan

Write-Host "Para generar el binario x86_64-unknown-linux-musl sin Docker:" -ForegroundColor White
Write-Host "--------------------------------------------------------------"
Write-Host "1. Asegúrate de haber subido el archivo '.github/workflows/miner-release.yml'."
Write-Host "2. Ve a la pestaña 'Actions' en tu repositorio de GitHub."
Write-Host "3. Selecciona 'Hydra Binary Forge' en el panel izquierdo."
Write-Host "4. Haz clic en 'Run workflow' -> Branch: main -> Run workflow."
Write-Host "5. El binario aparecerá automáticamente en la sección 'Releases' al finalizar."
Write-Host "--------------------------------------------------------------"

Write-Host "`n⚠️ [ADVERTENCIA]: No intentes compilar para Linux directamente en Windows 10" -ForegroundColor Gray
Write-Host "sin las librerías de enlazado MUSL, ya que el binario resultante no correrá en Colab." -ForegroundColor Gray

# Nota técnica para el registro de la Tesis
# El sistema utiliza GitHub Actions como 'Build Server' para garantizar la inmutabilidad
# del entorno de compilación, cumpliendo con el estándar de reproducibilidad científica.
