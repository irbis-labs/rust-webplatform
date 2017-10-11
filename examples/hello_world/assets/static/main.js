;(() => {
    "use strict";

    window.Module = {
        preRun: [],
        postRun: [main],
        wasmBinaryFile: "hello_world.wasm",
        // onRuntimeInitialized: main,
    };

    function main() {}
})();
