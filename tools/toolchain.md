# 满全法工具链清单

## 就绪
- zig 0.14.0           /mnt/data/catpaw/home/.local/zig/zig
- Nim 2.2.2            /mnt/data/catpaw/home/local/nim/bin/nim
- Go 1.24.7            /usr/local/go/bin/go
- Rustc 1.98           $HOME/.cargo/bin/rustc
- Cargo 1.98           $HOME/.cargo/bin/cargo
- Python3 3.12         /usr/bin/python3
- Clang 18.1           /usr/bin/clang
- LuaJIT 2.1           /mnt/data/catpaw/home/.meituan-catpaw/9199532794/bin/luajit
- WASMTIME 33.0.2      /mnt/data/catpaw/home/.meituan-catpaw/9199532794/bin/wasmtime
- mesa swrast (llvmpipe)  OpenGL 4.5 软件渲染已通过 Xvfb 验证
- pygame + Xvfb       → 真 PNG 渲染管线已跑通

## 安装中
- nix                 官方 install 脚本下载中 (releases.nixos.org, 需 ~5min)

## 未就绪
- mlir                apt 阻塞 (需 root); 需从源码编译 LLVM
- forth/joy/nanolang/cangjie    GitHub 下载被限流或链接失效; Chinese mirrors 搜索中

## 镜像源记录
- GitHub proxy: ghproxy.net (可代理 release tarballs, 偶有限流)
- Gitee mirror: gitee.com/mirrors/ (LuaJIT 源码已成功)
- GitCode: gitcode.com (搜索不可用, 需已知 repo path)
- cnb.cool: 返回空白
- jihulab.com: 返回空白
- LuaJIT 官方: github.com/LuaJIT/LuaJIT (v2.1 branch)

## 注解
这些"外部语言"的最终归宿不是直接使用,而是取骨架去错误后按满全法规则重写。
下载回来的是可执行体, 参照物。二进制不是核心, 容器才是。
Cargo 已配置 USTC 镜像 (mirrors.ustc.edu.cn)
