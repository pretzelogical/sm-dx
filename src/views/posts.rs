use dioxus::prelude::*;
use crate::components::PostsScroll;

#[component]
pub fn Posts() -> Element {
    rsx! {
        div {
          id: "posts",
          PostsScroll {}
        }
    }
}
