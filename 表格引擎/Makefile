NIM ?= nim
NIMCACHE ?= /tmp/opencode/nimcache

all: bin/引擎

bin/引擎: 主.nim 表.nim 边界.nim
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:$(NIMCACHE) -d:release -o:bin/引擎 主.nim

bin/验: 验.nim 边界.nim 表.nim
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:$(NIMCACHE) -d:release -o:bin/验 验.nim

验: bin/验
	./bin/验

.PHONY: all 验
