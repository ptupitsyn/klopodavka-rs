#!/bin/bash

wasm-pack build --target web

if [ "$1" != "--skip-rollup" ]
then
  cp ./www/main.js pkg
  rollup ./pkg/main.js --format iife --file ./publish/bundle.js
fi

cp ./pkg/*.wasm publish
cp ./www/*.* publish