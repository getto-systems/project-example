#!/bin/sh

coverage=$(cat $1 | tail -1 | awk '{ print $10 }')
echo "coverage: $coverage"

case "$coverage" in
  100% | 9?.?? | 9?.?%% | 9?%)
    echo ok
    ;;
  *)
    echo ng
    exit 1
    ;;
esac
