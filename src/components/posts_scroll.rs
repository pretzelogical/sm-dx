use dioxus::prelude::*;

#[component]
pub fn PostsScroll() -> Element {
    rsx! {
        ul {
            id: "postsScroll",
            class: "border border-red-600 flex flex-col py-2 gap-y-2 justify-center items-center overflow-y-scroll",
            for i in 1..=5 {
              Post {
                title: format!("Post {i}"),
                content: format!("Content {i}"),
                picture: format!("https://placehold.co/800x600?text=post_{i}"),
                date: format!("Date {i}")
            }
        }
        }
    }
}

#[component]
fn Post(title: String, content: String, picture: String, date: String) -> Element {
    rsx! {
        li {
            class: "flex border border-yellow-300 dark:bg-slate-700 rounded-lg",
            div {
                class: "flex flex-col",
                div {
                    class: "flex justify-between pl-2",class: "w-96",
                    p {
                        "{title}"
                    }
                    p {
                        "{date}"
                    }
                }
                div {
                    width: "400px",
                    img {
                        src: "{picture}",
                        class: "object-cover"
                    }
                }
            }
            div {
                class: "flex flex-col justify-evenly",
                p {
                    "{content}"
                }
                div {
                    button {
                        class: "btn-confirm",
                        "Like"
                    }
                    button {
                        class: "btn-neutral",
                        "Comment"
                    }
                }
            }
        }
    }
}