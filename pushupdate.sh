#!/usr/bin/env bash

read -p "This will delete the existing container, okay? [y/N] " conf
[ "${conf,,}" = "y" ] || exit

echo -n "Shutting down existing container: "
docker -v kill portfolio
echo -n "Deleting existing container: "
docker -v rm portfolio
echo "[Building latest version]"
docker build -t portfolio .
echo "
[Starting latest version]"
docker run --name 'portfolio' -p '4000:4000' -d portfolio:latest
