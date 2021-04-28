all: c

c: main.c bindings.h
	gcc $< target/debug/librust_in_c.so

rust:
	cargo build

bindings.h: rust
	cbindgen --lang c > bindings.h

run: c
	./a.out

clean:
	rm a.out bindings.h

.PHONY: rust bindings.h run clean
