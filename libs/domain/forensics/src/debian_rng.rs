// =================================================================
// APARATO: DEBIAN OPENSSL BUG SIMULATOR (CVE-2008-0166)
// RESPONSABILIDAD: GENERACIÓN DETERMINISTA DE CLAVES DÉBILES
// =================================================================

use prospector_core_math::private_key::SafePrivateKey;
use byteorder::{LittleEndian, ByteOrder};

/// Iterador de Entropía Defectuosa (Debian 2006-2008).
///
/// Recorre el espacio de Process IDs (PIDs) típicos de Linux (0..32768)
/// simulando el fallo crítico en el generador de números aleatorios de OpenSSL
/// que ocurrió debido a la eliminación accidental de líneas de código de seeding.
pub struct DebianIterator {
    current_pid: u64,
    end_pid: u64,
}

impl DebianIterator {
    /// Inicializa el escáner forense para un rango de PIDs.
    ///
    /// # Argumentos
    /// * `start`: PID inicial (usualmente 0 o 1000).
    /// * `end`: PID final (usualmente 32768, el máximo por defecto en kernels viejos).
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            current_pid: start,
            end_pid: end,
        }
    }

    /// Reconstruye matemáticamente una clave privada generada con entropía nula.
    ///
    /// En el incidente real, el buffer de entropía no se inicializaba correctamente,
    /// usando solo el PID como única variable cambiante.
    fn generate_weak_key(pid: u64) -> SafePrivateKey {
        // Buffer de 32 bytes (256 bits) para la clave privada
        let mut bytes = [0u8; 32];

        // 1. Inyectamos el PID en los primeros 8 bytes (Little Endian)
        // Esto simula la única variación real que ocurría en el sistema.
        ByteOrder::write_u64::<LittleEndian>(&mut bytes[0..8], pid);

        // 2. Relleno de Memoria (Padding)
        // En sistemas reales, el resto de la memoria podía ser ceros o basura residual.
        // Usamos un patrón constante hexadecimal conocido para simular este estado determinista.
        // (DEADBEEF es un marcador clásico en depuración de memoria).
        bytes[8] = 0xDE;
        bytes[9] = 0xAD;
        bytes[10] = 0xBE;
        bytes[11] = 0xEF;
        // Los bytes 12..31 permanecen en 0x00, completando la simulación.

        // 3. Instanciación Segura
        // Intentamos crear la clave privada. Si el patrón generado resulta en un escalar
        // inválido para la curva secp256k1 (extremadamente improbable, 1 en 2^128),
        // hacemos fallback a una clave aleatoria para no detener el worker.
        SafePrivateKey::from_bytes(&bytes).unwrap_or_else(|_| {
            // Log de advertencia podría ir aquí en un sistema con tracing inyectado
            SafePrivateKey::new_random()
        })
    }
}

impl Iterator for DebianIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pid >= self.end_pid {
            return None;
        }

        let pid = self.current_pid;
        self.current_pid += 1;

        // Generamos la clave vulnerable
        let pk = Self::generate_weak_key(pid);

        // Etiquetamos la fuente para que el reporte indique claramente el origen
        let source = format!("forensic_cve_2008_0166:pid_{}", pid);

        Some((source, pk))
    }
}
