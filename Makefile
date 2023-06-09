.DEFAULT:
build:
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/md5
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/sha1
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/sha2
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/sha3
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/sm3
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/ripemd
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/tiger
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/whirlpool
	@wasm-pack build --release --target bundler --scope axetroy --out-dir out ./crate/blake
