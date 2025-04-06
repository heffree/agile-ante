use std::sync::Arc;

use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PlayerRepository {
    connection_pool: Arc<SqlitePool>,
}

impl PlayerRepository {
    pub fn new(connection_pool: Arc<SqlitePool>) -> Self {
        Self { connection_pool }
    }

    pub async fn create_new(&self, device_id: &Uuid, name: &str) -> anyhow::Result<Uuid> {
        let device_id = device_id.clone();

        sqlx::query!(
            r"
        INSERT INTO players (device_id, name)
        VALUES (?1, ?2)
            ",
            device_id,
            name
        )
        .execute(&*self.connection_pool)
        .await?;

        Ok(device_id)
    }
}
