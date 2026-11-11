NIM = nim

all: bin/微加载 bin/验微加载

bin/微加载: 微加载.nim
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/微加载 微加载.nim

bin/验微加载: 验.nim bin/微加载
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/验微加载 验.nim

收: bin/微加载
	./bin/微加载 收全

报: bin/微加载
	./bin/微加载 报 --html

编目: bin/微加载
	./bin/微加载 收全
	./bin/微加载 编目

验: bin/验微加载
	./bin/微加载 收全
	./bin/验微加载

.PHONY: all 收 报 编目 验
