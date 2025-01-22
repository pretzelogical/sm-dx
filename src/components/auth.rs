use dioxus::prelude::*;
use dioxus::logger::tracing;
use crate::services::auth::{log_in, register, set_token};


#[component]
pub fn Auth() -> Element {
    let mut is_logging_in = use_signal(|| true);
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let auth_text = if is_logging_in() { "Login" } else { "Register" };
    rsx! {
        div {
            class: "w-80",
            div {
                class: "text-center text-2xl mb-8",
                p { "Pretzelnet - {auth_text}" }
            }
            div {
                class: "flex flex-col justify-center",
                div {
                    class: "flex flex-col",
                    label {
                        for: "username",
                        "Username"
                    }
                    input {
                        r#type: "text",
                        id: "username",
                        class: "rounded-lg px-2 dark:bg-slate-200 dark:text-slate-800",
                        oninput: move |e| username.set(e.value()),
                    }
                }
                div {
                    class: "flex flex-col",
                    label {
                        for: "password",
                        "Password"
                    }
                    input {
                        r#type: "password",
                        id: "password",
                        class: "rounded-lg px-2 dark:bg-slate-200 dark:text-slate-800",
                        oninput: move |e| password.set(e.value()),
                    }
                }
            }
            div {
                class: "flex justify-center my-3",
                    button {
                        class: "btn-confirm",
                        onclick: move |_| {
                            spawn(async move {
                                let res = {
                                    if is_logging_in() {
                                        log_in(&username().clone(), &password().clone()).await
                                    } else {
                                        register(&username().clone(), &password().clone()).await
                                    }
                                };
                                match res {
                                    Some(res) => {
                                        set_token(res.token);
                                    }
                                    None => {
                                        tracing::error!("Failed to log in");
                                    }
                                }
                            });
                        },
                        "{auth_text}"
                    }
            }
            div {
                class: "flex flex-col justify-center items-center my-3",
                if is_logging_in() {
                    p { "Already have an account?" }
                    button {
                        class: "btn-neutral",
                        onclick: move |_| is_logging_in.set(false),
                        "Register"
                    }
                } else {
                    p { "Don't have an account?" }
                    button {
                        class: "btn-neutral",
                        onclick: move |_| is_logging_in.set(true),
                        "Login"
                    }
                }
            }
        }
    }
}