# 开发环境记录

> 此文档记录 hunyuan-os 的完整开发环境配置，确保任何时刻可重建。

## 工具链清单

### Zig — 统帅/将军 (v0.13.0)

- **版本**: 0.13.0 (2024-06)
- **路径**: `/mnt/data/catpaw/home/.local/zig/zig-linux-x86_64-0.13.0/zig`
- **状态**: ✓ 稳定运行，唯一可用版本
- **用途**: 核心引擎主导语言，替代 C 成为主语言层
- **测试**: `zig test zig/src/root.zig` → 27/27 通过
- **注意**: 0.14.0 和 0.16.0 在此环境 segfault，不可用

### Nim — 兵工厂 (v2.2.0)

- **版本**: 2.2.0 (2024-10)
- **路径**: `/tmp/nim-2.2.0/bin/nim`
- **状态**: ✓ 编译通过，✗ 无法链接（无 gcc）
- **用途**: 代码生成器 (codegen.nim)
- **限制**: 仅 `--compileOnly` 和 `nim check` 可用
- **注意**: 位于 /tmp，重启后可能丢失

### C — 遗骸/引导

- **编译器**: zig cc (Zig 内置 C 编译器)
- **用途**: WASM 双目标编译 (wasm32 + wasm64)
- **脚本**: `zig-stem/build.sh`

### Rust — 护卫

- **版本**: 1.75+
- **路径**: `hunyuan-rs/`
- **状态**: 已有 24,500 行基础代码
- **用途**: 内存安全保证层

### Python — 辅助

- **版本**: 3.12
- **用途**: 辅助脚本、PDF/文档处理

## 运行环境

| 组件 | 版本 | 状态 |
|------|------|------|
| Linux 内核 | 5.4.241 | ✓ |
| 容器 | catpaw sandbox | ✓ |
| gcc | 缺失 | ✗ |
| make | 缺失 | ✗ |
| git | 2.x | ✓ |
| curl | 可用 | ✓ |

## 远端仓库

| 名称 | URL | 状态 |
|------|-----|------|
| origin | github.com:lolict/hunyuan-os | ✓ 主仓库 |
| Kndaili | github.com/lolict/Kndaili | ✓ |
| Kndarl | github.com/lolict/Kndarl | ✓ |
| hunyuan-universe | github.com/lolict/hunyuan-universe | ✓ |
| manquanfa-dict | github.com/lolict/manquanfa-dict | ✓ |
| manquanfa-encoding | github.com/lolict/manquanfa-encoding | ✓ |
| sacred-covenant | github.com/lolict/sacred-covenant | ✓ |
| yuan-ji-jiu-can | github.com/lolict/yuan-ji-jiu-can | ✓ |
| gitee | gitee.com:lolict/yuan-ji-jiu-can | ✓ |
| gitee-acd | gitee.com:lolict/arthur-conan-doyle | ✓ |
| gitee-ken | gitee.com:lolict/kenandaoer | ✓ |
| gitcode | gitcode.com:lolict/kndoyle-gtxt | ✓ |

## 重建步骤

```bash
# 1. 安装 Zig 0.13.0
curl -sL https://ziglang.org/download/0.13.0/zig-x86_64-linux-0.13.0.tar.xz | tar -xJ
export PATH=$PATH:$(pwd)/zig-x86_64-linux-0.13.0

# 2. 安装 Nim 2.2.0
curl -sL https://nim-lang.org/download/nim-2.2.0-linux_x64.tar.xz | tar -xJ

# 3. 克隆仓库
git clone git@github.com:lolict/hunyuan-os.git

# 4. 运行测试
cd hunyuan-os && zig test zig/src/root.zig
```

## 已知问题

1. **Zig 0.14.0+ segfault**: 此容器环境不兼容，只能用 0.13.0
2. **无 gcc**: Nim 无法链接为可执行文件，只能 --compileOnly
3. **Nim 在 /tmp**: 重启后需重新下载
4. **下载速度慢**: ziglang.org 约 1MB/min，需耐心等待
