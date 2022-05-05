#!/bin/sh

generate_cloudfront_key_pair_main() {
    local key_path
    key_path=$1

    if [ -z "$key_path" ]; then
        echo "usage: generate_cloudfront_key_pair.sh <key-path>"
        exit 1
    fi

    # Generate private key
    openssl genrsa -out $key_path/private_key.pem 2048

    # Generate public key
    openssl rsa -pubout -in $key_path/private_key.pem -out $key_path/public_key.pem
}

generate_cloudfront_key_pair_main "$@"
