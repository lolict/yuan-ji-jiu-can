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
# 包含盟机合网络骨架 njsg_mengji.c + 感知器官 mind_organ
kernel: kernel/src/njsg_kernel.c kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c kernel/src/njsg_mengji.c encoding/mind_organ.c
	@echo "注意: 全内核有预存在的离线认证类型问题, 使用 make test-global-control-kernel 验证 mind_organ 内核集成"
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding $< kernel/src/mlll_symbols.c kernel/src/mlll_kernel_bridge.c kernel/src/njsg_mengji.c encoding/mind_organ.c -o kernel/src/njsg_kernel.elf $(KERN_LIBS)

# 容器本体论统一桥接端到端测试
test-ontology-bridge: encoding/container_ontology_bridge.c \
                     encoding/container_ontology_bridge.h \
                     encoding/test_ontology_bridge.c \
                     encoding/encoding_bridge_emit.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding \
	  encoding/test_ontology_bridge.c \
	  encoding/container_ontology_bridge.c \
	  encoding/encoding_bridge_emit.c \
	  encoding/container_pipeline.c \
	  encoding/mccp_encdec.c \
	  encoding/fullquan_code.c \
	  encoding/mlll_vm.c \
	  encoding/mlll_cart.c \
	  encoding/hanzi_table.c \
	  encoding/husk_compiler.c \
	  encoding/container_codec.c \
	  encoding/knd_container.c \
	  encoding/global_control.c \
	  encoding/property_judge.c \
	  encoding/radix_star.c \
	  -o encoding/test_ontology_bridge.elf -lm
	@echo "═══ 容器本体论 · 统一桥接端到端 ═══"
	./encoding/test_ontology_bridge.elf

test: kernel
	@echo "═══ 囡囝共生内核测试 ═══"
	./kernel/src/njsg_kernel.elf

# 独立 fqm 工具链（纯 C，无外部协议）
fqm-tools: fqm_browser.c fqm_gtti.c fqm_call.c fqm_bare.c fqm_cpu.c fqm_net.c fqm_radix.c fqm_time.c fqm_yy.c fqm_judge.c fqm_link.c fqm_cluster.c fqm_flow.c
	$(CC) -O2 -o fqm_browser fqm_browser.c
	$(CC) -O2 -o fqm_gtti fqm_gtti.c
	$(CC) -O2 -o fqm_call fqm_call.c
	$(CC) -O2 -o fqm_bare fqm_bare.c
	$(CC) -O2 -o fqm_cpu fqm_cpu.c
	$(CC) -O2 -o fqm_net fqm_net.c
	$(CC) -O2 -o fqm_radix fqm_radix.c -lm
	$(CC) -O2 -o fqm_time fqm_time.c
	$(CC) -O2 -o fqm_yy fqm_yy.c
	$(CC) -O2 -o fqm_judge fqm_judge.c
	$(CC) -O2 -o fqm_link fqm_link.c
	$(CC) -O2 -o fqm_cluster fqm_cluster.c -lm
	$(CC) -O2 -o fqm_flow fqm_flow.c
	@echo "═══ fqm 独立工具链构建完成 ═══"

fqm-test: fqm-tools
	./fqm_browser
	./fqm_gtti
	./fqm_call
	./fqm_bare
	./fqm_cpu fqm_program.txt
	./fqm_net
	./fqm_radix
	./fqm_time
	./fqm_yy
	./fqm_judge
	./fqm_link
	./fqm_cluster
	./fqm_flow

clean:
	rm -f kernel/src/njsg_kernel.elf kernel/src/mlll_symbols.elf
	rm -f fqm_browser fqm_gtti fqm_call fqm_bare fqm_cpu fqm_net fqm_radix fqm_time fqm_yy fqm_judge fqm_link fqm_cluster fqm_flow

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

# 产权流侵权判定树测试
test-property-judge: encoding/property_judge.c encoding/property_judge.h encoding/test_property_judge.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding \
	  encoding/test_property_judge.c \
	  encoding/property_judge.c \
	  -o encoding/test_property_judge.elf -lm
	@echo "═══ 产权流侵权判定树 · vA+vB==2·v_core ═══"
	./encoding/test_property_judge.elf

# 部首星图端到端测试 (水/火/言三部)
test-radix-star: encoding/radix_star.c encoding/radix_star.h encoding/test_radix_star.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding \
	  encoding/test_radix_star.c \
	  encoding/radix_star.c \
	  -o encoding/test_radix_star.elf -lm
	@echo "═══ 部首星图 · 水/火/言三部端到端 ═══"
	./encoding/test_radix_star.elf

# 全局控制容器 → 内核桥接测试 (含感知器官 mind_organ)
test-global-control-kernel: kernel/src/global_control_bridge_kernel.c \
                            kernel/include/global_control_bridge_kernel.h \
                            encoding/global_control.c encoding/global_control.h \
                            encoding/mind_organ.c encoding/mind_organ.h
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -DBRIDGE_TEST -Iencoding -Ikernel/include \
	  kernel/src/global_control_bridge_kernel.c \
	  encoding/global_control.c \
	  encoding/mind_organ.c \
	  -o kernel/src/global_control_bridge_kernel.elf -lm
	@echo "═══ 全局控制容器 · 内核桥接 (含感知器官) ═══"
	./kernel/src/global_control_bridge_kernel.elf

# 全局控制容器测试 (声母遍历韵母 + 聂小倩借尸还魂 + 决策树)
test-global-control: encoding/global_control.c encoding/global_control.h encoding/test_global_control.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY -Iencoding \
	  encoding/test_global_control.c \
	  encoding/global_control.c \
	  -o encoding/test_global_control.elf -lm
	@echo "═══ 全局控制容器 — 声母遍历韵母 端到端 ═══"
	./encoding/test_global_control.elf

# 时间线状态机 · 九维漂移测试
test-ttl-state: kernel/src/ttl_state.c kernel/include/ttl_state.h kernel/src/ttl_test.c kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY kernel/src/ttl_test.c kernel/src/ttl_state.c kernel/src/mlll_symbols.c -o kernel/src/ttl_test.elf -lm
	@echo "═══ 时间线状态机 · 九维漂移测试 ═══"
	./kernel/src/ttl_test.elf

# MLLL 汉字拼音知识库 demo + 夫妻对偶查询
mlll-pinyin: kernel/src/mlll_pinyin_demo.c kernel/src/mlll_pinyin.c kernel/src/mlll_symbols.c
	$(CC) $(CFLAGS) -DMLLL_LIBRARY $^ -o kernel/src/mlll_pinyin_demo.elf -lm
	@echo "═══ MLLL 拼音知识库 + 夫妻对偶查询 ═══"
	./kernel/src/mlll_pinyin_demo.elf
