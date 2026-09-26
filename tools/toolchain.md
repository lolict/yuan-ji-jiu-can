# 满全法工具链清单

## 就绪 (已安装)
- zig 0.14.0           /mnt/data/catpaw/home/.local/zig/zig
- Nim 2.2.2            /mnt/data/catpaw/home/.local/nim/bin/nim
- Go 1.24.7            /usr/local/go/bin/go
- Rustc 1.98           $HOME/.cargo/bin/rustc
- Cargo 1.98           $HOME/.cargo/bin/cargo
- Python3 3.12         /usr/bin/python3
- Clang 18.1           /usr/bin/clang
- LuaJIT 2.1           /mnt/data/catpaw/home/.meituan-catpaw/9199532794/bin/luajit
- WASMTIME 33.0.2      /mnt/data/catpaw/home/.meituan-catpaw/9199532794/bin/wasmtime
- mesa swrast (llvmpipe)  OpenGL 4.5 软件渲染已通过 Xvfb 验证
- pygame + Xvfb       → 真 PNG 渲染管线已跑通

## 参考源 (ref-languages/)
- joy/                 rootmos/silly-joy (concatenative language, Haskell实现, 46 files)
- forth/               fuzzballcat/milliForth (340字节 FORTH in boot sector, 38 files)
- nanolang-main/       jordanhubbard/nanolang (629★ 编译语言框架, 174 files)
- cangjie/             cangjielanguage/cangjie_compiler (华为仓颉编译器, 56 files)
- nim-atomgit/         Nim 编译器 (从 AtomGit 镜像, 4329 files)

## 未就绪
- nix                  determinate installer 下载失败(需交互); 官方 install 脚本卡在 13MB/25MB
- mlir                apt 阻塞 (需 root); 需从源码编译 LLVM (~750MB)
- glsl/hlsl/vulkan   shader 验证器、编译器未获取

## 中国镜像测试结果
- gitcode.com          → 重定向到 atomgit.com (AtomGit 是 Codeberg 的中国镜像, Nim 已成功)
- gitee.com/mirrors/   → LuaJIT 镜像成功, cangjie 不存在
- cnb.cool            → 全部仓库返回 404
- jihulab.com         → 401 需认证
- (合法镜像) ghproxy.net → 代理 GitHub release tarballs, 成功率约 50%

## 注解
这些"外部语言"的最终归宿不是直接使用,而是取骨架去错误后按满全法规则重写。
下载回来的是可执行体, 参照物。二进制不是核心, 容器才是。
Cargo 已配置 USTC 镜像 (mirrors.ustc.edu.cn)
