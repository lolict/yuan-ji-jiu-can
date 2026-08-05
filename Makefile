## 囡囝共生 · 囡囝共生元 — 构建系统
##
## 用法:
##   make          — 构建内核
##   make test     — 构建 + 运行全部测试
##   make clean    — 清理产物
##   make docs     — 列出全部文档入口

CC       = clang
CFLAGS   = -O2 -std=c17 -Wall -Wextra -Wno-unused-variable -I.
LDLIBS   = -L/usr/lib/x86_64-linux-gnu \
           /usr/lib/x86_64-linux-gnu/libsodium.so.23 \
           /usr/lib/x86_64-linux-gnu/libcrypto.so.3
KERN_LIBS = -pthread -lm

.PHONY: all test clean docs

all: kernel

kernel: kernel/src/njsg_kernel.elf

kernel/src/njsg_kernel.elf: kernel/src/njsg_kernel.c
	$(CC) $(CFLAGS) $< -o $@ $(KERN_LIBS)

test: kernel
	@echo "═══ 囡囝共生内核测试 ═══"
	./kernel/src/njsg_kernel.elf

clean:
	rm -f kernel/src/njsg_kernel.elf

docs:
	@echo "═══ 囡囝共生 v0.8 文档入口 ═══"
	@echo ""
	@echo "核心白皮书:  docs/WHITEPAPER.md"
	@echo "角色谱系:    docs/CAST.md"
	@echo "架构文档:    docs/ARCHITECTURE.md"
	@echo "契约公约:    SOVEREIGNTY.md"
	@echo "路线图:      ROADMAP.md"
	@echo "迁移指南:    deploy/MIGRATION_GUIDE.md"
	@echo "飞书对接:    deploy/FEISHU.md"
	@echo "Dockerfile:  Dockerfile + docker-compose.yml"
