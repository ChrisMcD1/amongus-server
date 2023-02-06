#!/bin/bash
cd ~/amongus-server/deploy && \
    git pull > ~/logs/git-pull 2>&1 & \
    sh ./run-backend.sh && \
    sh ./run-frontend.sh

