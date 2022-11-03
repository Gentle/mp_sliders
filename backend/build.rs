fn main() {
    // the backend needs to be rebuilt when static files are updated
    // since those are baked into the wasm blob
    println!("cargo:rerun-if-changed=../static");
}
