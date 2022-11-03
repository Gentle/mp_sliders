use lunatic::{
    abstract_process,
    process::{ProcessRef, StartProcess},
    Mailbox, Process,
};
use protocol::ClientMessage;
use submillisecond::websocket::{
    Message, SplitSink, SplitStream, WebSocket, WebSocketConnection, WebSocketUpgrade,
};

use crate::{
    clients::{Clients, ClientsHandler},
    sliders::{Sliders, SlidersHandler},
};

pub struct WebSocketClient {
    writer: SplitSink,
}

#[abstract_process(visibility = pub)]
impl WebSocketClient {
    #[init]
    fn init(this: ProcessRef<Self>, ws_conn: WebSocketConnection) -> Self {
        let (writer, reader) = ws_conn.split();

        let clients = ProcessRef::<Clients>::lookup("clients").unwrap();
        clients.add(this.clone());
        fn read_handler(
            (mut reader, this): (SplitStream, ProcessRef<WebSocketClient>),
            _: Mailbox<()>,
        ) {
            let sliders = ProcessRef::<Sliders>::lookup("sliders").unwrap();
            loop {
                match reader.read_message() {
                    Ok(Message::Text(msg)) => {
                        println!("Websocket {msg:?}");
                        if let Ok(msg) = serde_json::from_str::<ClientMessage>(&msg) {
                            match msg {
                                ClientMessage::Add => {
                                    sliders.add();
                                }
                                ClientMessage::Remove => {
                                    sliders.remove();
                                }
                                ClientMessage::Update(id, value) => {
                                    sliders.update(id, value);
                                }
                            }
                        }
                        this.send_message(msg);
                    }
                    Ok(Message::Close(_)) => break,
                    Ok(_) => { /* Ignore other messages */ }
                    Err(err) => {
                        eprintln!("Read Message Error: {err:?}");
                        break;
                    }
                }
            }
            let clients = ProcessRef::<Clients>::lookup("clients").unwrap();
            clients.remove(this);
        }

        Process::spawn_link((reader, this), read_handler);

        WebSocketClient { writer }
    }

    #[handle_message]
    fn send_message(&mut self, message: String) {
        self.writer
            .write_message(Message::text(message))
            .unwrap_or_default();
    }
}

pub fn handler(ws: WebSocket) -> WebSocketUpgrade {
    ws.on_upgrade((), |conn, _| {
        WebSocketClient::start_link(conn, None);
    })
}
