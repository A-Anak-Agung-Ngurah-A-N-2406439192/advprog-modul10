import * as bindings from "./pkg/yewchat_bg.js";

async function init() {
    const wasmUrl = new URL("./pkg/yewchat_bg.wasm", import.meta.url);
    const response = await fetch(wasmUrl);
    const bytes = await response.arrayBuffer();

    const { instance } = await WebAssembly.instantiate(bytes, {
        "./yewchat_bg.js": bindings,
    });

    bindings.__wbg_set_wasm(instance.exports);
    bindings.run_app();
}

init();