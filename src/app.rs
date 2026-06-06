use crate::catalog::SITES;
use crate::model::{AppState, DiscoveryRecord, Site};
use crate::search::{
    categories, discovery_queries, filtered_sites, google_search, progress_percent, search_url,
    support_template,
};
use crate::status::{discovery_status, set_discovery_status, site_status, total_progress};
use crate::storage::{
    broker_element_id, clear_state, copy_text, export_state, load_state, save_state,
    scroll_to_broker,
};
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_navigate, use_params_map};
use leptos_router::{NavigateOptions, path};
use urlencoding::encode;
use web_sys::{Element, window};

const ROUTER_BASE: &str = "/opt-out";

fn close_header_menus() {
    if let Some(document) = window().and_then(|win| win.document())
        && let Ok(menus) = document.query_selector_all(".broker-menu[open], .tools-menu[open]")
    {
        for index in 0..menus.length() {
            if let Some(menu) = menus
                .item(index)
                .and_then(|menu| menu.dyn_into::<Element>().ok())
            {
                let _ = menu.remove_attribute("open");
            }
        }
    }
}

fn install_header_menu_dismissal() {
    let Some(document) = window().and_then(|win| win.document()) else {
        return;
    };
    let listener = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let click_is_in_menu = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .is_some_and(|target| {
                target
                    .closest(".broker-menu, .tools-menu")
                    .ok()
                    .flatten()
                    .is_some()
            });

        if !click_is_in_menu {
            close_header_menus();
        }
    });
    let _ = document.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref());
    listener.forget();
}

fn confirm_clear_state() {
    let confirmed = window()
        .and_then(|win| {
            win.confirm_with_message(
                "Are you sure you want to clear all local Opt-Out Desk data? This cannot be undone.",
            )
            .ok()
        })
        .unwrap_or(false);

    if confirmed {
        clear_state();
    }
}

/// Root application component that owns persistent state and view routing.
#[component]
pub(crate) fn App() -> impl IntoView {
    let state = RwSignal::new(load_state());

    Effect::new(move |_| {
        state.with(save_state);
    });

    view! {
        <Router base=ROUTER_BASE>
            <div class="shell">
                <Header state=state />
                <main>
                    <Hero state=state />
                    <Routes fallback=move || view! { <WorkflowView state=state /> }>
                        <Route path=path!("/") view=move || view! { <OnboardingView state=state /> } />
                        <Route path=path!("/workflow") view=move || view! { <WorkflowView state=state /> } />
                        <Route path=path!("/workflow/:site_id") view=move || view! { <WorkflowRoute state=state /> } />
                        <Route path=path!("/discovery") view=move || view! { <DiscoveryView state=state /> } />
                    </Routes>
                    <p class="sources">
                        "Workflow notes and seed broker candidates were assembled from public opt-out directories and broker pages on May 26, 2026. Always follow the form shown by the broker if it differs."
                    </p>
                </main>
            </div>
        </Router>
    }
}

/// Top navigation with view switches, broker status menu, and local tools.
#[component]
fn Header(state: RwSignal<AppState>) -> impl IntoView {
    install_header_menu_dismissal();
    let navigate = use_navigate();
    let go_setup = navigate.clone();
    let go_workflow = navigate.clone();
    let go_discovery = navigate.clone();

    view! {
        <header class="app-header">
            <div class="brand">
                <div class="mark" aria-hidden="true">"OD"</div>
                <div>
                    <h1>"Opt-Out Desk"</h1>
                    <p class="subtle">"A local checklist for data broker removals."</p>
                </div>
            </div>
            <nav class="top-nav" aria-label="Primary">
                <button
                    class="nav-button"
                    type="button"
                    aria-current=move || state.with(|s| (s.active_view == "setup").to_string())
                    on:click=move |_| go_setup("/", NavigateOptions::default())
                >
                    "Setup"
                </button>
                <button
                    class="nav-button"
                    type="button"
                    aria-current=move || state.with(|s| (s.active_view == "workflow").to_string())
                    on:click=move |_| go_workflow("/workflow", NavigateOptions::default())
                >
                    "Workflows"
                </button>
                <button
                    class="nav-button"
                    type="button"
                    aria-current=move || state.with(|s| (s.active_view == "discovery").to_string())
                    on:click=move |_| go_discovery("/discovery", NavigateOptions::default())
                >
                    "Discovery"
                </button>
            </nav>
            <div class="header-actions">
                <details class="broker-menu">
                    <summary class="menu-button">
                        <span>"Brokers"</span>
                        <span class="menu-count">{SITES.len()}</span>
                    </summary>
                    <div class="broker-menu-panel">
                    {move || SITES.iter().map(|site| {
                        let active_site_id = site.id.to_string();
                        let target_path = format!("/workflow/{}", site.id);
                        let navigate = navigate.clone();
                        let (label, class_name) = site_status(state, site);
                        view! {
                            <button
                                class="site-button"
                                type="button"
                                aria-current=move || state.with(|s| (s.active_site == active_site_id).to_string())
                                on:click=move |_| {
                                    close_header_menus();
                                    navigate(&target_path, NavigateOptions::default());
                                }
                            >
                                <span>{site.name}</span>
                                <span class=format!("pill {class_name}")>{label}</span>
                            </button>
                        }
                    }).collect_view()}
                    </div>
                </details>
                <details class="tools-menu">
                    <summary class="menu-button">"Tools"</summary>
                    <div class="tools-menu-panel">
                        <button class="site-button" type="button" on:click=move |_| export_state(state)>
                            <span>"Export progress"</span><span>"JSON"</span>
                        </button>
                        <button class="site-button" type="button" on:click=move |_| confirm_clear_state()>
                            <span>"Clear local data"</span><span>"Reset"</span>
                        </button>
                    </div>
                </details>
            </div>
            <div class="privacy-note">
                "Local only"
            </div>
        </header>
    }
}

/// Top summary area showing overall task progress.
#[component]
fn Hero(state: RwSignal<AppState>) -> impl IntoView {
    let progress = move || total_progress(state);
    view! {
        <section class="hero">
            <div>
                <h2>"Check each broker first, then remove only the listings that exist."</h2>
                <p class="subtle">
                    "Start with optional profile details to generate better searches. Each broker stays as one discovery task until you mark that your information was found."
                </p>
            </div>
            <div class="toolbar" aria-live="polite">
                <div class="progress-row">
                    <strong>{move || {
                        let (done, total) = progress();
                        format!("{done} of {total} active tasks complete")
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

/// Two-column workflow page for broker checks and the private workspace.
#[component]
fn WorkflowView(state: RwSignal<AppState>) -> impl IntoView {
    Effect::new(move |_| {
        state.update(|s| s.active_view = "workflow".to_string());
    });

    view! {
        <section class="grid">
            <BrokerQueue state=state />
            <Workspace state=state />
        </section>
    }
}

/// Workflow route that selects a broker from the `:site_id` URL parameter.
#[component]
fn WorkflowRoute(state: RwSignal<AppState>) -> impl IntoView {
    let params = use_params_map();
    let selected_site_id = move || {
        params
            .with(|params| params.get("site_id"))
            .filter(|site_id| SITES.iter().any(|site| site.id == site_id))
            .unwrap_or_else(|| state.with(|s| s.active_site.clone()))
    };

    Effect::new(move |_| {
        let site_id = selected_site_id();
        state.update(|s| {
            s.active_view = "workflow".to_string();
            s.active_site.clone_from(&site_id);
        });
        scroll_to_broker(&site_id);
    });

    view! { <WorkflowView state=state /> }
}

/// First-run setup view for optional search-profile entry.
#[component]
fn OnboardingView(state: RwSignal<AppState>) -> impl IntoView {
    let navigate = use_navigate();
    let start_workflow = navigate.clone();
    let skip_setup = navigate;

    Effect::new(move |_| {
        state.update(|s| s.active_view = "setup".to_string());
    });

    view! {
        <section class="grid">
            <div class="panel">
                <div class="panel-head">
                    <div>
                        <h3>"Set up your search profile"</h3>
                        <p class="subtle">"Optional details make broker searches and support templates faster. Leave anything blank if you prefer."</p>
                    </div>
                </div>
                <div class="panel-body">
                    <ProfileForm state=state />
                    <div class="actions">
                        <button class="btn" type="button" on:click=move |_| {
                            state.update(|s| s.onboarding_complete = true);
                            start_workflow("/workflow", NavigateOptions::default());
                        }>
                            "Start broker checks"
                        </button>
                        <button class="btn secondary" type="button" on:click=move |_| {
                            state.update(|s| s.onboarding_complete = true);
                            skip_setup("/workflow", NavigateOptions::default());
                        }>
                            "Skip for now"
                        </button>
                    </div>
                </div>
            </div>
            <div class="panel">
                <div class="panel-head">
                    <div>
                        <h3>"What happens next"</h3>
                        <p class="subtle">"The main workflow starts with one task per broker."</p>
                    </div>
                </div>
                <div class="panel-body">
                    <div class="onboarding-steps">
                        <div class="mini-step">
                            <strong>"1. Search"</strong>
                            <p>"Open the broker search link and check whether your record exists."</p>
                        </div>
                        <div class="mini-step">
                            <strong>"2. Decide"</strong>
                            <p>"Mark the broker as found or not found. Not-found brokers stay collapsed."</p>
                        </div>
                        <div class="mini-step">
                            <strong>"3. Remove"</strong>
                            <p>"When a record is found, that broker expands into the removal steps."</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Panel containing every broker as a discovery-first workflow card.
#[component]
fn BrokerQueue(state: RwSignal<AppState>) -> impl IntoView {
    view! {
        <div class="panel">
            <div class="panel-head">
                <div>
                    <h3>"Broker checks"</h3>
                    <p class="subtle">"Start with discovery. Removal steps appear only for brokers where your information is found."</p>
                </div>
            </div>
            <div class="panel-body">
                <div class="broker-queue">
                    {move || SITES.iter().map(|site| view! { <BrokerQueueItem state=state site=*site /> }).collect_view()}
                </div>
            </div>
        </div>
    }
}

/// Broker workflow card that expands removal steps only after discovery.
#[component]
fn BrokerQueueItem(state: RwSignal<AppState>, site: Site) -> impl IntoView {
    let discovery_status = move || discovery_status(state, site.id);
    let found = move || matches!(discovery_status().as_str(), "found" | "recheck");
    let status_label = move || site_status(state, &site);

    view! {
        <div
            id=broker_element_id(site.id)
            class=move || {
                if state.with(|s| s.active_site == site.id) {
                    "broker-card selected-broker"
                } else {
                    "broker-card"
                }
            }
        >
            <div class="broker-card-head">
                <div>
                    <strong>{site.name}</strong>
                    <div class="small subtle">{site.domain}</div>
                    <p class="small subtle">{site.summary}</p>
                </div>
                <span class=move || {
                    let (_, class_name) = status_label();
                    format!("pill {class_name}")
                }>
                    {move || status_label().0}
                </span>
            </div>
            <div class="discovery-step">
                <div class="broker-actions">
                    <a class="mini-btn search-action" href=move || search_url(&site, &state.get().profile) target="_blank" rel="noopener">"Search broker"</a>
                    <div class="choice-group" role="group" aria-label="Discovery result">
                        <button
                            class=move || {
                                if discovery_status() == "found" {
                                    "choice-btn found-choice selected-choice"
                                } else {
                                    "choice-btn found-choice"
                                }
                            }
                            type="button"
                            aria-pressed=move || (discovery_status() == "found").to_string()
                            on:click=move |_| set_discovery_status(state, site.id, "found")
                        >
                            "Found"
                        </button>
                        <button
                            class=move || {
                                if discovery_status() == "not-found" {
                                    "choice-btn not-found-choice selected-choice"
                                } else {
                                    "choice-btn not-found-choice"
                                }
                            }
                            type="button"
                            aria-pressed=move || (discovery_status() == "not-found").to_string()
                            on:click=move |_| set_discovery_status(state, site.id, "not-found")
                        >
                            "Not found"
                        </button>
                    </div>
                </div>
            </div>
            {move || {
                if found() {
                    view! {
                        <div class="expanded-steps">
                            <div class="callout">{site.caution}</div>
                            <ol class="steps">
                                {site.steps.iter().enumerate().map(|(index, step)| {
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
                                }).collect_view()}
                            </ol>
                            <div class="actions">
                                <a class="btn" href=site.opt_out_url target="_blank" rel="noopener">"Open opt-out"</a>
                                <button class="btn secondary" type="button" on:click=move |_| set_discovery_status(state, site.id, "removed")>"Mark removed"</button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="collapsed-steps" aria-hidden="true"></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Private notes and helper links shown beside the broker workflow queue.
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
                <ProfileForm state=state />
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

/// Form for editing locally stored profile and notes fields.
#[component]
fn ProfileForm(state: RwSignal<AppState>) -> impl IntoView {
    view! {
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
    }
}

/// Reusable controlled text input bound to a profile field.
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

/// Discovery page for broader search queries and filterable broker status.
#[component]
fn DiscoveryView(state: RwSignal<AppState>) -> impl IntoView {
    let category_filter = RwSignal::new("all".to_string());
    let status_filter = RwSignal::new("all".to_string());
    let navigate = use_navigate();

    Effect::new(move |_| {
        state.update(|s| s.active_view = "discovery".to_string());
    });

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
                            let target_path = format!("/workflow/{}", site.id);
                            let navigate = navigate.clone();
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
                                        <button
                                            class="mini-btn"
                                            type="button"
                                            on:click=move |_| navigate(&target_path, NavigateOptions::default())
                                        >
                                            "Workflow"
                                        </button>
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
