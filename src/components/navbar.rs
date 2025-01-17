use dioxus::prelude::*;


#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "flex gap-x-5 px-4 my-2",
            div {
                p {
                    class: "text-2xl",
                    "PretzelNet"
                }
            }
            NavbarLinkCollection {}
        }
    }
}


#[component]
fn NavbarLinkCollection() -> Element {
    rsx! {
        ul {
            class: "flex ml-auto text-xl",
            li {
                class: "border-4 border-r-0 rounded-l-2xl pb-1 px-2 border-slate-950 transition-all hover:bg-slate-400 hover:text-slate-800",
                button {
                    "Posts"
                }
            }
            li {
                class: "border-4 rounded-r-2xl pb-1 px-2 border-slate-950 transition-all hover:bg-slate-400 hover:text-slate-800",
                button {
                    "User"
                }
            }
        }
    }
}
