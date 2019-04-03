FROM rust:latest
COPY . /app
WORKDIR /app

RUN cargo build

FROM ubuntu:trusty
RUN mkdir /app
WORKDIR /app
COPY --from=0 /app/target/debug/slack_eltasko /app/

CMD ["bash", "-c", "/app/slack_eltasko"]
