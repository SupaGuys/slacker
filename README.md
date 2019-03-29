# Small readme

## How to install
* Install rust with cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* Run commands: cargo build && cargo run #

## Configure
App get its configure from default setting, or from ENV variables. List of ENV variables:
  * APP_NAME - just app name, app othen ENV variables will start with this
  * #{APP_NAME}_WEBHOOK_URL - webhook url where bot will send messages (configure in slack bot config, its bounded to chat)
  * #{APP_NAME}_TOKEN - slack token (currently not used)
  * #{APP_NAME}_ROUTE_BASE - required for rouing requests to app. E.g APP_NAME = app, APP_ROUTE_BASE="some", then requests will be received by url "/app/some/#{handler}"
  * #{APP_NAME}_LISTEN_PORT - port app will bind to (default 3000)
  * #{APP_NAME}_LISTEN_HOST - addr app will bind to (default 0.0.0.0) 


