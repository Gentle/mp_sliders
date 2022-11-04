#!/bin/bash
#RELEASE="--release"
RELEASE=""
cargo watch -s "trunk build $RELEASE" -s "cargo run $RELEASE"
