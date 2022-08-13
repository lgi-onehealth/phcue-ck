FROM debian:bullseye-slim

ARG VERSION="0.1.0"

WORKDIR /app

RUN apt update -y && apt install -y libssl-dev libcurl4-openssl-dev

ADD https://github.com/lgi-onehealth/phcue-ck/releases/download/v${VERSION}/phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.tar.gz .

RUN tar -xvf phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.tar.gz && \
    mv phcue-ck /usr/local/bin/phcue-ck && \
    chmod +x /usr/local/bin/phcue-ck && \
    rm phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.tar.gz

RUN adduser --system --group --no-create-home app

USER app

RUN phcue-ck -a ERR5556343