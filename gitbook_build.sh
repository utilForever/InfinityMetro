#!/usr/bin/env bash
gitbook install && gitbook build

git checkout gh-pages

cp -R _book/* .

git add .