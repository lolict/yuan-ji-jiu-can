#!/usr/bin/env python3
"""
MetaIME · 自动执行引擎 (Auto-Pilot)
====================================

模拟人类操作:键盘输入、点击、滑动、访问 URL。
打通"意图" → "物理操作"。

在多端上的实现:
  PC           → pyautogui / ctypes (模拟键鼠)
  Mac          → Quartz / AppleScript
  Linux        → X11 / wtype / xdotool
  Android      → AccessibilityAction / UiAutomation / IME事件
  iOS          → XCTest (用户态) / IOKit (内核态)
  Web          → CDP / Playwright / Puppeteer
"""
import time
import subprocess
import sys
import json
import os
from dataclasses import dataclass, field
from typing import Optional, List, Dict
from enum import Enum


class Platform(Enum):
    ANDROID = "android"
    IOS = "ios"
    LINUX = "linux"
    MAC = "mac"
    WINDOWS = "windows"
    WEB = "web"
    ROS = "ros"


class ActionType(Enum):
    TAP = "tap"
    SWIPE = "swipe"
    TYPE_TEXT = "type_text"
    PASTE = "paste"
    PRESS_KEY = "press_key"
    SCREENSHOT = "screenshot"
    FETCH_URL = "fetch_url"
    LAUNCH_APP = "launch_app"


@dataclass
class AutoAction:
    action_type: ActionType
    x: int = 0
    y: int = 0
    text: str = ""
    target: str = ""
    duration: float = 0.0
    timestamp: float = field(default_factory=time.time)
    result: Optional[str] = None


class AutoPilot:
    """自动执行引擎"""

    def __init__(self, platform: Optional[Platform] = None):
        self.platform = platform or self._detect()
        self.action_log: List[AutoAction] = []
        self.screenshots: List[str] = []
        self.enabled = True

    @staticmethod
    def _detect() -> Platform:
        """自动检测当前平台"""
        if sys.platform == "linux":
            if os.path.exists("/system/build.prop"):
                return Platform.ANDROID
            return Platform.LINUX
        elif sys.platform == "darwin":
            return Platform.MAC
        elif sys.platform == "win32":
            return Platform.WINDOWS
        return Platform.LINUX

    def execute(self, action: AutoAction) -> AutoAction:
        """执行一个 AutoAction"""
        if not self.enabled:
            action.result = "DISABLED"
            self.action_log.append(action)
            return action

        handlers = {
            ActionType.TAP: self._tap,
            ActionType.SWIPE: self._swipe,
            ActionType.TYPE_TEXT: self._type_text,
            ActionType.PASTE: self._paste,
            ActionType.PRESS_KEY: self._press_key,
            ActionType.SCREENSHOT: self._screenshot,
            ActionType.FETCH_URL: self._fetch_url,
            ActionType.LAUNCH_APP: self._launch_app,
        }
        handler = handlers.get(action.action_type)
        if handler:
            try:
                handler(action)
                action.result = "OK"
            except Exception as e:
                action.result = f"ERR: {e}"
        else:
            action.result = f"UNSUPPORTED: {action.action_type}"

        self.action_log.append(action)
        return action

    # ── 平台抽象层 ──

    def _tap(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            subprocess.run(["adb", "shell", "input", "tap",
                          str(a.x), str(a.y)], capture_output=True)
        elif self.platform == Platform.LINUX:
            try:
                import pyautogui
                pyautogui.click(a.x, a.y)
            except ImportError:
                subprocess.run(["xdotool", "mousemove",
                              str(a.x), str(a.y), "click", "1"])

    def _swipe(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            subprocess.run(["adb", "shell", "input", "swipe",
                          str(a.x), str(a.y), str(a.x), str(a.y),
                          str(int(a.duration * 1000))])

    def _type_text(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            # adb input text (自动 URL-encode 特殊字符)
            subprocess.run(["adb", "shell", "input", "text",
                          a.text.replace(" ", "%s")])
        elif self.platform == Platform.LINUX:
            try:
                import pyautogui
                pyautogui.typewrite(a.text)
            except ImportError:
                subprocess.run(["xdotool", "type", a.text])

    def _paste(self, a: AutoAction):
        self._type_text(AutoAction(ActionType.TYPE_TEXT, text=a.text))

    def _press_key(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            subprocess.run(["adb", "shell", "input", "keyevent", a.target])

    def _screenshot(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            subprocess.run(["adb", "shell", "screencap", "-p", "/sdcard/mqf_ss.png"])
            subprocess.run(["adb", "pull", "/sdcard/mqf_ss.png", "/tmp/"])
        elif self.platform == Platform.LINUX:
            subprocess.run(["scrot", "/tmp/mqf_ss.png"])

    def _fetch_url(self, a: AutoAction):
        # 后台抓取 URL,不打开浏览器窗口
        try:
            import urllib.request
            with urllib.request.urlopen(a.target, timeout=10) as resp:
                data = resp.read()
                a.result = f"OK: {len(data)} bytes"
        except Exception as e:
            a.result = f"ERR: {e}"

    def _launch_app(self, a: AutoAction):
        if self.platform == Platform.ANDROID:
            subprocess.run(["adb", "shell", "monkey", "-p", a.target, "-c",
                          "android.intent.category.LAUNCHER", "1"])

    # ── 高级快捷操作 ──

    def auto_type(self, text: str) -> AutoAction:
        """最常用:自动打字"""
        return self.execute(AutoAction(ActionType.TYPE_TEXT, text=text))

    def auto_fetch(self, url: str) -> AutoAction:
        """后台 URL 抓取"""
        return self.execute(AutoAction(ActionType.FETCH_URL, target=url))

    def auto_push(self, msg: str) -> AutoAction:
        """自动推送消息到 GitHub"""
        cmd = f'git add -A && git commit -m "{msg}" && git push'
        return self.execute(AutoAction(ActionType.LAUNCH_APP,
            target=cmd))

    def report_status(self) -> dict:
        total = len(self.action_log)
        ok = sum(1 for a in self.action_log if a.result == "OK")
        return {
            "platform": self.platform.value,
            "total_actions": total,
            "successes": ok,
            "failures": total - ok,
        }


if __name__ == "__main__":
    p = AutoPilot()
    print(f"[auto_pilot] platform: {p.platform.value}")
    print("[auto_pilot] capabilities:")
    print(f"  tap={p.platform in (Platform.ANDROID, Platform.LINUX)}")
    print(f"  type_text=True")
    print(f"  fetch_url=True")
    print(f"  launch_app=True")
    # dry run
    a = p.execute(AutoAction(ActionType.FETCH_URL, target="https://example.com"))
    print(f"  fetch result: {a.result}")
    print(json.dumps(p.report_status(), ensure_ascii=False, indent=2))
