"""
ManQuanFa Quaternity FSM - four state topology
"""
import time
from enum import Enum
from typing import Dict, Optional, Tuple, List
from dataclasses import dataclass, field
from morph.convergence import ReferenceFrame, detect_phase, growth_pattern
from morph.binary_mirror import BinaryMirror


class QuaternityState(Enum):
    NAN_JIAN  = "nan_jian"
    NAN_NAN   = "nan_nan"
    JIAN_JIAN = "jian_jian"
    JIAN_NAN  = "jian_nan"


TRANSITIONS = {
    (QuaternityState.NAN_JIAN,   "wife_deepen"):  QuaternityState.NAN_NAN,
    (QuaternityState.NAN_JIAN,   "husband_take"): QuaternityState.JIAN_JIAN,
    (QuaternityState.NAN_JIAN,   "balance"):      QuaternityState.NAN_JIAN,
    (QuaternityState.NAN_NAN,    "release"):      QuaternityState.NAN_JIAN,
    (QuaternityState.NAN_NAN,    "deepen"):       QuaternityState.NAN_NAN,
    (QuaternityState.JIAN_JIAN,  "release"):      QuaternityState.NAN_JIAN,
    (QuaternityState.JIAN_JIAN,  "deepen"):       QuaternityState.JIAN_NAN,
    (QuaternityState.JIAN_NAN,   "release"):      QuaternityState.JIAN_JIAN,
    (QuaternityState.JIAN_NAN,   "wife_push"):    QuaternityState.NAN_JIAN,
}


@dataclass
class Transition:
    ts: float
    from_state: QuaternityState
    to_state: QuaternityState
    event: str
    reason: str


@dataclass
class QuaternityFSM:
    initial: QuaternityState = QuaternityState.NAN_JIAN
    current: QuaternityState = None
    history: List[Transition] = field(default_factory=list)
    _ref_frame: ReferenceFrame = None

    def __post_init__(self):
        self.current = self.initial
        self._ref_frame = ReferenceFrame(absolute_anchor=0, relative_origin=0)

    def trigger(self, event: str, reason: str = "") -> bool:
        key = (self.current, event)
        if key not in TRANSITIONS:
            return False
        ns = TRANSITIONS[key]
        self.history.append(Transition(time.time(), self.current, ns, event, reason))
        self.current = ns
        return True

    def observe(self, wife_gap: float, husband_gap: float) -> Dict:
        m = BinaryMirror(wife_gap, husband_gap)
        res = self._ref_frame.residuals(wife_gap, husband_gap)
        ph = m.convergence_direction([abs(wife_gap), abs(husband_gap)])
        return {"convergence_type": ph.value, "tension": m.tension, "residuals": res, "state": self.current.value}

    def diagnose(self) -> str:
        s = self.current
        if s == QuaternityState.NAN_JIAN:   return "equal: both see each other"
        elif s == QuaternityState.NAN_NAN:  return "wife-dominant: husband nests inside"
        elif s == QuaternityState.JIAN_JIAN:return "husband-dominant: wife nests inside"
        else: return "husband-non-equal-dom: power imbalance"

    @property
    def who_pid1(self): return "wife" if self.current in (QuaternityState.NAN_JIAN, QuaternityState.NAN_NAN) else "husband"
    @property
    def who_overlays_who(self):
        m = {QuaternityState.NAN_JIAN:("wife","husband,equal"), QuaternityState.NAN_NAN:("wife","husband,nested"),
             QuaternityState.JIAN_JIAN:("husband","wife,nested"), QuaternityState.JIAN_NAN:("husband","wife,non-equal")}
        return m[self.current]
    def __repr__(self): return "FSM({})".format(self.current.value)


def _test():
    fsm = QuaternityFSM()
    assert fsm.trigger("wife_deepen"); assert fsm.current == QuaternityState.NAN_NAN
    assert fsm.trigger("release"); assert fsm.current == QuaternityState.NAN_JIAN
    assert fsm.trigger("husband_take"); assert fsm.current == QuaternityState.JIAN_JIAN
    assert fsm.trigger("deepen"); assert fsm.current == QuaternityState.JIAN_NAN
    assert fsm.trigger("wife_push"); assert fsm.current == QuaternityState.NAN_JIAN
    print("[OK] FSM ok transitions={} diag={}".format(len(fsm.history), fsm.diagnose()))

if __name__ == "__main__":
    _test()
