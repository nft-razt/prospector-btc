#[cfg(test)]
mod tests {
    use crate::work::{SearchStrategy, WorkOrder};
    use uuid::Uuid;

    #[test]
    fn test_work_order_serialization_with_large_numbers() {
        // ESCENARIO: Un rango más grande que u64 (simulado con string)
        // 2^64 = 18446744073709551616
        // Usamos valores que romperían un u64 normal para probar la robustez del String
        let huge_start = "18446744073709551617";
        let huge_end = "18446744073709551627";

        let order = WorkOrder {
            id: Uuid::new_v4().to_string(),
            target_duration_sec: 600,
            strategy: SearchStrategy::Combinatoric {
                prefix: "BTC".to_string(),
                suffix: "".to_string(),
                start_index: huge_start.to_string(),
                end_index: huge_end.to_string(),
            },
        };

        // 1. Serializar a JSON
        let json = serde_json::to_string(&order).expect("Fallo crítico en serialización");
        println!("JSON Payload generado: {}", json);

        // 2. Verificar que el JSON contiene los strings y no enteros crudos
        assert!(json.contains(huge_start));
        assert!(json.contains("\"type\":\"Combinatoric\""));

        // 3. Deserializar de vuelta (Roundtrip check)
        let recovered: WorkOrder =
            serde_json::from_str(&json).expect("Fallo crítico en deserialización");

        if let SearchStrategy::Combinatoric {
            start_index,
            end_index,
            ..
        } = recovered.strategy
        {
            assert_eq!(start_index, huge_start);
            assert_eq!(end_index, huge_end);
        } else {
            panic!("La estrategia recuperada no coincide con el tipo original");
        }
    }
}
