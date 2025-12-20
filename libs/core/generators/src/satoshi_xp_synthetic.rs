/**
 * =================================================================
 * APARATO: SATOSHI XP SYNTHETIC HYDRATOR (V140.0 - SOBERANO)
 * CLASIFICACIÓN: CORE GENERATOR (ESTRATO L1)
 * RESPONSABILIDAD: GENERACIÓN DE BUFFER XP SIN HARDWARE REAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Provee una plantilla binaria de alta fidelidad basada en el
 * estándar PERF_DATA_BLOCK de Windows XP SP3 (Build 2600).
 * Permite la validación de la tesis en ausencia de una VM.
 * =================================================================
 */

pub struct SatoshiXpSyntheticHydrator;

impl SatoshiXpSyntheticHydrator {
    /// Genera una plantilla base de 250,000 bytes con la firma "PERF".
    pub fn generate_gold_master_template() -> Vec<u8> {
        let mut buffer = vec![0u8; 250000];

        // 1. SIGNATURE: "PERF" (Offset 0)
        buffer[0] = b'P';
        buffer[1] = b'E';
        buffer[2] = b'R';
        buffer[3] = b'F';

        // 2. VERSION: Windows XP utiliza 0x00010001 (Offset 4)
        buffer[4] = 0x01;
        buffer[6] = 0x01;

        // 3. HEADER SIZE: Típicamente 160 bytes para el encabezado maestro
        buffer[8] = 160;

        // 4. PERFORMANCE FREQUENCY: 3,579,545 MHz (Offset 32)
        let frequency_bytes = 3579545u64.to_le_bytes();
        buffer[32..40].copy_from_slice(&frequency_bytes);

        // 5. SECCIÓN DE PROCESOS (Simulación de Estructura Object 230)
        // Inyectamos ruido predecible simulando procesos básicos de 2009
        Self::inject_process_noise(&mut buffer);

        buffer
    }

    fn inject_process_noise(buffer: &mut Vec<u8>) {
        let processes = ["system", "smss.exe", "lsass.exe", "services.exe", "explorer.exe", "bitcoin.exe"];
        let mut offset = 200; // Iniciamos después del header

        for name in processes {
            if offset + 50 > buffer.len() { break; }
            let name_bytes = name.as_bytes();
            buffer[offset..offset + name_bytes.len()].copy_from_slice(name_bytes);
            offset += 1000; // Espaciado simulado
        }
    }
}
