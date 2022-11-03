use futures::channel::mpsc::UnboundedSender;

use sycamore::reactive::{create_rc_signal, RcSignal, Scope};

use crate::websocket::websocket;

pub struct AppState {
    pub sliders: RcSignal<Vec<RcSignal<u8>>>,
    pub sender: UnboundedSender<String>,
}
impl AppState {
    pub fn init(cx: Scope) -> Self {
        let sender = websocket(cx);
        let this = Self {
            sliders: create_rc_signal(Vec::new()),
            sender,
        };
        this
    }
    pub fn resize(&self, new_len: usize) {
        self.sliders
            .modify()
            .resize_with(new_len, || create_rc_signal(128));
    }
    pub fn update(&self, id: usize, value: u8) {
        if let Some(x) = self.sliders.get().get(id) {
            if *x.get() != value {
                x.set(value);
            }
        }
    }
}
