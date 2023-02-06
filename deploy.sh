#!/bin/bash
cd ~/amongus-server/deploy && \
    git pull > ~/logs/git-pull & \
    ./run-backend.sh && \
    ./run-frontend.sh

