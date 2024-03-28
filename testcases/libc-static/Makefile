HELLO ?= hello
HELLO_SRC ?= hello.c
MUSL_CC ?= x86_64-linux-musl-gcc

.PHONY: all
all: $(HELLO) disasm

$(HELLO): $(HELLO_SRC)
	$(MUSL_CC) -o $@ $^ -g0 -Os -static -no-pie -fno-builtin -fno-builtin-function -fno-stack-protector -fno-stack-clash-protection

disasm: $(HELLO)
	objdump -d $< > $<.asm

.PHONY: clean
clean:
	rm -f $(HELLO) $(HELLO).asm
