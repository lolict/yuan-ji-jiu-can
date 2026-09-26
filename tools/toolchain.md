# 满全法工具链清单

## 就绪
- zig 0.14.0           /mnt/data/catpaw/home/.local/zig/zig
- nim 2.2.2            /mnt/data/catpaw/home/.local/nim/bin/nim
- go 1.24.7            /usr/local/go/bin/go
- rustc 1.98           $HOME/.cargo/bin/rustc
- cargo 1.98           $HOME/.cargo/bin/cargo
- python3 3.12         /usr/bin/python3
- clang 18.1           /usr/bin/clang
- mesa swrast (llvmpipe)  OpenGL 4.5 软件渲染已通过 Xvfb 验证
- pygame + Xvfb       → 真 PNG 渲染管线已跑通

## 未就绪
- nix                 安装中 (官方 install 脚本已下载)

- wasmtime             cargo 编译需 ~30 分钟, 超时
- mlir                apt 阻塞 (需 root); 需从源码编译 LuaJIT
- forth/joy/nanolang/cangjie    GitHub 下载被限流或链接失效

## 注解
这些"外部语言"的最终归宿不是直接使用,而是取骨架去错误后按满全法规则重写。
下载回来的是可执行体, 参照物。
二进制不是核心, 容器才是。
