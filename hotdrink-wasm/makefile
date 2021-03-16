# Modified version of https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/build.sh
# This does the same thing, but uses wasm-pack instead.

MODE=release

build:
	wasm-pack build --out-dir www/pkg --target no-modules --$(MODE)

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
