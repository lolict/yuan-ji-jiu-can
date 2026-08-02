"""
Convergence module - combines all math layers
"""
from typing import Any, List, Tuple, Dict

from .morphism import Path, Morphism, Object
from .ternary_arbiter import TernaryArbiter
from .binary_mirror import BinaryMirror, ConvergenceType
from .relational import selection


class ReferenceFrame:
    """
    Absolute: the anchor (God's-eye view)
    Relative: the differences we perceive
    """

    def __init__(self, absolute_anchor: Any = 0, relative_origin: Any = 0):
        self.absolute = absolute_anchor
        self.relative = relative_origin

    def to_absolute(self, relative_pos: Any) -> Any:
        return relative_pos - self.relative + self.absolute

    def to_relative(self, absolute_pos: Any) -> Any:
        return absolute_pos - self.absolute + self.relative

    def residuals(self, a: Any, b: Any) -> tuple:
        ref = self.absolute
        ra = (a - ref) if isinstance(a, (int, float)) and isinstance(ref, (int, float)) else 0
        rb = (b - ref) if isinstance(b, (int, float)) and isinstance(ref, (int, float)) else 0
        return (ra, rb)


def detect_phase(timeseries: List[tuple]) -> Dict:
    """Classify the current phase from a timeseries"""
    if len(timeseries) < 2:
        return {"type": "insufficient_data", "confidence": 0.5, "direction": "unknown"}

    values = [float(v) if isinstance(v, (int, float)) else 0 for _, v in timeseries]
    deltas = [values[i] - values[i - 1] for i in range(1, len(values))]
    avg_delta = sum(deltas) / len(deltas)
    signs = [1 if d > 0.001 else (-1 if d < -0.001 else 0) for d in deltas]
    non_zero_signs = [s for s in signs if s != 0]
    sign_changes = 0
    if non_zero_signs:
        sign_changes = sum(1 for i in range(1, len(non_zero_signs))
                           if non_zero_signs[i] != non_zero_signs[i - 1])

    if sign_changes >= len(deltas) // 2:
        phase = "collapse"
        direction = "fighting"
    elif abs(avg_delta) < 0.01:
        phase = "equilibrium"
        direction = "stable"
    elif avg_delta < 0:
        phase = "convergence"   # gap decreasing = growing closer
        direction = "approach"
    else:
        phase = "divergence"   # gap increasing = drifting apart
        direction = "recede"

    total_range = max(values) - min(values)
    if total_range == 0:
        confidence = 1.0
    else:
        variance = sum((v - sum(values) / len(values)) ** 2 for v in values) / len(values)
        confidence = max(0.1, 1.0 - (variance / (total_range * total_range + 1)))

    return {
        "type": phase,
        "confidence": round(confidence, 4),
        "direction": direction,
        "avg_delta": round(avg_delta, 6),
        "sign_changes": sign_changes,
    }


def growth_pattern(timeseries: List[tuple]) -> str:
    """Classify growth pattern"""
    result = detect_phase(timeseries)
    phase = result["type"]
    direction = result["direction"]
    if phase == "equilibrium":
        return "static"
    elif phase == "convergence":
        return "growing_closer"
    elif phase == "divergence":
        return "drifting_apart"
    elif phase == "collapse":
        return "conflict_zone"
    else:
        return "transient"


def _test():
    ref = ReferenceFrame(absolute_anchor=10, relative_origin=0)
    print("[OK] ref: abs(5) = {}".format(ref.to_absolute(5)))
    print("[OK] ref: residuals(8,12) = {}".format(ref.residuals(8, 12)))

    history = [(0, 10.0), (1, 8.5), (2, 7.2), (3, 5.8), (4, 4.5), (5, 3.8)]
    phase = detect_phase(history)
    print("[OK] detect_phase: {}, conf={}".format(phase["type"], phase["confidence"]))

    pattern = growth_pattern(history)
    print("[OK] growth_pattern: {}".format(pattern))

    eq_history = [(t, 5 + (t % 2) * 0.01) for t in range(10)]
    phase_eq = detect_phase(eq_history)
    print("[OK] equilibrium phase: {}".format(phase_eq["type"]))

    fight = [(0, 5), (1, 8), (2, 3), (3, 9), (4, 2), (5, 10)]
    phase_fight = detect_phase(fight)
    print("[OK] fight phase: {}".format(phase_fight["type"]))


if __name__ == "__main__":
    _test()
