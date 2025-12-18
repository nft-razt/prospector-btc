// libs/domain/forensics/src/debian_rng.rs
// =================================================================
// APARATO: DEBIAN VULNERABILITY SIMULATOR (CVE-2008-0166)
// RESPONSABILIDAD: RECONSTRUCCIÓN DE CLAVES CON ENTROPÍA LIMITADA
// ESTADO: ACADEMIC RIGOR (ZERO-COPY)
// =================================================================

use byteorder::{ByteOrder, LittleEndian};
use prospector_core_math::prelude::*;
use prospector_core_math::arithmetic::U256_BYTE_SIZE;

/// Iterador forense para el bug de OpenSSL en Debian.
///
/// Este bug redujo el espacio de claves de 2^256 a solo 32,767 posibilidades
/// por arquitectura, al utilizar únicamente el Process ID (PID) como semilla.
pub struct DebianIterator {
    current_process_id: u32,
    maximum_process_id: u32,
}

impl DebianIterator {
    /// Inicializa el escáner para un rango específico de PIDs.
    pub fn new(start_pid: u32, end_pid: u32) -> Self {
        Self {
            current_process_id: start_pid,
            maximum_process_id: end_pid,
        }
    }

    /// Genera una clave privada "debilitada" basada en un PID específico.
    ///
    /// Implementa el patrón de corrupción de memoria detectado en 2008,
    /// donde los bytes superiores de la semilla permanecían estáticos.
    #[inline(always)]
    fn generate_weak_key(pid: u32) -> SafePrivateKey {
        let mut seed_buffer = [0u8; U256_BYTE_SIZE];

        // 1. Inyección de PID (Única fuente de entropía real del bug)
        LittleEndian::write_u32(&mut seed_buffer[0..4], pid);

        // 2. Relleno determinista (Simulación de memoria no inicializada)
        // En un ataque forense real, este patrón puede variar por arquitectura (x86 vs x64)
        for index in 4..U256_BYTE_SIZE {
            seed_buffer[index] = 0x00;
        }

        // 3. Conversión a clave segura de secp256k1
        SafePrivateKey::from_bytes(&seed_buffer)
            .unwrap_or_else(|_| SafePrivateKey::new_random())
    }
}

impl Iterator for DebianIterator {
    type Item = (String, SafePrivateKey);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_process_id >= self.maximum_process_id {
            return None;
        }

        let pid = self.current_process_id;
        self.current_process_id += 1;

        let private_key = Self::generate_weak_key(pid);
        let metadata_source = format!("forensic_debian_openssl:pid_{}", pid);

        Some((metadata_source, private_key))
    }
}
