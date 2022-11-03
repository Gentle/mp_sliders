use std::time::Duration;

use lunatic::{abstract_process, process::ProcessRef, spawn_link};
use protocol::ServerMessage;

use crate::clients::{Clients, ClientsHandler};

pub struct Sliders {
    sliders: Vec<u8>,
    snapshot: Vec<u8>,
}

#[abstract_process(visibility = pub)]
impl Sliders {
    #[init]
    fn init(this: ProcessRef<Self>, _: ()) -> Self {
        spawn_link!(|this| {
            loop {
                lunatic::sleep(Duration::from_millis(15));
                this.snap();
            }
        });
        Self {
            sliders: Vec::new(),
            snapshot: Vec::new(),
        }
    }

    #[handle_message]
    fn add(&mut self) {
        self.sliders.push(128);
        let clients = ProcessRef::<Clients>::lookup("clients").unwrap();
        clients.broadcast(ServerMessage::Sliders(self.sliders.len()))
    }

    #[handle_message]
    fn remove(&mut self) {
        self.sliders.pop();
        let clients = ProcessRef::<Clients>::lookup("clients").unwrap();
        clients.broadcast(ServerMessage::Sliders(self.sliders.len()))
    }

    #[handle_message]
    fn update(&mut self, id: usize, value: u8) {
        if let Some(slider) = self.sliders.get_mut(id) {
            *slider = value;
        }
    }

    #[handle_message]
    fn snap(&mut self) {
        let updates = self
            .sliders
            .iter()
            .zip(self.snapshot.iter())
            .enumerate()
            .filter(|(_, (current, previous))| current != previous)
            .map(|(id, (current, _))| (id, *current))
            .collect::<Vec<_>>();

        self.snapshot = self.sliders.clone();
        if !updates.is_empty() {
            println!("Updates {updates:?}");
            let clients = ProcessRef::<Clients>::lookup("clients").unwrap();
            clients.broadcast(ServerMessage::Update(updates));
        }
    }

    #[handle_request]
    fn count(&self) -> usize {
        self.sliders.len()
    }

    #[handle_request]
    fn state(&self) -> Vec<u8> {
        self.sliders.clone()
    }
}
