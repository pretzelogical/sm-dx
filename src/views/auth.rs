use dioxus::prelude::*;
use crate::components::Auth;

#[component]
pub fn AuthView() -> Element {
    rsx! {
      div {
        class: "flex items-center justify-center h-screen",
        id: "userAuth",
        Auth {}
      }
    }
}

