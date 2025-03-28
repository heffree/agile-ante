use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::{sse::Event, Html, IntoResponse, Response, Sse},
    routing::{get, post, Router},
    Json,
};
use axum_extra::{headers, TypedHeader};
use futures::Stream;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::{broadcast, Mutex};

#[derive(Serialize, Clone, Default)]
struct PokerEvent {
    command: String,
    value: usize,
}

#[derive(Clone)]
struct AppState {
    event: Arc<Mutex<PokerEvent>>,
    broadcaster: broadcast::Sender<PokerEvent>,
}

#[tokio::main]
async fn main() {
    let event = Arc::new(Mutex::new(PokerEvent::default()));
    let (tx, _) = broadcast::channel(16);
    let state = AppState {
        event,
        broadcaster: tx,
    };
    // Define our app routes, including a fallback option for anything not matched.
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/assets/{*file}", get(static_handler))
        .route("/sse", get(sse_handler))
        .route("/increment", post(increment_handler))
        .with_state(state)
        .fallback_service(get(not_found));

    // Start listening on the given address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn increment_handler(State(state): State<AppState>) -> Json<PokerEvent> {
    let mut event = state.event.lock().await;
    event.command = "new_count".into();
    event.value = event.value + 1;

    let _ = state.broadcaster.send(event.clone());

    Json(event.clone())
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // subscribe to updates from the broadcaster
    let mut rx = state.broadcaster.subscribe();

    // create a stream that yields the updated table as JSON each time a new update is sent
    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(table) => {
                    let json = serde_json::to_string(&table).unwrap();
                    yield Ok(Event::default().data(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

// Finally, we use a fallback route for anything that didn't match.
async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

#[derive(Embed)]
#[folder = "src/client/dist/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
