#!/bin/bash

(kill $(lsof -ti :9090) || true) && \
    cd ~/among-us-server/backend && \
    cargo run --release > ~/logs/backend 2>&1
