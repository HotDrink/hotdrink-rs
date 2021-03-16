MODE=release

build:
	cargo build --$(MODE)

flamegraphs: flamegraphs/simple_planner.svg flamegraphs/hierarchical_planner.svg flamegraphs/constraint_system_update.svg

flamegraphs/%.svg: examples/%.rs
	cargo flamegraph --example $(basename $(notdir $@)) --output $@

README.md: src/lib.rs
	cargo readme --input $< > $@

doc: README.md
	cargo doc

clean:
	cargo clean
	rm -f perf.data perf.data.old

component-dot: examples/dot.rs
	mkdir -p dot
	cargo run --example dot

DOTS=$(wildcard dot/*.dot)
component-figures: component-dot $(DOTS:%.dot=%.png)
%.png: %.dot
	dot -Tpng $< -o $@


.PHONY: build flamegraphs clean component-figures dots
