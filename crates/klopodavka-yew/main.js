import init, { run_app } from './pkg/klopodavka_yew.js';
async function main() {
   await init('./pkg/klopodavka_yew_bg.wasm');
   run_app();
}
main();