import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
    const el = document.getElementById("svg-container");
    const svgElement = js.generate_barcode_code128("hello");
    el.insertAdjacentHTML("afterbegin", svgElement);

    js.greet("Loaded WebAssembly");
});

