use core::error;

use wasm_bindgen::JsCast;
use web_sys::window;
use dioxus::{logger::tracing, prelude::*};


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

#[derive(Debug, Clone)]
pub struct AuthServiceState {
    pub token: Option<String>,
    pub user: Option<String>,
}

// Sits above components that require authentication and checks if the user is
// authenticated, if not try to refresh, if that fails redirect to auth
#[component]
pub fn AuthService(children: Element) -> Element {
    use_context_provider(||
        AuthServiceState {
            token: None,
            user: None
        });
    use_effect(|| {
        if let Some(token) = get_token() {
            tracing::info!("Got token: {token}");
        } else {
            tracing::info!("No token");
        }
    });
    children
}