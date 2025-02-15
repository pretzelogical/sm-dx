use dioxus::prelude::*;
use crate::Route;
use crate::components::Navbar;
use crate::services::auth::AuthService;



#[component]
pub fn AppLayout() -> Element {
    rsx! {
        AuthService {
            Navbar {}
            div {
                class: "px-4",
                Outlet::<Route> {}
            }
        }
    }
}