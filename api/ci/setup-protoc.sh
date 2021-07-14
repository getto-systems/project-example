#!/bin/sh

setup_protoc() {
    local prefix
    prefix=$1

    if [ -z "$prefix" ]; then
        echo "usage: ./setup-protoc.sh <path/to/install/dir>"
        exit 1
    fi
    if [ ! -d $prefix ]; then
        echo "usage: ./setup-protoc.sh <path/to/install/dir>"
        exit 1
    fi

    local os
    os=$2
    if [ -z "$os" ]; then
        os=linux
    fi

    local project
    project=protocolbuffers/protobuf

    local version
    version=$(
        curl -sI https://github.com/${project}/releases/latest |
            grep "^location" |
            sed "s#.*/v##" |
            tr -d '\r'
    )

    mkdir -p tmp

    curl -sSL "https://github.com/${project}/releases/download/v${version}/protoc-${version}-${os}-x86_64.zip" >tmp/protoc.zip

    mkdir tmp/protoc
    cd tmp/protoc
    unzip ../protoc.zip

    mv -f bin/protoc ${prefix}

    cd ..
    rm -rf protoc.zip protoc
}

setup_protoc "$@"
