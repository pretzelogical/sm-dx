use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn UnauthenticatedLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}