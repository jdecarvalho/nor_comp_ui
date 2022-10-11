use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew_agent::{Dispatched};

use crate::services::event_bus::{Request, EventBus};
use crate::services::events::OutboundMessage;

pub struct WebsocketService {
    pub tx: Sender<OutboundMessage>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://localhost:8080").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<OutboundMessage>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(msg) = in_rx.next().await {
                let msg = serde_json::to_string(&msg).unwrap();
                log::debug!("Sending event to server: {}", msg);
                write.send(Message::Text(msg)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("Received text from websocket: {}", data);
                        event_bus.send(Request::EventBusMsg(data));
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log::debug!("Received bytes from websocket: {}", val);
                            event_bus.send(Request::EventBusMsg(val.into()));
                        }
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e)
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}

// TODO the context objecvt requires this trait, but i'm not sure this implementation even makes sense, or if it matters if it doesnt
impl PartialEq for WebsocketService {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
