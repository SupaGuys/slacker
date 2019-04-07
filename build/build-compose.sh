#!/usr/bin/env bash
PROJECT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )"&& cd .. && pwd )"
cd "${PROJECT_PATH}"

export SLKR_VERSION=$(head -1 "${PROJECT_PATH}/version")
docker-compose build
