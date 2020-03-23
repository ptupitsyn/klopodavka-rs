# Yew-based Klopodavka front end

Uses [Yew framework](https://yew.rs/docs/) to produce HTML/WebAssembly UI.

## Run

* `wasm-pack build --target web`
* `rollup ./main.js --format iife --file ./pkg/bundle.js`
* Serve this dir with any web server