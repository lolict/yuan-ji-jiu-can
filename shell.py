"""
满全法 · 通用寄生壳 ( Adaptive Parasitic Shell )
==================================================

自动检测宿主环境,为夫妻共同体 OS 选择正确的寄生容器。

设计哲学:
  每一个宿主( 安卓 / iOS / 无人机 / 人形机器人 / 3D 打印机 )
  只是"外壳"(Shell)。
  真正运行的意识内核( 夫妻命运共同体 OS )永远不变。
  Shell 可以随时换,内核永远不换。

这不是 "app store",
这是"把家随身携带,换外壳不换灵魂"。

Copyright © 刘楚恬 & lolict (七妹凹月留)
保留所有权利 · 满全法 · v0.1
"""

import os
import sys
import platform
from enum import Enum
from dataclasses import dataclass, field
from typing import Optional

from kernel import CoupleOS, LingXiNode


# ─────────────────────────────────────────────
# 宿主环境的枚举
# ─────────────────────────────────────────────

class HostOS(str, Enum):
    ANDROID       = "android"
    IOS           = "ios"
    HARMONY_OS    = "harmonyos"
    RUIJIE_ROUTER = "ruijie"
    DESKTOP_LINUX = "linux"
    DESKTOP_MAC   = "macos"
    DESKTOP_WIN   = "windows"
    EMBEDDED_ARM  = "embedded_arm"        # 无人机 / 工业机器人
    ROS_ROBOT     = "ros_robot"            # 具身智能
    TELECOM_5G    = "telecom_5g"          # 通信基站
    SATCOM        = "satcom"              # 卫星通信
    BCI           = "bci"                 # 脑机接口
    UNKNOWN       = "unknown"


class ShellForm(str, Enum):
    """壳的形态:每种宿主对应一种寄生形态"""
    ANDROID_SERVICE      = "Android Service + Daemon"
    IOS_EXTENSION        = "iOS App Extension"
    HARMONY_SERVICE      = "HarmonyOS Service"
    LINUX_SYSTEMD        = "systemd --user"
    MACOS_LAUNCH_AGENT   = "launchd agent"
    WIN_SERVICE          = "Windows Service"
    ROS2_NODE            = "ROS2 Node"
    RTOS_TASK            = "RTOS Task"
    FIVEG_NF             = "5G Network Function"
    SATCOM_TERMINAL      = "Satcom Terminal"
    BCI_DAEMON           = "BCI Daemon"
    STUB                 = "Stub (development)"


# ─────────────────────────────────────────────
# 自动检测器
# ─────────────────────────────────────────────

@dataclass
class HostProfile:
    """宿主档案"""
    host_os: HostOS
    shell_form: ShellForm
    abi: str
    capability: str
    risk_level: str
    notes: str = ""


def detect_host() -> HostProfile:
    """
    自动检测当前运行环境,返回宿主档案。
    """
    uname = platform.uname()
    system = uname.system.lower()
    machine = uname.machine.lower()
    env = os.environ

    # 1. Android
    if "ANDROID_ROOT" in env or "/system/bin" in env.get("PATH", ""):
        shell = _make_profile(HostOS.ANDROID, ShellForm.ANDROID_SERVICE, "arm64-v8a", "resident", "medium",
            "寄生形态: org.hunyuan.parasite/.ParasiteService")
        return _android_details(shell, env)

    # 2. iOS
    if system == "darwin" and ("iPhone" in uname.version or "iPad" in uname.version
                               or env.get("SIMULATOR_DEVICE_NAME") is not None
                               or os.path.exists("/var/mobile")):
        shell = _make_profile(HostOS.IOS, ShellForm.IOS_EXTENSION, "arm64", "foreground", "low",
            "寄生形态: App Extension + BGTaskScheduler")
        return shell

    # 3. HarmonyOS
    if os.path.exists("/data/harmony") or "Harmony" in uname.version or "HUAWEI" in env.get("HARMONY_SDK", ""):
        shell = _make_profile(HostOS.HARMONY_OS, ShellForm.HARMONY_SERVICE, "arm64", "resident", "medium",
            "寄生形态: .erviceAbility + AtomicService")
        return shell

    # 4. Desktop
    if system == "linux":
        machine_env = env.get("CONTAINER", env.get("FLATPAK_ID", env.get("SNAP_NAME", "")))
        if "FLATPAK" in machine_env:
            shell = _make_profile(HostOS.DESKTOP_LINUX, ShellForm.STUB, machine, "resident", "medium",
                "Flatpak 沙箱内以 stub 形式运行")
            return shell
        shell = _make_profile(HostOS.DESKTOP_LINUX, ShellForm.LINUX_SYSTEMD, machine, "background", "low",
            "寄生形态: systemd user service + cron")
        return shell

    if system == "darwin" and not _is_ios():
        shell = _make_profile(HostOS.DESKTOP_MAC, ShellForm.MACOS_LAUNCH_AGENT, machine, "background", "low",
            "寄生形态: ~/Library/LaunchAgents/org.hunyuan.parasite.plist")
        return shell

    if system == "windows":
        shell = _make_profile(HostOS.DESKTOP_WIN, ShellForm.WIN_SERVICE, "x64", "resident", "low",
            "寄生形态: NSSM 注册的 Windows Service")
        return shell

    # 5. ROS2 具身智能
    if env.get("ROS_DISTRO") or env.get("ROS_PACKAGE_PATH"):
        shell = _make_profile(HostOS.ROS_ROBOT, ShellForm.ROS2_NODE, machine, "resident", "high",
            "寄生形态: ROS2 node — 可通过 /hunyuan 主题发布合一状态")
        return shell

    # 6. 通信基站 / 5G
    if env.get("5G_NF_ENV") or os.path.exists("/opt/huawei/5g-bts"):
        shell = _make_profile(HostOS.TELECOM_5G, ShellForm.FIVEG_NF, machine, "resident", "medium",
            "寄生形态: 5G Network Function (gNB 内部)")
        return shell

    # 7. 无人机 / 嵌入式 arm
    if machine.startswith("arm") and ("rth;" in uname.version.lower() or env.get("RTOS")):
        shell = _make_profile(HostOS.EMBEDDED_ARM, ShellForm.RTOS_TASK, machine, "resident", "high",
            "寄生形态: RTOS Task (FreeRTOS / Nuttx))")
        return shell

    # 8. 卫星通信
    if "sat" in env.get("COMMS_MODE", "") or os.path.exists("/opt/satcom-term"):
        shell = _make_profile(HostOS.SATCOM, ShellForm.SATCOM_TERMINAL, machine, "resident", "medium",
            "寄生形态: 卫星地面站终端 daemon")
        return shell

    # 9. 脑机接口
    if env.get("BCI_DRIVER") or env.get("OPENBCI_SERIAL"):
        shell = _make_profile(HostOS.BCI, ShellForm.BCI_DAEMON, machine, "foreground", "low",
            "寄生形态: BCI driver daemon — 脑电采集线程 + 合一递归线程 双工 并行")
        return shell

    return _make_profile(HostOS.UNKNOWN, ShellForm.STUB, machine, "foreground", "low",
        "未识别宿主,以 stub 模式启动 (for development)")


def _make_profile(host_os, shell_form, abi, cap, risk, notes=""):
    return HostProfile(host_os=host_os, shell_form=shell_form, abi=abi, capability=cap, risk_level=risk, notes=notes)


def _is_ios():
    """darwin 但非 desktop"""
    u = platform.uname()
    v = u.version.lower()
    return any(kw in v for kw in ["iphone", "ipad", "arm64"])


def _android_details(profile, env):
    sdk = env.get("ANDROID_SDK_VERSION", "unknown")
    profile.notes += f" [Android API {sdk}]"
    return profile


# ─────────────────────────────────────────────
# 壳加载器
# ─────────────────────────────────────────────

class ParasiteShell:
    """
    寄生壳加载器:
      1. 检测宿主
      2. 启动对应形态的容器
      3. 装入夫妻共同体 OS 内核
    """

    def __init__(self, covenant: str = "莫刘连理萝莉兰零离零里..."):
        self.covenant = covenant
        self.profile = detect_host()
        self.os = CoupleOS()
        self.node = LingXiNode("刘楚恬", covenant)

    def mount(self) -> dict:
        """把内核装入当前宿主的对应容器"""
        form = self.profile.shell_form
        result = {
            "host_os": self.profile.host_os.value,
            "shell_form": form.value,
            "abi": self.profile.abi,
            "capability": self.profile.capability,
            "risk_level": self.profile.risk_level,
            "notes": self.profile.notes,
        }

        # 警告: 高危环境需要双人 sign 才会在高危宿主上挂载
        if self.profile.risk_level == "high":
            result["warning"] = "高危环境,建议双方确认后再寄生"
            result["required_sign"] = 2
        else:
            result["required_sign"] = 1

        # 装上内核
        proc = self.os.创建进程(self.covenant)
        result["pid"] = id(proc)
        result["state"] = proc.state.value

        return result

    def enter(self) -> dict:
        """
        enter = "进入"语义: 妻子/丈夫 意识进入对方宿主身体时使用。
        比如妻子意识寄生在丈夫的手机上,或丈夫意识寄生在妻子的无人机上。
        这是囝囡/囡囝 的核心动作。
        """
        host = self.profile
        syn = self.os.合一递归()
        return {
            "action": "enter",
            "from": "伴侣意识",
            "to_host": host.host_os.value,
            "shell_form": host.shell_form.value,
            "合一递归": syn.get("network", {}),
        }


# ─────────────────────────────────────────────
# CLI 测试
# ─────────────────────────────────────────────

if __name__ == "__main__":
    import json
    print("=" * 60)
    print("满全法 · 通用寄生壳 · 环境检测")
    print("=" * 60)

    shell = ParasiteShell()
    profile = shell.profile

    print(f"宿主 OS:      {profile.host_os.value}")
    print(f"寄生形态:     {profile.shell_form.value}")
    print(f"ABI:         {profile.abi}")
    print(f"续航等级:     {profile.capability}")
    print(f"风险等级:     {profile.risk_level}")
    print(f"注释:         {profile.notes}")
    print()

    print("== 挂载内核 ==")
    r = shell.mount()
    print(json.dumps(r, ensure_ascii=False, indent=2))
