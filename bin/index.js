const module = {};
let pointer;

window.addEventListener('load', async event => {
  const response = await fetch('index.wasm');
  const buffer = await response.arrayBuffer();
  const env = { }; // Pass math functions such as sin, cos, round, etc. `x: Math.x`.
  const wasm = await WebAssembly.instantiate(buffer, { env });

  module.alloc = wasm.instance.exports.alloc;
  module.dealloc = wasm.instance.exports.dealloc;
  module.fill = wasm.instance.exports.fill;

  const canvas = document.getElementById('screen');
  const width = canvas.getAttribute('width');
  const height = canvas.getAttribute('height');
  const context = canvas.getContext('2d');
  const length = width * height * 4;

  pointer = module.alloc(length);
  const memory = new Uint8ClampedArray(wasm.instance.exports.memory.buffer, pointer, length);
  const data = new ImageData(memory, width, height);

  let start = null;

  function step(timestamp) {
    if (start === null) start = timestamp;
    const progress = timestamp - start;
    if (progress > 100) {
      module.fill(pointer, width, height, timestamp);
      start = timestamp;
      window.requestAnimationFrame(draw);
    } else {
      window.requestAnimationFrame(step);
    }
  }

  function draw() {
    context.putImageData(data, 0, 0)
    window.requestAnimationFrame(step);
  }

  window.requestAnimationFrame(step);
});

window.addEventListener('unload', event => module.dealloc(pointer));
