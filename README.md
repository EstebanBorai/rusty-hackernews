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

1. Install `cargo watch`, even if its not required is conveninent to
have the project built on every file change in the project

2. Run `cargo watch -x run`

3. The website is available at http://0.0.0.0:3000

## Deployment

Deployment is done in Heroku using the [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust).
