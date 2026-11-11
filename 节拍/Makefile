NIM = nim

all: bin/节拍

bin/节拍: 节拍.nim 符号.toml 关系.chain 任务.chain 节拍.toml
	mkdir -p bin
	$(NIM) c --hints:off --nimcache:/tmp/opencode/nimcache -d:release -o:bin/节拍 节拍.nim

安装: bin/节拍
	ln -sf $(CURDIR)/bin/节拍 /root/.local/bin/节拍

验: bin/节拍
	./bin/节拍 验
