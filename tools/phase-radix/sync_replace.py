#!/usr/bin/env python3
"""
sync_replace.py — 全量空间同步替换

跨文件、跨类型、一次性遍历全部文本文件，
按替换规则表同步替换词汇，类似于"乘法"而非"加法":
不是逐个文件手动改 · 而是声明规则后全空间生效。

用法:
  python3 sync_replace.py --rules rules.json --dry-run
  python3 sync_replace.py --rules rules.json --apply

规则文件格式:
{
  "天觉": "天庭",
  "天觉空间": "天庭空间",
  "TianJue": "HeavenlyCourt"
}

规则按 key 长度降序匹配 · 避免短词干扰长词。
"""

import os
import sys
import json
import re
import hashlib
from pathlib import Path
from datetime import datetime

# ── 默认排除 ──────────────────────────────────────────────
SKIP_DIRS = {
    '.git', '__pycache__', 'node_modules', '.venv', 'venv',
    '.catpaw', '.meituan-catpaw', 'target',  # Rust 编译产物
    'build', 'dist', '.gradle', '.idea',
}
SKIP_EXTS = {
    '.pyc', '.pyo', '.so', '.o', '.bin', '.wasm',
    '.png', '.jpg', '.ico', '.woff', '.ttf', '.eot',
    '.rlib', '.rmeta', '.d',  # Rust 编译产物
    '.exe', '.dll', '.dylib',
    '.zip', '.gz', '.tar', '.rar', '.7z',
    '.pdf', '.doc', '.docx', '.xls', '.xlsx',
}
# 这些文件不应被自己的规则修改
SELF_EXCLUDE = {'sync_replace.py', 'rules_tianjue_to_tianting.json'}

# ── 核心: 全量同步替换引擎 ──────────────────────────────

class SyncReplacer:
    """全量空间同步替换器 · 跨文件一次性乘法"""
    
    def __init__(self, rules: dict, workspace: str):
        # 按 key 长度降序 · 长词优先匹配 · 避免碎片化
        self.rules = dict(sorted(rules.items(), key=lambda x: -len(x[0])))
        self.workspace = Path(workspace)
        self.stats = {"scanned": 0, "modified": 0, "replacements": 0, "files": []}
    
    def scan_files(self):
        """遍历所有文本文件"""
        for root, dirs, files in os.walk(self.workspace):
            # 跳过无关目录
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
            for fname in files:
                if fname in SELF_EXCLUDE:
                    continue
                ext = Path(fname).suffix.lower()
                if ext in SKIP_EXTS:
                    continue
                yield Path(root) / fname
    
    def process_file(self, filepath: Path, dry_run: bool = True) -> dict:
        """处理单个文件 · 全规则一次性应用"""
        try:
            # 先读前 1024 字节检测二进制
            with open(filepath, 'rb') as f:
                chunk = f.read(1024)
            if b'\x00' in chunk:
                return None  # 二进制文件跳过
            text = filepath.read_text(encoding='utf-8', errors='replace')
        except Exception:
            return None
        
        original = text
        file_replacements = 0
        detail = []
        
        # 乘法: 所有规则一次性应用 · 不是循环查找逐项替换
        for old, new in self.rules.items():
            if old in text:
                count = text.count(old)
                text = text.replace(old, new)
                file_replacements += count
                detail.append(f'  "{old}" → "{new}" × {count}')
        
        result = {
            "path": str(filepath),
            "modified": text != original,
            "replacements": file_replacements,
            "detail": detail,
        }
        
        if text != original and not dry_run:
            # 写回文件
            filepath.write_text(text, encoding='utf-8')
            result["written"] = True
        
        return result
    
    def execute(self, dry_run: bool = True) -> dict:
        """执行全量替换 · 返回统计"""
        self.stats = {"scanned": 0, "modified": 0, "replacements": 0, "files": []}
        
        for fpath in self.scan_files():
            self.stats["scanned"] += 1
            result = self.process_file(fpath, dry_run)
            if result and result["modified"]:
                self.stats["modified"] += 1
                self.stats["replacements"] += result["replacements"]
                self.stats["files"].append(result)
        
        return self.stats
    
    def report(self) -> str:
        s = self.stats
        lines = [
            f"═══ 全量空间同步替换报告 ═══",
            f"扫描文件: {s['scanned']}",
            f"修改文件: {s['modified']}",
            f"替换次数: {s['replacements']}",
            f"═══ 文件清单 ═══",
        ]
        for f in s["files"][:50]:
            lines.append(f"  [{f['replacements']:3d}次] {f['path']}")
            for d in f.get("detail", [])[:5]:
                lines.append(d)
        if len(s["files"]) > 50:
            lines.append(f"  ... 还有 {len(s['files']) - 50} 个文件")
        return "\n".join(lines)


# ── CLI ──────────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser(description="全量空间同步替换")
    parser.add_argument("--rules", required=True, help="规则 JSON 文件路径")
    parser.add_argument("--workspace", default=".", help="工作目录")
    parser.add_argument("--apply", action="store_true", help="实际写入 (默认 dry-run)")
    parser.add_argument("--report-file", help="输出报告到文件")
    args = parser.parse_args()
    
    # 加载规则
    rules_path = Path(args.rules)
    if not rules_path.exists():
        print(f"规则文件不存在: {rules_path}")
        sys.exit(1)
    
    with open(rules_path, 'r', encoding='utf-8') as f:
        rules = json.load(f)
    
    print(f"加载 {len(rules)} 条规则")
    print(f"工作目录: {args.workspace}")
    print(f"模式: {'写入' if args.apply else '预览 (dry-run)'}")
    print()
    
    replacer = SyncReplacer(rules, args.workspace)
    replacer.execute(dry_run=not args.apply)
    report = replacer.report()
    print(report)
    
    if args.report_file:
        Path(args.report_file).write_text(report, encoding='utf-8')
        print(f"\n报告已保存: {args.report_file}")


if __name__ == "__main__":
    main()
