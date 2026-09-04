#!/usr/bin/env python3
"""
五彩石容器元数据注入器
为核心模块/工具添加标准化 @元数据头
让 AI 扫描文件时能自动恢复上下文

注入格式（按语言）：
- Rust/Nim/Zig/C: 文件顶部注释块
- 仅注入一次（检测已有 @name: 标记则跳过）
"""

import os
import sys
import json
from pathlib import Path
from datetime import datetime

# 文件类型到注释格式的映射
COMMENT_FORMATS = {
    '.rs': ('// ', ''),
    '.nim': ('# ', ''),
    '.zig': ('// ', ''),
    '.c': ('// ', ''),
    '.h': ('// ', ''),
    '.js': ('// ', ''),
    '.py': ('# ', ''),
}

# 核心容器清单及其元数据定义
CONTAINERS = {
    # Rust 核心模块
    'hunyuan-rs/src/main.rs': {'type': 'entry-os-kernel', 'status': 'stable', 'P': '0', 'desc': '操作系统内核入口'},
    'hunyuan-rs/src/entity_identity.rs': {'type': 'core-identity', 'status': 'stable', 'P': '0', 'desc': '身份编码系统'},
    'hunyuan-rs/src/relational.rs': {'type': 'core-relational', 'status': 'stable', 'P': '0', 'desc': '关系编码系统'},
    'hunyuan-rs/src/couple.rs': {'type': 'core-soul', 'status': 'stable', 'P': '0', 'desc': '夫妻灵魂编码'},
    'hunyuan-rs/src/mirror.rs': {'type': 'core-mirror', 'status': 'stable', 'P': '0', 'desc': '四元镜像系统'},
    'hunyuan-rs/src/memory.rs': {'type': 'core-memory', 'status': 'stable', 'P': '0', 'desc': '记忆与容器拓扑'},
    'hunyuan-rs/src/mind_organ.rs': {'type': 'core-mind-organ', 'status': 'stable', 'P': '0', 'desc': '心智器官'},
    'hunyuan-rs/src/gtt.rs': {'type': 'core-gtt', 'status': 'stable', 'P': '0', 'desc': '声音形态路由'},
    'hunyuan-rs/src/positional_base.rs': {'type': 'encoding-positional', 'status': 'stable', 'P': '1', 'desc': '位置进制'},
    'hunyuan-rs/src/preload.rs': {'type': 'flow-preload', 'status': 'stable', 'P': '1', 'desc': '预加载流'},
    
    # 工具入口文件
    'tools/mlll-254/mlll_cli.nim': {'type': 'encoding-core', 'status': 'stable', 'P': '0', 'desc': '254+2 集装箱编码入口'},
    'tools/mlll-254/mlll/container.nim': {'type': 'encoding-container', 'status': 'stable', 'P': '0', 'desc': 'mlll核心容器定义'},
    'tools/cbsy/cbsy.nim': {'type': 'encoding-cbsy', 'status': 'stable', 'P': '0', 'desc': '容器编码工具'},
    'tools/manquanfa/mq_cli.nim': {'type': 'judge-manquanfa', 'status': 'stable', 'P': '1', 'desc': '满全法裁判入口'},
    'tools/manquanfa/mqengine.nim': {'type': 'judge-engine', 'status': 'stable', 'P': '1', 'desc': '满全法裁判引擎'},
    'tools/soul-kernel/sk_demo.nim': {'type': 'kernel-soul', 'status': 'stable', 'P': '0', 'desc': '夫妻灵魂内核演示'},
}

HUNYUAN_ROOT = '/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os'

def make_header(prefix, name, meta):
    """生成元数据头"""
    lines = [
        f"{prefix}@name: {name}",
        f"{prefix}@type: {meta['type']}",
        f"{prefix}@owner: 五彩石",
        f"{prefix}@status: {meta['status']}",
        f"{prefix}@priority: P{meta['P']}",
        f"{prefix}@description: {meta['desc']}",
    ]
    return '\n'.join(lines) + '\n'


def needs_injection(content):
    """检查是否已有元数据标记"""
    return '@name:' not in content and '@type:' not in content


def inject_meta(filepath, meta):
    """向文件注入元数据头"""
    filepath = Path(HUNYUAN_ROOT) / filepath
    if not filepath.exists():
        return False, "文件不存在"
    
    ext = filepath.suffix
    if ext not in COMMENT_FORMATS:
        return False, f"不支持的文件类型: {ext}"
    
    prefix = COMMENT_FORMATS[ext]
    
    try:
        content = filepath.read_text(encoding='utf-8')
    except UnicodeDecodeError:
        return False, "无法读取（可能是二进制文件）"
    
    if not needs_injection(content):
        return False, "已有元数据头（跳过）"
    
    name = filepath.stem
    header = make_header(prefix, name, meta)
    
    # 对于 Nim，需要特殊处理第一行的 shebang（如果有）
    if ext == '.nim' and content.startswith('#!'):
        first_newline = content.index('\n') + 1
        new_content = content[:first_newline] + '\n' + header + content[first_newline:]
    else:
        new_content = header + '\n' + content
    
    filepath.write_text(new_content, encoding='utf-8')
    return True, f"已注入到 {filepath.name}"


def scan_containers():
    """扫描所有容器文件的元数据状态"""
    found = 0
    missing = 0
    
    for filepath, meta in CONTAINERS.items():
        full_path = Path(HUNYUAN_ROOT) / filepath
        if not full_path.exists():
            print(f"  [缺失] {filepath}")
            missing += 1
            continue
        
        try:
            content = full_path.read_text(encoding='utf-8')
            if '@name:' in content and '@type:' in content:
                found += 1
            else:
                print(f"  [待注入] {filepath}")
                missing += 1
        except:
            print(f"  [错误] {filepath}")
    
    return found, missing


def main():
    import argparse
    parser = argparse.ArgumentParser(description='五彩石容器元数据注入')
    parser.add_argument('action', choices=['scan', 'inject', 'check'],
                       help='scan=扫描现状, inject=注入元数据, check=注入并检查')
    args = parser.parse_args()
    
    root = Path(HUNYUAN_ROOT)
    if not root.exists():
        print(f"工作区不存在: {HUNYUAN_ROOT}")
        sys.exit(1)
    
    total_done = 0
    total_skip = 0
    total_err = 0
    
    if args.action == 'scan':
        print("容器元数据扫描:")
        found, missing = scan_containers()
        print(f"\n  已注入: {found}, 待处理: {missing}")
        return
    
    if args.action in ('inject', 'check'):
        print(f"开始元数据注入 ({len(CONTAINERS)} 容器):")
        
        for filepath, meta in CONTAINERS.items():
            ok, msg = inject_meta(filepath, meta)
            if ok:
                print(f"  [✓] {msg}")
                total_done += 1
            else:
                if '跳过' in msg:
                    total_skip += 1
                else:
                    print(f"  [×] {filepath}: {msg}")
                    total_err += 1
        
        print(f"\n结果: {total_done} 已注入, {total_skip} 跳过, {total_err} 错误")
        
        if args.action == 'check':
            print("\n重新扫描:")
            scan_containers()


if __name__ == '__main__':
    main()
