<div align="center">

  <h1><code>snappy-wasm</code></h1>

  <strong>JavaScript compression/decompression with [snappy](https://github.com/google/snappy) for browsers and Node.js, powered by WebAssembly.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Installation

```
npm i snappy-wasm
```

## 🚴 Usage

### Browser / ES Modules


```js
import init from 'snappy-wasm'

// ...
const snappy = await init()
```

Note that additional configuration may be required to support top-level await in your environment.

### Node.js

```js
const snappy = require('snappy')

```

### Compress data

```js
const data = "data"
const compressed = snappy.compress(data)
```

This returns a `Uint8Array` instance

### Decompress data

```js
const decompressed = snappy.decompress(compressed)
```

Use `snappy.compress_raw(data)` and `snappy.decompress_raw(compressed)` to
compress/decompress raw data (provided and returned as `Uint8Array`)

### 🛠️ Build with `wasm-pack build` (via npm script)

```
npm run build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM

```
npm run build
npm publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## 👾 Development

Install the following

* [Node.js 16+](https://nodejs.org/en/)
* [Rust 2018](https://www.rust-lang.org/tools/install)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
