import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
    js.greet("Loaded WebAssembly");

    const button = document.getElementById("button");

    button.addEventListener('click', generateBarcode);

    function generateBarcode() {

        const barcodeValue = document.getElementById("barcode-value");
        const svgElement = js.generate_barcode_code128(barcodeValue.value);

        const svgDiv = document.getElementById("svg-container");
        svgDiv.insertAdjacentHTML("afterbegin", svgElement);
    }
});
