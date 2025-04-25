onmessage = function (message) {
  import("../../../solver_wasm/target/pkg-web/solver_wasm.js").then((wasm) => {
    wasm.default().then(() => self.postMessage(wasm.generate(message.data)));
  });
};
