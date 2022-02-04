# ✏️ Jotsy: Just your notes

Jotsy is a self-hosted, free and open-source note taking app with a goal of simplicity in mind. It is powered by [Skytable](https://github.com/skytable/skytable).

> **🚧 NOTICE:** This is currently under development

## Features

- 🏢 Multi-user
- ✍️ Effective and distraction free notemaking
- 🔐 Secure authentication and session management
- 🌱 Extremely simple to self-host
- 🌲 Extremely light on resources
- 🍃 Extremely lightweight on the browser

## Getting started

1. First, install `docker-compose` by following the instructions [here](https://docs.docker.com/compose/install/)
2. Run this:
   ```sh
   mkdir jotsy && cd jotsy && wget https://raw.githubusercontent.com/ohsayan/jotsy/next/docker-compose.yml && sudo docker-compose up -d
   ```
3. Open your browser and head to http://localhost:2022

## Stack

- Backend:
  - Database: [Skytable](https://github.com/skytable/skytable)
  - Language: [Rust](https://rust-lang.org)
  - Framework/tools:
    - [Tokio](https://tokio.rs): Asynchronous runtime
    - [Axum](https://github.com/tokio-rs/axum): A web framework built on top of Tokio
- Frontend:
  - CSS Framework: [Bootstrap](https://getbootstrap.com/)
  - Scripting: Pure JavaScript (uses AJAX)
  - Markup: Pure HTML

## License

This web app is distributed under the [Apache-2.0 License](./LICENSE).
