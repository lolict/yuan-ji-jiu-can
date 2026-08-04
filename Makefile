## 满全法 · 囡囝共生元 — 构建系统
##
## 用法:
##   make          — 构建全部
##   make test     — 构建 + 运行全部测试
##   make kernel   — 构建空间关系代数内核
##   make clean    — 清理产物

CC       = clang
CFLAGS   = -O2 -std=c17 -Wall -Wextra -Wno-unused-variable -I.
LDLIBS   = -L/usr/lib/x86_64-linux-gnu \
           /usr/lib/x86_64-linux-gnu/libsodium.so.23 \
           /usr/lib/x86_64-linux-gnu/libcrypto.so.3
KERN_LIBS = -pthread -lm

.PHONY: all test clean kernel

all: kernel

kernel: kernel/src/mqf_kernel.elf

kernel/src/mqf_kernel.elf: kernel/src/mqf_kernel.c
	$(CC) $(CFLAGS) $< -o $@ $(KERN_LIBS)

test: kernel
	@echo "═══ 满全法内核测试 ═══"
	./kernel/src/mqf_kernel.elf

clean:
	rm -f kernel/src/mqf_kernel.elf
