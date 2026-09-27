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
- glslang 16.6.0       /mnt/data/catpaw/home/.meituan-catpaw/9199532794/bin/glslang (SPIR-V生成器+验证器, --spirv-val 内建)
- glslang libraries     /mnt/data/catpaw/home/.meituan-catpaw/9199532794/lib/glslang/ (libglslang.a, libSPIRV-Tools.a, libSPIRV-Tools-opt.a, libSPIRV.a)
- mesa swrast (llvmpipe)  OpenGL 4.5 软件渲染已通过 Xvfb 验证
- pygame + Xvfb       → 真 PNG 渲染管线已跑通

## 内置/编译能力
- SPIR-V generation   `glslang --stdin -S <stage> -V -o out.spv` → 成功生成 SPIR-V 字节码
- SPIR-V validation    `glslang --spirv-val` (内建 SPIRV-Tools, 静态链接)
- SPIR-V disassembly   `glslang --spirv-dis` → 可读反汇编输出
- OpenGL 渲染 (无 GPU)  mesa llvmpipe via Xvfb → pygame/SDL2 渲染 PNG

## 参考源 (ref-languages/)
- joy/                 rootmos/silly-joy (concatenative language, Haskell实现, 46 files)
- forth/               fuzzballcat/milliForth (340字节 FORTH in boot sector, 38 files)
- nanolang-main/       jordanhubbard/nanolang (629★ 编译语言框架, 174 files)
- cangjie/             cangjielanguage/cangjie_compiler (华为仓颉编译器, 56 files)

## 未就绪
- nix                  blocked: 无 root, /nix store 路径硬编码 + 用户命名空间不可用 (unshare -rm 无法 mkdir /nix)
- mlir                 blocked: LLVM ~750MB 源码包 + apt 需 root (dpkg lock-frontend)
- spirv-tools          独立包下载失败 (main-tot tag 已移除; v2026.x 无 binary assets) — 功能被 glslang 内建覆盖

## 中国镜像测试结果
- gitcode.com          → 重定向到 atomgit.com (AtomGit 是 Codeberg 的中国镜像, Nim 已成功)
- gitee.com/mirrors/   → LuaJIT 镜像成功, cangjie 不存在
- cnb.cool            → 全部仓库返回 404
- jihulab.com         → 401 需认证
- ghproxy.net         → 代理 GitHub release tarballs, <10MB 成功率约 50% (大文件 >20MB 易超时)

## 注解
这些"外部语言"的最终归宿不是直接使用,而是取骨架去错误后按满全法规则重写。
下载回来的是可执行体, 参照物。二进制不是核心, 容器才是。
Cargo 已配置 USTC 镜像 (mirrors.ustc.edu.cn)
glslang 自带的 libSPIRV-Tools.a 静态链接让满全法容器可以编译时直接内建 SPIR-V 验证能力。
