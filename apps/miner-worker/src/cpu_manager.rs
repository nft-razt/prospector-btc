/**
 * =================================================================
 * APARATO: ZERO-ALLOCATION HARDWARE MONITOR (V120.0 - THERMAL AWARE)
 * CLASIFICACIÓN: WORKER INFRASTRUCTURE (ESTRATO L1-WORKER)
 * RESPONSABILIDAD: EXTRACCIÓN DE TELEMETRÍA DE HARDWARE EN TIEMPO REAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la vigilancia térmica y de carga del nodo sin impacto en
 * el rendimiento matemático. Accede directamente a los descriptores
 * de sistema para monitorizar el 'Thermal Throttling' en la nube.
 * =================================================================
 */

use std::fs;
use std::path::Path;

/// Estructura de telemetría de hardware de alta precisión.
pub struct NodeHardwareMetrics {
    pub cpu_frequency_megahertz: u32,
    pub system_load_average: f32,
    pub core_temperature_celsius: f32,
    pub memory_utilization_bytes: u64,
}

pub struct HardwareMonitor;

impl HardwareMonitor {
    /**
     * Captura el estado actual de los recursos del sistema.
     * Diseñado para entornos Linux (Google Colab / Docker).
     */
    pub fn capture_instantaneous_metrics() -> NodeHardwareMetrics {
        NodeHardwareMetrics {
            cpu_frequency_megahertz: Self::read_cpu_frequency(),
            system_load_average: Self::read_load_average(),
            core_temperature_celsius: Self::read_thermal_status(),
            memory_utilization_bytes: Self::read_memory_usage(),
        }
    }

    /**
     * Lee la frecuencia actual del escalador de la CPU.
     */
    fn read_cpu_frequency() -> u32 {
        let freq_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq";
        fs::read_to_string(freq_path)
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse::<u32>()
            .unwrap_or(0) / 1000 // Convertir de kHz a MHz
    }

    /**
     * Extrae el promedio de carga del sistema (Load Avg 1min).
     */
    fn read_load_average() -> f32 {
        fs::read_to_string("/proc/loadavg")
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .and_then(|val| val.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    /**
     * Monitorea la temperatura del paquete térmico (Thermal Zone 0).
     */
    fn read_thermal_status() -> f32 {
        let thermal_path = "/sys/class/thermal/thermal_zone0/temp";
        fs::read_to_string(thermal_path)
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse::<f32>()
            .unwrap_or(0.0) / 1000.0
    }

    /**
     * Calcula el consumo de RAM actual del contenedor.
     */
    fn read_memory_usage() -> u64 {
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            let mut mem_total = 0u64;
            let mut mem_available = 0u64;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    mem_total = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
                }
                if line.starts_with("MemAvailable:") {
                    mem_available = line.split_whitespace().nth(1).unwrap_or("0").parse::<u64>().unwrap_or(0);
                }
            }
            return (mem_total.saturating_sub(mem_available)) * 1024; // Convertir a bytes
        }
        0
    }
}
