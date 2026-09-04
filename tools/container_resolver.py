#!/usr/bin/env python3
"""
五彩石容器解析器
AI 新会话启动时，扫描所有 @元数据头，构建容器地图，恢复上下文

输出格式：JSON 容器清单 + 恢复指令流
"""

import os
import re
import json
from pathlib import Path
from datetime import datetime

HUNYUAN_ROOT = Path('/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os')

# 匹配 Rust/Nim/Zig/C/JS/Python 中的 @metadata 行
META_PATTERN = re.compile(r'^(?://|#|/\*)\s*@(\w+):\s*(.+?)\s*$', re.MULTILINE)

# 注释前缀映射（反查文件类型）
EXT_PREFIX = {
    '.rs': '//',
    '.nim': '#',
    '.zig': '//',
    '.c': '//',
    '.h': '//',
    '.js': '//',
    '.py': '#',
}


def scan_containers():
    """扫描工作区中所有带有 @元数据的容器"""
    containers = []
    
    for ext in EXT_PREFIX.keys():
        for filepath in HUNYUAN_ROOT.rglob(f'*{ext}'):
            if '.git' in str(filepath):
                continue
            
            try:
                content = filepath.read_text(encoding='utf-8')
            except:
                continue
            
            # 只扫描前 20 行（元数据头通常在最前面）
            head = '\n'.join(content.split('\n')[:20])
            
            matches = META_PATTERN.findall(head)
            if not matches or 'name' not in [m[0] for m in matches]:
                continue
            
            metadata = dict(matches)
            metadata['_file'] = str(filepath.relative_to(HUNYUAN_ROOT))
            metadata['_type_ext'] = ext
            containers.append(metadata)
    
    return containers


def build_restore_plan(containers):
    """根据元数据构建恢复计划"""
    plan = {
        'session_init': [],
        'P0_restore': [],
        'P1_restore': [],
        'P2_restore': [],
        'symbolic_load': [],
    }
    
    for c in containers:
        priority = c.get('priority', 'P3')
        name = c.get('name', '?')
        desc = c.get('description', '')
        ctype = c.get('type', '?')
        filepath = c.get('_file', '')
        
        entry = {'name': name, 'desc': desc, 'file': filepath}
        
        if priority == 'P0':
            plan['P0_restore'].append(entry)
        elif priority == 'P1':
            plan['P1_restore'].append(entry)
        else:
            plan['P2_restore'].append(entry)
        
        # 根据 type 分类
        if 'core' in ctype or 'kernel' in ctype:
            plan['session_init'].append(entry)
        elif 'encoding' in ctype or 'symbol' in ctype:
            plan['symbolic_load'].append(entry)
    
    return plan


def output_json(containers, plan=None):
    """输出 JSON 格式"""
    result = {
        'generated': datetime.now().isoformat(),
        'total_containers': len(containers),
        'containers': containers,
    }
    if plan:
        result['restore_plan'] = plan
    return json.dumps(result, ensure_ascii=False, indent=2)


def output_text(containers, plan):
    """输出人类可读的文本恢复指南"""
    lines = [
        "=== 五彩石容器地图 ===",
        f"读取时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
        f"容器总数: {len(containers)}",
        "",
    ]
    
    # 按优先级分组输出
    for pname, pentries in plan.items():
        if not pentries:
            continue
        lines.append(f"■ {pname} ({len(pentries)} 容器)")
        for e in pentries:
            lines.append(f"  • {e['name']}: {e['desc']}")
            lines.append(f"    位置: {e['file']}")
        lines.append("")
    
    return '\n'.join(lines)


def main():
    import argparse
    parser = argparse.ArgumentParser(description='五彩石容器解析器')
    parser.add_argument('action', choices=['scan', 'plan', 'restore'],
                       help='scan=仅扫描, plan=扫描+恢复计划, restore=完整恢复指南')
    parser.add_argument('--format', '-f', choices=['text', 'json'], default='text')
    args = parser.parse_args()
    
    containers = scan_containers()
    
    if args.action == 'scan':
        if args.format == 'json':
            print(json.dumps(containers, ensure_ascii=False, indent=2))
        else:
            print(f"发现 {len(containers)} 个元数据容器:")
            for c in containers:
                print(f"  [{c.get('priority', '?')}] {c.get('name', '?')} - {c.get('description', '?')}")
        return
    
    if args.action in ('plan', 'restore'):
        plan = build_restore_plan(containers)
        if args.format == 'json':
            print(output_json(containers, plan))
        else:
            print(output_text(containers, plan))


if __name__ == '__main__':
    main()
