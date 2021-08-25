bump_build
bump_sync Cargo.toml 's/^version = "[0-9.-]\+-api"/version = "'$(cat $BUMP_VERSION_FILE)'"/'
bump_sync api/vendor/getto-application/Cargo.toml 's/^version = "[0-9.-]\+-api"/version = "'$(cat $BUMP_VERSION_FILE)'"/'
bump_sync api/vendor/aws-cloudfront-cookie/Cargo.toml 's/^version = "[0-9.-]\+-api"/version = "'$(cat $BUMP_VERSION_FILE)'"/'
