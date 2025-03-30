use std::{error::Error, sync::Arc};

use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone)]
pub struct RoomRepository {
    connection_pool: Arc<SqlitePool>,
}

impl RoomRepository {
    pub fn new(connection_pool: Arc<SqlitePool>) -> Self {
        Self { connection_pool }
    }

    pub async fn create_new(&self) -> Result<Uuid, Box<dyn Error>> {
        let gen_external_id = Uuid::new_v4();

        sqlx::query!(
            r#"
        INSERT INTO rooms (external_id)
        VALUES (?1)
            "#,
            gen_external_id
        )
        .execute(&*self.connection_pool)
        .await?;

        Ok(gen_external_id)
    }

    pub async fn get_rooms(&self) -> Result<Vec<Uuid>, Box<dyn Error>> {
        let rooms = sqlx::query_scalar(
            r#"
        SELECT external_id FROM rooms
            "#,
        )
        .fetch_all(&*self.connection_pool)
        .await?;
        Ok(rooms)
    }
}
