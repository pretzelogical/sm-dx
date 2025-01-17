use dioxus::prelude::*;

#[component]
pub fn PostsScroll() -> Element {
    rsx! {
        div {
          id: "postsScroll",
          class: "px-4",
          ul {
            for i in 1..=5 {
              li {
                p {
                  "post {i}"
                }
              }
            }
          }
        }
    }
}
