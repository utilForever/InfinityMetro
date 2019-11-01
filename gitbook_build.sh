#!/usr/bin/env bash
gitbook install && gitbook build

git checkout gh-pages
git pull origin gh-pages --rebase

cp -R _book/* .

git add .