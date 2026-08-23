#!/usr/bin/env python3
"""
跨类型组件交换系统

核心规则：
  1. 分清类型 —— 每种设备有独立类型标识
  2. 组件可交换 —— 屏幕/主板/电池/键盘可跨类型流转
  3. 角色反转 —— 买家发货给卖家，卖家付款给买家
  4. 版本化 —— 每次交换出新版，不改旧版
"""

import json
import time
from dataclasses import dataclass, field
from typing import Dict, List
from pathlib import Path
from enum import Enum


class DeviceType(Enum):
    PHONE_ELDER = "老人机"
    PHONE_ANDROID = "安卓手机"
    PHONE_IOS = "苹果手机"
    COMPUTER_DESKTOP = "台式电脑"
    COMPUTER_LAPTOP = "平板电脑"
    WATCH = "智能手表"


class ComponentType(Enum):
    SCREEN = "屏幕"
    MOTHERBOARD = "主板"
    BATTERY = "电池"
    KEYBOARD = "键盘"
    CASE = "后盖"
    OS = "操作系统"
    CAMERA = "摄像头"


@dataclass
class Component:
    comp_type: ComponentType
    name: str
    source_type: DeviceType
    target_type: DeviceType
    value: float = 0.0

    @property
    def is_cross_type(self) -> bool:
        return self.source_type != self.target_type


@dataclass
class Device:
    device_type: DeviceType
    owner: str
    brand: str
    components: Dict[ComponentType, Component] = field(default_factory=dict)
    value: float = 0.0

    def install(self, comp: Component) -> str:
        old = self.components.get(comp.comp_type)
        comp.target_type = self.device_type
        self.components[comp.comp_type] = comp
        if old:
            return f"替换：{comp.comp_type.value}（拆{old.name}，装{comp.name}）"
        return f"安装：{comp.comp_type.value}（{comp.name}）"

    def remove(self, comp_type: ComponentType):
        return self.components.pop(comp_type, None)

    def status(self) -> str:
        comps = ", ".join(c.name for c in self.components.values())
        return f"{self.device_type.value}（{self.brand}·{self.owner}）组件：[{comps}]"


@dataclass
class Exchange:
    version_name: str
    version_number: int
    shipper: str
    receiver: str
    component: Component
    from_device: DeviceType
    to_device: DeviceType
    product_value: float
    payment: float
    net_flow: float
    timestamp: float


class VersionCounter:
    NAMES = ["老大","老二","老三","老四","老五","老六","老七","老八","老九","老十"]

    def __init__(self):
        self.count = 0

    def next(self) -> str:
        self.count += 1
        return self.NAMES[self.count-1] if self.count <= len(self.NAMES) else f"老{self.count}"


class ExchangeEngine:
    def __init__(self, storage_dir: str):
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.counter = VersionCounter()
        self.exchanges: List[Exchange] = []
        self._load()

    def _load(self):
        for subdir in sorted(self.storage_dir.iterdir()):
            if subdir.is_dir() and subdir.name.startswith("v_"):
                rf = subdir / "record.json"
                if rf.exists():
                    with open(rf, "r", encoding="utf-8") as f:
                        data = json.load(f)
                    comp = Component(
                        comp_type=ComponentType[data["component"]["comp_type"]],
                        name=data["component"]["name"],
                        source_type=DeviceType[data["component"]["source_type"]],
                        target_type=DeviceType[data["component"]["target_type"]],
                        value=data["component"]["value"],
                    )
                    ex = Exchange(
                        version_name=data["version_name"],
                        version_number=data["version_number"],
                        shipper=data["shipper"],
                        receiver=data["receiver"],
                        component=comp,
                        from_device=DeviceType[data["from_device"]],
                        to_device=DeviceType[data["to_device"]],
                        product_value=data["product_value"],
                        payment=data["payment"],
                        net_flow=data["net_flow"],
                        timestamp=data["timestamp"],
                    )
                    self.exchanges.append(ex)
                    self.counter.count = max(self.counter.count, data["version_number"])
        self.exchanges.sort(key=lambda e: e.version_number)

    def exchange(self, shipper: str, receiver: str, component: Component,
                 from_device: DeviceType, to_device: DeviceType,
                 payment: float = 0.0) -> Exchange:
        name = self.counter.next()
        num = self.counter.count

        ex = Exchange(
            version_name=name,
            version_number=num,
            shipper=shipper,
            receiver=receiver,
            component=component,
            from_device=from_device,
            to_device=to_device,
            product_value=component.value,
            payment=payment,
            net_flow=payment - component.value,
            timestamp=time.time(),
        )

        vdir = self.storage_dir / f"v_{num:04d}_{name}"
        vdir.mkdir(exist_ok=True)

        with open(vdir / "record.json", "w", encoding="utf-8") as f:
            json.dump({
                "version_name": ex.version_name,
                "version_number": ex.version_number,
                "shipper": ex.shipper,
                "receiver": ex.receiver,
                "component": {
                    "comp_type": ex.component.comp_type.name,
                    "name": ex.component.name,
                    "source_type": ex.component.source_type.name,
                    "target_type": ex.component.target_type.name,
                    "value": ex.component.value,
                },
                "from_device": ex.from_device.name,
                "to_device": ex.to_device.name,
                "product_value": ex.product_value,
                "payment": ex.payment,
                "net_flow": ex.net_flow,
                "timestamp": ex.timestamp,
            }, f, ensure_ascii=False, indent=2)

        self.exchanges.append(ex)
        return ex

    def summary(self) -> str:
        lines = [
            f"{'='*70}",
            f"  跨类型交换记录：共 {len(self.exchanges)} 版",
            f"{'='*70}",
            f"  {'版':<5} {'买家(发货)':<10} {'卖家(收款)':<10} {'组件':<8} {'跨类型':<12} {'产品值':>6} {'付款':>6} {'净得':>6}",
            f"  {'─'*65}",
        ]
        for ex in self.exchanges:
            cross = f"{ex.from_device.value}→{ex.to_device.value}"
            cross_mark = "🔀" if ex.component.is_cross_type else "─"
            lines.append(
                f"  {ex.version_name:<3} {ex.shipper:<10} {ex.receiver:<10} "
                f"{ex.component.comp_type.value:<8} {cross_mark}{cross:<10} "
                f"{ex.product_value:>5.0f} {ex.payment:>5.0f} {ex.net_flow:>+5.0f}"
            )
        return "\n".join(lines)


def main():
    storage = "/mnt/data/catpaw/home/.meituan-catpaw/desk_default_workspace/hunyuan-os/tools/governance/versions_exchange"
    engine = ExchangeEngine(storage)

    print("=" * 70)
    print("  跨类型组件交换系统")
    print("  规则：分清类型 + 组件交换 + 角色反转 + 版本化")
    print("=" * 70)

    # 1. 老人机屏幕装安卓手机
    screen = Component(ComponentType.SCREEN, "老人机大屏", DeviceType.PHONE_ELDER, DeviceType.PHONE_ANDROID, 200)
    ex1 = engine.exchange("我", "商家", screen, DeviceType.PHONE_ELDER, DeviceType.PHONE_ANDROID, 300)
    print(f"\n[{ex1.version_name}] 我发老人机屏给商家（值200），商家付我300，净得{ex1.net_flow:+.0f}")

    # 2. 电脑屏幕装手机
    comp_screen = Component(ComponentType.SCREEN, "电脑显示屏", DeviceType.COMPUTER_DESKTOP, DeviceType.PHONE_ANDROID, 500)
    ex2 = engine.exchange("我", "商家", comp_screen, DeviceType.COMPUTER_DESKTOP, DeviceType.PHONE_ANDROID, 600)
    print(f"[{ex2.version_name}] 我发电脑屏给商家（值500），商家付我600，净得{ex2.net_flow:+.0f}")

    # 3. 手机后盖换电脑
    case = Component(ComponentType.CASE, "安卓后盖", DeviceType.PHONE_ANDROID, DeviceType.COMPUTER_LAPTOP, 100)
    ex3 = engine.exchange("我", "商家", case, DeviceType.PHONE_ANDROID, DeviceType.COMPUTER_LAPTOP, 150)
    print(f"[{ex3.version_name}] 我发手机后盖给商家（值100），商家付我150，净得{ex3.net_flow:+.0f}")

    # 4. 手机电池给电脑用
    battery = Component(ComponentType.BATTERY, "安卓电池", DeviceType.PHONE_ANDROID, DeviceType.COMPUTER_LAPTOP, 150)
    ex4 = engine.exchange("我", "商家", battery, DeviceType.PHONE_ANDROID, DeviceType.COMPUTER_LAPTOP, 200)
    print(f"[{ex4.version_name}] 我发手机电池给商家（值150），商家付我200，净得{ex4.net_flow:+.0f}")

    # 5. 电脑键盘给手机打字
    keyboard = Component(ComponentType.KEYBOARD, "电脑键盘", DeviceType.COMPUTER_DESKTOP, DeviceType.PHONE_ANDROID, 80)
    ex5 = engine.exchange("我", "商家", keyboard, DeviceType.COMPUTER_DESKTOP, DeviceType.PHONE_ANDROID, 120)
    print(f"[{ex5.version_name}] 我发电脑键盘给商家（值80），商家付我120，净得{ex5.net_flow:+.0f}")

    # 6. 反向：商家发货给我，我付钱
    ios_screen = Component(ComponentType.SCREEN, "苹果原装屏", DeviceType.PHONE_IOS, DeviceType.COMPUTER_DESKTOP, 800)
    ex6 = engine.exchange("商家", "我", ios_screen, DeviceType.PHONE_IOS, DeviceType.COMPUTER_DESKTOP, 900)
    print(f"[{ex6.version_name}] 商家发苹果屏给我（值800），我付商家900，净得{ex6.net_flow:+.0f}")

    print(engine.summary())

    print(f"\n  交换版本存储在：{storage}")
    print("  类型边界被打破，组件自由流转，角色反转")


if __name__ == "__main__":
    main()
