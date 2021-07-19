#!/bin/sh

protobuf_main() {
    protobuf_generate_all "src/auth/auth_ticket"
    protobuf_generate_all "src/auth/password"
    protobuf_generate_all "src/auth/password/reset"
    protobuf_generate_all "src/avail"
    protobuf_generate_all "src/outline"
    protobuf_generate_all "src/example"
}

protobuf_generate_all() {
    local root
    local protobuf
    local dest

    root=$1
    protobuf=${root}/z_protobuf/
    dest=${root}/_ui/y_protobuf/

    rm -rf $dest/*

    local proto
    local file

    for proto in $(find "$protobuf" -name '*.proto'); do
        file=${proto#$protobuf}
        file=${file#/}
        file=${file%.proto}

        case "$file" in
        api | db)
            echo "${root} : ${file}"
            protobuf_generate "$proto" "$dest" "$file"
            ;;
        esac
    done
}
protobuf_generate() {
    local source_proto
    local destination_dir
    local destination_base_path

    source_proto=$1
    destination_dir=$2
    destination_base_path=$3

    local pb_path
    local data_path

    pb_path="${destination_dir}/${destination_base_path}_pb.js"
    data_path="${destination_dir}/${destination_base_path}_pb.d.ts"

    mkdir -p $destination_dir &&
        rm -f $pb_path $data_path &&
        pbjs -t static-module -w es6 -p ./src/ -o $pb_path $source_proto &&
        pbts -o $data_path $pb_path &&
        :
}

protobuf_main "$@"
