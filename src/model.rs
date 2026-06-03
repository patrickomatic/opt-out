use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Optional personal details used to generate broker searches and templates.
#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Profile {
    /// Given name used in broker name searches.
    pub(crate) first_name: String,
    /// Family name used in broker name searches.
    pub(crate) last_name: String,
    /// City used for location-scoped searches.
    pub(crate) city: String,
    /// State or region used for location-scoped searches.
    pub(crate) state: String,
    /// Phone number used for reverse lookup and phone-indexed brokers.
    pub(crate) phone: String,
    /// Street address used for address-indexed broker discovery.
    pub(crate) address: String,
    /// Email address the user may provide to broker opt-out forms.
    pub(crate) email: String,
    /// Free-form local notes, commonly profile URLs and confirmation dates.
    pub(crate) notes: String,
}

/// Per-broker discovery status and the last time that status was changed.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct DiscoveryRecord {
    /// Current broker status, such as `unchecked`, `found`, or `removed`.
    pub(crate) status: String,
    /// ISO timestamp captured when the status was last updated.
    pub(crate) last_checked: String,
}

impl Default for DiscoveryRecord {
    /// Creates an unchecked discovery record with no timestamp.
    fn default() -> Self {
        Self {
            status: "unchecked".to_string(),
            last_checked: String::new(),
        }
    }
}

/// Complete persisted application state.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct AppState {
    /// Whether the setup screen has been completed or skipped.
    #[serde(default)]
    pub(crate) onboarding_complete: bool,
    /// Current top-level view: setup, workflow, or discovery.
    pub(crate) active_view: String,
    /// Broker id selected in the workflow area.
    pub(crate) active_site: String,
    /// User profile fields used by local helpers.
    pub(crate) profile: Profile,
    /// Completed removal checklist step indexes keyed by broker id.
    pub(crate) progress: BTreeMap<String, BTreeSet<usize>>,
    /// Discovery records keyed by broker id.
    pub(crate) discovery: BTreeMap<String, DiscoveryRecord>,
}

impl Default for AppState {
    /// Creates the first-run state shown when no saved state exists.
    fn default() -> Self {
        Self {
            onboarding_complete: false,
            active_view: "setup".to_string(),
            active_site: "fastbackgroundcheck".to_string(),
            profile: Profile::default(),
            progress: BTreeMap::new(),
            discovery: BTreeMap::new(),
        }
    }
}

/// A single human-readable removal step for a broker workflow.
#[derive(Clone, Copy)]
pub(crate) struct Step {
    /// Short step title displayed beside the checkbox.
    pub(crate) title: &'static str,
    /// Supporting instruction text for the step.
    pub(crate) body: &'static str,
}

/// Static broker metadata and workflow configuration.
#[derive(Clone, Copy)]
pub(crate) struct Site {
    /// Stable id used as the state-map key.
    pub(crate) id: &'static str,
    /// Display name shown in navigation and lists.
    pub(crate) name: &'static str,
    /// Broker domain used for generated site-scoped searches.
    pub(crate) domain: &'static str,
    /// Grouping label used by the discovery filter.
    pub(crate) category: &'static str,
    /// Human-readable difficulty label.
    pub(crate) difficulty: &'static str,
    /// Important form or workflow requirements shown as chips.
    pub(crate) signals: &'static [&'static str],
    /// Brief broker description.
    pub(crate) summary: &'static str,
    /// Broker-specific warning shown before removal steps.
    pub(crate) caution: &'static str,
    /// Current known opt-out or suppression URL.
    pub(crate) opt_out_url: &'static str,
    /// Strategy for generating the broker search link.
    pub(crate) search_kind: SearchKind,
    /// Checklist steps shown when a matching listing is found.
    pub(crate) steps: &'static [Step],
}

/// Search-link generation strategy for a broker.
#[derive(Clone, Copy)]
pub(crate) enum SearchKind {
    /// Append an encoded name/location query to a broker URL prefix.
    Direct(&'static str),
    /// Generate a Google `site:` search using name and location fields.
    GoogleSite,
    /// Generate a Google `site:` search using phone when present, otherwise name.
    GooglePhoneOrName,
    /// Generate a Google `site:` search using name and street address.
    GoogleNameAddress,
    /// Append an encoded phone number to a broker URL prefix.
    PhonePath(&'static str),
}
