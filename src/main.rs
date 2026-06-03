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
use leptos::prelude::*;

/// Installs panic logging and mounts the Leptos app into the document body.
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
