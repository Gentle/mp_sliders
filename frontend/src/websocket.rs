use futures::channel::mpsc::{self, UnboundedSender};

use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use protocol::ServerMessage;
use sycamore::futures::spawn_local_scoped;
use sycamore::reactive::{use_context, Scope};

use crate::state::AppState;

fn get_ws_url() -> String {
    let location = web_sys::window().unwrap().location();
    let protocol = match location.protocol().unwrap().as_str() {
        "https:" => "wss:",
        "http:" => "ws:",
        // FIXME: unknown protocol defaults to ws?
        _ => "ws:",
    };
    let host = location.host().unwrap();
    format!("{}//{}/websocket", &protocol, &host)
}

pub fn websocket(cx: Scope) -> UnboundedSender<String> {
    let ws = WebSocket::open(&get_ws_url()).unwrap();
    let (mut write, mut read) = ws.split();
    let (tx, mut rx) = mpsc::unbounded();

    spawn_local_scoped(cx, async move {
        while let Some(message) = rx.next().await {
            write.send(Message::Text(message)).await.unwrap();
        }
    });

    spawn_local_scoped(cx, async move {
        while let Some(Ok(msg)) = read.next().await {
            //console_log!("1. {:?}", &msg);
            match &msg {
                Message::Text(msg) => {
                    if let Ok(msg) = serde_json::from_str::<ServerMessage>(msg) {
                        match msg {
                            ServerMessage::Sliders(count) => {
                                let state = use_context::<AppState>(cx);
                                state.resize(count);
                            }
                            ServerMessage::Update(sliders) => {
                                let state = use_context::<AppState>(cx);
                                console_log!("Received snapshot {sliders:?}");
                                for (id, value) in sliders.into_iter() {
                                    state.update(id, value);
                                }
                            }
                        }
                    }
                }
                Message::Bytes(_) => todo!(),
            }
        }
        console_log!("WebSocket Closed");
    });
    tx
}
