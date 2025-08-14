import("./pkg").then((wasm) => {
  console.log("Available exports:", Object.keys(wasm));
  console.log("main function:", typeof wasm.main);
  if (typeof wasm.main === "function") {
    wasm.main();
  }
});
