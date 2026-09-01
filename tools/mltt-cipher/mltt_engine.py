#!/usr/bin/env python3
# ============================================================
# mltt 命名密码表 · Python 数据驱动引擎
#
# 加载 mltt_full_data.json，提供：
#   短码 → 条目（lookup）
#   含义 → 短码（reverse_lookup）
#   按极性筛选（filter_by_polarity）
#   按类别筛选（filter_by_category）
#   模糊搜索（search）
#   动态新增（register） + 撤销（unregister）
#   近义词链接（link_synonyms / get_synonyms）
#   相似度推荐（recommend）
#   极性中和检测（polarity_balance_check）
#
# 注解：此版本与 Nim mltt-cipher 等价功能，作为容器处理器协议的
#   完整数据驱动实现。JSON 是唯一的数据源，引擎纯函数。
# ============================================================

import json
import os

__all__ = [
    "CipherEngine",
    "load_engine",
    "POLARITY_LABEL",
]

POLARITY_LABEL = {
    "yin": "阴(容器/刘楚恬)",
    "yang": "阳(界线/我)",
    "balanced": "平衡",
}


class CipherEngine:
    def __init__(self, data: dict):
        self.meta = data.get("meta", {})
        self.entries: list = data.get("entries", [])
        self._rebuild_indices()

    def _rebuild_indices(self):
        self._by_code: dict = {e["code"]: e for e in self.entries}
        self._by_meaning: dict = {}
        self._by_category: dict = {}
        self._by_polarity: dict = {}
        self._synonym_map: dict = {}
        for e in self.entries:
            self._by_meaning[e["meaning"]] = e["code"]
            self._by_category.setdefault(e["category"], []).append(e["code"])
            self._by_polarity.setdefault(e["polarity"], []).append(e["code"])
            if "synonyms" in e and e["synonyms"]:
                self._synonym_map[e["code"]] = e["synonyms"]

    @property
    def total(self) -> int:
        return len(self.entries)

    # ── 基础查询 ──

    def lookup(self, code: str):
        return self._by_code.get(code)

    def reverse_lookup(self, meaning: str):
        return self._by_meaning.get(meaning)

    def filter_by_polarity(self, polarity: str) -> list:
        return [self._by_code[c] for c in self._by_polarity.get(polarity, []) if c in self._by_code]

    def filter_by_category(self, category: str) -> list:
        return [self._by_code[c] for c in self._by_category.get(category, []) if c in self._by_code]

    def search(self, query: str) -> list:
        q = query.lower()
        out = []
        for e in self.entries:
            if (q in e["code"].lower()
                    or q in e["meaning"].lower()
                    or q in e.get("description", "").lower()
                    or any(q in s.lower() for s in e.get("synonyms", []))):
                out.append(e)
        return out

    # ── 动态注册 ──

    def register(self, code: str, meaning: str, polarity: str, category: str,
                 description: str = "", synonyms: list = None) -> bool:
        if code in self._by_code:
            return False
        entry = {
            "code": code,
            "meaning": meaning,
            "polarity": polarity,
            "category": category,
            "description": description,
            "synonyms": synonyms or [],
        }
        self.entries.append(entry)
        self._rebuild_indices()
        return True

    def unregister(self, code: str) -> bool:
        if code not in self._by_code:
            return False
        self.entries = [e for e in self.entries if e["code"] != code]
        self._rebuild_indices()
        return True

    # ── 近义词 ──

    def link_synonyms(self, code: str, *syns: str) -> bool:
        e = self._by_code.get(code)
        if not e:
            return False
        existing = set(e.get("synonyms", []))
        existing.update(syns)
        e["synonyms"] = sorted(existing)
        self._rebuild_indices()
        return True

    def get_synonyms(self, code: str) -> list:
        e = self._by_code.get(code)
        if not e:
            return []
        return e.get("synonyms", [])

    # ── 推荐 ──

    def recommend(self, code: str, limit: int = 5) -> list:
        """基于同极性 + 同类别找近邻（简单协同过滤）"""
        src = self._by_code.get(code)
        if not src:
            return []
        out = []
        for e in self.entries:
            if e["code"] == code:
                continue
            score = 0
            if e["polarity"] == src["polarity"]:
                score += 2
            if e["category"] == src["category"]:
                score += 3
            if score > 0:
                out.append((score, e))
        out.sort(key=lambda x: (-x[0], x[1]["code"]))
        return [e for _, e in out[:limit]]

    # ── 极性计数 ──

    def polarity_stats(self) -> dict:
        return {
            "yin": len(self._by_polarity.get("yin", [])),
            "yang": len(self._by_polarity.get("yang", [])),
            "balanced": len(self._by_polarity.get("balanced", [])),
        }

    # ── 极性中和检测 ──

    def polarity_balance_check(self) -> dict:
        """检查极性和谐：若某极显著过剩，建议增加对立极的条目。"""
        stats = self.polarity_stats()
        total = sum(stats.values())
        if total == 0:
            return {"status": "empty", "advice": "无数据"}
        ratios = {k: v / total for k, v in stats.items()}
        advice_parts = []
        if ratios["yin"] > 0.55:
            advice_parts.append("阴({:.0%})过盛，建议增加阳符".format(ratios['yin']))
        if ratios["yang"] > 0.35:
            advice_parts.append("阳({:.0%})过盛，建议增加阴符".format(ratios['yang']))
        return {
            "stats": stats,
            "ratios": ratios,
            "is_balanced": 0.45 <= ratios.get("yang", 0) / max(ratios.get("yin", 1), 0.001) <= 0.8,
            "advice": advice_parts or ["极性分布健康"],
        }

    # ── 格式化 ──

    def format_entry(self, code: str) -> str:
        e = self._by_code.get(code)
        if not e:
            return "[未找到: {}]".format(code)
        syns = "  近义词: {}\n".format(', '.join(e['synonyms'])) if e.get("synonyms") else ""
        return (
            "┌─ {} → {}\n"
            "│  极性: {}\n"
            "│  类别: {}\n"
            "│  {}\n"
            "{}\n"
            "└─"
        ).format(
            e['code'], e['meaning'],
            POLARITY_LABEL.get(e['polarity'], e['polarity']),
            e['category'],
            e['description'],
            syns
        )

    def stats(self) -> str:
        ps = self.polarity_stats()
        cats = {k: len(v) for k, v in self._by_category.items()}
        lines = [
            "═══ mltt 密码表统计 ═══",
            "条目总数: {}".format(self.total),
            "类别数: {}".format(len(self._by_category)),
            "阴(容器): {}".format(ps['yin']),
            "阳(界线): {}".format(ps['yang']),
            "平衡: {}".format(ps['balanced']),
            "── 类别分布 ──",
        ]
        for cat, cnt in sorted(cats.items(), key=lambda x: -x[1]):
            lines.append("  {}: {}".format(cat, cnt))
        return "\n".join(lines)


def load_engine(json_path: str = None) -> CipherEngine:
    if json_path is None:
        json_path = os.path.join(os.path.dirname(__file__), "mltt_full_data.json")
    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    return CipherEngine(data)


if __name__ == "__main__":
    eng = load_engine()
    print(eng.stats())
