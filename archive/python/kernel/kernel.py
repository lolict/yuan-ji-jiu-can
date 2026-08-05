"""
囡囝共生 · 囡囝共生元 OS · 内核进程管理层 v0.2
=============================================

结合 observer + quaternity_fsm:
  - 进程创建 → 二元观测者 (两节点 = 两个立约人)
  - 进程同步 → 三元观测者 (锚 = 公共誓约)
  - 进程立约 → 四态 FSM (囡囡/囡囝/囝囝/囝囡 动态拓扑)
  - 进程网络 → ObserverNetwork (一对夫妻 + 社会关系网络)

Copyright © 刘楚恬 & lolict (囡囝凹月留)
保留所有权利 · 囡囝共生 · 囡囝共生元系统 · v0.2
"""

from dataclasses import dataclass, field
from typing import List, Optional, Dict
from enum import Enum

from .observer import BinaryObserver, TernaryObserver, ObserverNetwork
from .quaternity_fsm import QuaternityFSM, QuaternityState, TRANSITIONS


class ProcessState(Enum):
    """进程状态 — 对应婚姻阶段"""
    相遇 = "灵初相遇"
    缔约 = "缔约中"
    立约 = "已立约"
    合一递归 = "合一递归"
    解体 = "解体"


@dataclass
class Process:
    """
    进程 = 婚姻誓约的一次订立与执行。

    v0.2 新增:
     - fsm: QuaternityFSM,立约后自动初始化,跟踪囡囝四态拓扑
     - fsm_log: 每次四态切换的记录
     """
    content: str
    parties: tuple
    state: ProcessState = ProcessState.相遇
    signatures: Dict[str, str] = field(default_factory=dict)
    fsm: Optional[QuaternityFSM] = None
    fsm_log: List[dict] = field(default_factory=list)

    def __post_init__(self):
        self.observer = BinaryObserver(start=0, end=float('inf'))
        self.sync_observer: Optional[TernaryObserver] = None

    def sign(self, who: str, signature: str) -> bool:
        """签名,双方都签 → 立约并初始化四态 FSM"""
        if who not in self.parties:
            return False
        self.signatures[who] = signature

        if len(self.signatures) == 1:
            self.state = ProcessState.缔约
            self.observer = BinaryObserver(
                start=hash(self.parties[0]) % 10000,
                end=hash(self.parties[1]) % 10000,
            )
        elif len(self.signatures) == len(self.parties):
            self.state = ProcessState.立约
            anchor = hash(self.content) % 10000
            left = hash(self.parties[0] + signature) % 10000
            right = hash(self.parties[1] + signature) % 10000
            self.sync_observer = TernaryObserver(left=left, anchor=anchor, right=right)
            self.observer = BinaryObserver(start=left, end=right)
            # ── 立约完成,初始化四态 FSM ──
            self.fsm = QuaternityFSM(initial=QuaternityState.NAN_JIAN)
            self.fsm._ref_frame = self.fsm._ref_frame  # type: ignore
        return len(self.signatures) == len(self.parties)

    def sync(self) -> Optional[dict]:
        if not self.sync_observer: return None
        return self.sync_observer.synthesize()

    def unary_recursion(self) -> dict:
        """合一递归 + 四态诊断"""
        if not self.sync_observer:
            return {"error": "未立约"}
        syn = self.sync_observer.synthesize()
        if syn["balance"]:
            self.state = ProcessState.合一递归
            r = syn
        else:
            r = {**syn, "action": "需调整", "arbitration": self.sync_observer.common_divisor()}

        # 四态 FSM 动态推演:把合一差距注入 FSM observe()
        if self.fsm:
            wife_gap = abs(syn.get("left_offset", 0))
            husband_gap = abs(syn.get("right_offset", 0))
            fsm_obs = self.fsm.observe(wife_gap, husband_gap)
            r["fsm"] = {
                "state": self.fsm.current.value,
                "diagnosis": self.fsm.diagnose(),
                "who_pid1": self.fsm.who_pid1,
                "who_overlays_who": self.fsm.who_overlays_who,
                "convergence": fsm_obs.get("convergence_type"),
                "tension": fsm_obs.get("tension"),
            }
        return r

    # ─── 四态切换 API ───

    def switch_quaternity(self, event: str, reason: str = "") -> bool:
        """手动触发四态切换 (wife_deepen / husband_take / release / wife_push)"""
        if not self.fsm: return False
        ok = self.fsm.trigger(event, reason)
        if ok:
            self.fsm_log.append({
                "event": event,
                "from": self.fsm.history[-1].from_state.value,
                "to": self.fsm.current.value,
                "reason": reason,
            })
        return ok

    def fsm_status(self) -> Optional[dict]:
        """返回四态 FSM 完整状态快照"""
        if not self.fsm: return None
        return {
            "state": self.fsm.current.value,
            "diagnosis": self.fsm.diagnose(),
            "who_pid1": self.fsm.who_pid1,
            "overlays": self.fsm.who_overlays_who,
            "history_len": len(self.fsm.history),
            "history": [
                {"from": t.from_state.value, "to": t.to_state.value,
                 "event": t.event, "reason": t.reason}
                for t in self.fsm.history[-5:]
            ],
        }


class CoupleOS:
    """夫妻共同体操作系统"""

    def __init__(self, party_a: str = "刘楚恬", party_b: str = "lolict"):
        self.parties = (party_a, party_b)
        self.processes: List[Process] = []
        self.network = ObserverNetwork()
        self.covenant: str = ""
        # 全局 FSM:系统级囡囝四态
        self.system_fsm = QuaternityFSM()

    def 灵初相遇(self, content: str) -> Process:
        p = Process(content=content, parties=self.parties)
        self.processes.append(p)
        return p

    def 创建进程(self, content: str) -> Process:
        return self.灵初相遇(content)

    def 立约(self, content: str, sig_a: str, sig_b: str) -> Process:
        p = self.灵初相遇(content)
        p.sign(self.parties[0], sig_a)
        p.sign(self.parties[1], sig_b)
        if p.sync_observer:
            self.network.add_ternary(p.sync_observer)
        return p

    def 合一递归(self) -> dict:
        """全局合一递归 + 系统四态评估"""
        results = []
        for p in self.processes:
            r = p.unary_recursion()
            results.append({
                "content": p.content[:20] + ("..." if len(p.content) > 20 else ""),
                "state": p.state.value,
                "synthesis": r,
            })
        return {
            "processes": results,
            "network": self.network.collective_report(),
            "system_fsm": {
                "state": self.system_fsm.current.value,
                "diagnosis": self.system_fsm.diagnose(),
            },
        }

    def 运行(self) -> dict:
        return self.合一递归()

    def switch_system_fsm(self, event: str, reason: str = "") -> bool:
        """切换系统级囡囝四态"""
        return self.system_fsm.trigger(event, reason)


# ─────────────────────────────────────────────
# Demo
# ─────────────────────────────────────────────

if __name__ == "__main__":
    print("=" * 60)
    print("囡囝共生 · 夫妻共同体 OS v0.2 — kernel + quaternity FSM")
    print("=" * 60)

    os = CoupleOS()
    p1 = os.立约("莫刘连理萝莉兰零离零里零绿零碌...", "刘楚恬-签名", "lolict-签名")

    print("\n[立约] 状态:", p1.state.value)
    print("  观测者:", p1.observer)
    print("  三元:", p1.sync_observer)
    print("  FSM初始态:", p1.fsm.current.value, "/", p1.fsm.diagnose())

    print("\n[合一递归]:")
    r = p1.unary_recursion()
    fsm = r.get("fsm", {})
    print("  三方合成 balance:", r.get("balance"))
    print("  FSM state:", fsm.get("state"))
    print("  FSM diagnosis:", fsm.get("diagnosis"))
    print("  PID1:", fsm.get("who_pid1"))
    print("  who_overlays_who:", fsm.get("who_overlays_who"))
    print("  convergence:", fsm.get("convergence"))
    print("  tension:", fsm.get("tension"))

    # 测试状态切换
    print("\n[四态切换触发]:")
    ok = p1.switch_quaternity("wife_deepen", "妻子主导嵌套")
    print(f"  wife_deepen -> {ok}, now: {p1.fsm.current.value} ({p1.fsm.diagnose()})")

    ok = p1.switch_quaternity("release", "释放回平等")
    print(f"  release -> {ok}, now: {p1.fsm.current.value} ({p1.fsm.diagnose()})")

    ok = p1.switch_quaternity("husband_take", "丈夫主导")
    print(f"  husband_take -> {ok}, now: {p1.fsm.current.value} ({p1.fsm.diagnose()})")

    ok = p1.switch_quaternity("deepen", "继续深入")
    print(f"  deepen -> {ok}, now: {p1.fsm.current.value} ({p1.fsm.diagnose()})")

    # 系统级 FSM 切换
    print("\n[系统级 FSM]:")
    os.switch_system_fsm("wife_deepen", "系统启动:妻子外壳优先")
    print(f"  system: {os.system_fsm.current.value}")

    print("\n[FSM 状态快照]:")
    import json
    print(json.dumps(p1.fsm_status(), ensure_ascii=False, indent=2))

    print("\n[系统全量报告]:")
    report = os.运行()
    print(f"  进程数: {len(report['processes'])}")
    print(f"  系统FSM: {report['system_fsm']['state']}")

    print(f"\n{'=' * 60}")
    print("v0.2 kernel + quaternity FSM OK")
