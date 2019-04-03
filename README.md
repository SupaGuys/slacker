# Small readme

## How to install
* Install rust and cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* Run commands: cargo build && cargo run # _Note: cargo build will compile app and binary will be located in $PROJECT_PATH/target/debug/NAME

## Configure
App get its config from default settings(Config::new_from_env_or_default) or from ENV variables. List of ENV variables:
  * APP_NAME - just app name, other config parameters will depend on this name (APP_NAME will be used as prefix)
  * #{APP_NAME}_WEBHOOK_URL - webhook url where bot will send messages (configure in slack bot config, its bounded to chat, default bounded to slack/aalign/iwannafuckthingsup)
  * #{APP_NAME}_TOKEN - slack token (currently not used)
  * #{APP_NAME}_ROUTE_BASE - required for rouing requests to app. E.g APP_NAME = app, APP_ROUTE_BASE="some", then requests will be received by url "/app/some/#{handler}"
  * #{APP_NAME}_LISTEN_PORT - port app will bind to (default 3000)
  * #{APP_NAME}_LISTEN_HOST - addr app will bind to (default 0.0.0.0) 

## Tests
Will come up with next releases.
