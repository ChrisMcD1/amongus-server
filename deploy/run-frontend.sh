#!/bin/bash

(kill $(lsof -ti :3000 || 0) || true) && \
    cd ~/amongus-server/frontend && \
    npm run start > ~/logs/frontend 2>&1
