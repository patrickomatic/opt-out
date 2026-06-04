#![deny(clippy::pedantic)]

//! Client-side Opt-Out Desk application.
//!
//! The app is intentionally static and browser-hostable. All user-entered
//! profile details, broker statuses, and checklist progress live in browser
//! `localStorage` under [`storage::STORAGE_KEY`].

mod app;
mod catalog;
mod model;
mod search;
mod status;
mod storage;

use app::App;
use leptos::mount::mount_to;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Installs panic logging and mounts the Leptos app into the app root.
fn main() {
    console_error_panic_hook::set_once();

    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };

    if let Some(static_intro) = document.get_element_by_id("static-intro") {
        static_intro.remove();
    }

    if let Some(app_root) = document
        .get_element_by_id("app")
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
    {
        mount_to(app_root, App).forget();
    }
}
