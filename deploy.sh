#!/bin/bash
cd ~/amongus-server/deploy && \
    git pull > ~/logs/git-pull 2>&1 & \
    sh ~/amongus-server/deploy/run-backend.sh && \
    sh ~/amongus-server/deploy/run-frontend.sh

