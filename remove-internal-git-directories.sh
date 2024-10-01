#!/usr/bin/env bash

find . -name ".git" -type d -print -prune | grep -v "^./.git$" | xargs rm -rf
