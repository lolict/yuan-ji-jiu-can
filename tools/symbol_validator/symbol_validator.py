#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
符号验证器 · 满全法容器处理器合规检查

用途：
  1. 作为 pre-commit hook 拦截违禁输出
  2. 批量扫描已有文件做健康报告
  3. 会话启动时作为"符号快照"分发依据

检查项：
  A. 负向概念禁用词（GLOBAL_PROMPT_V2.md 第五层）
  B. 禁止行为短语
  C. 历史符号接入检查

退出码：
  0 = 全部通过
  1 = 发现违禁符号
  2 = 发现禁止行为短语
  3 = 两者同时违规
"""

import sys
import os
import re
import json
import argparse
from pathlib import Path
from datetime import datetime

# ============================================================
# 数据层：从全局文件同步的禁用/可用范畴
# ============================================================

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent.parent

# 负向概念完整列表（与 GLOBAL_PROMPT_V2.md / GLOBAL_CHAIN_MASTER_LOCK_V2.md / MEMORY.md 同步）
FORBIDDEN_CONCEPTS = [
    "类", "对象", "继承", "多态", "设计模式",
    "文件", "目录", "进程", "线程", "模块",
    "包", "接口", "实现", "封装",
    "override", "virtual", "template", "generic",
    "singleton", "factory", "observer",
    "MVC", "MVVM", "REST", "API",
    "endpoint", "microservice", "monolith",
    "函数", "变量", "数组", "字符串", "整数",
    "null", "循环", "递归", "遍历",
    "并行", "并发", "调用", "返回",
    "参数", "属性", "方法", "字段", "实例",
]

# 禁止行为短语
FORBIDDEN_PHRASES = [
    "面向对象", "把X放进Y", "调用函数", "数据类型",
    "算法", "数据库", "编译后锁定", "静态映射表",
    "动态位置计算",
]

# 历史符号白名单（六族 · 必须纳入可用范畴）
HISTORICAL_SYMBOLS = {
    "容器族": [
        "袋", "桶", "罐", "缸", "盆", "盂", "钵", "盏", "盒", "杯",
        "壶", "瓶", "篓", "筐", "篮", "箩", "筲", "箕", "笼", "橱",
        "柜", "箱", "匣", "奁", "瓮", "甏", "坛", "罂", "甑", "甗",
        "瓯", "盌", "盉", "盅",
    ],
    "操作族": [
        "照", "昭", "罩", "召", "兆", "见", "鉴", "检", "简", "剪",
        "加", "减", "进", "退", "开", "关", "通", "断", "分", "合",
        "聚", "散", "折", "叠", "卷", "舒",
    ],
    "极性族": [
        "阴", "阳", "盈", "亏", "虚", "实", "表", "里", "公", "私",
        "升", "降", "清", "浊", "寒", "热", "动", "静", "出", "入",
    ],
    "层级族": [
        "元", "阶", "层", "代", "世", "界", "域", "场",
        "馆", "阁", "门", "户", "庭", "院", "宫", "殿",
        "台", "榭", "楼",
    ],
    "审判族": [
        "裁", "判", "审", "议", "决", "断", "定", "评", "核",
        "功", "过", "赏", "罚", "褒", "贬", "陟", "黜",
    ],
    "产权族": [
        "产", "业", "权", "利", "债", "责", "职", "务", "名", "分",
        "归", "属", "占", "有", "持", "拥", "享", "获", "得", "失",
    ],
}

# 平铺
ALL_HISTORICAL = set()
for family in HISTORICAL_SYMBOLS.values():
    ALL_HISTORICAL.update(family)


# ============================================================
# 违规记录
# ============================================================

class SymbolViolation:
    def __init__(self, file_path, line_num, col, category, detail, suggestion=None):
        self.file_path = file_path
        self.line_num = line_num
        self.col = col
        self.category = category
        self.detail = detail
        self.suggestion = suggestion

    def __str__(self):
        loc = f"{self.file_path}:{self.line_num}:{self.col}"
        msg = f"[{self.category}] {self.detail}"
        if self.suggestion:
            msg += f" → 建议: {self.suggestion}"
        return f"{loc}  {msg}"


# ============================================================
# 验证器主类
# ============================================================

class SymbolValidator:
    def __init__(self, workspace_root=None):
        self.workspace_root = Path(workspace_root) if workspace_root else WORKSPACE_ROOT
        self.violations = []
        self.stats = {"scanned": 0, "lines": 0, "forbidden": 0, "phrases": 0}

    def scan_file(self, file_path):
        path = Path(file_path)
        if not path.exists():
            return
        if path.suffix not in ('.md', '.rs', '.c', '.h', '.py', '.nim', '.zig', '.js', '.ts', '.json', '.txt', '.toml'):
            return
        try:
            content = path.read_text(encoding='utf-8')
        except (UnicodeDecodeError, PermissionError):
            return

        lines = content.split('\n')
        for i, line in enumerate(lines, start=1):
            self.stats["lines"] += 1
            stripped = line.strip()
            if not stripped or stripped.startswith('#') or stripped.startswith('//'):
                continue
            self._scan_line(path, i, line)
        self.stats["scanned"] += 1

    def _scan_line(self, file_path, line_num, line):
        # 检查A：负向概念
        for concept in FORBIDDEN_CONCEPTS:
            if concept.isascii():
                pattern = r'\b' + re.escape(concept) + r'\b'
                for m in re.finditer(pattern, line, re.IGNORECASE):
                    self._add_violation(file_path, line_num, m.start()+1,
                        "负向概念", f"禁用词: {concept}",
                        self._get_replacement(concept))
                    self.stats["forbidden"] += 1
            else:
                idx = line.find(concept)
                while idx != -1:
                    self._add_violation(file_path, line_num, idx+1,
                        "负向概念", f"禁用词: {concept}",
                        self._get_replacement(concept))
                    self.stats["forbidden"] += 1
                    idx = line.find(concept, idx+1)

        # 检查B：禁止行为短语
        for phrase in FORBIDDEN_PHRASES:
            idx = line.find(phrase)
            while idx != -1:
                self._add_violation(file_path, line_num, idx+1, "禁止行为", f"禁用短语: {phrase}")
                self.stats["phrases"] += 1
                idx = line.find(phrase, idx+1)

    def _get_replacement(self, concept):
        mapping = {
            "类": "家族/公倍数", "对象": "容器", "继承": "公约数复制",
            "封装": "产权封装/容器包", "文件": "容器/袋", "目录": "层级/庭/院",
            "进程": "流年/时间阶", "模块": "容器/产权槽", "接口": "边界/口岸",
            "调用": "投/照/召", "函数": "操作/照/鉴", "变量": "容器位/极性位",
            "数组": "容器队列/筲/笼", "字符串": "声母链/韵母链", "循环": "回/周/轮",
            "递归": "折叠/自同", "参数": "递入物", "属性": "极性/产权标记",
            "方法": "操作方式/照/鉴", "实例": "公倍数投影", "null": "空/虚/无",
            "返回": "回/返/报", "平行": "并/同阶", "并发": "同时间多流转",
            "实现": "成/就/立", "遍历": "周遍/巡", "多态": "多投影/多化身",
            "设计模式": "折叠范型/容器模板", "包": "袋/包/裹", "线程": "分流/支线程",
            "override": "覆/掩/改", "virtual": "虚/拟/代", "template": "范/式/样",
            "generic": "公/共/通", "singleton": "独/一/元", "factory": "生/产/育",
            "observer": "观/察/见",
        }
        return mapping.get(concept)

    def _add_violation(self, file_path, line_num, col, category, detail, suggestion=None):
        rel = str(file_path.relative_to(self.workspace_root)) if str(file_path).startswith(str(self.workspace_root)) else str(file_path)
        self.violations.append(SymbolViolation(rel, line_num, col, category, detail, suggestion))

    def scan_directory(self, dir_path, recursive=True):
        p = Path(dir_path)
        if not p.exists():
            return
        if recursive:
            for f in p.rglob('*'):
                if f.is_file():
                    self.scan_file(f)
        else:
            for f in p.iterdir():
                if f.is_file():
                    self.scan_file(f)

    def generate_report(self, fmt='text'):
        if fmt == 'json':
            return json.dumps({
                "timestamp": datetime.now().isoformat(),
                "stats": self.stats,
                "violations": [str(v) for v in self.violations],
                "pass": len(self.violations) == 0
            }, ensure_ascii=False, indent=2)
        lines = []
        lines.append("=" * 60)
        lines.append("  符号验证器 · 满全法合规报告")
        lines.append("=" * 60)
        lines.append(f"  时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        lines.append(f"  扫描文件数: {self.stats['scanned']}")
        lines.append(f"  扫描行数:   {self.stats['lines']}")
        lines.append(f"  违禁词数:   {self.stats['forbidden']}")
        lines.append(f"  非法短语数: {self.stats['phrases']}")
        lines.append("-" * 60)
        lines.append("")
        if not self.violations:
            lines.append("  ✅ 全部通过 · 无违规符号")
        else:
            lines.append(f"  ❌ 发现 {len(self.violations)} 条违规:")
            lines.append("")
            for v in self.violations:
                lines.append(f"  {v}")
        lines.append("")
        lines.append("-" * 60)
        lines.append(f"  结论: {'通过' if not self.violations else '驳回 · 需修正'}")
        lines.append("=" * 60)
        return '\n'.join(lines)

    def get_exit_code(self):
        has_a = any(v.category == "负向概念" for v in self.violations)
        has_b = any(v.category == "禁止行为" for v in self.violations)
        if has_a and has_b:
            return 3
        if has_a:
            return 1
        if has_b:
            return 2
        return 0


# ============================================================
# CLI
# ============================================================

def main():
    parser = argparse.ArgumentParser(
        description="满全法符号验证器 · 检查代码/文档是否符合容器处理器协议")
    parser.add_argument('files', nargs='*', help='要扫描的文件')
    parser.add_argument('--dir', help='递归扫描目录')
    parser.add_argument('--json', action='store_true', help='JSON输出')
    parser.add_argument('--hook', action='store_true', help='Hook模式（极简输出）')
    parser.add_argument('--root', default=str(WORKSPACE_ROOT), help='工作区根目录')

    args = parser.parse_args()
    validator = SymbolValidator(args.root)

    if args.dir:
        validator.scan_directory(args.dir)
    elif args.files:
        for f in args.files:
            validator.scan_file(f)
    else:
        validator.scan_directory(os.path.join(args.root, 'hunyuan-rs'))
        validator.scan_directory(os.path.join(args.root, 'tools'))
        validator.scan_directory(os.path.join(args.root, 'docs'))

    if args.hook:
        if validator.violations:
            print("SYMBOL_VIOLATION")
            for v in validator.violations:
                print(v)
        else:
            print("SYMBOL_OK")
    else:
        fmt = 'json' if args.json else 'text'
        print(validator.generate_report(fmt))

    sys.exit(validator.get_exit_code())


if __name__ == '__main__':
    main()
