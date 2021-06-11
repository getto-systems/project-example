#!/bin/sh

coverage_main() {
    local prof_dir
    local output_dir
    prof_dir="target/coverage"
    output_dir="./ui/public/dist/coverage/api"

    export RUSTFLAGS="-Zinstrument-coverage"
    export LLVM_PROFILE_FILE="${prof_dir}/%m-%p.profraw"

    cargo +nightly build
    cargo +nightly test

    if [ "$?" != 0 ]; then
        coverage_cleanup
        exit 1
    fi

    echo "generate coverage report..."
    grcov "${prof_dir}" \
        -s . \
        --binary-path ./target/debug \
        --llvm \
        --branch \
        --ignore-not-existing \
        --ignore 'api/**' \
        --ignore '**/x_*/**' \
        --ignore '**/y_*/**' \
        --ignore '**/z_*/**' \
        --ignore '**/test.rs' \
        --ignore '**/init.rs' \
        --ignore '**/data.rs' \
        --ignore '**/event.rs' \
        --ignore '**/infra.rs' \
        --ignore '**/infra/**' \
        --ignore 'src/main.rs' \
        -t html -o "${output_dir}"

    coverage_cleanup

    if [ -z "$(grep abbr "${output_dir}/index.html" | head -1 | grep "100 %")" ]; then
        echo "line coverage < 100%"
        exit 1
    fi
}
coverage_cleanup() {
    echo "clean up profile files"
    rm -rf "${prof_dir}"
}

coverage_main
