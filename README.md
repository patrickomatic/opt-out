# Opt-Out Desk

A static Leptos/WASM app for tracking manual data broker opt-out workflows and
discovering more people-search listings.

## Development

This app is built with Rust, Leptos, and Trunk.

This repository includes `rust-toolchain.toml`, so Rustup will use stable Rust
and install the `wasm32-unknown-unknown` target automatically.

If your shell finds Homebrew Rust first, `cargo build` can fail with:

```text
can't find crate for `core`
the `wasm32-unknown-unknown` target may not be installed
```

Fix that by putting Rustup before Homebrew Rust in `PATH`:

```sh
export PATH="/usr/local/opt/rustup/bin:$PATH"
rustup target add wasm32-unknown-unknown
```

Then confirm these point at Rustup, not Homebrew:

```sh
which cargo
which rustc
rustup which rustc
```

```sh
cargo build
trunk serve
```

For a one-off build without changing `PATH`, run Cargo through Rustup:

```sh
PATH="/usr/local/opt/rustup/bin:$PATH" cargo build
```

Build static files into `dist/`:

```sh
trunk build
```

`dist/` can be hosted by GitHub Pages, Cloudflare Pages, Netlify, Vercel, or any
other static host.

## Deployment

The repository includes a GitHub Pages workflow. On every push to `main`,
`.github/workflows/deploy.yml` builds the app with:

```sh
trunk build --release --public-url /opt-out/
```

Then it uploads `dist/` to GitHub Pages.

One-time repository setup is required because the default GitHub Actions token
cannot create the Pages site for this repo:

1. Open `Settings` -> `Pages`.
2. Set **Source** to **GitHub Actions**.
3. Re-run the `Deploy` workflow or push to `main`.

After the first successful deploy, the site will be available at:

```text
https://patrickomatic.github.io/opt-out/
```

## Current Brokers

- FastBackgroundCheck
- Spokeo
- Whitepages
- BeenVerified
- PeopleFinders
- TruthFinder
- Intelius
- Instant Checkmate
- US Search
- Radaris
- ThatsThem
- CyberBackgroundChecks
- SearchPeopleFree
- TruePeopleSearch
- FastPeopleSearch
- FamilyTreeNow
- Nuwber
- NumLookup

The site stores profile details and checklist progress only in the browser's
local storage. It does not submit forms automatically, bypass captchas, or
handle email or phone verification.
