#!/bin/bash
trunk build --release \
&& cargo build -p backend --release