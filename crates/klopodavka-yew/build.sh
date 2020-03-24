#!/bin/bash

wasm-pack build --target web

cp ./www/main.js pkg

rollup ./pkg/main.js --format iife --file ./publish/bundle.js

cp ./pkg/*.wasm publish
cp ./www/*.* publish