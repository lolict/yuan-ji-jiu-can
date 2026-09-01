#!/usr/bin/env python3
# ============================================================
# 全映射引擎 · Python 五系统统一
#
# 打通：九元层级 ↔ YinYang极性 ↔ 二进制 ↔ mltt短码 ↔ 高维偏移率
#
# 数据来源：fm_data_full.json
# 等价于 Nim 版 tools/full-mapping/fm/（mapper.nim + types.nim）
# ============================================================

import json
import math
import os

__all__ = [
    "FullMappingEngine",
    "load_full_mapping_engine",
    "POLARITY",
]

POLARITY = {
    "yin": "阴 · 隐藏 · 容器 · 刘楚恬",
    "yang": "阳 · 显现 · 界线 · 我",
    "balanced": "平衡 · 纠缠 · 叠加态 · 九层",
}


class FullMappingEngine:
    def __init__(self, data: dict):
        self.meta = data.get("meta", {})
        self.entries = data.get("entries", [])
        self._by_level = {e["level"]: e for e in self.entries}
        self._by_mltt = {e["mltt"]: e for e in self.entries}
        self._by_binary = {e["binary"]: e for e in self.entries}
        self._by_polarity = {}
        for e in self.entries:
            self._by_polarity.setdefault(e["polarity"], []).append(e["level"])

    @property
    def total_levels(self) -> int:
        return len(self.entries)

    # ── 层级 → 五系统等价表示 ──

    def get_level(self, level: int) -> dict:
        return self._by_level.get(level)

    def offset_rate(self, level: int) -> float:
        """偏移率 = level × 0.1"""
        return level * 0.1

    def multiplier(self, level: int) -> float:
        """乘数 = 1 + 偏移率"""
        return 1.0 + self.offset_rate(level)

    def static_regression(self, value: float, from_level: int) -> float:
        """平行世界数值折算回初代数"""
        if from_level <= 0:
            return value
        return value / self.multiplier(from_level)

    def convert_to_level(self, value: float, from_level: int, to_level: int) -> float:
        """跨层级换算"""
        base = self.static_regression(value, from_level)
        if to_level <= 0:
            return base
        return base * self.multiplier(to_level)

    # ── 极性 ↔ YinYang ↔ 二进制 ──

    @staticmethod
    def polarity_for_level(level: int) -> str:
        if level == 8:
            return "balanced"
        return "yang" if level % 2 == 0 else "yin"

    @staticmethod
    def polarity_to_yy(polarity: str) -> str:
        return {"yin": "阴", "yang": "阳", "balanced": "平衡"}.get(polarity, "?")

    @staticmethod
    def polarity_to_binary(polarity: str) -> str:
        return {"yin": "0", "yang": "1", "balanced": "0.5"}.get(polarity, "?")

    # ── 俘获 ──

    def capture_level(self, value: float, from_level: int) -> dict:
        base_val = self.static_regression(value, from_level)
        return {
            "original_level": from_level,
            "target_level": 0,
            "original_value": value,
            "converted_value": base_val,
            "conv_method": "capture",
            "notes": f"第{from_level}代 → 俘获为初代数 {base_val:.4f}",
        }

    # ── 阶和 ──

    def tier_sum_sharpness(self, levels: list) -> float:
        """给定层级集合的锐度（偏移量标准差）"""
        rates = [self.offset_rate(l) for l in levels]
        mean = sum(rates) / len(rates)
        var = sum((r - mean) ** 2 for r in rates) / len(rates)
        return math.sqrt(var)

    # ── 2²⁵⁶ 高位对齐 ──

    def high_address_space(self) -> str:
        return "2^256 ≈ 1.16e77"

    def alignment_id(self, level: int) -> str:
        off = self.offset_rate(level)
        return f"L{level} @(δ={off:.2f})"

    # ── 全映射报告 ──

    def full_map(self, symbol: str) -> dict:
        result = {"symbol": symbol, "total_levels": self.total_levels, "entries": []}
        for e in self.entries:
            entry = {
                "level": e["level"],
                "level_name": e["level_name"],
                "chinese_name": e["chinese_name"],
                "offset_rate": e["offset_rate"],
                "multiplier": e["multiplier"],
                "polarity": e["polarity"],
                "yin_yang": e["yin_yang"],
                "binary_string": e["binary"],
                "base10_value": e["base10"],
                "mltt_code": e["mltt"],
                "description": e["description"],
                "symbol_at_level": f"{symbol} @ {e['chinese_name']}",
            }
            result["entries"].append(entry)
        return result

    # ── 格式化 ──

    def format_entry(self, level: int) -> str:
        e = self._by_level.get(level)
        if not e:
            return f"[层级 {level} 未找到]"
        return (
            f"{e['level_name']:4s}({e['chinese_name']:4s}) δ={e['offset_rate']:.2f} "
            f"M={e['multiplier']:.2f} {e['yin_yang']:4s} {e['binary']:5s} {e['mltt']:8s}"
        )

    def polarity_summary(self, level: int) -> str:
        e = self._by_level.get(level)
        if not e:
            return ""
        pol_desc = POLARITY.get(e["polarity"], "未知")
        return f"Level {level+1} ({e['level_name']}): 极性={e['yin_yang']} → YinYang={e['yin_yang']} → 二进制={e['binary']}  |  {pol_desc}"

    def stats(self) -> str:
        yin = len(self._by_polarity.get("yin", []))
        yang = len(self._by_polarity.get("yang", []))
        balanced = len(self._by_polarity.get("balanced", []))
        return (
            "═══ 全映射引擎 ═══\n"
            f"层级数: {self.total_levels}\n"
            f"极性: 阴={yin}  阳={yang}  平衡={balanced}\n"
            f"寻址空间: {self.high_address_space()}\n"
            f"层级范围: {self.entries[0]['level_name']} — {self.entries[-1]['level_name']}"
        )


def load_full_mapping_engine(json_path: str = None) -> FullMappingEngine:
    if json_path is None:
        json_path = os.path.join(os.path.dirname(__file__), "fm_data_full.json")
    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    return FullMappingEngine(data)


if __name__ == "__main__":
    eng = load_full_mapping_engine()
    print(eng.stats())
    print()
    for lvl in range(9):
        print(eng.format_entry(lvl))
