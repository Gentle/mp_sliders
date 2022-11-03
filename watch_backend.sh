#!/bin/bash
RUN_COMMAND="cargo run"
if [ "$1 " == "--release " ]; then
    RUN_COMMAND="$RUN_COMMAND --release"
fi
cargo watch -i frontend -i style.sass -i index.html -s "$RUN_COMMAND"