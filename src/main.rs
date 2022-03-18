use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let app = Router::new()
        .route("/", get(dioxusapp));

    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn dioxusapp() -> Html<String> {
    fn dioxusapp(cx: Scope) -> Element {
        cx.render(rsx!(
            head {
                link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
            }
            body {
                class: "w-full h-full bg-blue-500",
                div {
                }
            }
        ))
    }
    let mut app = VirtualDom::new(dioxusapp);
    let _ = app.rebuild();
    Html(dioxus::ssr::render_vdom(&app))
}
