use std::{convert::Infallible, time::Duration};

use axum::{
    extract::{Path, State},
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use futures::Stream;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{application::Application, domain::poker_event::PokerEvent};

pub fn get_room_connection_routes() -> Router<Application> {
    Router::new().route("/room-connection/{id}", get(room_connection_handler))
}

async fn room_connection_handler(
    State(mut state): State<Application>,
    Path(id): Path<String>,
    cookies: CookieJar,
) -> (
    CookieJar,
    Sse<impl Stream<Item = Result<Event, Infallible>>>,
) {
    let cookie_name = "device_id";
    let (cookies, device_id) = if let Some(cookie) = cookies.clone().get(cookie_name) {
        (cookies, cookie.value().to_string())
    } else {
        let new_id = Uuid::new_v4().to_string();

        let updated_cookies = cookies.add(Cookie::new(cookie_name, new_id.clone()));

        (updated_cookies, new_id)
    };

    println!("{device_id} device connected to room {id}");
    let room_arc = state.rooms.get_mut(&id).unwrap();
    let mut room = room_arc.lock().await;
    room.add_player(&device_id);

    let join_event = PokerEvent::PlayerJoined {
        id: device_id.clone().into(),
    };
    let _ = room.broadcaster.send(join_event);

    let room_arc_stream = room_arc.clone();
    let mut rx = room.broadcaster.subscribe();
    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    let json = serde_json::to_string(&event).unwrap();
                    yield Ok(Event::default().data(json));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => {
                    let mut room = room_arc_stream.lock().await;
                    room.remove_player(&device_id);

                    let leave_event = PokerEvent::PlayerLeft { id: device_id.into() };
                    let _ = room.broadcaster.send(leave_event);

                    break;
                },
            }
        }
    };

    (
        cookies,
        Sse::new(stream).keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keep-alive"),
        ),
    )
}
