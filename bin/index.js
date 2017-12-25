let module;
let pointer;

window.addEventListener('load', async event => {
  const response = await fetch('index.wasm');
  const env = {
    sin: Math.sin,
    cos: Math.cos,
    tan: Math.tan,
    fmod: (a, b) => a % b,
  }; // Pass math functions such as sin, cos, round, etc. `x: Math.x`.
  module = await WebAssembly.instantiate(await response.arrayBuffer(), { env });

  const canvas = document.getElementById('screen');
  const context = canvas.getContext('2d');
  const width = canvas.getAttribute('width');
  const height = canvas.getAttribute('height');
  const length = width * height * 4; // RGBA
  pointer = module.instance.exports.alloc(length);

  const memory = new Uint8ClampedArray(module.instance.exports.memory.buffer, pointer, length);
  const data = new ImageData(memory, width, height);

  const durations = Array(25);
  let durationIndex = 0;
  let lastTimestamp = 0;

  window.requestAnimationFrame(function step(timestamp) {
    module.instance.exports.fill(pointer, width, height, timestamp);
    context.putImageData(data, 0, 0);
    durations[durationIndex++ % durations.length] = timestamp - lastTimestamp;
    if (durationIndex < durations.length) {
      document.title = `Collecting… ${((durationIndex / durations.length) * 100).toFixed(0)} %`;
    } else {
      const tpf = durations.reduce((a, b) => a + b, 0) / durations.length;
      const fps = 1000 / tpf;
      document.title = `FPS: ~${fps.toFixed(1)} | TPF: ~${tpf.toFixed(2)} ms`;
    }
    
    lastTimestamp = timestamp;
    window.requestAnimationFrame(step);
  });
});

window.addEventListener('unload', event => module.instance.exports.dealloc(pointer));
