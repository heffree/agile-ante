use agile_ante::{
    self,
    application::Application,
    configuration::{ApplicationSettings, DatabaseSettings, Settings},
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let (_, application) = Application::build(Settings {
        application: ApplicationSettings {
            port: 8080,
            enable_demo_mode: false,
        },
        database: DatabaseSettings {
            filename: "poker.db".into(),
            enable_in_memory: false,
        },
    })
    .await?;

    application.await?;

    Ok(())
}
