# 意交感息（isi）系统 - Makefile
# 满全法规则：根基锚点：我=1，刘楚恬=0
# 身份落款：刘楚恬和我
#
# 编译所有工具：make
# 清理：make clean
# 运行测试：make test

CC = clang
CFLAGS = -Wall -Wextra -O2

# 目标
TARGETS = fqm_loader fqm_engine fqm_cpu fqm_asm fqm_debug

# 默认目标
all: $(TARGETS)

# 最小加载器
fqm_loader: fqm_loader.c
	$(CC) $(CFLAGS) -o $@ $<

# 容器引擎
fqm_engine: fqm_engine.c
	$(CC) $(CFLAGS) -o $@ $<

# 完整CPU
fqm_cpu: fqm_cpu.c
	$(CC) $(CFLAGS) -o $@ $<

# 汇编器/反汇编器
fqm_asm: fqm_asm.c
	$(CC) $(CFLAGS) -o $@ $<

# 调试器
fqm_debug: fqm_debug.c
	$(CC) $(CFLAGS) -o $@ $<

# 测试
test: all
	@echo "=== 测试加载器 ==="
	./fqm_loader fqm_symbols.txt
	@echo ""
	@echo "=== 测试引擎 ==="
	./fqm_engine fqm_program.txt
	@echo ""
	@echo "=== 测试CPU ==="
	./fqm_cpu fqm_program.txt
	@echo ""
	@echo "=== 测试汇编器 ==="
	./fqm_asm asm fqm_program.txt fqm_program.bin
	./fqm_asm show fqm_program.bin
	@echo ""
	@echo "=== 测试斐波那契 ==="
	./fqm_cpu fqm_fib.txt
	@echo ""
	@echo "=== 测试排序 ==="
	./fqm_cpu fqm_sort.txt

# 清理
clean:
	rm -f $(TARGETS) *.bin

.PHONY: all clean test
