#!/bin/bash
cd ~/amongus-server/deploy && \
    git pull > ~/logs/git-pull 2>&1 & \
    ./run-backend.sh && \
    ./run-frontend.sh

