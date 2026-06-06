use crate::catalog::SITES;
use crate::model::{AppState, DiscoveryRecord, Site};
use leptos::prelude::*;
use std::collections::BTreeSet;

/// Converts a broker's discovery and checklist state into a label and CSS class.
pub(crate) fn site_status(state: RwSignal<AppState>, site: &Site) -> (&'static str, &'static str) {
    let discovery = discovery_status(state, site.id);
    if discovery == "not-found" {
        return ("Not found", "done");
    }
    if discovery == "removed" {
        return ("Removed", "done");
    }
    if discovery == "recheck" {
        return ("Recheck", "doing");
    }
    if discovery != "found" {
        return ("Unchecked", "todo");
    }

    let complete = state.with(|s| {
        s.progress
            .get(site.id)
            .map(BTreeSet::len)
            .unwrap_or_default()
    });
    if complete == site.steps.len() {
        ("Ready to verify", "done")
    } else if complete > 0 {
        ("Removing", "doing")
    } else {
        ("Found", "found")
    }
}

/// Counts completed tasks and total active tasks across discovery and removal.
pub(crate) fn total_progress(state: RwSignal<AppState>) -> (usize, usize) {
    state.with(|s| {
        SITES.iter().fold((0, 0), |(done, total), site| {
            let discovery = s
                .discovery
                .get(site.id)
                .map_or("unchecked", |record| record.status.as_str());
            let discovery_done = usize::from(discovery != "unchecked");
            let include_removal = matches!(discovery, "found" | "recheck");
            let removal_total = if include_removal { site.steps.len() } else { 0 };
            let removal_done = if include_removal {
                s.progress
                    .get(site.id)
                    .map(BTreeSet::len)
                    .unwrap_or_default()
            } else {
                0
            };
            (
                done + discovery_done + removal_done,
                total + 1 + removal_total,
            )
        })
    })
}

/// Returns a broker's saved discovery status or `unchecked` when absent.
pub(crate) fn discovery_status(state: RwSignal<AppState>, site_id: &str) -> String {
    state.with(|s| {
        s.discovery
            .get(site_id)
            .map_or_else(|| "unchecked".to_string(), |record| record.status.clone())
    })
}

/// Updates a broker discovery status and records the update timestamp.
pub(crate) fn set_discovery_status(state: RwSignal<AppState>, site_id: &str, status: &str) {
    state.update(|s| {
        s.discovery.insert(
            site_id.to_string(),
            DiscoveryRecord {
                status: status.to_string(),
                last_checked: js_sys::Date::new_0().to_iso_string().into(),
            },
        );
        if status == "found" {
            s.progress.entry(site_id.to_string()).or_default().insert(0);
        }
        if status == "not-found" {
            s.progress.remove(site_id);
        }
    });
}
