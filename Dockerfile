# ---------------------------- BUILDTIME --------------------------- #
FROM rust:1.33

COPY [".", "/app/"]
WORKDIR /app
RUN set +x                                          \
    && find . -type f -print -exec chmod 644 {} \;  \
    && find . -type d -print -exec chmod 755 {} \;  \
    && cargo build

# ---------------------------- RUNTIME ------------------------------ #
FROM ubuntu:trusty

RUN mkdir /app
WORKDIR /app
COPY --from=0 /app/target/debug/slack_eltasko /app/

CMD ["bash", "-c", "/app/slack_eltasko"]
