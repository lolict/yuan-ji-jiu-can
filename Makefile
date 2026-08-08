## 囡囝共生 · 囡囝共生元 — 构建系统
##
## 用法:
##   make          — 构建 mlll 符号演示（默认）
##   make kernel   — 构建内核 + mlll 库联动
##   make test     — 构建 + 运行全部测试
##   make clean    — 清理产物
##   make docs     — 列出全部文档入口

CC       = clang
CFLAGS   = -O2 -std=c17 -Wall -Wextra -Wno-unused-variable -I. -Ikernel/include
KERN_LIBS = -pthread -lm

.PHONY: all test clean docs mlll-symbols kernel

all: mlll-symbols

# 独立 mlll 演示（含 main）
mlll-symbols: kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) $< -o kernel/src/mlll_symbols.elf -lm
	./kernel/src/mlll_symbols.elf

# 内核 + mlll 库联合编译（-DMLLL_LIBRARY 排除 mlll 的 main）
kernel: kernel/src/njsg_kernel.c kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $< kernel/src/mlll_symbols.c -o kernel/src/njsg_kernel.elf $(KERN_LIBS)

test: kernel
	@echo "═══ 囡囝共生内核测试 ═══"
	./kernel/src/njsg_kernel.elf

clean:
	rm -f kernel/src/njsg_kernel.elf kernel/src/mlll_symbols.elf

docs:
	@echo "═══ 囡囝共生 v2.0 文档入口 ═══"
	@echo ""
	@echo "代数基础:  docs/MLLL-ALGEBRAFOUNDATION.md"
	@echo "符号头文件: kernel/include/mlll_symbols.h"
	@echo "符号源文件: kernel/src/mlll_symbols.c (841 symbols)"
	@echo "白皮书:    docs/WHITEPAPER.md"
	@echo "架构文档:  docs/ARCHITECTURE.md"
	@echo "路线图:    ROADMAP.md"
