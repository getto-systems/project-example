#!/bin/sh

protobuf_main() {
    local destination_dir
    destination_dir="src/y_protobuf"

    local pb_path
    local data_path

    # すべての proto を一度に処理しないと namespace の構築がうまくいかない
    # rust の prost は package が必須で、namespace は package の指定によって構築される
    pb_path="${destination_dir}/proto.js"
    data_path="${destination_dir}/proto.d.ts"

    protobuf_find_proto

    mkdir -p "$destination_dir" &&
        rm -f $pb_path $data_path &&
        protobuf_find_proto | xargs pbjs -t static-module -w es6 -p ./src/ -o "$pb_path" &&
        pbts -o "$data_path" "$pb_path" &&
        :
}
protobuf_find_proto() {
    find ./src -name api.proto -o -name db.proto
}

protobuf_main "$@"
