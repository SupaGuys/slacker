# Slack reminder bot

## How to install

* Install rust and cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* Run commands: cargo build && cargo run # _Note: cargo build will compile app and binary will be located in $PROJECT_PATH/target/debug/NAME

## Configuration

App get its config from default settings(Config::new_from_env_or_default) or from ENV variables.
List of ENV variables:

* *APP_NAME* - just app name, other config parameters will depend on this name (APP_NAME will be used as prefix)
* *${APP_NAME}_WEBHOOK_URL* - webhook url where bot will send messages (configure in slack bot config, its bounded to chat, default bounded to _slack/aalign/iwannafuckthingsup_ )
* *${APP_NAME}_TOKEN* - slack token (currently not used) _(default NONE)_
* *${APP_NAME}_ROUTE_BASE* - required for rouing requests to app. _(default APPNAME)_
  E.g APP_NAME = app, APP_ROUTE_BASE="some", then requests will bereceived by url "/app/some/#{handler}"
* *${APP_NAME}_LISTEN_PORT* - port app will bind to _(default 3000)_
* *${APP_NAME}_LISTEN_HOST* - addr app will bind to _(default 0.0.0.0)_

## Tests

Will come up with next releases.

## Development with docker compose

There is a sample configuration in *dev/la-tarea* folder.
You need to configure following:
Build and run wrapper for linux or windows. (Just for convenience, of course you can go with manual environment variables)
You need to create *.env* file with environmet variables:

```bash
# Template:
APP_NAME=Your-Bot-Name
Your-Bot-Name_LISTEN_HOST=Your-Host
Your-Bot-Name_LISTEN_PORT=Your-Port
Your-Bot-Name_ROUTE_BASE=Your-Router-Base-Name
Your-Bot-Name_TOKEN=NONE
Your-Bot-Name_WEBHOOK_URL=Your-Slack-Web-Hook
VERSION=Your-Docker-Image-Version
# Example:
APP_NAME=LA_TAREA
LA_TAREA_LISTEN_HOST=0.0.0.0
LA_TAREA_LISTEN_PORT=3000
LA_TAREA_ROUTE_BASE=la_tarea
LA_TAREA_TOKEN=NONE
LA_TAREA_WEBHOOK_URL=https://hooks.slack.com/services/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
VERSION=${BOT_VERSION}
```

## Curl examples to test deployment locally

### Commands

```bash
# Template:
curl -XPOST http://localhost:3000/YOUR-BOT-NAME/show_help
curl -XPOST http://localhost:3000/YOUR-BOT-NAME/list
curl -XPOST http://localhost:3000/YOUR-BOT-NAME/add

# Example:
curl -XPOST http://localhost:3000/LA_TAREA/show_help
#
curl -XPOST http://localhost:3000/LA_TAREA/list
curl -XPOST http://localhost:3000/LA_TAREA/add
```

### Output

* show_help

```bash
Info on how to work with bot
It supports following commands:
        /show_help - print this help
        /add - add task to remind. Internally it will use `at` to schedule time. Syntax: `/add "Text you want to send (IMPORTANT!! Text must be INSIDE quotes)" remind interval[e.g. 10min, 1hour, 1year]"`. You can also use command like this: `/add Some text you wish to send` and in this case text will be send immediately. You will receive response on result of this operation.
        /list - will show output of atq. It will not be informative in fact, but it's added here to optimize this function in future.
```
