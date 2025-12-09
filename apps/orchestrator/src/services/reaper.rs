pub async fn spawn_reaper(db: TursoClient) {
    tokio::spawn(async move {
        loop {
            // Lógica de limpieza
            // Detectar nodos que no han reportado en 5 min y marcarlos como muertos
            // Resetear jobs de 'processing' a 'pending' si el worker murió

            // "Arqueología de Fallos": Si un job falla 3 veces, marcar como 'toxic'
            // para investigar manualmente (quizás crashea el worker por un bug matemático).

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}
