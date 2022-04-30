#!/bin/sh

build_main() {
  local host
  local project
  local image
  local version
  local tag

  host=registry.gitlab.com

  cat docker login ${host}

  path=getto-systems-base/projects/example

  tag=${host}/${path}:latest

  docker build -f api/app/test/Dockerfile -t $tag . &&
    docker push $tag
}

build_main "$@"
