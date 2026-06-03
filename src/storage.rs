use crate::model::AppState;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use urlencoding::encode;
use web_sys::{HtmlAnchorElement, window};

/// Browser `localStorage` key used for the serialized application state.
pub(crate) const STORAGE_KEY: &str = "optOutDeskState.v2";

/// Reads saved state from browser `localStorage`, falling back to defaults.
pub(crate) fn load_state() -> AppState {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

/// Serializes the current state into browser `localStorage`.
pub(crate) fn save_state(state: &AppState) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
        && let Ok(raw) = serde_json::to_string(state)
    {
        let _ = storage.set_item(STORAGE_KEY, &raw);
    }
}

/// Removes saved browser state and reloads the app back to first-run defaults.
pub(crate) fn clear_state() {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(STORAGE_KEY);
    }
    if let Some(win) = window() {
        let _ = win.location().reload();
    }
}

/// Downloads the current state as a JSON data URL.
pub(crate) fn export_state(state: RwSignal<AppState>) {
    let Ok(raw) = serde_json::to_string_pretty(&state.get_untracked()) else {
        return;
    };
    let Some(document) = window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(element) = document.create_element("a") else {
        return;
    };
    let Ok(anchor) = element.dyn_into::<HtmlAnchorElement>() else {
        return;
    };
    anchor.set_href(&format!(
        "data:application/json;charset=utf-8,{}",
        encode(&raw)
    ));
    anchor.set_download("opt-out-progress.json");
    anchor.click();
}

/// Copies helper text to the browser clipboard when clipboard access exists.
pub(crate) fn copy_text(text: &str) {
    if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
        let _ = clipboard.write_text(text);
    }
}

/// Stable DOM id for broker workflow cards.
pub(crate) fn broker_element_id(site_id: &str) -> String {
    format!("broker-{site_id}")
}

/// Scrolls after the reactive view update has had a browser tick to render.
pub(crate) fn scroll_to_broker(site_id: &str) {
    let element_id = broker_element_id(site_id);
    let Some(win) = window() else {
        return;
    };
    let callback = Closure::once(move || {
        if let Some(element) = window()
            .and_then(|win| win.document())
            .and_then(|document| document.get_element_by_id(&element_id))
        {
            element.scroll_into_view();
        }
    });
    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        0,
    );
    callback.forget();
}
