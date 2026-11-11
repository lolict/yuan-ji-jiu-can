# 汉字地址 · AOT
# 一次编译，生成机器码。之后直接：查 江
# 内核零改动。语义在表里，机器码在这个二进制里。

NIM ?= nim
NIMFLAGS = --hints:off --nimcache:/tmp/opencode/nimcache -d:release
BINDIR = bin
OUTDIR ?= $(BINDIR)

.PHONY: all 查 选址 防止符 拼装 许可 进制串联 约定 验 安装 验收 clean

all: 查 选址 防止符 拼装 许可 进制串联 约定 验

$(OUTDIR):
	mkdir -p $(OUTDIR)

查: $(OUTDIR) 呼唤.nim 字.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/查 查.nim

选址: $(OUTDIR) 呼唤.nim 字.toml 轴.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/选址 选址.nim

防止符: $(OUTDIR) 呼唤.nim 轴.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/防止符 防止符.nim

拼装: $(OUTDIR) 呼唤.nim 笔画.toml 结构.toml 感知.toml 字.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/拼装 拼装.nim

许可: $(OUTDIR) 呼唤.nim 许可.toml 进制串联.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/许可 许可.nim

进制串联: $(OUTDIR) 呼唤.nim 进制串联.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/进制串联 进制串联.nim

约定: $(OUTDIR) 呼唤.nim 约定.chain 码位表.toml 进制串联.toml
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/约定 约定.nim

验: $(OUTDIR) 查 选址 防止符 拼装 许可 进制串联 约定
	$(NIM) c $(NIMFLAGS) -o:$(OUTDIR)/验 验.nim

安装: all
	mkdir -p $(HOME)/.local/bin
	ln -sf $(CURDIR)/$(OUTDIR)/查 $(HOME)/.local/bin/查
	ln -sf $(CURDIR)/$(OUTDIR)/选址 $(HOME)/.local/bin/选址
	ln -sf $(CURDIR)/$(OUTDIR)/防止符 $(HOME)/.local/bin/防止符
	ln -sf $(CURDIR)/$(OUTDIR)/拼装 $(HOME)/.local/bin/拼装
	ln -sf $(CURDIR)/$(OUTDIR)/许可 $(HOME)/.local/bin/许可
	ln -sf $(CURDIR)/$(OUTDIR)/进制串联 $(HOME)/.local/bin/进制串联
	ln -sf $(CURDIR)/$(OUTDIR)/约定 $(HOME)/.local/bin/约定
	ln -sf $(CURDIR)/$(OUTDIR)/验 $(HOME)/.local/bin/验

验收: 安装
	验

clean:
	@echo "不删文件。要清二进制，自己移走 bin/"
