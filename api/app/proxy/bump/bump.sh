bump_build
for target in $(find . -name Cargo.toml); do
  bump_sync $target 's/^version = "[0-9.-]\+-proxy"/version = "'$(cat $BUMP_VERSION_FILE)'"/'
done
