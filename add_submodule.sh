#!/bin/bash

for dir in *; do
  if [ -d "$dir" ] && [ ! -L "$dir" ]; then
    url="git@github.com:kotaYkw/rust-tutorial.git"
    git submodule add "$url" "$dir"
  fi
done

git add .
git commit -m "Add submodules"