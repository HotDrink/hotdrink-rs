CC=gcc -g -Wall -Wextra -Werror
BINDINGS=bindings.h

ifeq ($(OS),Windows_NT)
LIB=target/hotdrink_c.dll
EXE=a.exe
else
LIB=target/libhotdrink_c.so
EXE=a.out
endif

all: $(EXE)

$(EXE): main.c $(BINDINGS)
	$(CC) $< $(LIB)

$(BINDINGS): $(LIB)
	cbindgen --lang c > $@

$(LIB): $(wildcard src/lib.rs)
	cargo build -Z unstable-options --out-dir target

run: c
	$(EXE)

clean:
	rm $(EXE) $(BINDINGS)

.PHONY: run clean
