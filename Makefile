.DEFAULT:
build:
	@wasm-pack build --release --target web --out-dir out/web
	@wasm-pack build --release --target nodejs --out-dir out/nodejs
	@wasm-pack build --release --target no-modules --out-dir out/no-modules
	@wasm-pack build --release --target bundler --out-dir out/bundler