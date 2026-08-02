## 满全法 · 囡囝共生元 — 构建系统
##
## 用法:
##   make          — 构建全部
##   make test     — 构建 + 运行全部测试
##   make crypto   — 构建加密层测试
##   make clean    — 清理产物
##   make python   — 运行 python cli.py 的 report (保留参考实现)

CC       = clang
CFLAGS   = -O2 -std=c17 -Wall -Wextra -Werror -I.
LDLIBS   = -L/usr/lib/x86_64-linux-gnu \
           /usr/lib/x86_64-linux-gnu/libsodium.so.23 \
           /usr/lib/x86_64-linux-gnu/libcrypto.so.3

.PHONY: all test clean python

all: core/mqf.elf core/mqf_crypto_test.elf

core/mqf.elf: core/main.c core/mqf_kernel.c core/mqf_kernel.h encoding/mccp.h
	$(CC) $(CFLAGS) core/main.c core/mqf_kernel.c -o $@

core/mqf_crypto_test.elf: core/test_crypto.c core/mqf_crypto.c core/mqf_crypto.h core/mqf_kernel.h encoding/mccp.h
	$(CC) $(CFLAGS) core/test_crypto.c core/mqf_crypto.c -o $@ $(LDLIBS)

test: all
	@echo "═══ 内核测试 ═══"
	LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu ./core/mqf.elf
	@echo ""
	@echo "═══ 加密层测试 ═══"
	LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu ./core/mqf_crypto_test.elf

clean:
	rm -f core/mqf.elf core/mqf_crypto_test.elf

python:
	python3 cli.py report
