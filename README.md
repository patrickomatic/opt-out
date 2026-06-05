# Opt-Out Desk

Live site: https://patrickomatic.github.io/opt-out/

Opt-Out Desk is a privacy workflow tool for manually removing your contact
details from people-search and background-check sites. It helps you:

- track opt-out steps for each broker
- keep private notes locally in the browser
- generate discovery searches for phone, address, and name variants
- export progress as JSON
- recheck brokers later when listings reappear

It does not submit opt-out forms for you, bypass captchas, or handle email or
phone verification. The point is to keep the process organized and less tedious.

## What’s Included

- Workflow tracking for common data-broker removal flows
- Discovery search generation for finding additional listings
- Local-only storage in browser `localStorage`
- Export/reset controls
- Static hosting through GitHub Pages
- GoatCounter page-view analytics

## Development

This repository uses Rust, Leptos, and Trunk.

The repo is configured for `wasm32-unknown-unknown` via `rust-toolchain.toml`.
Use the `make` targets for local work because they force the rustup-managed
toolchain even when Homebrew Rust appears earlier in your shell `PATH`.

Recommended commands:

```sh
make doctor
make check
make e2e
make serve
make build
```

`make e2e` runs a Rust WebDriver smoke test and expects `chromedriver` on
your `PATH`. CI installs Chrome and ChromeDriver automatically.

If your shell picks up Homebrew Rust first, put `~/.cargo/bin` ahead of
`/usr/local/bin` so rustup's `cargo` and `rustc` are used for direct Cargo
commands:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
which cargo rustc
rustup target add wasm32-unknown-unknown
```

Lower-level commands:

```sh
cargo build
cargo clippy -- -D warnings -D clippy::pedantic
trunk serve --public-url /opt-out/
trunk build --release --public-url /opt-out/
```

The app uses Leptos Router with `/opt-out` as its base path, matching the
GitHub Pages project URL. Open local development builds at
`http://localhost:8080/opt-out/`.

## Analytics

The site uses GoatCounter for lightweight page-view analytics. The tracking
script is in `index.html` and points to:

```html
https://opt-out-desk.goatcounter.com/count
```

Create a free GoatCounter site using `opt-out-desk` as the site code. If that
code is unavailable, update the `data-goatcounter` URL in `index.html` to match
the code you register.

## SEO Assets

The root `index.html` includes static metadata, social preview tags, structured
data, and a crawlable fallback intro before the WASM app mounts.

Regenerate the social preview and touch icon after visual or copy changes with:

```sh
node scripts/generate_seo_assets.mjs
```

## Deployment

Pushes to `main` run CI and publish the site with GitHub Pages Actions.
GitHub Pages must be enabled once in the repository settings with **Source:
GitHub Actions**.

## File Map

- `src/main.rs`: Module wiring and app mount point.
- `src/app.rs`: Leptos components, layout, and route-driven page rendering.
- `src/catalog.rs`: Static broker catalog and workflow steps.
- `src/model.rs`: Shared state and catalog data types.
- `src/search.rs`: Search URL generation, discovery queries, and support
  template helpers.
- `src/status.rs`: Broker status, progress, and discovery-state helpers.
- `src/storage.rs`: Local-storage persistence, export/reset actions, clipboard
  helpers, and broker-card scrolling.
- `style.css`: Global CSS variables, layout, responsive rules, forms, panels,
  broker cards, status chips, action buttons, and static fallback intro.
- `index.html`: Trunk entry document with SEO metadata, static fallback
  content, asset copying, CSS, and the Rust/WASM app.
- `robots.txt`, `sitemap.xml`: Static crawl directives for GitHub Pages.
- `favicon.svg`, `apple-touch-icon.png`, `og-image.png`: Browser and social
  preview assets.
- `site.webmanifest`: Static web app metadata.
- `scripts/generate_seo_assets.mjs`: Deterministic generator for PNG SEO
  assets.
- `Cargo.toml`: Rust package metadata and client-side dependencies.
- `Cargo.lock`: Locked dependency graph for reproducible builds.
- `Makefile`: Rustup-aware local command wrapper for toolchain setup, checks,
  builds, and development serving.
- `Trunk.toml`: Trunk build configuration for the static output directory and
  default relative public URL.
- `rust-toolchain.toml`: Rust toolchain, WASM target, rustfmt, and clippy
  components expected by local development and CI.
- `.github/workflows/ci.yml`: CI workflow for formatting, clippy, and Trunk
  build checks on pushes and pull requests.
- `.github/workflows/deploy.yml`: GitHub Pages workflow that builds the release
  app with `/opt-out/` as the public URL and deploys `dist`.
- `.gitignore`: Ignores local Rust and Trunk build outputs.
- `AGENTS.md`: Codex-facing repository guidance for future agent sessions.
- `CLAUDE.md`: Claude Code-facing copy of the same repository guidance.
- `LICENSE`: Project license.

Generated local directories:

- `dist/`: Trunk build output for the static site; ignored by Git.
- `target/`: Cargo build output; ignored by Git.
