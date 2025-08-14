// index.js
async function init() {
  const wasm = await import("./pkg");
  wasm.main();
}

init();
