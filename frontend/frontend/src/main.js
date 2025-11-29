import init, { init_app } from "../pkg/src_rust.js";  // path depends on your layout

async function run() {
  await init();
  init_app();
}

run();
