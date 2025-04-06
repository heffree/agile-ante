use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    serve::Serve,
    Router,
};
use reqwest::StatusCode;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tokio::sync::{broadcast, Mutex};

use crate::{
    configuration::Settings,
    db::room_repository::RoomRepository,
    domain::room::Room,
    routes::{client, room, room_connection},
};

#[derive(Clone)]
pub struct Application {
    pub rooms: HashMap<String, Arc<Mutex<Room>>>,
    pub room_repo: RoomRepository,
}

impl Application {
    /// Spawns an instance of the Agile Ante web server.
    pub async fn build(
        configuration: Settings,
    ) -> anyhow::Result<(u16, Serve<tokio::net::TcpListener, Router, Router>)> {
        println!("Building Application");
        let connection_options = SqliteConnectOptions::new()
            .filename(configuration.database.filename)
            .in_memory(configuration.database.enable_in_memory)
            .create_if_missing(true);
        let connection_pool = SqlitePool::connect_with(connection_options).await?;

        println!("Running migrations");
        sqlx::migrate!("./migrations").run(&connection_pool).await?;

        let addr = SocketAddr::from(([0, 0, 0, 0], configuration.application.port));
        let (tx, _) = broadcast::channel(16);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let addr = listener.local_addr()?;
        let port = addr.port();

        let mut state = Application {
            rooms: HashMap::new(),
            room_repo: RoomRepository::new(connection_pool.into()),
        };

        state
            .rooms
            .insert("testing".into(), Arc::new(Room::new(tx).into()));

        let app = Router::new()
            .merge(room_connection::get_room_connection_routes())
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

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
