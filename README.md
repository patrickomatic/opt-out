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

## Development

This repository uses Rust, Leptos, and Trunk.

The repo is configured for `wasm32-unknown-unknown` via `rust-toolchain.toml`
so `cargo build` expects the rustup-managed toolchain and target to be
available.

If your shell picks up Homebrew Rust first, put `~/.cargo/bin` ahead of
`/usr/local/bin` so rustup's `cargo` and `rustc` are used:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
which cargo rustc
rustup target add wasm32-unknown-unknown
```

Useful commands:

```sh
cargo build
cargo clippy -- -D warnings -D clippy::pedantic
trunk serve
trunk build --release --public-url /opt-out/
```

## Deployment

Pushes to `main` run CI and publish the site with GitHub Pages Actions.
GitHub Pages must be enabled once in the repository settings with **Source:
GitHub Actions**.

## File Map

- `src/main.rs`: Leptos client app, static broker catalog, local-storage state,
  workflow views, discovery helpers, export/reset actions, and search URL
  generation.
- `style.css`: Global CSS variables, layout, responsive rules, forms, panels,
  broker cards, status chips, and action buttons.
- `index.html`: Trunk entry document that links the CSS and Rust/WASM app.
- `Cargo.toml`: Rust package metadata and client-side dependencies.
- `Cargo.lock`: Locked dependency graph for reproducible builds.
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
