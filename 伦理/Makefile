NIM = nim

all: bin/验伦理 bin/验伦理铺

bin/验伦理: 验.nim 伦理.chain
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/验伦理 验.nim

bin/验伦理铺: 验铺.nim 伦理.chain 伦理映射.chain ../球面/球面映射.chain
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/验伦理铺 验铺.nim

验: bin/验伦理 bin/验伦理铺
	./bin/验伦理
	./bin/验伦理铺

.PHONY: all 验
