#!/bin/bash

echo Building Dockerfile from github repo
docker build -t facts-bot .
echo Running container as factsbotcontainer with --rm
echo Warning: the required keys must be included in this folder. It must be called keys, it must contain a file for each key required. Read github readme for info on what keys are quired
docker run --name factsbotcontainer --rm -it facts-bot