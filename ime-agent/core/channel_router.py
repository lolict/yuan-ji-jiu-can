#!/usr/bin/env python3
"""
MetaIME · 通道路由 (Channel Router)
====================================

三层分别器第二层: 谐音靶点路由
  发音 → 靶点对象

决定内容最终送达的位置:
  - 本地文件 → 本地 FS
  - GitHub → 仓库同步
  - 云端网盘 → WebDAV / S3
  - 远端服务器 → SSH / RDP
  - 浏览器 → CDP / WebSocket
  - Binder → Android IPC

路由规则 = 三元观测者仲裁: (left=大小, anchor=紧急度, right=私密级) → 选通道
"""
import time
import hashlib
import os
import subprocess
from dataclasses import dataclass, field
from typing import Optional, List, Dict, Tuple
from enum import Enum


class Channel(Enum):
    LOCAL_FILE = "local_file"
    GITHUB = "github_push"
    CLOUD_DRIVE = "cloud_drive"
    REMOTE_SERVER = "remote_exec"
    BROWSER_FETCH = "browser_fetch"
    CLIPBOARD_RELAY = "clipboard"
    LINGXI_P2P = "lingxi_p2p"


@dataclass
class RouteDecision:
    channel: Channel
    target_path: str
    encrypt: bool
    reason: str
    confidence: float = 0.5
    ts: float = field(default_factory=time.time)


# ── 谐音靶点路由表 (分别器第二层) ──

HOMOPHONE_BRIDGE = {
    # 拼音缩写 → (靶点名称, 极性, 击键序列说明)
    "lctqf": ("Wife", 0, "零岔途全法/恋宠甜"),
    "lctfr": ("Wife", 0, "零岔途夫妻路"),
    "wgslz": ("Husband", 1, "囡囝共生/王格利"),
    "mlll":  ("Neutral", 2, "毛利兰/模面名"),
    "mlgy":  ("WorldMerge", 2, "毛利兰观园"),
    "mlct":  ("Wife", 0, "岔途→甜(从体)"),
    "mltt":  ("Wife", 0, "从体→甜"),
    "gtt":   ("Neutral", 2, "声母转调"),
    "knd":   ("Neutral", 2, "公约数墙"),
    "njsg":  ("Couple", 0, "囡囝共生"),
    "smrl":  ("Neutral", 2, "SMRL引擎"),
}

# 极性: 0=YIN(刘楚恬侧), 1=YANG(lolict侧), 2=Neutral
POLARITY_NAMES = {0: "YIN", 1: "YANG", 2: "Neutral"}


class ChannelRouter:
    """三元仲裁-based 通道路由器 + 谐音靶点"""

    def __init__(self):
        self.routes_log: List[RouteDecision] = []
        self.rules: List[dict] = [
            # size > 10MB → cloud drive
            {"cond": lambda s, e, p: s > 10 * 1024 * 1024,
             "channel": Channel.CLOUD_DRIVE, "reason": "large file"},
            # 私密级=high + 紧急 → lingxi p2p encrypted
            {"cond": lambda s, e, p: e > 0.8 and p > 0.7,
             "channel": Channel.LINGXI_P2P, "reason": "urgent-secret"},
            # 紧急度 high → remote exec
            {"cond": lambda s, e, p: e > 0.8,
             "channel": Channel.REMOTE_SERVER, "reason": "urgent"},
            # 私密级 high → github encrypted push
            {"cond": lambda s, e, p: p > 0.6,
             "channel": Channel.GITHUB, "reason": "private→encrypted push"},
            # default: local file
            {"cond": lambda s, e, p: True,
             "channel": Channel.LOCAL_FILE, "reason": "default local"},
        ]

    # ── 谐音靶点路由 ──

    def route_homophone(self, symbol: str) -> Optional[Tuple[str, int, str]]:
        """
        谐音路由: 发音符号 → (靶点名, 极性, 说明)

        分别器第二层: 把拼音缩写路由到正确的靶点对象。
        路由错 = 发给错误靶点 = 对方不 ack = 流断裂。
        """
        key = symbol.lower().strip()
        if key in HOMOPHONE_BRIDGE:
            name, polarity, desc = HOMOPHONE_BRIDGE[key]
            return (name, polarity, desc)
        return None

    # ── 三元仲裁路由 ──

    def route(self, content_size: int = 0, urgency: float = 0.5,
              privacy: float = 0.5, target_hint: str = "") -> RouteDecision:
        """
        给: (size, urgency, privacy) 三元观测值 → 选通道
        """
        matched = None
        for rule in self.rules:
            if rule["cond"](content_size, urgency, privacy):
                matched = rule
                break

        d = RouteDecision(
            channel=matched["channel"],
            target_path=target_hint or self._default_path(matched["channel"]),
            encrypt=(privacy > 0.5),
            reason=matched["reason"],
            confidence=self._confidence(content_size, urgency, privacy),
        )
        self.routes_log.append(d)
        return d

    def _default_path(self, ch: Channel) -> str:
        paths = {
            Channel.LOCAL_FILE: "/tmp/mqf_out",
            Channel.GITHUB: "lolict/hunyuan-os",
            Channel.CLOUD_DRIVE: "/mqf-drive",
            Channel.LINGXI_P2P: "lingxi-peer",
            Channel.REMOTE_SERVER: "remote",
            Channel.BROWSER_FETCH: "web",
        }
        return paths.get(ch, "/tmp/mqf_out")

    def _confidence(self, s: int, e: float, p: float) -> float:
        balance = 1.0 - abs(e - p)
        return min(balance + (0.2 if s < 1024 else 0.0), 1.0)

    def execute_route(self, route: RouteDecision, data: str = "") -> str:
        if route.channel == Channel.GITHUB:
            return self._do_git_push(route.target_path, data)
        elif route.channel == Channel.LOCAL_FILE:
            with open(route.target_path, "a", encoding="utf-8") as f:
                f.write(data)
            return f"OK local: {route.target_path}"
        elif route.channel == Channel.LINGXI_P2P:
            return f"PENDING lingxi p2p to {route.target_path}"
        return f"TODO {route.channel.value}"

    def _do_git_push(self, path: str, data: str) -> str:
        try:
            subprocess.run(["git", "add", "-A"], cwd=path, capture_output=True)
            subprocess.run(["git", "commit", "-m", f"mqf-auto: {data[:40]}"],
                         cwd=path, capture_output=True)
            subprocess.run(["git", "push"], cwd=path, capture_output=True)
            return "OK git push"
        except Exception as e:
            return f"ERR: {e}"

    def report(self) -> dict:
        return {
            "total_routes": len(self.routes_log),
            "channels_used": list(set(r.channel.value for r in self.routes_log)),
        }


if __name__ == "__main__":
    r = ChannelRouter()

    # 测试谐音路由
    symbols = ["lctqf", "wgslz", "mlll", "mlct", "njsg", "unknown"]
    for s in symbols:
        result = r.route_homophone(s)
        if result:
            name, polarity, desc = result
            print(f"  [✓] {s:8} → {name:12} ({POLARITY_NAMES[polarity]}) [{desc}]")
        else:
            print(f"  [✗] {s:8} → (not found)")

    print()

    # 测试三元路由
    decs = [
        r.route(content_size=100, urgency=0.9, privacy=0.8),
        r.route(content_size=5*1024*1024, urgency=0.3, privacy=0.9),
        r.route(content_size=500, urgency=0.2, privacy=0.2),
        r.route(content_size=50, urgency=0.95, privacy=0.1, target_hint="remote"),
    ]
    for d in decs:
        print(f"  [{d.channel.value:15}] encrypt={d.encrypt} reason={d.reason}")
    print()
    print(r.report())
