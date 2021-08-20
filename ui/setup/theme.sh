#!/bin/sh

if [ -z "$APP_ROOT" ]; then
    APP_ROOT=.
fi

ext='.theme-backup'

version=2.34.0
find $APP_ROOT/ui/public -name '*.html' | xargs sed -i$ext "s|[^/]*/getto.css|${version}/getto.css|"
find $APP_ROOT/storybook/.storybook -name '*.html' | xargs sed -i$ext "s|[^/]*/getto.css|${version}/getto.css|"

title="GETTO Example"
find $APP_ROOT/ui/public -name '*.html' | xargs sed -i$ext "s|<title>.*</title>|<title>${title}</title>|"

find $APP_ROOT/ui/public -name '*.html'$ext | xargs rm
find $APP_ROOT/storybook/.storybook -name '*.html'$ext | xargs rm
