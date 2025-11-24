import init, { get_app } from "../pkg/src_rust.js";  // path depends on your layout

async function run() {
  await init();
  const app = document.getElementById("app");
  app.innerHTML = get_app();
}

run();

if (import.meta.hot) {
  import.meta.hot.accept("../../pkg/my_crate.js", async (updated) => {
    console.log("🔁 Reloading WASM module...");
    await updated.default();
  });
}
