macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

pub mod state;
pub mod websocket;

use futures::{executor::block_on, SinkExt};
use protocol::ClientMessage;
use state::AppState;
use sycamore::prelude::*;
use web_sys::Event;

#[component(inline_props)]
fn Slider<G: Html>(cx: Scope, slider_id: usize, slider: RcSignal<u8>) -> View<G> {
    let slider = create_ref(cx, slider);
    let stringy = create_signal(cx, String::new());
    // updating the string
    create_effect(cx, || {
        let value = slider.get();
        let value = value.to_string();
        stringy.set(value);
    });
    let state = use_context::<AppState>(cx);
    // updating the number
    create_effect(cx, move || {
        let value = str::parse::<u8>(&stringy.get()).unwrap_or_default();
        slider.set(value);
        let mut sender = state.sender.clone();
        block_on(async move {
            sender
                .send(serde_json::to_string(&ClientMessage::Update(slider_id, value)).unwrap())
                .await
                .unwrap();
        });
    });
    view! { cx,
        div(class="inner slidecontainer") {
            input(
                type="range",
                min="0",
                max="255",
                bind:value=stringy,
                class="slider",
            )
        }
    }
}

#[component]
fn App<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let state = use_context::<AppState>(cx);
    let sliders = create_memo(cx, || {
        state
            .sliders
            .get()
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<_>>()
    });
    let handle_add = move |_: Event| {
        block_on(async {
            let mut sender = use_context::<AppState>(cx).sender.clone();
            sender
                .send(serde_json::to_string(&ClientMessage::Add).unwrap())
                .await
                .unwrap()
        });
    };
    let handle_remove = move |_: Event| {
        block_on(async {
            let mut sender = use_context::<AppState>(cx).sender.clone();
            sender
                .send(serde_json::to_string(&ClientMessage::Remove).unwrap())
                .await
                .unwrap()
        });
    };
    view! { cx,
        div(class="outer") {
            div(class="buttons") {
                button(on:click=handle_add) { "+" }
                button(on:click=handle_remove) { "-" }
            }
            Indexed(
                iterable=sliders,
                view=|cx, (id, slider)| view! { cx,
                    Slider(slider_id=id, slider=slider)
                },
            )
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| {
        let state = AppState::init(cx);
        provide_context(cx, state);

        view! { cx, App }
    });
}
