#![deny(clippy::pedantic)]

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use web_sys::{HtmlAnchorElement, window};

const STORAGE_KEY: &str = "optOutDeskState.v2";

#[derive(Clone, Default, Deserialize, Serialize)]
struct Profile {
    first_name: String,
    last_name: String,
    city: String,
    state: String,
    phone: String,
    address: String,
    email: String,
    notes: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct DiscoveryRecord {
    status: String,
    last_checked: String,
}

impl Default for DiscoveryRecord {
    fn default() -> Self {
        Self {
            status: "unchecked".to_string(),
            last_checked: String::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct AppState {
    active_view: String,
    active_site: String,
    profile: Profile,
    progress: BTreeMap<String, BTreeSet<usize>>,
    discovery: BTreeMap<String, DiscoveryRecord>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            active_view: "workflow".to_string(),
            active_site: "fastbackgroundcheck".to_string(),
            profile: Profile::default(),
            progress: BTreeMap::new(),
            discovery: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Copy)]
struct Step {
    title: &'static str,
    body: &'static str,
}

#[derive(Clone, Copy)]
struct Site {
    id: &'static str,
    name: &'static str,
    domain: &'static str,
    category: &'static str,
    difficulty: &'static str,
    signals: &'static [&'static str],
    summary: &'static str,
    caution: &'static str,
    opt_out_url: &'static str,
    search_kind: SearchKind,
    steps: &'static [Step],
}

#[derive(Clone, Copy)]
enum SearchKind {
    Direct(&'static str),
    GoogleSite,
    GooglePhoneOrName,
    GoogleNameAddress,
    PhonePath(&'static str),
}

const GENERIC_STEPS: &[Step] = &[
    Step {
        title: "Search for a matching listing",
        body: "Use the generated search link and also search by phone, address, and name variants.",
    },
    Step {
        title: "Save evidence locally",
        body: "Paste profile URLs, matching details, and the date found into notes.",
    },
    Step {
        title: "Open the opt-out page",
        body: "Follow the broker's current removal form and complete any required verification.",
    },
    Step {
        title: "Confirm any email request",
        body: "Click confirmation links promptly and record the confirmation date.",
    },
    Step {
        title: "Recheck later",
        body: "Search again after the broker's stated processing window and mark the result.",
    },
];

const FASTBACKGROUND_STEPS: &[Step] = &[
    Step {
        title: "Find your listing",
        body: "Search by name, phone, and address. Save every matching profile URL in notes.",
    },
    Step {
        title: "Open the opt-out form",
        body: "Enter your name and opt-out email, then complete the captcha.",
    },
    Step {
        title: "Confirm by email",
        body: "Click their email link within 24 hours.",
    },
    Step {
        title: "Submit matching details",
        body: "Use the same name, city, and address shown on the listing so they can identify the record.",
    },
    Step {
        title: "Verify removal",
        body: "Check the listing again after 3 days and record the date.",
    },
];

const SPOKEO_STEPS: &[Step] = &[
    Step {
        title: "Find the exact profile",
        body: "Search Spokeo and open the result that matches your phone number or address.",
    },
    Step {
        title: "Copy the profile URL",
        body: "Paste the profile URL into notes before you start the removal form.",
    },
    Step {
        title: "Open Spokeo opt-out",
        body: "Paste the profile URL and your email address, then complete the captcha.",
    },
    Step {
        title: "Confirm by email",
        body: "Click the confirmation link Spokeo sends.",
    },
    Step {
        title: "Check for no results",
        body: "Revisit the profile URL and search results after a few days.",
    },
];

const WHITEPAGES_STEPS: &[Step] = &[
    Step {
        title: "Search your listing",
        body: "Start from the suppression request page or search Whitepages by name and location.",
    },
    Step {
        title: "Select the matching record",
        body: "Use the listing that shows your current or old address, phone number, or relatives.",
    },
    Step {
        title: "Request removal",
        body: "Choose the removal or suppression option for that record.",
    },
    Step {
        title: "Complete verification",
        body: "Be ready for phone verification if their flow asks for it.",
    },
    Step {
        title: "Recheck free and premium results",
        body: "Search again after a few days and save what changed.",
    },
];

const SITES: &[Site] = &[
    Site {
        id: "fastbackgroundcheck",
        name: "FastBackgroundCheck",
        domain: "fastbackgroundcheck.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["captcha", "email confirmation"],
        summary: "Email-gated opt-out form. They send a link, then ask for matching record information.",
        caution: "Their own page says the email link expires after 24 hours and removal can take up to 3 days after submission.",
        opt_out_url: "https://www.fastbackgroundcheck.com/optout",
        search_kind: SearchKind::Direct("https://www.fastbackgroundcheck.com/name/"),
        steps: FASTBACKGROUND_STEPS,
    },
    Site {
        id: "spokeo",
        name: "Spokeo",
        domain: "spokeo.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["profile URL", "captcha", "email confirmation"],
        summary: "Requires a specific Spokeo profile URL, an email address, captcha, and email confirmation.",
        caution: "Spokeo listings may return when they refresh source data, so schedule a recurring check.",
        opt_out_url: "https://www.spokeo.com/optout",
        search_kind: SearchKind::GoogleSite,
        steps: SPOKEO_STEPS,
    },
    Site {
        id: "whitepages",
        name: "Whitepages",
        domain: "whitepages.com",
        category: "People search",
        difficulty: "Hard",
        signals: &["phone verification", "premium listings"],
        summary: "Suppression flow generally starts from their opt-out page and can require phone verification.",
        caution: "Whitepages and Whitepages Premium can behave like separate surfaces. Check both search results and premium-style listings.",
        opt_out_url: "https://www.whitepages.com/suppression-requests",
        search_kind: SearchKind::Direct("https://www.whitepages.com/name/"),
        steps: WHITEPAGES_STEPS,
    },
    Site {
        id: "beenverified",
        name: "BeenVerified",
        domain: "beenverified.com",
        category: "Background check",
        difficulty: "Medium",
        signals: &["email confirmation", "profile URL"],
        summary: "People-search and background-check site that commonly requires finding the exact listing first.",
        caution: "Check related brands and duplicate records after removal.",
        opt_out_url: "https://www.beenverified.com/app/optout/search",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "peoplefinders",
        name: "PeopleFinders",
        domain: "peoplefinders.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["profile URL", "email confirmation"],
        summary: "People-search site with name, phone, address, and relative matching.",
        caution: "Look for multiple records under old addresses or name variants.",
        opt_out_url: "https://www.peoplefinders.com/opt-out",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "truthfinder",
        name: "TruthFinder",
        domain: "truthfinder.com",
        category: "Background check",
        difficulty: "Medium",
        signals: &["suppression center", "email confirmation"],
        summary: "Background-check brand often connected to the same profile ecosystem as other search sites.",
        caution: "Search for old cities and aliases, not just current records.",
        opt_out_url: "https://www.truthfinder.com/opt-out/",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "intelius",
        name: "Intelius",
        domain: "intelius.com",
        category: "Background check",
        difficulty: "Medium",
        signals: &["suppression center", "email confirmation"],
        summary: "Background-check and people-search site with overlapping data sources.",
        caution: "If one profile is removed, search again for near-duplicate listings.",
        opt_out_url: "https://www.intelius.com/opt-out/",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "instantcheckmate",
        name: "Instant Checkmate",
        domain: "instantcheckmate.com",
        category: "Background check",
        difficulty: "Medium",
        signals: &["suppression center", "email confirmation"],
        summary: "Background-check site that can expose age, relatives, locations, and contact details.",
        caution: "Treat removal as recurring because background-check sites refresh data frequently.",
        opt_out_url: "https://www.instantcheckmate.com/opt-out/",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "ussearch",
        name: "US Search",
        domain: "ussearch.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["suppression center"],
        summary: "Older people-search brand that may share removal infrastructure with related companies.",
        caution: "Search for exact address snippets when name search is noisy.",
        opt_out_url: "https://www.ussearch.com/opt-out/",
        search_kind: SearchKind::GoogleNameAddress,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "radaris",
        name: "Radaris",
        domain: "radaris.com",
        category: "People search",
        difficulty: "Hard",
        signals: &["account or verification", "profile claiming"],
        summary: "People-search site with profile pages, relatives, addresses, and possible aliases.",
        caution: "Removal flows can be more involved than simple email-confirmed opt-outs.",
        opt_out_url: "https://radaris.com/page/how-to-remove",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "thatsthem",
        name: "ThatsThem",
        domain: "thatsthem.com",
        category: "Reverse lookup",
        difficulty: "Easy",
        signals: &["simple form"],
        summary: "Reverse phone, email, and address lookup site.",
        caution: "Phone and address searches may reveal different records.",
        opt_out_url: "https://thatsthem.com/optout",
        search_kind: SearchKind::PhonePath("https://thatsthem.com/phone/"),
        steps: GENERIC_STEPS,
    },
    Site {
        id: "cyberbackgroundchecks",
        name: "CyberBackgroundChecks",
        domain: "cyberbackgroundchecks.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["captcha", "email confirmation"],
        summary: "People-search site with detailed address and phone profiles.",
        caution: "Search exact phone and exact address because name results can be broad.",
        opt_out_url: "https://www.cyberbackgroundchecks.com/removal",
        search_kind: SearchKind::GooglePhoneOrName,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "searchpeoplefree",
        name: "SearchPeopleFree",
        domain: "searchpeoplefree.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["email confirmation"],
        summary: "People-search site that can show phone numbers, relatives, and addresses.",
        caution: "Search by phone after completing name-based removals.",
        opt_out_url: "https://www.searchpeoplefree.com/opt-out",
        search_kind: SearchKind::GooglePhoneOrName,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "truepeoplesearch",
        name: "TruePeopleSearch",
        domain: "truepeoplesearch.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["email confirmation", "record URL"],
        summary: "Free people-search site with phone, address, and relatives.",
        caution: "Record URLs can differ between name and phone searches.",
        opt_out_url: "https://www.truepeoplesearch.com/removal",
        search_kind: SearchKind::GooglePhoneOrName,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "fastpeoplesearch",
        name: "FastPeopleSearch",
        domain: "fastpeoplesearch.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["email confirmation", "record URL"],
        summary: "People-search site often indexed by phone and address.",
        caution: "Use exact phone searches to catch records missed by name.",
        opt_out_url: "https://www.fastpeoplesearch.com/removal",
        search_kind: SearchKind::GooglePhoneOrName,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "familytreenow",
        name: "FamilyTreeNow",
        domain: "familytreenow.com",
        category: "Genealogy",
        difficulty: "Easy",
        signals: &["simple form"],
        summary: "Genealogy-style people-search site that can expose relatives and historical addresses.",
        caution: "Historical addresses and family links may make records easy to identify.",
        opt_out_url: "https://www.familytreenow.com/optout",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "nuwber",
        name: "Nuwber",
        domain: "nuwber.com",
        category: "People search",
        difficulty: "Medium",
        signals: &["record URL", "email confirmation"],
        summary: "People-search site with profile URLs, contact details, and address history.",
        caution: "Check for duplicate records with middle initials or old locations.",
        opt_out_url: "https://nuwber.com/removal/link",
        search_kind: SearchKind::GoogleSite,
        steps: GENERIC_STEPS,
    },
    Site {
        id: "numlookup",
        name: "NumLookup",
        domain: "numlookup.com",
        category: "Reverse lookup",
        difficulty: "Easy",
        signals: &["phone lookup"],
        summary: "Reverse phone lookup site.",
        caution: "Phone-only sites may rehydrate from telecom or marketing data sources.",
        opt_out_url: "https://www.numlookup.com/optout",
        search_kind: SearchKind::PhonePath("https://www.numlookup.com/"),
        steps: GENERIC_STEPS,
    },
];

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let state = RwSignal::new(load_state());

    Effect::new(move |_| {
        state.with(save_state);
    });

    view! {
        <div class="shell">
            <Sidebar state=state />
            <main>
                <Hero state=state />
                {move || {
                    let is_discovery = state.with(|s| s.active_view == "discovery");
                    if is_discovery {
                        view! { <DiscoveryView state=state /> }.into_any()
                    } else {
                        view! { <WorkflowView state=state /> }.into_any()
                    }
                }}
                <p class="sources">
                    "Workflow notes and seed broker candidates were assembled from public opt-out directories and broker pages on May 26, 2026. Always follow the form shown by the broker if it differs."
                </p>
            </main>
        </div>
    }
}

#[component]
fn Sidebar(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <aside>
            <div class="brand">
                <div class="mark" aria-hidden="true">"OD"</div>
                <div>
                    <h1>"Opt-Out Desk"</h1>
                    <p class="subtle">"A local checklist for data broker removals."</p>
                </div>
            </div>
            <div class="privacy-note">
                "This page runs entirely in your browser. Profile details are stored only in this browser's local storage unless you clear them."
            </div>
            <div class="sidebar-section">
                <div class="side-label">"Views"</div>
                <div class="nav-list">
                    <button
                        class="nav-button"
                        type="button"
                        aria-current=move || state.with(|s| s.active_view == "workflow").to_string()
                        on:click=move |_| state.update(|s| s.active_view = "workflow".to_string())
                    >
                        "Workflows"
                    </button>
                    <button
                        class="nav-button"
                        type="button"
                        aria-current=move || state.with(|s| s.active_view == "discovery").to_string()
                        on:click=move |_| state.update(|s| s.active_view = "discovery".to_string())
                    >
                        "Discovery"
                    </button>
                </div>
            </div>
            <div class="sidebar-section">
                <div class="side-label">"Brokers"</div>
                <div class="site-list">
                    {move || SITES.iter().map(|site| {
                        let site_id = site.id.to_string();
                        let (label, class_name) = site_status(state, site);
                        view! {
                            <button
                                class="site-button"
                                type="button"
                                aria-current=move || state.with(|s| s.active_site == site_id).to_string()
                                on:click=move |_| state.update(|s| {
                                    s.active_site = site.id.to_string();
                                    s.active_view = "workflow".to_string();
                                })
                            >
                                <span>{site.name}</span>
                                <span class=format!("pill {class_name}")>{label}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            <div class="sidebar-section">
                <div class="side-label">"Tools"</div>
                <div class="site-list">
                    <button class="site-button" type="button" on:click=move |_| export_state(state)>
                        <span>"Export progress"</span><span>"JSON"</span>
                    </button>
                    <button class="site-button" type="button" on:click=move |_| clear_state()>
                        <span>"Clear local data"</span><span>"Reset"</span>
                    </button>
                </div>
            </div>
        </aside>
    }
}

#[component]
fn Hero(state: RwSignal<AppState>) -> impl IntoView {
    let progress = move || total_progress(state);
    view! {
        <section class="hero">
            <div>
                <h2>"Remove exposed phone, address, and profile listings without losing track."</h2>
                <p class="subtle">
                    "Enter only what helps you build search links and copy request text. The app does not submit forms for you because these sites rely on captchas, email links, and phone verification."
                </p>
            </div>
            <div class="toolbar" aria-live="polite">
                <div class="progress-row">
                    <strong>{move || {
                        let (done, total) = progress();
                        format!("{done} of {total} steps complete")
                    }}</strong>
                    <span class=move || {
                        let (done, total) = progress();
                        format!("pill {}", if done == total && total > 0 { "done" } else if done > 0 { "doing" } else { "todo" })
                    }>
                        {move || {
                            let (done, total) = progress();
                            if done == total && total > 0 { "All complete" } else if done > 0 { "In progress" } else { "Not started" }
                        }}
                    </span>
                </div>
                <progress max="100" value=move || {
                    let (done, total) = progress();
                    progress_percent(done, total)
                }></progress>
                <p class="hint">"Treat removal as recurring maintenance. Recheck every 3 to 6 months because listings can reappear from refreshed data sources."</p>
            </div>
        </section>
    }
}

#[component]
fn WorkflowView(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <section class="grid">
            <BrokerWorkflow state=state />
            <Workspace state=state />
        </section>
    }
}

#[component]
fn BrokerWorkflow(state: RwSignal<AppState>) -> impl IntoView {
    let active_site = move || current_site(state);
    view! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h3>{move || active_site().name}</h3>
                    <p class="subtle">{move || active_site().summary}</p>
                </div>
                <span class=move || {
                    let (_, class_name) = site_status(state, &active_site());
                    format!("pill {class_name}")
                }>
                    {move || site_status(state, &active_site()).0}
                </span>
            </div>
            <div class="panel-body">
                <div class="callout">{move || active_site().caution}</div>
                <ol class="steps">
                    {move || {
                        let site = active_site();
                        site.steps.iter().enumerate().map(|(index, step)| {
                            let checked = move || state.with(|s| {
                                s.progress.get(site.id).is_some_and(|steps| steps.contains(&index))
                            });
                            view! {
                                <li class="step">
                                    <input
                                        type="checkbox"
                                        prop:checked=checked
                                        aria-label=step.title
                                        on:change=move |event| {
                                            let is_checked = event_target_checked(&event);
                                            state.update(|s| {
                                                let steps = s.progress.entry(site.id.to_string()).or_default();
                                                if is_checked {
                                                    steps.insert(index);
                                                } else {
                                                    steps.remove(&index);
                                                }
                                            });
                                        }
                                    />
                                    <div>
                                        <strong>{step.title}</strong>
                                        <p>{step.body}</p>
                                    </div>
                                </li>
                            }
                        }).collect_view()
                    }}
                </ol>
                <div class="actions">
                    <a class="btn" href=move || active_site().opt_out_url target="_blank" rel="noopener">"Open opt-out"</a>
                    <a class="btn secondary" href=move || search_url(&active_site(), &state.get().profile) target="_blank" rel="noopener">"Search listing"</a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Workspace(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h3>"Your private workspace"</h3>
                    <p class="subtle">"Used for local links, notes, and request templates."</p>
                </div>
            </div>
            <div class="panel-body">
                <form class="form-grid">
                    <div class="split">
                        <TextField label="First name" value=move || state.get().profile.first_name on_input=move |value| state.update(|s| s.profile.first_name = value) />
                        <TextField label="Last name" value=move || state.get().profile.last_name on_input=move |value| state.update(|s| s.profile.last_name = value) />
                    </div>
                    <div class="split">
                        <TextField label="City" value=move || state.get().profile.city on_input=move |value| state.update(|s| s.profile.city = value) />
                        <TextField label="State" value=move || state.get().profile.state on_input=move |value| state.update(|s| s.profile.state = value) />
                    </div>
                    <TextField label="Phone number" value=move || state.get().profile.phone on_input=move |value| state.update(|s| s.profile.phone = value) />
                    <TextField label="Street address" value=move || state.get().profile.address on_input=move |value| state.update(|s| s.profile.address = value) />
                    <TextField label="Email for opt-outs" value=move || state.get().profile.email on_input=move |value| state.update(|s| s.profile.email = value) />
                    <label>
                        "Listing URL or notes"
                        <textarea
                            prop:value=move || state.get().profile.notes
                            placeholder="Paste profile URLs, confirmation dates, or support notes here."
                            on:input=move |event| state.update(|s| s.profile.notes = event_target_value(&event))
                        ></textarea>
                    </label>
                </form>
                <div class="actions">
                    <button class="btn secondary" type="button" on:click=move |_| copy_text(&support_template(&state.get().profile))>
                        "Copy support request"
                    </button>
                    <button class="btn neutral" type="button" on:click=move |_| copy_text("Recheck data broker listings in 3 months.")>
                        "Copy recheck reminder"
                    </button>
                </div>
                <div class="links">
                    {move || SITES.iter().take(8).map(|site| view! {
                        <div class="link-row">
                            <div>
                                <strong>{site.name}</strong>
                                <div class="small subtle">{site.summary}</div>
                            </div>
                            <a href=search_url(site, &state.get().profile) target="_blank" rel="noopener">"Search"</a>
                        </div>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn TextField(
    label: &'static str,
    value: impl Fn() -> String + Copy + Send + 'static,
    on_input: impl Fn(String) + Copy + Send + 'static,
) -> impl IntoView {
    view! {
        <label>
            {label}
            <input
                prop:value=value
                on:input=move |event| on_input(event_target_value(&event))
            />
        </label>
    }
}

#[component]
fn DiscoveryView(state: RwSignal<AppState>) -> impl IntoView {
    let category_filter = RwSignal::new("all".to_string());
    let status_filter = RwSignal::new("all".to_string());

    view! {
        <section>
            <div class="grid">
                <div class="panel">
                    <div class="panel-head">
                        <div>
                            <h3>"Search query generator"</h3>
                            <p class="subtle">"Open targeted searches for phone, address, people-search wording, and broker clones."</p>
                        </div>
                    </div>
                    <div class="panel-body">
                        <div class="callout">"Use exact quotes for personal identifiers. Avoid saving screenshots or exports with sensitive data unless you intend to keep them."</div>
                        <div class="query-list">
                            {move || discovery_queries(&state.get().profile).into_iter().map(|(label, query)| view! {
                                <div class="link-row">
                                    <div>
                                        <strong>{label}</strong>
                                        <div class="small subtle">{query.clone()}</div>
                                    </div>
                                    <div class="broker-actions">
                                        <a class="mini-btn" href=google_search(&query) target="_blank" rel="noopener">"Google"</a>
                                        <a class="mini-btn" href=format!("https://www.bing.com/search?q={}", encode(&query)) target="_blank" rel="noopener">"Bing"</a>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>
                    </div>
                </div>
                <div class="panel">
                    <div class="panel-head">
                        <div>
                            <h3>"Discovery status"</h3>
                            <p class="subtle">"Mark candidate brokers as you check them. This list is intentionally broader than the guided workflows."</p>
                        </div>
                    </div>
                    <div class="panel-body">
                        <div class="split">
                            <label>
                                "Category"
                                <select on:change=move |event| category_filter.set(event_target_value(&event))>
                                    <option value="all">"All categories"</option>
                                    {categories().into_iter().map(|category| view! {
                                        <option value=category>{category}</option>
                                    }).collect_view()}
                                </select>
                            </label>
                            <label>
                                "Status"
                                <select on:change=move |event| status_filter.set(event_target_value(&event))>
                                    <option value="all">"All statuses"</option>
                                    <option value="unchecked">"Unchecked"</option>
                                    <option value="found">"Found"</option>
                                    <option value="not-found">"Not found"</option>
                                    <option value="removed">"Removed"</option>
                                    <option value="recheck">"Recheck"</option>
                                </select>
                            </label>
                        </div>
                    </div>
                </div>
            </div>
            <div class="panel spaced">
                <div class="panel-head">
                    <div>
                        <h3>"Broker candidates"</h3>
                        <p class="subtle">{move || {
                            let shown = filtered_sites(&category_filter.get(), &status_filter.get(), state).len();
                            format!("{shown} of {} brokers shown", SITES.len())
                        }}</p>
                    </div>
                </div>
                <div class="panel-body">
                    <div class="broker-table">
                        {move || filtered_sites(&category_filter.get(), &status_filter.get(), state).into_iter().map(|site| {
                            let site_id = site.id.to_string();
                            let discovery_status = move || state.with(|s| {
                                s.discovery
                                    .get(site.id)
                                    .map_or_else(|| "unchecked".to_string(), |d| d.status.clone())
                            });
                            view! {
                                <div class="broker-row">
                                    <div>
                                        <strong>{site.name}</strong>
                                        <div class="small subtle">{site.domain}</div>
                                        <div class="broker-meta">
                                            <span class="pill todo">{site.category}</span>
                                            <span class=if site.difficulty == "Hard" { "pill doing" } else { "pill todo" }>{site.difficulty}</span>
                                        </div>
                                    </div>
                                    <div class="small subtle">{site.signals.join(", ")}</div>
                                    <label>
                                        "Status"
                                        <select prop:value=discovery_status on:change=move |event| {
                                            let status = event_target_value(&event);
                                            state.update(|s| {
                                                s.discovery.insert(site_id.clone(), DiscoveryRecord {
                                                    status,
                                                    last_checked: js_sys::Date::new_0().to_iso_string().into(),
                                                });
                                            });
                                        }>
                                            <option value="unchecked">"Unchecked"</option>
                                            <option value="found">"Found"</option>
                                            <option value="not-found">"Not found"</option>
                                            <option value="removed">"Removed"</option>
                                            <option value="recheck">"Recheck"</option>
                                        </select>
                                    </label>
                                    <div class="broker-actions">
                                        <a class="mini-btn" href=search_url(&site, &state.get().profile) target="_blank" rel="noopener">"Search"</a>
                                        <a class="mini-btn" href=site.opt_out_url target="_blank" rel="noopener">"Opt out"</a>
                                        <button class="mini-btn" type="button" on:click=move |_| state.update(|s| {
                                            s.active_site = site.id.to_string();
                                            s.active_view = "workflow".to_string();
                                        })>"Workflow"</button>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}

fn load_state() -> AppState {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn save_state(state: &AppState) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
        && let Ok(raw) = serde_json::to_string(state)
    {
        let _ = storage.set_item(STORAGE_KEY, &raw);
    }
}

fn clear_state() {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.remove_item(STORAGE_KEY);
    }
    if let Some(win) = window() {
        let _ = win.location().reload();
    }
}

fn export_state(state: RwSignal<AppState>) {
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

fn copy_text(text: &str) {
    if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
        let _ = clipboard.write_text(text);
    }
}

fn current_site(state: RwSignal<AppState>) -> Site {
    let active = state.with(|s| s.active_site.clone());
    SITES
        .iter()
        .copied()
        .find(|site| site.id == active)
        .unwrap_or(SITES[0])
}

fn site_status(state: RwSignal<AppState>, site: &Site) -> (&'static str, &'static str) {
    let complete = state.with(|s| {
        s.progress
            .get(site.id)
            .map(BTreeSet::len)
            .unwrap_or_default()
    });
    if complete == site.steps.len() {
        ("Done", "done")
    } else if complete > 0 {
        ("In progress", "doing")
    } else {
        ("Not started", "todo")
    }
}

fn total_progress(state: RwSignal<AppState>) -> (usize, usize) {
    let total = SITES.iter().map(|site| site.steps.len()).sum();
    let done = state.with(|s| {
        SITES
            .iter()
            .map(|site| {
                s.progress
                    .get(site.id)
                    .map(BTreeSet::len)
                    .unwrap_or_default()
            })
            .sum()
    });
    (done, total)
}

fn search_url(site: &Site, profile: &Profile) -> String {
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

fn discovery_queries(profile: &Profile) -> Vec<(String, String)> {
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

fn filtered_sites(category: &str, status: &str, state: RwSignal<AppState>) -> Vec<Site> {
    SITES
        .iter()
        .copied()
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

fn categories() -> Vec<&'static str> {
    let mut categories = SITES.iter().map(|site| site.category).collect::<Vec<_>>();
    categories.sort_unstable();
    categories.dedup();
    categories
}

fn support_template(profile: &Profile) -> String {
    format!(
        "Hello,\n\nI am requesting removal or suppression of my personal information from your site. Please remove records matching:\n\nName: {}\nLocation: {}\nPhone: {}\nAddress: {}\nProfile URL(s): {}\n\nI am the subject of this request. Please confirm once the records have been removed or tell me what additional information is required to identify the listing.\n\nThank you.",
        empty_placeholder(&full_name(profile), "[your full name]"),
        empty_placeholder(&location(profile), "[city, state]"),
        empty_placeholder(&profile.phone, "[phone number, if relevant]"),
        empty_placeholder(&profile.address, "[street address, if relevant]"),
        empty_placeholder(&profile.notes, "[paste profile URLs here]"),
    )
}

fn full_name(profile: &Profile) -> String {
    [profile.first_name.as_str(), profile.last_name.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn location(profile: &Profile) -> String {
    [profile.city.as_str(), profile.state.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

fn empty_placeholder(value: &str, placeholder: &str) -> String {
    if value.is_empty() {
        placeholder.to_string()
    } else {
        value.to_string()
    }
}

fn google_search(query: &str) -> String {
    format!("https://www.google.com/search?q={}", encode(query))
}

fn progress_percent(done: usize, total: usize) -> i32 {
    done.saturating_mul(100)
        .checked_div(total)
        .and_then(|percent| i32::try_from(percent).ok())
        .unwrap_or_default()
}

fn encode(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}
