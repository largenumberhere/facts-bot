#!/bin/bash

echo Building Dockerfile from github repo
docker build -t factsbot .
echo Running container as factsbotcontainer with --rm
echo Warning: You must include keys within this folder. You must make a subdirectory called keys, it must contain a file for each key required. Read github readme for info on what keys are quired
sudo docker run --name factsbotcontainer --rm -it factsbot