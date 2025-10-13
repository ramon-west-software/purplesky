# Introduction

This project is an exercise to create an atproto client (such as Bluesky) using Dioxus, a cross-platform Rust framework. The intention is to learn about mobile development, the Rust programming lanuage, and the Dioxus framework. This project in particular is inspired by the [Pinksky] (https://pinksky.app/) and [Flashes] (https://www.flashes.blue/) atproto clients. I found myself wanting for Pinksky to support videos like Flashes and for Flashes to support multiple users and platforms like Pinksky, so I decided to be the change I wish to see in the world and learn a new paradigm, language, and framework along the way. 

# Development

This project currently includes minimal organization with a `main.rs` file, a `route.rs` file, a `/components` directory, and a few default Dioxus assets. It was setup using the default configurations, with the exception of 'true' for using Dioxus Router.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ components/ # Any components that are used by the app should be placed here
│  ├─ main.rs # main.rs is the entry point to the application
│  ├─ route.rs # route.rs contains all available pages for navigation (Home, Search, Upload ...)
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for the project
```

### Serving the App

Run the following command in the root of the project to start developing with the default platform (configured as Mobile):

```bash
dx serve
```

To run on a specific platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform ios
dx serve --platform android
dx serve --platform web
```

# Changelog

## [Unreleased]

### v0.0.1 
- initial commit 

### v0.0.2
- login (using atproto auth service)
- navbar (bare-bone pages for Home, Search, Upload, Reels, and Profile)
