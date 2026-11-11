CC ?= gcc
CFLAGS ?= -O2 -Wall -Wextra

all: chir_wasm

chir_wasm: chir_wasm.c
	$(CC) $(CFLAGS) -o $@ $<

# 从 Cangjie 源重新生成 CHIR，跑 11 个整数/控制流样例
test: chir_wasm
	./tests/run.sh

clean:
	rm -f chir_wasm

.PHONY: all test clean
