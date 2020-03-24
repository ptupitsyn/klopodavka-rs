import init, { run_app } from './klopodavka_yew';
async function main() {
   await init('./klopodavka_yew_bg.wasm');
   run_app();
}
main();