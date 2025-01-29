use dioxus::prelude::*;
use layouts::{UnauthenticatedLayout, AppLayout};
use views::{Posts, AuthView};

mod layouts;
mod components;
mod views;
mod services;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(UnauthenticatedLayout)]
    #[route("/")]
    AuthView {},

    #[layout(AppLayout)]
    #[route("/posts")]
    Posts {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
