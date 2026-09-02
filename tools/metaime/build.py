#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
元输入法工具链 · 构建脚本

将 Zig 编译为独立二进制。无 Zig 环境时发出指南。

用法:
    python3 build.py              # 构建全部
    python3 build.py --test       # 运行 Zig 单元测试
    python3 build.py --clean      # 清理构建产物
    python3 build.py --check      # 检查 Zig 可用性
"""

import os
import sys
import shutil
import subprocess
import urllib.request
import tarfile
import tempfile

ZIG_VERSION = "0.14.0"
ZIG_URL_TEMPLATE = "https://ziglang.org/download/{version}/zig-x86_64-linux-{version}.tar.xz"


def find_zig() -> str:
    """查找 Zig 编译器"""
    zig = shutil.which("zig")
    if zig:
        return zig
    # 检查本地缓存
    local_paths = [
        "/tmp/zig-x86-*/zig",
        os.path.expanduser("~/.local/bin/zig"),
        "./zig/bin/zig",
    ]
    for p in local_paths:
        import glob
        matches = glob.glob(p)
        if matches:
            return matches[0]
    return ""


def install_zig() -> str:
    """下载并安装 Zig 到本地缓存"""
    print("[build] Zig 未找到，下载 {}...".format(ZIG_VERSION))

    url = ZIG_URL_TEMPLATE.format(version=ZIG_VERSION)
    tar_path = tempfile.mktemp(suffix=".tar.xz")

    def progress_hook(block_num, block_size, total_size):
        downloaded = block_num * block_size
        if total_size > 0:
            pct = min(100, downloaded * 100 / total_size)
            mb = downloaded / 1024 / 1024
            print(f"\r[build] {pct:.1f}% ({mb:.1f}MB / {total_size/1024/1024:.1f}MB)", end="", flush=True)

    urllib.request.urlretrieve(url, tar_path, reporthook=progress_hook)
    print()

    print("[build] 解压...")
    with tarfile.open(tar_path) as tar:
        tar.extractall(path="/tmp/")

    zig_dir = f"/tmp/zig-x86_64-linux-{ZIG_VERSION}"
    zig_bin = os.path.join(zig_dir, "zig")
    os.chmod(zig_bin, 0o755)

    os.remove(tar_path)
    print(f"[build] Zig 已安装: {zig_bin}")
    return zig_bin


def build(zig: str, project_root: str) -> bool:
    """编译所有 Zig 工具"""
    zig_src = os.path.join(project_root, "zig")
    build_dir = os.path.join(project_root, "zig-out")

    if not os.path.isdir(zig_src):
        print(f"[build] 错误: 找不到 zig/ 目录: {zig_src}")
        return False

    print(f"[build] 使用 Zig: {zig}")
    print(f"[build] 源码目录: {zig_src}")
    print(f"[build] 输出目录: {build_dir}")

    # 编译统一入口
    main_zig = os.path.join(zig_src, "src", "main.zig")
    out_bin = os.path.join(build_dir, "metaime")

    os.makedirs(build_dir, exist_ok=True)

    cmd = [zig, "build-exe", main_zig, "-OReleaseFast", "-fstrip", "-fsingle-threaded", "-o", out_bin]
    print(f"[build] {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=zig_src)

    if result.returncode != 0:
        print("[build] 编译失败!")
        return False

    size = os.path.getsize(out_bin)
    print(f"[build] 成功: {out_bin} ({size/1024:.1f}KB)")
    return True


def build_dll(zig: str, project_root: str) -> bool:
    """编译为动态库（给 C/Rust/Nim 调用）"""
    zig_src = os.path.join(project_root, "ffi_bridge.zig")
    build_dir = os.path.join(project_root, "zig-out")

    if not os.path.isfile(zig_src):
        print(f"[build] FFI 桥接未找到，跳过 DLL 构建")
        return True

    out_lib = os.path.join(build_dir, "libmetaime_zig.so")
    os.makedirs(build_dir, exist_ok=True)

    cmd = [zig, "build-lib", zig_src, "-dynamic", "-OReleaseFast", "-fstrip", "-o", out_lib]
    print(f"[build] {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=project_root)

    if result.returncode != 0:
        print("[build] DLL 编译失败!")
        return False

    size = os.path.getsize(out_lib)
    print(f"[build] 成功: {out_lib} ({size/1024:.1f}KB)")
    return True


def run_tests(zig: str, project_root: str) -> bool:
    """运行 Zig 单元测试"""
    zig_src = os.path.join(project_root, "zig")
    main_zig = os.path.join(zig_src, "src", "main.zig")

    cmd = [zig, "test", main_zig]
    print(f"[build] Zig 测试: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=zig_src)
    return result.returncode == 0


def clean(project_root: str):
    """清理构建产物"""
    build_dir = os.path.join(project_root, "zig-out")
    if os.path.exists(build_dir):
        shutil.rmtree(build_dir)
        print(f"[build] 已清理: {build_dir}")
    else:
        print("[build] 无产物可清理")


def main():
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "--clean":
            clean(project_root)
            return
        if cmd == "--check":
            zig = find_zig()
            if zig:
                result = subprocess.run([zig, "version"], capture_output=True, text=True)
                print(f"[build] Zig 可用: {zig} (版本 {result.stdout.strip()})")
            else:
                print("[build] Zig 不可用。运行 'python3 build.py' 以下载安装。")
            return

    zig = find_zig()
    if not zig:
        zig = install_zig()

    if not zig:
        print("[build] Zig 无法获取。请手动安装: https://ziglang.org/download/")
        sys.exit(1)

    # 验证
    result = subprocess.run([zig, "version"], capture_output=True, text=True)
    print(f"[build] Zig 版本: {result.stdout.strip()}")

    success = True
    if len(sys.argv) > 1 and sys.argv[1] == "--test":
        success = run_tests(zig, project_root)
    else:
        success = build(zig, project_root)
        if success:
            build_dll(zig, project_root)

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
