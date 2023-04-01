#!/bin/bash

aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 101357155028.dkr.ecr.us-east-1.amazonaws.com/amongus-repository 
docker build -t 101357155028.dkr.ecr.us-east-1.amazonaws.com/amongus-repository:latest . 
docker push 101357155028.dkr.ecr.us-east-1.amazonaws.com/amongus-repository:latest 
