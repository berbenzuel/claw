import init, { get_app } from "../pkg/src_rust.js";  // path depends on your layout

async function run() {
  await init();
  const app = document.getElementById("app");
  app.innerHTML = get_app();
}

run();
