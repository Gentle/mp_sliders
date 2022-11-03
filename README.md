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

### run both frontend and backend

```bash
# in first terminal
./watch_backend.sh
# in second terminal
./watch_frontend.sh
```

both have an optional `--release` parameter if you want to test the release build

http://localhost:8080 is the frontend server with hot reloading. Websockets are automatically forwarded to the backend server

http://localhost:3000 is the actual server, this one should also be fully functional but does not have hot reloading

## Release Build

you can compile a release build by running `./build_release.sh`.

The resulting binary will be in `target/wasm32-wasi/release/backend.wasm`, to run the server you need to call `lunatic backend.wasm`

The server wasm is self-contained, it includes the frontend statically baked into the server wasm and serves those files from memory. This means that you will only need the `lunatic` binary and the `backend.wasm` file for deployments