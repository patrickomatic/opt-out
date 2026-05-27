# Repo Notes

- Project: Opt-Out Desk
- Live site: https://patrickomatic.github.io/opt-out/
- Purpose: browser-based privacy workflow app for manual opt-outs
- Storage: browser `localStorage` only, under `optOutDeskState.v2`
- Build: Rust + Leptos + Trunk
- Default target: `wasm32-unknown-unknown`
- Local lint/build commands: `cargo build`, `cargo clippy -- -D warnings -D clippy::pedantic`, `trunk serve`, `trunk build --release --public-url /opt-out/`
- CI: `.github/workflows/ci.yml`
- Deploy: `.github/workflows/deploy.yml`
- Important note: GitHub Pages must be enabled once with **Source: GitHub Actions**

When making changes, keep the app static and browser-hostable. Avoid adding
server-side dependencies unless the user asks for them.
