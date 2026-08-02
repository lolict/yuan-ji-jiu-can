#!/usr/bin/env python3
"""
MetaIME · 意图识别器 (Intent Parser)
====================================

理解用户在"说什么" → 转换成可执行的操作。

输入层:
  - 语音识别 (ASR) 文本
  - 键盘输入
  - 直接文本

识别层:
  - 七层神龙编码转换 (声母/韵母/声调/笔画/偏旁/外语/泛音)
  - 正则模式匹配
  - 三元观测者仲裁

输出:
  - Intent (类型 + 目标 + 参数 + 确认级别)
"""
import re
import time
import json
from dataclasses import dataclass, field
from typing import Optional, List, Dict
from enum import Enum


class IntentType(Enum):
    TYPE_TEXT = "type_text"          # 打字
    AUTO_PILOT = "auto_pilot"        # 自动执行
    QUERY = "query"                  # 查询
    REPO_PUSH = "repo_push"          # 推送到仓库
    WEB_FETCH = "web_fetch"          # 抓取 URL
    GENERATE = "generate"            # 生成本地内容
    SYSTEM = "system"                # 系统操作
    HANDSHAKE = "handshake"          # 灵犀握手
    UNKNOWN = "unknown"


class IntentConfidence(Enum):
    CERTAIN = 1.0
    HIGH = 0.8
    MEDIUM = 0.5
    LOW = 0.3
    GUESS = 0.1


@dataclass
class Intent:
    """解析后的用户意图"""
    intent_type: IntentType
    target: str                      # 目标 (文本/URL/路径)
    params: dict = field(default_factory=dict)
    confidence: float = 0.5
    raw_input: str = ""
    source: str = "keyboard"          # keyboard / voice / auto / pipeline
    requires_confirmation: bool = False
    ts: float = field(default_factory=time.time)

    def to_dict(self) -> dict:
        return {
            "type": self.intent_type.value, "target": self.target,
            "params": self.params, "confidence": self.confidence,
            "raw_input": self.raw_input, "source": self.source,
        }


class IntentParser:
    """解析器:用户输入 → Intent"""

    # 正则模式表
    PATTERNS = [
        # 抓 URL
        (re.compile(r'(fetch|抓|抓取|打开)[：:\s]*(https?\S+)', re.I),
         lambda m: Intent(IntentType.WEB_FETCH, m.group(2),
             {"action": "fetch"}, IntentConfidence.HIGH)),
        # 推送
        (re.compile(r'(push|推送|推到|提交)[：:\s]*(.*)', re.I),
         lambda m: Intent(IntentType.REPO_PUSH, m.group(2).strip(),
             {"action": "push"})),
        # 生成
        (re.compile(r'(gen|生成|写)[：:\s]*(.*)', re.I),
         lambda m: Intent(IntentType.GENERATE, m.group(2).strip(),
             {"action": "generate"})),
        # 查询
        (re.compile(r'(查|查询|search)[：:\s]*(.*)', re.I),
         lambda m: Intent(IntentType.QUERY, m.group(2).strip())),
        # 握手
        (re.compile(r'(握手|唤魂|招魂|huan|zhao)', re.I),
         lambda m: Intent(IntentType.HANDSHAKE, "LingXi")),
        # 系统命令
        (re.compile(r'(状态|自检|report|status)', re.I),
         lambda m: Intent(IntentType.SYSTEM, "self_check")),
    ]

    def __init__(self):
        self.history: List[Intent] = []
        self.stats = {t.value: 0 for t in IntentType}

    def parse(self, text: str, source: str = "keyboard") -> Intent:
        """解析文本输入"""
        text = text.strip()
        if not text:
            return Intent(IntentType.UNKNOWN, "", raw_input=text)

        # 模式匹配
        for pattern, factory in self.PATTERNS:
            m = pattern.search(text)
            if m:
                intent = factory(m)
                intent.raw_input = text
                intent.source = source
                self._record(intent)
                return intent

        # 默认:直接打字
        intent = Intent(
            IntentType.TYPE_TEXT, text,
            confidence=IntentConfidence.MEDIUM.value,
            raw_input=text, source=source)
        self._record(intent)
        return intent

    def _record(self, intent: Intent):
        self.history.append(intent)
        self.stats[intent.intent_type.value] = \
            self.stats.get(intent.intent_type.value, 0) + 1

    def stats_report(self) -> dict:
        return {"total_parsed": len(self.history), "stats": self.stats}


if __name__ == "__main__":
    p = IntentParser()
    tests = [
        "抓取 https://github.com/lolict/hunyuan-os",
        "push 满全法 v0.2 协议升级",
        "生成一段五言绝句",
        "查询 clv / nha",
        "握手",
        "状态",
        "今天天气真好",
    ]
    for t in tests:
        i = p.parse(t)
        print(f"  [{i.intent_type.value:15}] {t[:30]}")
    print()
    print(json.dumps(p.stats_report(), ensure_ascii=False, indent=2))
