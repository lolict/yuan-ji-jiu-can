"""
Binary Mirror - one side moves, one stays

Key insight:
  A binary mirror is a pair (A, B) where one side can move,
  and one stays fixed. The question is: how much asymmetry is there?

  Equilibrium: both sides have equal gap → symmetric → stable
  Forced: one side has zero gap → unstable → catastrophic

  This models relationships: one person moves closer, the other stays.
  The "difference in tension" IS the power of the relationship.

  4 phase states:
    convergence: gap decreasing → growing together
    equilibrium: gap stable → smooth but static
    divergence: gap increasing → drifting apart
    collapse: gap → different signs → fight
"""
import math
from typing import Tuple
from enum import Enum


class ConvergenceType(Enum):
    CONVERGENCE = "convergence"
    EQUILIBRIUM = "equilibrium"
    DIVERGENCE = "divergence"
    COLLAPSE    = "collapse"


class BinaryMirror:
    def __init__(self, stationary: float, mobile: float):
        """
        stationary: the fixed side (e.g., origin = 0)
        mobile: the moving side (e.g., another person's position)
        """
        self.stationary = stationary
        self.mobile = mobile
        self._history = []

    @property
    def gap(self) -> float:
        """Current gap between stationary and mobile"""
        return self.mobile - self.stationary

    @property
    def tension(self) -> float:
        """
        Tension = |gap|. Zero gap = no tension.
        High tension = far apart = high energy.
        """
        return abs(self.gap)

    def is_symmetric(self, other_gap: float = 0.0, tolerance: float = 0.01) -> bool:
        """Is current gap symmetric to another gap (the pair's inverse)?"""
        return abs(self.gap - other_gap) < tolerance

    def move(self, delta: float) -> float:
        """Move the mobile side by delta. Returns new gap."""
        self.mobile += delta
        self._history.append((self.stationary, self.mobile))
        return self.gap

    def convergence_direction(self, historical_gaps: list) -> ConvergenceType:
        """
        Given historical gaps, determine what phase we're in.
        This is how we detect "are we growing closer or apart?"
        """
        if len(historical_gaps) < 2:
            return ConvergenceType.EQUILIBRIUM

        recent = historical_gaps[-5:]  # last 5 points
        if len(recent) < 2:
            recent = historical_gaps

        deltas = [recent[i] - recent[i - 1] for i in range(1, len(recent))]
        avg_delta = sum(deltas) / len(deltas)

        # Check if signs are opposite (collapse): but we're using |gap|
        signs = [1 if d > 0 else (-1 if d < 0 else 0) for d in deltas]
        if signs and len(set(signs)) > 1 and 0 not in signs:
            # Oscillating signs could mean fighting
            return ConvergenceType.COLLAPSE

        if abs(avg_delta) < 0.01:
            return ConvergenceType.EQUILIBRIUM
        elif avg_delta < 0:
            return ConvergenceType.CONVERGENCE
        else:
            return ConvergenceType.DIVERGENCE

    def rate(self, historical_gaps: list) -> float:
        """Rate of change of gaps (velocity of relationship)"""
        if len(historical_gaps) < 2:
            return 0.0
        return historical_gaps[-1] - historical_gaps[-2]

    def predict_next_gap(self, historical_gaps: list) -> float:
        """Simple linear prediction: where are we heading?"""
        if not historical_gaps:
            return self.gap
        elif len(historical_gaps) == 1:
            return historical_gaps[0]
        return historical_gaps[-1] + self.rate(historical_gaps)

    def duality(self) -> Tuple[float, float]:
        """Return the two possible states: (gap, -gap)"""
        return (self.gap, -self.gap)

    def invert(self) -> 'BinaryMirror':
        """Flip: stationary <-> mobile"""
        return BinaryMirror(self.mobile, self.stationary)

    def apply_force(self, force: float) -> float:
        """External force acts on mobile side. Force > 0 = closer, < 0 = apart"""
        # Damped movement
        self.move(-force)  # negative because force closer = gap reduces
        return self.gap

    def __repr__(self):
        return "Mirror(st={}, m={}, gap={:.3f})".format(self.stationary, self.mobile, self.gap)


def convergence_rate(phase_series: list) -> dict:
    """
    Given a series of (time, gap) tuples, compute convergence rate
    and classify the current phase.

    Returns dict with:
        rate: float (velocity)
        acceleration: float
        phase: ConvergenceType
        stability_score: 0..1 (1 = very stable)
    """
    gaps = [g for _, g in phase_series]
    times = [t for t, _ in phase_series]

    if len(gaps) < 2:
        return {
            "rate": 0.0,
            "acceleration": 0.0,
            "phase": ConvergenceType.EQUILIBRIUM,
            "stability_score": 1.0,
        }

    dt = times[-1] - times[0]
    if dt == 0:
        dt = 1.0
    rate = (gaps[-1] - gaps[0]) / dt

    # Acceleration
    if len(gaps) >= 3:
        v1 = (gaps[-2] - gaps[0]) / (times[-2] - times[0]) if times[-2] != times[0] else 0
        v2 = (gaps[-1] - gaps[-2]) / (times[-1] - times[-2]) if times[-1] != times[-2] else 0
        dt2 = times[-1] - times[-2]
        acceleration = (v2 - v1) / dt2 if dt2 != 0 else 0
    else:
        acceleration = 0.0

    # Classify
    if abs(rate) < 0.05:
        phase = ConvergenceType.EQUILIBRIUM
    elif rate < 0:
        phase = ConvergenceType.CONVERGENCE
    else:
        phase = ConvergenceType.DIVERGENCE

    # Stability: how "peaceful" the trajectory is
    if len(gaps) >= 3:
        diffs = [abs(gaps[i] - gaps[i - 1]) for i in range(1, len(gaps))]
        avg_diff = sum(diffs) / len(diffs)
        stability = 1.0 / (1.0 + avg_diff * 10)
    else:
        stability = 1.0

    return {
        "rate": round(rate, 6),
        "acceleration": round(acceleration, 6),
        "phase": phase,
        "stability_score": round(stability, 4),
    }


def growth_pattern(phase_series: list) -> str:
    """Classify the growth pattern"""
    result = convergence_rate(phase_series)
    rate = result["rate"]
    accel = result["acceleration"]

    if abs(rate) < 0.01:
        return "static"
    elif rate > 0 and accel > 0:
        return "accelerating_growth"
    elif rate > 0 and accel < 0:
        return "decelerating_growth"
    elif rate < 0 and accel < 0:
        return "accelerating_decline"
    elif rate > 0 and accel < 0:
        return "decelerating_decline"
    else:
        return "oscillating"


def _test():
    m = BinaryMirror(0, 5)
    print("[OK] mirror: gap={:.2f}, tension={:.2f}".format(m.gap, m.tension))

    m.move(-2)  # closer
    m.move(-1)  # closer
    m.move(0.5)  # slight pull apart
    print("[OK] after moves: history={}".format(len(m._history)))

    gaps_over_time = [(0, 5), (1, 4), (2, 3.5), (3, 2.8), (4, 2.2), (5, 1.9)]
    mirror = BinaryMirror(0, 1.9)
    phase = mirror.convergence_direction([g for _, g in gaps_over_time])
    print("[OK] phase: {}".format(phase.value))

    prediction = mirror.predict_next_gap([g for _, g in gaps_over_time])
    print("[OK] predicted gap: {:.2f}".format(prediction))

    # Duality
    d1, d2 = m.duality()
    print("[OK] duality: ({:.2f}, {:.2f})".format(d1, d2))

    # Invert
    inv = m.invert()
    print("[OK] inverted: {}".format(inv))

    # Convergence rate analysis
    result = convergence_rate(gaps_over_time)
    print("[OK] rate analysis: phase={}, rate={:.3f}, stability={:.3f}".format(
        result["phase"].value, result["rate"], result["stability_score"]))

    pattern = growth_pattern(gaps_over_time)
    print("[OK] growth pattern: {}".format(pattern))


if __name__ == "__main__":
    _test()
