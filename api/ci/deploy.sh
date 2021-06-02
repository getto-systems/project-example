#!/bin/sh

deploy_main() {
  if [ ! -f "${GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON}" ]; then
    echo "key file : GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON is not exists"
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
  image=example/api
  version=$(cat $API_BUMP_VERSION_FILE)

  tag=${host}/${project}/${image}:${version}

  export HOME=$(pwd)

  gcloud auth activate-service-account --key-file=${GOOGLE_CLOUD_SERVICE_ACCOUNT_KEY_JSON}
  gcloud run deploy example-api --image="$tag" --platform=managed --region="$region" --project="$project"
}

deploy_main
