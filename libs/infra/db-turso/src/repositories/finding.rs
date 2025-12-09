// libs/infra/db-turso/src/repositories/finding.rs
use crate::TursoClient;
use crate::errors::DbError;
use prospector_domain_models::finding::Finding;
use uuid::Uuid;
use libsql::params; // <-- IMPORTANTE

pub struct FindingRepository {
    client: TursoClient,
}

impl FindingRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    pub async fn save(&self, finding: &Finding) -> Result<(), DbError> {
        let conn = self.client.get_connection()?;

        let query = "INSERT OR IGNORE INTO findings (id, address, private_key_wif, source_entropy, wallet_type) VALUES (?1, ?2, ?3, ?4, ?5)";

        // CORRECCIÓN: Usamos la macro params! y clonamos los valores para pasarlos como String dueños, no referencias.
        conn.execute(query, params![
            Uuid::new_v4().to_string(),
            finding.address.clone(),
            finding.private_key_wif.clone(),
            finding.source_entropy.clone(),
            finding.wallet_type.clone()
        ]).await?;

        Ok(())
    }
}
