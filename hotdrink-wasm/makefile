# Modified version of https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/build.sh
# This does the same thing, but uses wasm-pack instead.

MODE=release

build:
	wasm-pack build --$(MODE)

build-threaded: RUSTFLAGS='-C target-feature=+atomics,+bulk-memory'
build-threaded:
	@echo RUSTFLAGS=$(RUSTFLAGS)
	wasm-pack build --$(MODE) --target no-modules -- -Z build-std=panic_abort,std --features thread

watch:
	cargo watch -w src -s make -d 5

README.md: src/lib.rs
	cargo readme --input $< > $@

doc: README.md
	cargo doc

BENCHES=$(wildcard tests/*.rs)
bench: $(BENCHES)
	wasm-pack test --chrome --release

clean:
	cargo clean

.PHONY: wasm doc clean bench
