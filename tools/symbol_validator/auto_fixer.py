#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
符号自动修复器 · 满全法违禁词替换

读取符号验证器报告的违规项，将负向概念替换为历史符号。
支持交互模式和自动模式，修复前自动备份。

退出码：
  0 = 无修复（已是最新）
  1 = 有修复并已写入
  2 = 有冲突需人工裁决
"""

import sys
import os
import re
import json
import shutil
import argparse
from pathlib import Path
from datetime import datetime

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent
VALIDATOR_PATH = Path(__file__).resolve().parent / "symbol_validator.py"

# 完整替换映射（与 symbol_validator.py 同步）
REPLACEMENT_MAP = {
    # 中文
    "类": "公族",
    "对象": "容器",
    "继承": "承袭",
    "多态": "多化",
    "设计模式": "折叠范型",
    "文件": "卷",
    "目录": "庭录",
    "进程": "流",
    "线程": "丝",
    "模块": "构件",
    "包": "囊",
    "接口": "口岸",
    "实现": "成",
    "封装": "封",
    "函数": "操",
    "变量": "容位",
    "数组": "列",
    "字符串": "声母链",
    "整数": "整数(编码层)",
    "null": "空",
    "循环": "回",
    "递归": "叠",
    "遍历": "周巡",
    "并行": "并走",
    "并发": "同发",
    "调用": "投",
    "返回": "报",
    "参数": "传物",
    "属性": "极性",
    "方法": "诀",
    "字段": "位",
    "实例": "例",
    # ASCII
    "override": "覆写",
    "virtual": "虚设",
    "template": "范样",
    "generic": "通型",
    "singleton": "独例",
    "factory": "造坊",
    "observer": "观者",
    "MVC": "三垣",
    "MVVM": "四象",
    "REST": "静息",
    "API": "接口(外部)",
    "endpoint": "端点",
    "microservice": "微服",
    "monolith": "巨石",
    "database": "库",
    "server": "服务器",
    "client": "客户机",
}


def run_validator(directory):
    """运行符号验证器获取报告"""
    import subprocess
    result = subprocess.run(
        [sys.executable, str(VALIDATOR_PATH), "--dir", directory, "--json"],
        capture_output=True, text=True, cwd=str(WORKSPACE_ROOT)
    )
    if result.returncode in (0, 1, 2, 3):
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError:
            return None
    return None


def parse_violation(violation_str):
    """解析单条违规字符串 → (file, line, col, category, detail, suggestion)"""
    # 格式: path:line:col  [category] detail → 建议: suggestion
    m = re.match(r'^(.+?):(\d+):(\d+)\s+\[(.+?)\]\s+(.+?)(?:\s+→\s+建议:\s+(.+))?$', violation_str)
    if not m:
        return None
    return {
        "file": m.group(1),
        "line": int(m.group(2)),
        "col": int(m.group(3)),
        "category": m.group(4),
        "detail": m.group(5),
        "suggestion": m.group(6),
    }


def extract_forbidden_word(detail):
    """从详情中提取违禁词"""
    m = re.search(r'禁用词:\s*(.+)', detail)
    if m:
        return m.group(1).strip()
    m = re.search(r'禁用短语:\s*(.+)', detail)
    if m:
        return m.group(1).strip()
    return None


def fix_file(file_path, violations, dry_run=False):
    """修复单个文件"""
    p = Path(file_path)
    if not p.is_absolute():
        p = WORKSPACE_ROOT / file_path

    if not p.exists():
        return 0, 0

    try:
        lines = p.read_text(encoding='utf-8').split('\n')
    except (UnicodeDecodeError, PermissionError):
        return 0, 0

    fixed = 0
    conflicts = 0

    for v in violations:
        if v["file"] != file_path and not p.match(f"*/{v['file']}"):
            continue
        word = extract_forbidden_word(v["detail"])
        if not word or word not in REPLACEMENT_MAP:
            continue
        replacement = REPLACEMENT_MAP[word]
        line_idx = v["line"] - 1
        if line_idx >= len(lines):
            continue
        line = lines[line_idx]
        # 精确替换指定列
        col = v["col"] - 1
        actual = line[col:col+len(word)]
        if actual.lower() == word.lower():
            if not dry_run:
                lines[line_idx] = line[:col] + replacement + line[col+len(word):]
            fixed += 1
        else:
            # 列可能偏移，搜索整行
            if word in line:
                if not dry_run:
                    lines[line_idx] = line.replace(word, replacement, 1)
                fixed += 1
            else:
                conflicts += 1

    if not dry_run and fixed > 0:
        backup = p.with_suffix(p.suffix + f'.{datetime.now().strftime("%Y%m%d_%H%M%S")}.bak')
        shutil.copy2(p, backup)
        p.write_text('\n'.join(lines), encoding='utf-8')

    return fixed, conflicts


def main():
    parser = argparse.ArgumentParser(description="符号自动修复器 · 满全法违禁词替换")
    parser.add_argument('--dir', default=str(WORKSPACE_ROOT / "hunyuan-rs"), help="要修复的目录")
    parser.add_argument('--dry-run', action='store_true', help="只报告不修改")
    parser.add_argument('--file', help="修复单个文件")
    args = parser.parse_args()

    target = args.file or args.dir
    print(f"扫描: {target}")
    report = run_validator(target)

    if not report or "violations" not in report:
        print("⚠️  未获取到验证报告")
        sys.exit(2)

    violations = report.get("violations", [])
    if not violations:
        print("✅ 无需修复 · 无违禁符号")
        sys.exit(0)

    print(f"发现 {len(violations)} 处违规")
    if args.dry_run:
        print("（dry-run 模式，不写入修改）")

    total_fixed = 0
    total_conflicts = 0
    parsed = [parse_violation(v) for v in violations]
    parsed = [p for p in parsed if p]
    files = set(p["file"] for p in parsed)

    for f in files:
        f_fixed, f_conflicts = fix_file(f,
            [p for p in parsed if p["file"] == f],
            dry_run=args.dry_run)
        total_fixed += f_fixed
        total_conflicts += f_conflicts
        if f_fixed > 0:
            print(f"  {f}: 修复{f_fixed}处"+(" [dry-run]" if args.dry_run else ""))

    print(f"-" * 40)
    print(f"总计: 修复{total_fixed}处 · 冲突{total_conflicts}处")
    if total_conflicts > 0:
        sys.exit(2)
    if total_fixed > 0:
        sys.exit(1)
    sys.exit(0)


if __name__ == '__main__':
    main()
