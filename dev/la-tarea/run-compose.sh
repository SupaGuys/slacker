#!/usr/bin/env bash
export PROJECT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )"&& cd .. && cd .. && pwd )"
export LOCAL_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )"  && pwd )"
export BOT_NAME="$( dirname "${BASH_SOURCE[0]}" )"
export BOT_VERSION="$( head -1 "${PROJECT_PATH}/version" )"

cd "${PROJECT_PATH}"
docker-compose up
