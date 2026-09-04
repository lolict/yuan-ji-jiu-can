#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
元输入法工具链 · 构建脚本

将所有 Zig 模块编译为独立二进制。运行时零常驻。

用法:
    python3 build.py              # 构建 metaime 二进制
    python3 build.py --test       # 运行所有 Zig 单元测试
    python3 build.py --run        # 构建并运行
    python3 build.py --clean      # 清理构建产物
"""

import os
import sys
import shutil
import subprocess


def find_zig():
    """查找 Zig 编译器 (pip 内置或系统路径)"""
    # 1. pip 内置
    pip_zig = os.path.expanduser(
        "~/.local/lib/python3.*/site-packages/ziglang/zig"
    )
    import glob
    matches = glob.glob(pip_zig)
    if matches:
        return matches[0]
    # 2. 系统路径
    zig = shutil.which("zig")
    if zig:
        return zig
    # 3. 本地解压
    local = glob.glob("/tmp/zig-x86*/zig")
    if local:
        return local[0]
    return ""


def build(zig, src_dir, out_bin):
    """编译 Zig 为独立二进制"""
    main_zig = os.path.join(src_dir, "src", "main.zig")
    cmd = [zig, "build-exe", main_zig, "-OReleaseFast", f"-femit-bin={out_bin}"]
    print(f"[build] {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=src_dir)
    return result.returncode == 0


def run_tests(zig, src_dir):
    """运行所有 Zig 单元测试"""
    main_zig = os.path.join(src_dir, "src", "main.zig")
    cmd = [zig, "test", main_zig]
    print(f"[build] {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=src_dir)
    return result.returncode == 0


def clean(out_dir):
    """清理构建产物"""
    if os.path.exists(out_dir):
        shutil.rmtree(out_dir)
        print(f"[build] 已清理: {out_dir}")


def main():
    project_dir = os.path.dirname(os.path.abspath(__file__))
    src_dir = os.path.join(project_dir, "zig")
    out_dir = os.path.join(project_dir, "zig-out")
    out_bin = os.path.join(out_dir, "metaime")

    if len(sys.argv) > 1 and sys.argv[1] == "--clean":
        clean(out_dir)
        return

    zig = find_zig()
    if not zig:
        print("[build] Zig 未找到。请 pip install ziglang")
        sys.exit(1)

    result = subprocess.run([zig, "version"], capture_output=True, text=True)
    print(f"[build] Zig: {result.stdout.strip()}")

    if len(sys.argv) > 1 and sys.argv[1] == "--test":
        success = run_tests(zig, src_dir)
    elif len(sys.argv) > 1 and sys.argv[1] == "--run":
        os.makedirs(out_dir, exist_ok=True)
        success = build(zig, src_dir, out_bin)
        if success:
            print(f"\n[build] 运行: {out_bin}")
            subprocess.run([out_bin])
    else:
        os.makedirs(out_dir, exist_ok=True)
        success = build(zig, src_dir, out_bin)
        if success:
            size = os.path.getsize(out_bin)
            print(f"[build] 二进制: {out_bin} ({size/1024:.1f}KB)")

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
