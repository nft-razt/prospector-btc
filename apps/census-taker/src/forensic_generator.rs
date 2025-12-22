/**
 * =================================================================
 * APARATO: FORENSIC DNA GENERATOR (V100.0 - SATOSHI XP READY)
 * CLASIFICACIÓN: CORE ETL UTILITY (ESTRATO L1)
 * RESPONSABILIDAD: GENERACIÓN SINTÉTICA DE PERF_DATA_BLOCK
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la reconstrucción determinista de la memoria de Windows XP.
 * No requiere una VM activa; utiliza los desplazamientos técnicos (offsets)
 * descubiertos en la auditoría de Bitcoin v0.1.0 para posicionar
 * firmas de procesos y contadores en los bytes exactos que OpenSSL
 * consumía para RandAddSeed().
 * =================================================================
 */

use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::{Context, Result};
use tracing::{info, instrument};

/// Constantes de la Arquitectura Windows XP Professional SP3
const SYSTEM_PERFORMANCE_BUFFER_SIZE: usize = 250_000;
const SIGNATURE_OFFSET: usize = 0;
const QPC_OFFSET: usize = 24;
const QPF_OFFSET: usize = 32;

pub struct ForensicTemplateGenerator;

impl ForensicTemplateGenerator {
    /**
     * Genera una plantilla binaria "Gold Master" de Windows XP.
     *
     * @param target_output_path Ruta donde se guardará el archivo .bin
     */
    #[instrument]
    pub fn generate_xp_sp3_gold_master(target_output_path: &Path) -> Result<()> {
        info!("🧬 [DNA_GENESIS]: Synthesizing Windows XP SP3 Performance Template...");

        // 1. ALLOCATION: Reserva de memoria alineada
        let mut dna_buffer = vec![0u8; SYSTEM_PERFORMANCE_BUFFER_SIZE];

        // 2. SIGNATURE INJECTION: Firma "PERF" (Offset 0)
        dna_buffer[SIGNATURE_OFFSET..SIGNATURE_OFFSET + 4].copy_from_slice(b"PERF");

        // 3. HARDWARE CLOCK FREQUENCY (QPF): 3.579.545 MHz (Offset 32)
        // Valor estandarizado para la mayoría de motherboards de 2009.
        let frequency_bytes = 3579545u64.to_le_bytes();
        dna_buffer[QPF_OFFSET..QPF_OFFSET + 8].copy_from_slice(&frequency_bytes);

        // 4. PROCESS LINEAGE INJECTION: Simulación de procesos críticos
        // Inyectamos nombres de procesos en offsets predecibles de XP Professional
        Self::inject_process_signature(&mut dna_buffer, "system", 500);
        Self::inject_process_signature(&mut dna_buffer, "smss.exe", 1500);
        Self::inject_process_signature(&mut dna_buffer, "lsass.exe", 3500);
        Self::inject_process_signature(&mut dna_buffer, "explorer.exe", 8500);
        Self::inject_process_signature(&mut dna_buffer, "bitcoin.exe", 12500);

        // 5. NOISE SATURATION: Inyectamos entropía base para simular RAM real
        // Esto asegura que el mezclador SHA-1 no trabaje sobre ceros.
        for i in (20000..SYSTEM_PERFORMANCE_BUFFER_SIZE).step_by(32) {
            let noise = i.wrapping_mul(0x45d9f3b) as u32;
            dna_buffer[i..i + 4].copy_from_slice(&noise.to_be_bytes());
        }

        // 6. PERSISTENCIA EN DISCO
        let mut file = File::create(target_output_path)
            .context("CRITICAL_IO: Failed to create DNA artifact.")?;
        file.write_all(&dna_buffer)?;

        info!("✅ [DNA_GENESIS_COMPLETE]: Gold Master crystallized at {:?}", target_output_path);
        Ok(())
    }

    /**
     * Inyecta una cadena de proceso en el buffer simulando la estructura PERF_OBJECT_TYPE.
     */
    fn inject_process_signature(buffer: &mut Vec<u8>, name: &str, offset: usize) {
        let bytes = name.as_bytes();
        if offset + bytes.len() < buffer.len() {
            buffer[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
    }
}
