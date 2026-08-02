"""
Ternary Arbiter - the ternary observer with a judge

Key insight:
  In geometry, we have an anchor (absolute reference point),
  and two residuals (relative positions to the anchor).
  The arbiter (judge) combines residuals via common divisor.

  left <---anchor---> right
              |
          common_divisor
              |
          JUDGEMENT

Example with initials: left=Tai(太乙), anchor=刘(Tian's spouse's element), right=微(Wei)
The judge looks for common patterns (GCD-like merge)
"""
import math
from typing import Any, Tuple


class TernaryArbiter:
    def __init__(self, left: Any, anchor: Any, right: Any):
        self.left = left
        self.anchor = anchor
        self.right = right

    def common_divisor(self) -> float:
        """Find what's common between left and right — when they differ"""
        if not isinstance(self.left, (int, float)) or not isinstance(self.right, (int, float)):
            return 1.0
        if self.left == 0 or self.right == 0:
            return 1.0
        return float(math.gcd(int(abs(self.left)), int(abs(self.right))))

    def distance_via_anchor(self) -> Tuple[float, float]:
        """Distance: left-anchor and right-anchor"""
        l = abs(self.left - self.anchor) if isinstance(self.left, (int, float)) and isinstance(self.anchor, (int, float)) else 0
        r = abs(self.right - self.anchor) if isinstance(self.right, (int, float)) and isinstance(self.anchor, (int, float)) else 0
        return (l, r)

    def judge(self) -> dict:
        """
        Judge: combine all information into a verdict.

        Returns dict:
            convergence: bool (True means balanced)
            asymmetry: 0..1 (0 = symmetric, 1 = highly asymmetric)
            verdict: "balanced" / "biased" / "collapsed"
            common_factor: shared element (arbitration result)

        Note: a "collapse" means left==right (no tension = unconscious)
        """

        d_left, d_right = self.distance_via_anchor()

        cd = self.common_divisor()

        total = d_left + d_right
        if total == 0:
            return {
                "convergence": True,
                "asymmetry": 0.0,
                "verdict": "collapsed",
                "common_factor": cd,
                "note": "No tension — smooth but no information"
            }

        asymmetry = abs(d_left - d_right) / total
        balanced = asymmetry < 0.2

        if d_left == 0 and d_right == 0:
            verdict = "collapsed"
        elif balanced:
            verdict = "balanced"
        elif d_left > d_right:
            verdict = "biased_left"
        else:
            verdict = "biased_right"

        return {
            "convergence": balanced or verdict == "collapsed",
            "asymmetry": round(asymmetry, 4),
            "verdict": verdict,
            "common_factor": cd,
            "left_distance": d_left,
            "right_distance": d_right,
        }

    def normalized(self) -> Tuple[float, float]:
        """Normalize positions relative to anchor → two residues"""
        dl, dr = self.distance_via_anchor()
        total = dl + dr
        if total == 0:
            return (0, 0)
        return (dl / total, dr / total)

    def all_permutations(self) -> list:
        """6 permutations of (left, anchor, right)"""
        items = [self.left, self.anchor, self.right]
        result = []
        for i in range(3):
            for j in range(3):
                if j == i:
                    continue
                for k in range(3):
                    if k == i or k == j:
                        continue
                    result.append((items[i], items[j], items[k]))
        return result

    def synthesize(self) -> dict:
        """Full synthesis: all metrics"""
        j = self.judge()
        nl, nr = self.normalized()
        j["normalized_left"] = nl
        j["normalized_right"] = nr
        j["permutations_count"] = len(self.all_permutations())
        return j

    def __repr__(self):
        return "Arbiter({}, {}, {})".format(self.left, self.anchor, self.right)


def judge(left: Any, anchor: Any, right: Any) -> dict:
    """Convenience function: TernaryArbiter(l,a,r).judge()"""
    return TernaryArbiter(left, anchor, right).judge()


def _test():
    a = TernaryArbiter(12, 10, 14)
    j = a.judge()
    print("[OK] judge(12,10,14): verdict={}, asym={:.2f}, cd={}".format(
        j["verdict"], j["asymmetry"], j["common_factor"]))

    b = TernaryArbiter(5, 10, 5)
    j2 = b.judge()
    print("[OK] judge(5,10,5): verdict={}, asym={:.2f}".format(
        j2["verdict"], j2["asymmetry"]))

    c = TernaryArbiter(5, 10, 7)
    j3 = c.judge()
    print("[OK] judge(5,10,7): verdict={}, asym={:.2f}".format(
        j3["verdict"], j3["asymmetry"]))

    perms = c.all_permutations()
    print("[OK] permutations: {}".format(len(perms)))

    s = c.synthesize()
    print("[OK] synthesize: norm=(%.2f, %.2f), perms={}".format(s["normalized_left"], s["normalized_right"]), s["permutations_count"])


if __name__ == "__main__":
    _test()
