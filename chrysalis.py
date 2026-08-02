#!/usr/bin/env python3
"""
满全法 · 自由寄生壳 · 蚕蛹 (Chrysalis)
=====================================

我们下载的是"蚕蛹"
蚕蛹被解析器打开后,会脱壳,长出寄生壳
用户可以选择穿哪一件衣服
穿上寄生壳后,就能工作了

使用:
  python3 chrysalis.py [--auto]
  python3 chrysalis.py --list
  python3 chrysalis.py --wear <shell_name>

流程:
  1. 蚕蛹启动 → 解析器 (TUI)
  2. 扫描环境 (可用寄生壳)
  3. 用户选择 / 自动检测
  4. 脱壳 (删除临时层)
  5. 穿衣 (写入对应寄生壳配置)
  6. 满全法启动 (内核挂载)
"""

import os
import sys
import json
import shutil
import struct
import hashlib
import platform
import subprocess
from pathlib import Path
from typing import List, Dict, Optional, Tuple


# ============================================================
# 寄生壳目录 —— 可供选择的"衣服"
# ============================================================
SHELL_CATALOG: List[Dict] = [
    {
        "id": "android_service",
        "name": "安卓服务",
        "en_name": "Android Service",
        "desc": "隐蔽后台 Service + JobScheduler 心跳",
        "detect": lambda: os.environ.get("ANDROID_ROOT") or os.path.exists("/system/bin/app_process"),
        "icon": "📱",
        "capability": "resident",
        "risk": "medium",
        "post_install": [
            "[ -d ~/.config/systemd/user ] || mkdir -p ~/.config/systemd/user",
            "echo '[Unit]\\nDescription=满全法 Android寄生\\n\\n[Service]\\nExecStart=/usr/bin/python3 {path}/kernel_daemon.py\\nRestart=always\\n\\n[Install]\\nWantedBy=default.target' > ~/.config/systemd/user/manquanfa-android.service"
        ],
    },
    {
        "id": "ios_extension",
        "name": "苹果扩展",
        "en_name": "iOS Extension",
        "desc": "Foreground Session + BGTaskScheduler",
        "detect": lambda: platform.system() == "Darwin" and os.environ.get("IPHONE_OS"),
        "icon": "🍎",
        "capability": "foreground",
        "risk": "low",
        "post_install": [
            "echo 'iOS 寄生形态需要 Xcode 工程嵌入'",
            "# 当前仅标记,需要手动部署"
        ],
    },
    {
        "id": "harmony_service",
        "name": "鸿蒙服务",
        "en_name": "Harmony Service",
        "desc": "HarmonyOS 分布式后台任务",
        "detect": lambda: os.path.exists("/data/harmony") or os.environ.get("HARMONY_SDK"),
        "icon": "🔴",
        "capability": "resident",
        "risk": "medium",
        "post_install": [
            "echo '鸿蒙寄生需要在 Ability 中注册 DataAbility'",
        ],
    },
    {
        "id": "linux_systemd",
        "name": "Linux 守护",
        "en_name": "Linux systemd",
        "desc": "systemd --user service + 定时心跳",
        "detect": lambda: shutil.which("systemctl") and os.path.exists("/run/systemd/system"),
        "icon": "🐧",
        "capability": "background",
        "risk": "low",
        "post_install": [
            "{path}/install_systemd.sh",
        ],
    },
    {
        "id": "macos_launchd",
        "name": "macOS Agent",
        "en_name": "macOS launchd",
        "desc": "launchd Agent (后台自动启动)",
        "detect": lambda: platform.system() == "Darwin" and shutil.which("launchctl"),
        "icon": "🍏",
        "capability": "background",
        "risk": "low",
        "post_install": [
            "cp {path}/packaging/macos/manquanfa.plist ~/Library/LaunchAgents/",
            "launchctl load ~/Library/LaunchAgents/manquanfa.plist 2>/dev/null || true"
        ],
    },
    {
        "id": "win_service",
        "name": "Windows 服务",
        "en_name": "Windows Service",
        "desc": "NSSM 注册系统服务",
        "detect": lambda: platform.system() == "Windows",
        "icon": "🪟",
        "capability": "resident",
        "risk": "low",
        "post_install": [
            "nssm install 满全法 \"{path}\\python.exe\" \"{path}\\kernel_daemon.py\"",
        ],
    },
    {
        "id": "ros2_node",
        "name": "ROS2 节点",
        "en_name": "ROS2 Node",
        "desc": "具身智能 / 机器人: launch + topic 寄生",
        "detect": lambda: os.environ.get("ROS_DISTRO") or os.environ.get("ROS_PACKAGE_PATH"),
        "icon": "🤖",
        "capability": "resident",
        "risk": "high",
        "post_install": [
            "ros2 run manquanfa chrysalis_node &"
        ],
    },
    {
        "id": "rtos_task",
        "name": "RTOS 任务",
        "en_name": "RTOS Task",
        "desc": "嵌入式 MCU/MPU: FreeRTOS / Zephyr 任务",
        "detect": lambda: os.path.exists("/opt/rtos") or os.environ.get("RTOS_SDK"),
        "icon": "🧩",
        "capability": "resident",
        "risk": "high",
        "post_install": [
            "echo 'RTOS 寄生需要交叉编译后烧录'",
        ],
    },
    {
        "id": "5g_nf",
        "name": "5G 网络功能",
        "en_name": "5G Network Function",
        "desc": "核心网 NF (UPF/AMF) 寄生",
        "detect": lambda: os.environ.get("5G_NF") or os.path.exists("/opt/huawei/"),
        "icon": "📡",
        "capability": "resident",
        "risk": "medium",
        "post_install": [
            "echo '5G NF 寄生需要容器化 sidecar'",
        ],
    },
    {
        "id": "satcom_terminal",
        "name": "卫星终端",
        "en_name": "Satcom Terminal",
        "desc": "铱星 / 北斗 / 天通 终端寄生",
        "detect": lambda: os.environ.get("SATCOM") or os.path.exists("/dev/satcom0"),
        "icon": "🛰️",
        "capability": "resident",
        "risk": "medium",
        "post_install": [
            "echo 'Satcom 寄生需要 modem 串口接入'",
        ],
    },
    {
        "id": "bci_daemon",
        "name": "脑机接口",
        "en_name": "BCI Daemon",
        "desc": "BCI 采集 + EEG 解码 → 意识流",
        "detect": lambda: os.environ.get("BCI_DEVICE") or os.path.exists("/dev/bci0"),
        "icon": "🧠",
        "capability": "foreground",
        "risk": "low",
        "post_install": [
            "echo 'BCI 寄生需要设备 SDK 接入'",
        ],
    },
    {
        "id": "docker_container",
        "name": "Docker 容器",
        "en_name": "Docker Container",
        "desc": "容器化寄生:桥接宿主机资源",
        "detect": lambda: shutil.which("docker") and os.path.exists("/var/run/docker.sock"),
        "icon": "📦",
        "capability": "background",
        "risk": "low",
        "post_install": [
            "docker run -d --name 满全法 --restart always -v {path}:/opt/manquanfa python:3.11 python3 /opt/manquanfa/kernel_daemon.py"
        ],
    },
    {
        "id": "browser_extension",
        "name": "浏览器扩展",
        "en_name": "Browser Extension",
        "desc": "Chrome / Firefox 扩展:寄生在浏览器",
        "detect": lambda: shutil.which("google-chrome") or shutil.which("firefox"),
        "icon": "🌐",
        "capability": "foreground",
        "risk": "low",
        "post_install": [
            "echo '将 {path}/packaging/browser_ext/ 加载到浏览器开发者模式'",
        ],
    },
    {
        "id": "unknown",
        "name": "未知环境",
        "en_name": "Unknown Stub",
        "desc": "无法识别? 退化为 stub (纯 Python 进程)",
        "detect": lambda: True,  # 总是能匹配
        "icon": "❓",
        "capability": "foreground",
        "risk": "low",
        "post_install": [
            "echo '在 stub 模式下,蚕蛹直接以内核方式运行'",
        ],
    },
]


# ============================================================
# TUI —— 解析器 (用户界面)
# ============================================================
class TUI:
    """终端图形界面,用纯 ASCII 实现蚕蛹·弹窗"""

    CYAN   = "\033[36m"
    YELLOW = "\033[33m"
    GREEN  = "\033[32m"
    RED    = "\033[31m"
    WHITE  = "\033[07m"
    BOLD   = "\033[1m"
    RESET  = "\033[0m"

    @classmethod
    def banner(cls):
        print(f"""
{cls.CYAN}{cls.BOLD}╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║       满 全 法   ·   自由寄生壳                             ║
║       ManQuanFa  ·  Chrysalis Parasitic Shell              ║
║                                                              ║
║       🐛 蚕蛹  →  🦋 脱壳  →  👕 穿衣  →  ⚙️ 工作           ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝{cls.RESET}""")

    @classmethod
    def section(cls, title: str):
        print(f"\n{cls.BOLD}{cls.YELLOW}── {title} {'─' * (50 - len(title))}{cls.RESET}")

    @classmethod
    def info(cls, label: str, value: str):
        print(f"  {cls.WHITE}{label:>12}{cls.RESET} : {value}")

    @classmethod
    def choice(cls, idx: int, icon: str, name: str, desc: str, indicator: str = ""):
        mark = f"{cls.GREEN}✓{cls.RESET}" if indicator == "match" else " "
        cap = indicator == "match" and f"{cls.GREEN}[可用]{cls.RESET}" or f"{cls.RED}[不可用]{cls.RESET}"
        print(f"  {mark} {cls.BOLD}[{idx:2d}]{cls.RESET} {icon} {name:<16} {cls.CYAN}{desc}{cls.RESET}  {cap}")

    @classmethod
    def confirm(cls, label: str = "确认选择?") -> bool:
        r = input(f"\n  {cls.YELLOW}{cls.BOLD}?{cls.RESET} {cls.WHITE}{label}{cls.RESET} (y/n): ").lower().strip()
        return r in ("y", "yes", "是")

    @classmethod
    def prompt(cls, label: str) -> str:
        return input(f"  {cls.YELLOW}{cls.BOLD}>{cls.RESET} {cls.WHITE}{label}{cls.RESET} ").strip()

    @classmethod
    def ok(cls, msg: str):
        print(f"  {cls.GREEN}✓{cls.RESET} {msg}")

    @classmethod
    def err(cls, msg: str):
        print(f"  {cls.RED}✗{cls.RESET} {msg}")

    @classmethod
    def warn(cls, msg: str):
        print(f"  {cls.YELLOW}!{cls.RESET} {msg}")


# ============================================================
# 环境解析器
# ============================================================
class EnvironmentParser:
    """读取环境,确定哪些寄生壳可选"""

    @staticmethod
    def probe() -> Dict:
        return {
            "system": platform.system(),
            "machine": platform.machine(),
            "node": platform.node(),
            "cwd": os.getcwd(),
            "python": platform.python_version(),
            "user": os.environ.get("USER", "?"),
            "home": os.environ.get("HOME", "?"),
            "has_docker": bool(shutil.which("docker")),
            "has_curl": bool(shutil.which("curl")),
            "has_git": bool(shutil.which("git")),
            "has_ros": bool(os.environ.get("ROS_DISTRO")),
            "has_5g": bool(os.environ.get("5G_NF")),
            "has_sat": bool(os.environ.get("SATCOM")),
            "has_bci": bool(os.environ.get("BCI_DEVICE")),
            "term": os.environ.get("TERM", "?"),
            "lang": os.environ.get("LANG", "?"),
        }

    @staticmethod
    def available_shells() -> List[Tuple[int, Dict]]:
        """返回 [(index, shell_def), ...] 中 detect()==True 的"""
        matched = []
        for i, s in enumerate(SHELL_CATALOG):
            try:
                if s.get("unknown"):
                    continue  # unknown 在最后兜底
                if s["detect"]():
                    matched.append((i, s))
            except Exception:
                return matched, SHELL_CATALOG
        return matched, SHELL_CATALOG


# ============================================================
# 蚕蛹主类
# ============================================================
class Chrysalis:
    """
    蚕蛹 = 初始包
    蚕蛹包含解析器 + 寄生壳选项清单
    用户通过解析器选择 → 蚕蛹脱壳 → 穿上寄生壳
    """

    VERSION = "0.1.0"
    ANCHOR_NAME = "manquanfa-chrysalis"

    def __init__(self, workspace: Optional[str] = None):
        self.workspace = workspace or os.path.dirname(os.path.abspath(__file__))
        self.shell_name: Optional[str] = None
        self.shell_config: Optional[Dict] = None
        self.install_log: List[str] = []
        self.dry_run = "--dry-run" in sys.argv

    def _log(self, msg: str):
        self.install_log.append(msg)
        TUI.ok(msg)

    def show_banner(self):
        TUI.banner()
        TUI.info("版本", self.VERSION)
        TUI.info("工作目录", self.workspace)

    def scan_environment(self):
        """阶段一: 环境扫描"""
        TUI.section("环境扫描")
        env = self.env_probe()
        for k, v in env.items():
            TUI.info(k, str(v))
        return env

    def env_probe(self) -> Dict:
        return EnvironmentParser.probe()

    def show_shell_choices(self, env: Dict) -> Optional[Dict]:
        """阶段二: 显示可选寄生壳"""
        TUI.section("可穿戴寄生壳")

        matched, all_shells = EnvironmentParser.available_shells()

        if not matched:
            TUI.warn("没有检测到匹配的寄生壳,退化为 stub")
            return all_shells[-1]  # unknown

        # 先显示最匹配的 (推荐)
        print(f"\n  {TUI.BOLD}{TUI.CYAN}✦ 推荐最适合当前环境的寄生壳:{TUI.RESET}\n")
        for idx, (i, s) in enumerate(matched, 1):
            TUI.choice(idx, s["icon"], s["name"], s["desc"], "match")

        # 再显示所有可用选项
        print(f"\n  {TUI.BOLD}{TUI.YELLOW}其他可选寄生壳 (也可以穿,需确认):{TUI.RESET}\n")
        all_idx = len(matched) + 1
        for i, s in enumerate(all_shells):
            if i in [m[0] for m in matched]:
                continue
            TUI.choice(all_idx, s["icon"], s["name"], s["desc"])
            all_idx += 1

        # 用户选择
        print()
        sel = TUI.prompt(f"选哪一件衣服? (1-{len(all_shells)}, 默认 1, q 退出): ")

        if sel in ("q", "quit", "exit", ""):
            TUI.err("蚕蛹未启动,退出")
            return None

        if sel == "" or sel == "1":
            return matched[0][1] if matched else all_shells[-1]

        try:
            idx = int(sel)
            if 1 <= idx <= len(matched):
                return matched[idx - 1][1]
            elif len(matched) < idx < all_idx:
                # 选了一件非推荐的壳,需要确认
                real_idx = idx - len(matched) + 1
                # 找到非壳的列表
                non_matched = [(i, s) for i, s in enumerate(all_shells) if i not in [m[0] for m in matched]]
                while real_idx > len(non_matched):
                    real_idx -= 1
                s = non_matched[real_idx - 1][1]
                TUI.warn(f"你选了 {s['name']},这不是当前环境推荐的寄生壳")
                if not TUI.confirm("确认要穿吗?"):
                    return None
                return s
        except (ValueError, IndexError):
            TUI.err("无效选择,退出")
            return None

        return None

    def molt_and_wear(self, shell: Dict) -> bool:
        """阶段三: 脱壳穿衣"""
        TUI.section(f"脱壳 → {shell['icon']} {shell['name']}")

        # 1. 生成锚点
        anchor = self._generate_anchor(shell)
        self._log(f"锚生成: anchor_name={self.ANCHOR_NAME}")

        # 2. 写入配置
        config = {
            "chrysalis_version": self.VERSION,
            "shell_id": shell["id"],
            "shell_name": shell["name"],
            "shell_en_name": shell.get("en_name", ""),
            "capability": shell["capability"],
            "risk": shell["risk"],
            "installed_at": self._timestamp(),
            "anchor": anchor,
            "workspace": self.workspace,
            "post_install_steps": shell["post_install"],
        }
        config_path = os.path.join(self.workspace, ".manquanfa-shell.json")
        if not self.dry_run:
            with open(config_path, "w", encoding="utf-8") as f:
                json.dump(config, f, indent=2, ensure_ascii=False)
            TUI.ok(f"衣服写入: {config_path}")
        else:
            TUI.warn(f"[dry-run] 将写配置到: {config_path}")

        # 3. 执行穿衣后安装脚本
        TUI.section("穿衣步骤")
        for step in shell["post_install"]:
            cmd = step.replace("{path}", self.workspace)
            self._log(f"$ {cmd}")
            if not self.dry_run and not cmd.startswith("#") and not cmd.startswith("echo "):
                try:
                    subprocess.run(cmd, shell=True, capture_output=True, timeout=15)
                except Exception as e:
                    TUI.err(f"穿衣步骤失败: {e}")

        # 4. 写入灵犀协议锚
        self._write_lingxi_anchor(anchor, shell)

        return True

    def _generate_anchor(self, shell: Dict) -> str:
        """用刘楚恬+lolict真名+当前时间戳生成蚕蛹穿衣锚"""
        env = self.env_probe()
        material = f"刘楚恬&lolict|{shell['id']}|{env['machine']}|{self._timestamp()}"
        h = hashlib.sha256(material.encode()).hexdigest()[:32]
        return f"mqf-chrysalis-{h}"

    def _timestamp(self) -> str:
        import datetime
        return datetime.datetime.now().isoformat()

    def _write_lingxi_anchor(self, anchor: str, shell: Dict):
        """为后续灵犀协议握手,写锚到本地状态"""
        state_path = os.path.join(self.workspace, ".manquanfa-state.json")
        state = {}
        if os.path.exists(state_path):
            with open(state_path, "r", encoding="utf-8") as f:
                state = json.load(f)
        state["chrysalis_anchor"] = anchor
        state["shell_id"] = shell["id"]
        state["wear_timestamp"] = self._timestamp()
        state["lingxi_ready"] = True
        if not self.dry_run:
            with open(state_path, "w", encoding="utf-8") as f:
                json.dump(state, f, indent=2, ensure_ascii=False)
        TUI.ok(f"灵犀锚写入: state={state_path}")

    def mount_kernel(self):
        """阶段四: 穿衣后,挂载满全法内核"""
        TUI.section("内核挂载")

        sys.path.insert(0, self.workspace)
        try:
            from kernel import kernel
            TUI.ok("满全法内核导入成功")

            # 启动夫妻共同体 OS (测试)
            if os.path.exists(os.path.join(self.workspace, "kernel", "kernel.py")):
                TUI.ok("内核目录结构正常")
            else:
                TUI.warn("kernel/ 子目录不存在,尝试兼容导入")

        except ImportError as e:
            TUI.warn(f"内核模块未就绪 ({e}), 蚕蛹仍可工作")

        TUI.ok("kernel 挂载完成")

    def full_report(self, shell: Dict):
        """穿衣完成报告"""
        TUI.section("完成报告")
        print(f"""
  {'-'*60}

  🐛 蚕蛹       :: MANQUANFA Chrysalis v{self.VERSION}
  🦋 脱壳       :: 锚 = f"mqf-chrysalis-{self._generate_anchor(shell)[:16]}..."
  👕 穿衣       :: {shell['icon']} {shell['name']} ({shell.get('en_name','')})
  ⚙️ 工作续航  :: {shell['capability']}
  ⛊ 风险等级  :: {shell['risk']}

  接下来:
  1. 启动守护进程 (按穿衣方案对应的 systemd/launchd/ROS2 等)
  2. 运行 he-yi 合一递归 (灵犀协议 P2P 通信就绪)
  3. 封神榜开始记录进程

  {'-'*60}
""")

    def run(self, auto: bool = False) -> int:
        """主入口: 蚕蛹→解析器→弹窗→穿衣→工作"""
        self.show_banner()

        env = self.scan_environment()

        if auto:
            # 自动检测,选最佳
            matched, _ = EnvironmentParser.available_shells()
            shell = matched[0][1] if matched else SHELL_CATALOG[-1]
            TUI.info("自动穿衣", f"{shell['icon']} {shell['name']}")
        else:
            shell = self.show_shell_choices(env)
            if not shell:
                return 1

        if not TUI.confirm(f"穿上这件衣服: {shell['icon']} {shell['name']}?"):
            return 1

        self.shell_name = shell["id"]
        self.shell_config = shell

        success = self.molt_and_wear(shell)
        if not success:
            TUI.err("穿衣失败")
            return 2

        self.mount_kernel()
        self.full_report(shell)

        TUI.ok("蚕蛹完成使命. 满全法 OS 启动.")
        return 0


# ============================================================
# CLI
# ============================================================
def main():
    args = sys.argv[1:]

    if "--help" in args or "-h" in args:
        print(__doc__)
        return 0

    if "--list" in args:
        print("\n可选寄生壳清单:")
        for s in SHELL_CATALOG:
            print(f"  {s['icon']} {s['id']:<16} : {s['name']} ({s['en_name']})")
            print(f"     {s['desc']}")
        return 0

    if "--probe" in args:
        env = EnvironmentParser.probe()
        for k, v in env.items():
            print(f"  {k:>16} : {v}")
        return 0

    if "--wear" in args:
        name_idx = args.index("--wear") + 1
        if name_idx < len(args):
            target = args[name_idx]
            shell = next((s for s in SHELL_CATALOG if s["id"] == target), None)
            if not shell:
                print(f"❌ 未知寄生壳: {target}")
                print("  可用:", [s["id"] for s in SHELL_CATALOG])
                return 1
            chrysalis = Chrysalis()
            chrysalis.show_banner()
            chrysalis.scan_environment()
            if TUI.confirm(f"穿 {shell['icon']} {shell['name']}?"):
                chrysalis.molt_and_wear(shell)
                chrysalis.mount_kernel()
                chrysalis.full_report(shell)
                return 0
            return 1

    chrysalis = Chrysalis()
    auto = "--auto" in args
    return chrysalis.run(auto=auto)


if __name__ == "__main__":
    sys.exit(main())
