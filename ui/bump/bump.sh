bump_build
bump_sync package.json 's/"version": "[0-9.-]\+-ui"/"version": "'$(cat $BUMP_VERSION_FILE)'"/'
