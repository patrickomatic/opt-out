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
and `./.cargo/config.toml`.

If your shell picks up Homebrew Rust first, set Rustup ahead of it:

```sh
export PATH="/usr/local/opt/rustup/bin:$PATH"
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
