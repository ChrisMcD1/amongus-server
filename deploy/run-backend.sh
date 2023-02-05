#!/bin/bash

(kill $(lsof -ti :9090) || true) && \
    cd ~/amongus-server/backend && \
    cargo run --release > ~/logs/backend 2>&1
