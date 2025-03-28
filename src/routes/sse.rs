use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};
use futures::Stream;
use tokio::sync::broadcast;

use crate::domain::app_state::AppState;

pub fn get_sse_routes() -> Router<AppState> {
    Router::new().route("/sse", get(sse_handler))
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{:?}` connected", user_agent);

    let mut rx = state.rooms[0].broadcaster.subscribe();

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    let json = serde_json::to_string(&event).unwrap();
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
