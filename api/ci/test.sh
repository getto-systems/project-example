#!/bin/sh

coverage_main() {
    # TODO ci 環境だと compile で失敗するので止めてあるが、なんとかしたい
    # インクリメンタルビルド的なことをしてるはずで、それが悪さをしていると推測
    if [ -z "$RUSTUP_HOME" ]; then
        RUSTUP_HOME="${HOME}/.rustup"
    fi

    local toolchain_dir
    local llvm_profdata
    local llvm_cov
    toolchain_dir="${RUSTUP_HOME}/toolchains"
    coverage_setup

    local target_dir
    local prof_dir
    local output_dir
    target_dir="target/debug/deps"
    prof_dir="target/coverage"
    output_dir="ui/public/dist/coverage/api"

    export RUSTFLAGS="-C instrument-coverage"
    export LLVM_PROFILE_FILE="${prof_dir}/prof-%p-%m.profraw"

    cargo +nightly test
    if [ "$?" != 0 ]; then
        coverage_cleanup
        exit 1
    fi

    echo "generate coverage report..."
    rm -rf "${output_dir}"
    mkdir -p "${output_dir}"

    local prof_data
    prof_data="${prof_dir}/merged.prodata"
    find "${prof_dir}" -type f -name '*.profraw' | xargs $llvm_profdata merge -sparse -o "${prof_data}"

    if [ ! -f "${prof_data}" ]; then
        echo "failed to merge prof data"
        exit 1
    fi

    local crate_name
    crate_name="$(cat Cargo.toml | grep name | head -1 | cut -d'"' -f2 | sed 's/-/_/g')"

    local object_files
    object_files=$(
        cargo +nightly test --no-run --message-format=json |
            grep '"filenames"' |
            grep "target/debug/deps/${crate_name}" |
            sed 's/^.*"filenames":\[\(.*\)\].*$/\1/' |
            sed 's/[",]/ /g' |
            sed 's/ \+/ /g' |
            sed 's/ $//g' |
            sed 's/ /-object /g'
    )

    local ignore_regex
    ignore_regex='(\.cargo|rustc|^api/|/[xy]_|/init/|/(main|test|init|data|infra|helper)\.rs)'

    $llvm_cov report ${object_files} \
        -Xdemangler=rustfilt \
        -instr-profile="${prof_data}" \
        --ignore-filename-regex="${ignore_regex}"

    $llvm_cov show ${object_files} \
        -Xdemangler=rustfilt \
        -instr-profile="${prof_data}" \
        --ignore-filename-regex="${ignore_regex}" \
        --format=html \
        --output-dir=${output_dir}

    coverage_cleanup
}
coverage_setup() {
    llvm_profdata=$(find "${toolchain_dir}" -type f -name llvm-profdata | head -1)
    llvm_cov=$(find "${toolchain_dir}" -type f -name llvm-cov | head -1)

    if [ ! -x "${llvm_profdata}" ]; then
        echo "llvm-profdata not found"
        exit 1
    fi
    if [ ! -x "${llvm_cov}" ]; then
        echo "llvm-cov not found"
        exit 1
    fi
}
coverage_cleanup() {
    echo "clean up profile files"
    rm -rf "${prof_dir}"
}

coverage_main
