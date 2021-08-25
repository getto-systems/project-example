bump_build
bump_sync Cargo.toml 's/^version = "[0-9.-]\+-domain"/version = "'$(cat $BUMP_VERSION_FILE)'"/'
