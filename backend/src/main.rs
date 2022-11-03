use backend::{clients::Clients, sliders::Sliders};
use lunatic::process::StartProcess;
use submillisecond::{router, Application, static_router, response::Html};

fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

fn main() -> std::io::Result<()> {
    Clients::start_link((), Some("clients"));
    Sliders::start_link((), Some("sliders"));

    let router = router! {
        GET "/websocket" => backend::websocket::handler
        _ => static_router!(
            "./static",
            index
        )
    };

    Application::new(router)
    .serve("0.0.0.0:3000")
}
