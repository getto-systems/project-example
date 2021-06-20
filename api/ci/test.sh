#!/bin/sh

coverage_main() {
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

    export RUSTFLAGS="-Zinstrument-coverage"
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

    local object_file
    local output_file
    for object_file in $(find "${target_dir}" -type f -perm -a+x -name "${crate_name}"'-*'); do
        output_file="${output_dir}/$(basename "${object_file}").info"
        $llvm_cov export "${object_file}" \
            -Xdemangler=rustfilt \
            -instr-profile="${prof_data}" \
            --ignore-filename-regex='(\.cargo|rustc|^api/|/[xyz]_|/infra/|/(main|test|init|data|event|infra)\.rs)' \
            --format=lcov >"${output_file}"

        if [ -z "$(cat "${output_file}")" ]; then
            rm -f "${output_file}"
        fi
    done

    grcov "${output_dir}" -t html -o "${output_dir}"

    coverage_cleanup
    coverage_check
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
coverage_check() {
    local line_coverage
    line_coverage=$(grep abbr "${output_dir}/index.html" | head -1 | cut -d'>' -f 2 | cut -d'%' -f 1)
    case "${line_coverage}" in
    100*)
        echo "OK; line coverage: ${line_coverage}"
        ;;

    *)
        echo "NG; line coverage: ${line_coverage} < 100%"
        exit 1
        ;;
    esac
}
coverage_cleanup() {
    echo "clean up profile files"
    rm -rf "${prof_dir}"
}

coverage_main
