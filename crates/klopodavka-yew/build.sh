#!/bin/bash

wasm-pack build --target web

cp ./main.js pkg

cp pkg/*.wasm www
cp pkg/*.js www
cp ./main.js www
cd www || exit
rollup main.js --format iife --file bundle.js