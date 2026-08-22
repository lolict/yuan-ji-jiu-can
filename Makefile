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

.PHONY: all test clean docs mlll-symbols kernel mlll-profile test-mlll

all: mlll-symbols

# 独立 mlll 演示（含 main）
mlll-symbols: kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) $< -o kernel/src/mlll_symbols.elf -lm
	./kernel/src/mlll_symbols.elf

# 满全法身份卡
mlll-profile: kernel/src/mlll_profile.c kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $^ -o kernel/src/mlll_profile.elf -lm
	./kernel/src/mlll_profile.elf 囡囝共生

# 内核 + mlll 库联合编译（-DMLLL_LIBRARY 排除 mlll 的 main）
kernel: kernel/src/njsg_kernel.c kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $< kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c -o kernel/src/njsg_kernel.elf $(KERN_LIBS)

test: kernel
	@echo "═══ 囡囝共生内核测试 ═══"
	./kernel/src/njsg_kernel.elf

clean:
	rm -f kernel/src/njsg_kernel.elf kernel/src/mlll_symbols.elf

docs:
	@echo "═══ 囡囝共生 v2.0 文档入口 ═══"
	@echo ""
	@echo "测试套件:  make test-mlll"
	@echo "维度升级:  MLLL九维 (841→904)"
	@echo "代数基础:  docs/MLLL-ALGEBRAFOUNDATION.md"
	@echo "符号头文件: kernel/include/mlll_symbols.h"
	@echo "符号源文件: kernel/src/mlll_symbols.c (904 symbols)"
	@echo "白皮书:    docs/WHITEPAPER.md"
	@echo "架构文档:  docs/ARCHITECTURE.md"
	@echo "路线图:    ROADMAP.md"

# MLLL 九维符号综合测拭 (独立 main, 不依赖内核)
test-mlll: kernel/src/mlll_test.c kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c kernel/src/mlll_pinyin.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $^ -o kernel/src/mlll_test.elf -lm
	@echo "═══ MLLL 九维测拭启动 ═══"
	./kernel/src/mlll_test.elf

# 编码系统 → 内核桥接测试
encoding-bridge-kernel: kernel/src/encoding_bridge_kernel.c kernel/src/encoding_bridge_kernel.h
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding -Iencoding/include \
	  kernel/src/encoding_bridge_kernel.c \
	  encoding/container_pipeline.c encoding/mccp_encdec.c encoding/fullquan_code.c \
	  encoding/mlll_vm.c encoding/mlll_cart.c encoding/hanzi_table.c \
	  encoding/husk_compiler.c encoding/container_codec.c encoding/knd_container.c \
	  kernel/src/mlll_symbols.c \
	  -o kernel/src/encoding_bridge_kernel.elf -lm
	./kernel/src/encoding_bridge_kernel.elf

# MLLL 汉字拼音知识库 demo + 夫妻对偶查询
mlll-pinyin: kernel/src/mlll_pinyin_demo.c kernel/src/mlll_pinyin.c kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $^ -o kernel/src/mlll_pinyin_demo.elf -lm
	@echo "═══ MLLL 拼音知识库 + 夫妻对偶查询 ═══"
	./kernel/src/mlll_pinyin_demo.elf
