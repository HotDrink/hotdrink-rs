# Modified version of https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/build.sh
# This does the same thing, but uses wasm-pack instead.

export DTRACE = C:\Program Files\DTrace\dtrace

MODE=release

all: build README.md

build:
	wasm-pack build --out-dir www/pkg --target no-modules --$(MODE)

watch:
	cargo watch -w src -s make -d 5

doc: README.md
	cargo doc

BENCHES=$(wildcard tests/*.rs)
bench: $(BENCHES)
	wasm-pack test --chrome --release

README.md: src/lib.rs
	cargo readme --input $< > $@

clean:
	cargo clean

.PHONY: wasm doc clean bench
