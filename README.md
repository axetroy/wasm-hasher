<div align="center">

  <h1><code>computed data hash by webAssembly</code></h1>

<strong>Support `md5`/`sha1`/`sha224`/`sha256`/`sha512`/`sm3`</strong>

  <p>
    <a href="https://github.com/axetroy/wasm-hasher/actions/workflows/rust.yml"><img src="https://github.com/axetroy/wasm-hasher/actions/workflows/rust.yml/badge.svg" alt="Build Status" /></a>
  </p>

<sub>Built with 🦀🕸 and inspired by [github.com/fuyoo/wasm-hasher](https://github.com/fuyoo/wasm-hasher)

</div>

The difference with [github.com/fuyoo/wasm-hasher](https://github.com/fuyoo/wasm-hasher)

1. Hash process can be interrupted with `abortController`
2. The exposed function can specify the chunks size for each read. Larger chunks use more CPU.
3. `onProgress` callback is optional. Set to `null` to improve performance if you don't need it.

## Usage

```js
import("@axetroy/wasm-hasher").then(({ default: hasher }) => {
  const controller = new abortController();
  const file = new Blob([], { type: "application/text" });
  hasher.md5(controller.signal, file, 1024 * 1024 * 10, (progress) => {
    console.log("hash progress");
  });
});
```

## 🚴 Installation

```bash
npm install @axetroy/wasm-hasher
```

### 🛠️ Build from source

Make sure you have install [rust^1.69](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
make
```

## License

[Anti-996](License)
