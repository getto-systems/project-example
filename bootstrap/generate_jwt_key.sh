#!/bin/sh

generate_jwt_key_main() {
    local key_name
    key_name=$1

    if [ -z "$key_name" ]; then
        echo "usage: generate_jwt_key.sh <key-name>"
        exit 1
    fi

    local private_key="${key_name}-key.pem"
    local pkcs8_key="${key_name}-key.pkcs8.pem"
    local public_key="${key_name}-key.pub.pem"

    # Generate private key
    openssl ecparam -name secp384r1 -genkey -noout -out $private_key

    # Convert private key
    openssl pkcs8 -in $private_key -topk8 -nocrypt -out $pkcs8_key

    # Generate public key
    openssl ec -in $private_key -pubout -out $public_key
}

generate_jwt_key_main "$@"
