use reqwest::Method;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::window;
use dioxus::{logger::tracing, prelude::*};
use crate::services::client::unauth_client;


// #[cfg(target_arch = "wasm32")]
pub fn storage() -> Option<web_sys::Storage> {
    window().unwrap().local_storage().ok()?
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn storage() -> Option<web_sys::Storage> {
//     todo!()
// }

// #[cfg(target_arch = "wasm32")]
pub fn set_token(token: String) -> Option<String> {
    let storage = storage().unwrap();
    if let Err(error) = storage.set_item("token", &token) {
        if error.is_instance_of::<js_sys::Error>() {
            let error: js_sys::Error = error.dyn_into().unwrap();
            let error: String = error.message().into();
            tracing::error!("Failed to set token: {error}");
        }
        None
    } else {
        Some(token)
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn set_token() -> Option<String> {
//     todo!()
// }

// #[cfg(target_arch = "wasm32")]
pub fn get_token() -> Option<String> {
    let storage = storage().unwrap();
    storage.get_item("token").ok()?
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn get_token() -> Option<String> {
//     todo!()
// }


#[derive(Debug, Deserialize)]
pub struct VerifySessionResponseUserInfo {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct VerifySessionResponse {
    pub token: String,
    pub user: VerifySessionResponseUserInfo
}

pub async fn refresh_session(token: &String) -> Option<VerifySessionResponse> {
    let client = unauth_client();
    let response = client
        .request(Method::GET, "http://127.0.0.1:8080/auth")
        .bearer_auth(token.clone())
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let session_data = response
                    .json::<VerifySessionResponse>()
                    .await;
                session_data.ok()
            } else {
                None
            }
        }
        Err(error) => {
            tracing::error!("Failed to refresh session: {error}");
            tracing::error!("Status code: {:#?}", error.status());
            None
        }
    }
}

pub async fn log_in(username: &String, password: &String) -> Option<VerifySessionResponse> {
    let client = unauth_client();
    let response = client
        .request(Method::POST, "http://127.0.0.1:8080/auth/login")
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let session_data = response
                    .json::<VerifySessionResponse>()
                    .await;
                tracing::info!("Session data: {session_data:?}");
                session_data.ok()
            } else {
                None
            }
        },
        Err(error) => {
            tracing::error!("Failed to log in: {error}");
            None
        }
    }
}

pub async fn register(username: &String, password: &String) -> Option<VerifySessionResponse> {
    let client = unauth_client();
    let response = client
        .request(Method::POST, "http://127.0.0.1:8080/auth/register")
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let session_data = response
                    .json::<VerifySessionResponse>()
                    .await;
                tracing::info!("Session data: {session_data:?}");
                session_data.ok()
            } else {
                None
            }
        },
        Err(error) => {
            tracing::error!("Failed to register: {error}");
            None
        }
    }
}


#[derive(Debug, Clone)]
pub struct AuthServiceState {
    pub token: Signal<Option<String>>,
    pub uid: Signal<Option<i64>>,
    pub name: Signal<Option<String>>,
}

// Sits above components that require authentication and checks if the user is
// authenticated, if not try to refresh, if that fails redirect to auth
#[component]
pub fn AuthService(children: Element) -> Element {
    let mut auth_state = use_context_provider(||
        AuthServiceState {
            token: Signal::new(None),
            uid: Signal::new(None),
            name: Signal::new(None),
        });
    use_effect(move || {
        match get_token() {
            Some(token) => {
                spawn(async move {
                    let session_data = refresh_session(&token).await;
                    match session_data {
                        Some(session_data) => {
                            auth_state.token.set(Some(session_data.token));
                            auth_state.uid.set(Some(session_data.user.id));
                            auth_state.name.set(Some(session_data.user.name));
                        }
                        None => {
                            tracing::error!("Failed to refresh session");
                        }
                    }
                });
            }
            None => {
                todo!("Redirect to auth");
            }
        }
    });
    children
}

