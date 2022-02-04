#!/bin/sh

deploy_main() {
  if [ ! -f "${GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON}" ]; then
    echo "key file : GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON is not exists"
    exit 1
  fi

  local target
  target=$1

  if [ -z "$target" ]; then
    echo "usage: deploy.sh <proxy | auth | core>"
    exit 1
  fi

  local host
  local region
  local project
  local image
  local version
  local tag
  local account

  host=asia.gcr.io
  region=asia-northeast1

  project=getto-projects
  image=example/${target}
  version=$(cat api/app/${target}/VERSION)

  tag=${host}/${project}/${image}:${version}

  export HOME=$(pwd)

  gcloud auth activate-service-account --key-file=${GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON}
  gcloud run deploy example-${target} --image="$tag" --platform=managed --region="$region" --project="$project"
}

deploy_main "$@"
