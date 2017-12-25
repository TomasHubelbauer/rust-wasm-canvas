let module;
let pointer;

window.addEventListener('load', async event => {
  const response = await fetch('index.wasm');
  const env = { }; // Pass math functions such as sin, cos, round, etc. `x: Math.x`.
  module = await WebAssembly.instantiate(await response.arrayBuffer(), { env });

  const canvas = document.getElementById('screen');
  const context = canvas.getContext('2d');
  const width = canvas.getAttribute('width');
  const height = canvas.getAttribute('height');
  const length = width * height * 4; // RGBA
  pointer = module.instance.exports.alloc(length);

  const memory = new Uint8ClampedArray(module.instance.exports.memory.buffer, pointer, length);
  const data = new ImageData(memory, width, height);

  let lastTimestamp;
  window.requestAnimationFrame(function step(timestamp) {
    module.instance.exports.fill(pointer, width, height, timestamp);
    context.putImageData(data, 0, 0);
    document.title = `${(1000 / (timestamp - lastTimestamp)).toFixed(2)} FPS`;
    lastTimestamp = timestamp;
    window.requestAnimationFrame(step);
  });
});

window.addEventListener('unload', event => module.instance.exports.dealloc(pointer));
