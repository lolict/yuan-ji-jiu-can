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

## 参考骨架 (已提取但未运行)
- nix 2.10.3           $HOME/nix-2.10.3/ (45 entries, 编译器 GCC 11.3, glibc 2.34)
- nix 2.35.2           $HOME/nix-2.35.2-backup/ (65 entries, 编译器 GCC 15.2, glibc 2.42)
  - 两个版本均已 patchelf patch: INTERP→/lib64/ld-linux-x86-64.so.2, RUNPATH→本地 store
  - 全部 NEEDED 路径已修正 (libnixstore: /nix/store/.../libsqlite3.so → libsqlite3.so)
  - 所有 .so 的 GLIBC 版本需求 ≤ 2.38, GLIBCXX ≤ 3.4.32, 系统均满足
  - 所有库的 ELF binding/relocation 已完成 (ldd --list 验证 0 not found)
  - 仍然 segfault: CRT init 阶段 (system ld-linux + nix ld-linux-2.42 均同)
  - 根因: 容器环境 (kernel 5.4.241 tlinux, Ubuntu 24.04 user态) 与 ELF TLS/rseq 实现的兼容问题
  - 保留为参考骨架; 不需特权 — 二进制本身已全修补可用, 仍被环境 SIGSEGV
- patchelf 0.18.0       /tmp/patchelf/patchelf-0.18.0/patchelf (从源码编译, clang++ -std=c++17)
- nix 2.35.2 源码      /tmp/nix-src/ (git clone --branch 2.35.2, 用于研究骨架结构)
- mlir 18.1.3           $HOME/mlir-18.1.3/ (apt-get download: libmlir-18-dev, libmlir-18, llvm-18-dev, llvm-18-tools)
  - 来源: Ubuntu 24.04 noble-updates apt 镜像 (mirrors.cloud.tencent.com), apt-get download 无需 root
  - 8 .so: libMLIR.so.18.1 (主库), libmlir_async_runtime, libmlir_c_runner_utils, libmlir_float16_utils,
    libmlir_arm_runner_utils, libmlir_arm_sme_abi_stubs, libmlir_runner_utils, libRemarks.so.18.1
  - 696 个 MLIR 头文件 (IR/Dialects/Transforms/Passes/Conversion/Tools/ExecutionEngine)
  - ldd 全通过: 系统已有 libLLVM.so.18.1 (libllvm-18 预装); libstdc++/libz/libzstd/libxml2/libedit/libffi 均满足
  - RUNPATH=$ORIGIN/../lib, 所有 MLIR .so 内部互引通过 soname symlink 自洽
  - 保留为参考骨架: 研读 MLIR API/IR/Dialect 结构服务满全法容器处理器的改造目标

## 未就绪
- spirv-tools          独立包下载失败 (main-tot tag 已移除; v2026.x 无 binary assets) — 功能被 glslang 内建覆盖

## 中国镜像测试结果
- gitcode.com          → 重定向到 atomgit.com (AtomGit 是 Codeberg 的中国镜像, Nim 已成功)
- gitee.com/mirrors/   → LuaJIT 镜像成功, cangjie 不存在
- cnb.cool            → 全部仓库返回 404
- jihulab.com         → 401 需认证
- ghproxy.net         → 代理 GitHub release tarballs, <10MB 成功率约 50% (大文件 >20MB 易超时)
- mirrors.cloud.tencent.com/ubuntu → Ubuntu 官方镜像, apt-get download 无需 root 可直接下 .deb (MLIR 即此路径)

## 注解
这些"外部语言"的最终归宿不是直接使用,而是取骨架去错误后按满全法规则重写。
下载回来的是可执行体, 参照物。二进制不是核心, 容器才是。
Cargo 已配置 USTC 镜像 (mirrors.ustc.edu.cn)
glslang 自带的 libSPIRV-Tools.a 静态链接让满全法容器可以编译时直接内建 SPIR-V 验证能力。
