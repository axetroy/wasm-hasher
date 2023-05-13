<div align="center">

  <h1><code>computed data hash by webAssembly</code></h1>

<strong>Support `md5`/`sha1`/`sha224`/`sha256`/`sha512`/`sm3`/`ripemd`/`tiger`</strong>

  <p>
    <a href="https://github.com/axetroy/wasm-hasher/actions/workflows/rust.yml"><img src="https://github.com/axetroy/wasm-hasher/actions/workflows/rust.yml/badge.svg" alt="Build Status" /></a>
  </p>

<sub>Built with 🦀🕸 and inspired by [github.com/fuyoo/wasm-hasher](https://github.com/fuyoo/wasm-hasher)

</div>

The difference with [github.com/fuyoo/wasm-hasher](https://github.com/fuyoo/wasm-hasher)

1. Hash process can be interrupted with `abortController`
2. The exposed function can specify the chunks size for each read. Larger chunks use more CPU.
3. `onProgress` callback is optional. Set to `null` to improve performance if you don't need it.
4. separate packages, smaller wasm files

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
npm install @axetroy/wasm-hasher-md5
npm install @axetroy/wasm-hasher-sha1
npm install @axetroy/wasm-hasher-sha2
npm install @axetroy/wasm-hasher-sha3
npm install @axetroy/wasm-hasher-sm3
npm install @axetroy/wasm-hasher-ripemd
```

### Packages

| Package                     | Description                  | Version                                                                                                                              |
| --------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| @axetroy/wasm-hasher        | Including all supported hash | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher)               |
| @axetroy/wasm-hasher-md5    | Including md5 hash           | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-md5.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-md5)       |
| @axetroy/wasm-hasher-sha1   | Including sha1 hash          | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha1.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha1)     |
| @axetroy/wasm-hasher-sha2   | Including sha2 hash          | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha2.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha2)     |
| @axetroy/wasm-hasher-sha3   | Including sha3 hash          | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha3.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sha3)     |
| @axetroy/wasm-hasher-sm3    | Including sm3 hash           | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sm3.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-sm3)       |
| @axetroy/wasm-hasher-ripemd | Including ripemd hash        | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-ripemd.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-ripemd) |
| @axetroy/wasm-hasher-tiger  | Including tiger hash         | [![npm version](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-tiger.svg)](https://badge.fury.io/js/@axetroy%2Fwasm-hasher-tiger)   |

### 🛠️ Build from source

Make sure you have install [rust^1.69](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
make
```

## License

[Anti-996](License)
