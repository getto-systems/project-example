#!/bin/sh

build_main() {
  if [ ! -f "${GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON}" ]; then
    echo "key file : GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON is not exists"
    exit 1
  fi

  local target
  target=$1

  if [ -z "$target" ]; then
    echo "usage: build.sh <proxy | auth | core>"
    exit 1
  fi

  local host
  local project
  local image
  local version
  local tag

  host=asia.gcr.io

  cat $GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON | docker login -u _json_key --password-stdin https://${host}

  project=getto-projects
  image=example/${target}
  version=$(cat api/app/${target}/VERSION)

  tag=${host}/${project}/${image}:${version}

  docker build -f api/app/${target}/Dockerfile -t $tag . &&
    docker push $tag
}

build_main "$@"
