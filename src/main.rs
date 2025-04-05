use std::env;

use agile_ante::{
    self,
    application::Application,
    configuration::{ApplicationSettings, DatabaseSettings, Settings},
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let db_filename = env::var("DATABASE_URL").unwrap();
    let (_, application) = Application::build(Settings {
        application: ApplicationSettings {
            port: 8080,
            enable_demo_mode: false,
        },
        database: DatabaseSettings {
            filename: db_filename,
            enable_in_memory: false,
        },
    })
    .await?;

    application.await?;

    Ok(())
}
