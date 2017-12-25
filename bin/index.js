window.addEventListener('load', async event => {
    const response = await fetch('index.wasm');
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, {});
    console.log(wasm.instance.exports.add_one(1));
});
