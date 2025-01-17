use dioxus::prelude::*;
use crate::components::PostsScroll;

#[component]
pub fn Posts() -> Element {
    rsx! {
        div {
          id: "posts",
          class: "px-4",
          PostsScroll {}
        }
    }
}
