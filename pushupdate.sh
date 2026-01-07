#!/usr/bin/env bash

read -p "This will shut down and delete the existing container, okay? [y/N] " conf
[ "${conf,,}" = "y" ] || exit

echo -n "Shutting down existing container: "
docker kill portfolio
echo -n "Deleting existing container: "
docker rm portfolio
echo "[Building latest version]"
docker build -t portfolio .
echo "
[Starting latest version]"
docker run --name 'portfolio' -p '4000:4000' -d portfolio:latest
