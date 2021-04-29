CC=gcc -g -Wall -Wextra -Werror
HEADER=src/hotdrink.h

ifeq ($(OS),Windows_NT)
LIB=target/hotdrink_c.dll
EXE=a.exe
else
LIB=target/libhotdrink_c.so
EXE=a.out
endif

all: $(EXE)

# Run the example
run: $(EXE)
	./$<

# Compile the c-program and link to the Rust library
$(EXE): src/main.c $(HEADER)
	$(CC) $< $(LIB)

# Generate bindings with cbindgen
$(HEADER): $(LIB)
	cbindgen --lang c > $@

# Compile hotdrink-c
$(LIB): $(wildcard src/*.rs)
	cargo build -Z unstable-options --out-dir target

clean:
	rm $(EXE) $(BINDINGS)

.PHONY: run clean
