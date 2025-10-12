# Introduction

This project is an exercise to create an atproto client (such as BlueSky) using Dioxus, a cross-platform Rust framework.

# Development

This bare-bones project currently includes minimal organization with a single `main.rs` file and a few assets. It was setup using the default configurations, with the exception of 'true' for using Dioxus Router.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving the App

Run the following command in the root of the project to start developing with the default platform:

```bash
dx serve
```

To run on a specificplatform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform ios
dx serve --platform android
dx serve --platform web
```

