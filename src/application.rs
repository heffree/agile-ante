use std::net::SocketAddr;

use axum::{response::Html, routing::get, serve::Serve, Router};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tokio::sync::{broadcast, Mutex};

use crate::{
    configuration::Settings,
    db::room::RoomRepository,
    domain::room::Room,
    routes::{client, room, sse},
};

#[derive(Clone)]
pub struct Application {
    pub rooms: Vec<Room>,
    pub room_repo: RoomRepository,
}

impl Application {
    /// Spawns an instance of the Agile Ante web server.
    pub async fn build(
        configuration: Settings,
    ) -> anyhow::Result<(u16, Serve<tokio::net::TcpListener, Router, Router>)> {
        let connection_options = SqliteConnectOptions::new()
            .filename(configuration.database.filename)
            .in_memory(configuration.database.enable_in_memory)
            .create_if_missing(true);
        let connection_pool = SqlitePool::connect_with(connection_options).await?;

        sqlx::migrate!("./migrations").run(&connection_pool).await?;

        let addr = SocketAddr::from(([0, 0, 0, 0], configuration.application.port));
        let (tx, _) = broadcast::channel(16);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let addr = listener.local_addr()?;
        let port = addr.port();

        let state = Application {
            rooms: vec![Room {
                count: Mutex::new(0).into(),
                broadcaster: tx,
            }],
            room_repo: RoomRepository::new(connection_pool.into()),
        };

        let app = Router::new()
            .merge(sse::get_sse_routes())
            .merge(room::get_room_routes())
            .with_state(state)
            .merge(client::get_client_routes())
            .fallback_service(get(not_found));

        println!("listening on {addr}");
        let serve_future = axum::serve(listener, app);

        Ok((port, serve_future))
    }
}

pub async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}
