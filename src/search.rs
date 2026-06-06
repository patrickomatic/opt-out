use crate::catalog::SITES;
use crate::model::{AppState, Profile, SearchKind, Site};
use leptos::prelude::*;
use urlencoding::encode as url_encode;

/// Returns brokers ordered with not-found items at the bottom.
pub(crate) fn ordered_sites(state: RwSignal<AppState>) -> Vec<Site> {
    let mut sites = SITES.iter().copied().enumerate().collect::<Vec<_>>();
    sites.sort_by_key(|(index, site)| {
        state.with(|s| {
            let record = s.discovery.get(site.id);
            let status = record.map_or("unchecked", |entry| entry.status.as_str());
            let bottom_rank = usize::from(status == "not-found");
            let updated_at = record.map_or("", |entry| entry.last_checked.as_str());
            (bottom_rank, updated_at.to_string(), *index)
        })
    });
    sites.into_iter().map(|(_, site)| site).collect()
}

/// Builds the best available broker search URL from site metadata and profile data.
pub(crate) fn search_url(site: &Site, profile: &Profile) -> String {
    match site.search_kind {
        SearchKind::Direct(prefix) => {
            let query = [
                profile.first_name.as_str(),
                profile.last_name.as_str(),
                profile.city.as_str(),
                profile.state.as_str(),
            ]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
            if query.is_empty() {
                format!("https://www.{}/", site.domain)
            } else {
                format!("{prefix}{}", encode(&query))
            }
        }
        SearchKind::GoogleSite => google_search(&format!(
            "{} {} site:{}",
            full_name(profile),
            location(profile),
            site.domain
        )),
        SearchKind::GooglePhoneOrName => {
            let needle = if profile.phone.is_empty() {
                full_name(profile)
            } else {
                profile.phone.clone()
            };
            google_search(&format!("{needle} site:{}", site.domain))
        }
        SearchKind::GoogleNameAddress => google_search(&format!(
            "{} {} site:{}",
            full_name(profile),
            profile.address,
            site.domain
        )),
        SearchKind::PhonePath(prefix) => {
            if profile.phone.is_empty() {
                format!("https://www.{}/", site.domain)
            } else {
                format!("{prefix}{}", encode(&profile.phone))
            }
        }
    }
}

/// Creates broad search-engine discovery queries from the local profile.
pub(crate) fn discovery_queries(profile: &Profile) -> Vec<(String, String)> {
    let mut queries = Vec::new();
    let name = full_name(profile);
    let place = location(profile);
    if !profile.phone.is_empty() {
        queries.push(("Exact phone".to_string(), format!("\"{}\"", profile.phone)));
    }
    if !profile.address.is_empty() {
        queries.push((
            "Exact address".to_string(),
            format!("\"{}\"", profile.address),
        ));
    }
    if !name.is_empty() && !place.is_empty() {
        queries.push((
            "Name and city".to_string(),
            format!("\"{name}\" \"{place}\""),
        ));
    }
    if !name.is_empty() {
        queries.push((
            "People-search language".to_string(),
            format!("\"{name}\" \"possible relatives\""),
        ));
        queries.push((
            "Background-check language".to_string(),
            format!("\"{name}\" \"background check\""),
        ));
    }
    if !profile.phone.is_empty() {
        queries.push((
            "Reverse lookup".to_string(),
            format!("\"{}\" \"reverse phone\"", profile.phone),
        ));
    }
    if !profile.address.is_empty() {
        queries.push((
            "Public records".to_string(),
            format!("\"{}\" \"public records\"", profile.address),
        ));
    }
    if !name.is_empty() && !place.is_empty() {
        queries.push((
            "Broker clones".to_string(),
            format!("\"{name}\" \"{place}\" \"age\""),
        ));
    }
    if queries.is_empty() {
        queries.push((
            "Phone in quotes".to_string(),
            "\"555-123-4567\"".to_string(),
        ));
        queries.push((
            "Name and city".to_string(),
            "\"Jane Doe\" \"Rockville, MD\"".to_string(),
        ));
        queries.push((
            "People-search language".to_string(),
            "\"Jane Doe\" \"possible relatives\"".to_string(),
        ));
    }
    queries
}

/// Returns broker candidates matching the selected category and status filters.
pub(crate) fn filtered_sites(category: &str, status: &str, state: RwSignal<AppState>) -> Vec<Site> {
    ordered_sites(state)
        .into_iter()
        .filter(|site| category == "all" || category == site.category)
        .filter(|site| {
            if status == "all" {
                return true;
            }
            let current = state.with(|s| {
                s.discovery
                    .get(site.id)
                    .map_or_else(|| "unchecked".to_string(), |d| d.status.clone())
            });
            current == status
        })
        .collect()
}

/// Returns sorted, unique broker categories for the discovery filter.
pub(crate) fn categories() -> Vec<&'static str> {
    let mut categories = SITES.iter().map(|site| site.category).collect::<Vec<_>>();
    categories.sort_unstable();
    categories.dedup();
    categories
}

/// Generates a reusable broker support request from profile fields.
pub(crate) fn support_template(profile: &Profile) -> String {
    format!(
        "Hello,\n\nI am requesting removal or suppression of my personal information from your site. Please remove records matching:\n\nName: {}\nLocation: {}\nPhone: {}\nAddress: {}\nProfile URL(s): {}\n\nI am the subject of this request. Please confirm once the records have been removed or tell me what additional information is required to identify the listing.\n\nThank you.",
        empty_placeholder(&full_name(profile), "[your full name]"),
        empty_placeholder(&location(profile), "[city, state]"),
        empty_placeholder(&profile.phone, "[phone number, if relevant]"),
        empty_placeholder(&profile.address, "[street address, if relevant]"),
        empty_placeholder(&profile.notes, "[paste profile URLs here]"),
    )
}

/// Joins first and last name while omitting blank parts.
fn full_name(profile: &Profile) -> String {
    [profile.first_name.as_str(), profile.last_name.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Joins city and state while omitting blank parts.
fn location(profile: &Profile) -> String {
    [profile.city.as_str(), profile.state.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns a placeholder when a template field has no user-provided value.
fn empty_placeholder(value: &str, placeholder: &str) -> String {
    if value.is_empty() {
        placeholder.to_string()
    } else {
        value.to_string()
    }
}

/// Builds a Google search URL for an arbitrary query.
pub(crate) fn google_search(query: &str) -> String {
    format!("https://www.google.com/search?q={}", encode(query))
}

/// Converts completed and total counts into a progress-bar percentage.
pub(crate) fn progress_percent(done: usize, total: usize) -> i32 {
    done.saturating_mul(100)
        .checked_div(total)
        .and_then(|percent| i32::try_from(percent).ok())
        .unwrap_or_default()
}

/// URL-encodes a string for query strings and path fragments.
fn encode(value: &str) -> String {
    url_encode(value).into_owned()
}
