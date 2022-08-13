#!/usr/bin/env bash

set -e
export BASE_DIR=$(pwd)
export VERSION=$(grep ^version ${BASE_DIR}/Cargo.toml | sed -e 's/version = \"//g' -e 's/\"//g')
export RELEASE_DIR=${BASE_DIR}/release
export LINUX_TARGET_BIN=${BASE_DIR}/target/x86_64-unknown-linux-musl/release/phcue-ck
export MAC_TARGET_BIN=${BASE_DIR}/target/x86_64-apple-darwin/release/phcue-ck

rm -rf ${RELEASE_DIR} && mkdir -p ${RELEASE_DIR}

cp ${LINUX_TARGET_BIN} ${RELEASE_DIR}/phcue-ck && \
    chmod +x ${RELEASE_DIR}/phcue-ck && \
    cd ${RELEASE_DIR} && \
    strip phcue-ck && \
    tar -czvf phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.tar.gz phcue-ck && \
    sha256sum phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.tar.gz > phcue-ck_v${VERSION}_x86_64-unknown-linux-musl.sha256sum && \
    rm ${RELEASE_DIR}/phcue-ck && \
    cd ${BASE_DIR}

cp ${MAC_TARGET_BIN} ${RELEASE_DIR}/phcue-ck && \
    chmod +x ${RELEASE_DIR}/phcue-ck && \
    cd ${RELEASE_DIR} && \
    strip phcue-ck && \
    zip phcue-ck_v${VERSION}_x86_64-apple-darwin.zip phcue-ck && \
    sha256sum phcue-ck_v${VERSION}_x86_64-apple-darwin.zip > phcue-ck_v${VERSION}_x86_64-apple-darwin.sha256sum && \
    rm ${RELEASE_DIR}/phcue-ck && \
    cd ${BASE_DIR}

