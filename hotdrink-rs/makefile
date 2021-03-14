MODE=release

flamegraphs: flamegraphs/simple_planner.svg flamegraphs/hierarchical_planner.svg flamegraphs/constraint_system_update.svg

flamegraphs/%.svg: examples/%.rs
	cargo flamegraph --example $(basename $(notdir $@)) --output $@

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


.PHONY: flamegraphs clean component-figures dots
