<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./docs/flux-capacitor.jpeg" width="120" />
  </div>
  <h1 align="center">fluxcap</h1>
  <h4 align="center">
    HackerNews UI built in Rust using the Official HackerNews API. The Name "FluxCap" comes from the Flux Capacitor used in "Back to the Future" as it has a "Y" shape.
  </h4>
</div>

## Development

## Docker

> You must have Docker and Docker Compose installed in your system

1. Create a copy of `.env.sample` into a fille name `.env`

2. Execute `docker-compose -f ./docker-compose.dev.yml up --build`

3. Install the `sqlx-cli` using cargo. Follow the [crate documentation](https://lib.rs/crates/sqlx-cli)

4. Run database migrations `sqlx migrate run`

> When done, remember to teardown the Docker system by executing `docker-compose -f ./docker-compose.dev.yml down`

### Client

1. Install `trunk` following the [official documentation](https://trunkrs.dev/#install).

2. Install de Rust target for WASM using `rustup target add wasm32-unknown-unknown`

3. `cd` into the `client` directory and run `trunk serve`

### Server

1. Install `cargo watch`, even if its not required is conveninent to
have the project built on every file change in the project

2. Run `cargo watch -x "run --package server"`

3. The website is available at http://0.0.0.0:3000

## Deployment

Deployment is done in Heroku using the [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust).
