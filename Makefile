.DEFAULT_GOAL := build

build:
	@wasm-pack build --release --no-opt --target bundler --scope axetroy --out-dir out
	@make build-crates

CRATES := md5 sha1 sha2 sha3 sm3 ripemd tiger whirlpool blake

build-crates:
	@for crate in $(CRATES); do \
		wasm-pack build --release --no-opt --target bundler --scope axetroy --out-dir out ./crate/$$crate; \
	done