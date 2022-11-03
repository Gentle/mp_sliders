use lunatic::{abstract_process, process::ProcessRef};
use protocol::ServerMessage;

use crate::{
    sliders::{Sliders, SlidersHandler},
    websocket::{WebSocketClient, WebSocketClientHandler},
};

pub struct Clients {
    clients: Vec<ProcessRef<WebSocketClient>>,
}

#[abstract_process(visibility = pub)]
impl Clients {
    #[init]
    fn init(_: ProcessRef<Self>, _: ()) -> Self {
        Clients {
            clients: Vec::new(),
        }
    }

    #[handle_message]
    fn add(&mut self, client: ProcessRef<WebSocketClient>) {
        let sliders = ProcessRef::<Sliders>::lookup("sliders").unwrap();
        let slider_state = sliders.state().drain(..).enumerate().collect::<Vec<_>>();
        self.clients.push(client.clone());
        let message = serde_json::to_string(&ServerMessage::Sliders(slider_state.len())).unwrap();
        client.send_message(message);
        let message = serde_json::to_string(&ServerMessage::Update(slider_state)).unwrap();
        client.send_message(message);
    }

    #[handle_message]
    fn remove(&mut self, client: ProcessRef<WebSocketClient>) {
        self.clients = self.clients.drain(..).filter(|x| x != &client).collect();
    }

    #[handle_request]
    fn broadcast(&self, message: ServerMessage) {
        println!("Trying Send");
        let message = serde_json::to_string(&message).unwrap();
        println!("Send message {message}");
        for client in &self.clients {
            client.send_message(message.clone())
        }
    }
}
