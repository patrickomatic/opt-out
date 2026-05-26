# Opt-Out Desk

A static Leptos/WASM app for tracking manual data broker opt-out workflows and
discovering more people-search listings.

## Development

This app is built with Rust, Leptos, and Trunk.

```sh
trunk serve
```

For this machine, if Homebrew Rust is earlier in `PATH`, run Trunk with the
Rustup compiler explicitly:

```sh
RUSTC=/usr/local/opt/rustup/bin/rustc trunk serve
```

Build static files into `dist/`:

```sh
RUSTC=/usr/local/opt/rustup/bin/rustc trunk build
```

`dist/` can be hosted by GitHub Pages, Cloudflare Pages, Netlify, Vercel, or any
other static host.

## Deployment

The repository includes a GitHub Pages workflow. On every push to `main`,
`.github/workflows/deploy.yml` builds the app with:

```sh
trunk build --release --public-url /opt-out/
```

Then it uploads `dist/` to GitHub Pages. In the GitHub repository settings,
enable Pages with **Source: GitHub Actions**. After the first successful deploy,
the site will be available at:

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
