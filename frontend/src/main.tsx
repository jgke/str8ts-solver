import { PropsWithChildren, StrictMode, Suspense, use } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { App } from "./app";
import init from "../../solver_wasm/target/pkg-web";

const wasmPromise = init();

// eslint-disable-next-line react-refresh/only-export-components
function InitWasm(props: PropsWithChildren) {
  use(wasmPromise);

  return props.children;
}

const target = document.getElementById("wasm-render-target") || document.getElementById("root");

createRoot(target!).render(
  <StrictMode>
    <Suspense fallback="Loading...">
      <InitWasm>
        <App />
      </InitWasm>
    </Suspense>
  </StrictMode>,
);
