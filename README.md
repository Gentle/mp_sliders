# Multiplayer Sliders - a Fullstack Demo

## Technologies used

- Rust's great wasm and wasi support
- [Lunatic](https://lunatic.solutions/) as a runtime
- [Submillisecond](https://github.com/lunatic-solutions/submillisecond) for http and websockets
- [Sycamore](https://sycamore-rs.netlify.app/) frontend in pure Rust
- [Trunk](https://trunkrs.dev/) as a bundler

## What is it?

this application synchronizes sliders across clients, open it in several tabs or several browsers to see synchronization

this is mainly built to showcase how a full stack rust application could be made

## How to run

### make sure you have a working cargo installation and wasm32 toolchain

```bash
rustup update
rustup target add wasm32-wasi wasm32-unknown-unknown
```

### Install Dependencies

```bash
# watch for changes
cargo install cargo-watch
# trunk bundles wasm for browsers and acts as devserver
cargo install --locked trunk
# lunatic runs the resulting wasi code of the server
cargo install lunatic-runtime
```

### run watchers for both frontend and backend

this is how you probably should run the server during development.

backend will run the actual webserver with the websocket

```bash
# in first terminal
./watch_backend.sh
```

Frontend (served by trunk) is the frontend devserver which comes with hot reloading, which is why this setup might be better

```bash
# in second terminal
./watch_frontend.sh
```

both have an optional `--release` parameter if you want to test the release build

http://localhost:8080 is the frontend server

http://localhost:3000 is the actual server

both should be able to handle clients but the 3000 one doesn't have hot reloading

### Running a single watcher

```bash
# debug build for everyday development
cargo watch -s "trunk build" -s "cargo run"
# alternatively, release build for testing performance
cargo watch -s "trunk build --release" -s "cargo run --release"
```

this watcher will watch for any changes, rebuild the frontend and then the backend and run the backend server. Only port 3000 will be oben with this setup

## Release Build

you can compile a release build by running `trunk build --release && cargo build --release` or alternatively `./build_release.sh`.

The resulting binary will be in `target/wasm32-wasi/release/backend.wasm`, to run the server you need to call `lunatic backend.wasm`

The server wasm is self-contained, it includes the frontend statically baked into the binary and serves those files from memory. This means that you will only need the `lunatic` binary and the `backend.wasm` file for deployments