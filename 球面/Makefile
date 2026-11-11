NIM = nim

all: bin/验球面

bin/验球面: 验.nim 球面.chain 球面映射.chain 流账.chain
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/验球面 验.nim

验: bin/验球面
	./bin/验球面

.PHONY: all 验
