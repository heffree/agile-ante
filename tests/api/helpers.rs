use std::future::IntoFuture;

use agile_ante::{
    application::Application,
    configuration::{ApplicationSettings, DatabaseSettings, Settings},
    routes::room::GetRoomsResponse,
};
//use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use uuid::Uuid;

pub struct TestApp {
    address: String,
    api_client: reqwest::Client,
    //db_pool: SqlitePool,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        let configuration = Settings {
            application: ApplicationSettings {
                // Port 0 gives us a random available port
                port: 0,
                enable_demo_mode: false,
            },
            database: DatabaseSettings {
                filename: Uuid::new_v4().into(),
                enable_in_memory: true,
            },
        };
        let (port, application) = Application::build(configuration.clone()).await.unwrap();
        let _ = tokio::spawn(application.into_future());

        let api_client = reqwest::Client::builder().build().unwrap();
        //let db_pool = SqlitePool::connect_with(
        //    SqliteConnectOptions::new().filename(&configuration.database.filename),
        //)
        //.await
        //.unwrap();

        TestApp {
            address: format!("http://localhost:{}", port),
            api_client,
            //db_pool,
        }
    }
    pub async fn create_room(&self) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/create-room", &self.address))
            .header("Content-Type", "application/json")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_rooms(&self) -> Vec<Uuid> {
        let res = self
            .api_client
            .get(&format!("{}/get-rooms", &self.address))
            .send()
            .await
            .expect("Failed to execute request");

        let payload: GetRoomsResponse = res.json().await.unwrap();
        payload.room_ids
    }
}
