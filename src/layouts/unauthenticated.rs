use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn UnauthenticatedLayout() -> Element {
    rsx! {
        div {
            class: "px-4",
            Outlet::<Route> {}
        }
    }
}