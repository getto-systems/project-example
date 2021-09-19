#!/bin/sh

prepare_main() {
    prepare_app api
    prepare_app auth
    prepare_app example

    prepare_merge
}
prepare_app() {
    local target
    target=$1

    cargo sqlx prepare -- --bin ${target}
    mv sqlx-data.json sqlx-data.${target}.json
}
prepare_merge() {
    node $APP_ROOT/api/ci/prepare-sqlx.js
}

prepare_main
