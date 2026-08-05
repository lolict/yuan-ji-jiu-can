"""
囡囝四态 · 形式化自测
======================

四种主导权拓扑,各有:
  - 外壳意识 (init / scheduler)
  - 内核意识 (被包裹 / 寄生)
  - 资源流向 (外壳分配资源给内核)
  - 故障边界 (外壳崩溃则内核也跟着死;内核崩溃则外壳可重启内核)

四态自由切换条件:
  - 双方 sign 同意切换
  - 锚点 hash 必须保持不变 (同一条誓约,只是重新分配 namespace)
  - 切换期间所有非 volatile 状态落盘 (snapshot)

Copyright © 刘楚恬 & lolict (囡囝凹月留)
保留所有权利 · 囡囝共生 · v0.1
"""

import unittest
from observer import BinaryObserver, TernaryObserver
from mythology import MYTHOS, lookup


class TestQuaternityTopology(unittest.TestCase):
    """四态的基本代数"""

    def test_four_states_defined(self):
        topo = MYTHOS["nan_jian_topology"]["quaternity"]
        self.assertIn("nan_nan", topo)
        self.assertIn("nan_jian", topo)
        self.assertIn("jian_jian", topo)
        self.assertIn("jian_nan", topo)

    def test_dominance_is_defined(self):
        topo = MYTHOS["nan_jian_topology"]["quaternity"]
        for key, state in topo.items():
            self.assertIn("dominance", state, f"{key} missing dominance")
            self.assertIn(state["dominance"], ["妻子意识", "丈夫意识"])

    def test_symmetry(self):
        topo = MYTHOS["nan_jian_topology"]["quaternity"]
        self.assertNotEqual(
            topo["nan_nan"]["dominance"],
            topo["jian_jian"]["dominance"],
        )
        self.assertNotEqual(
            topo["nan_jian"]["dominance"],
            topo["jian_nan"]["dominance"],
        )

    def test_nan_nan_she_is_pid1(self):
        self.assertEqual(
            MYTHOS["nan_jian_topology"]["quaternity"]["nan_nan"]["dominance"],
            "妻子意识",
        )

    def test_jian_jian_he_is_pid1(self):
        self.assertEqual(
            MYTHOS["nan_jian_topology"]["quaternity"]["jian_jian"]["dominance"],
            "丈夫意识",
        )

    def test_jian_nan_not_equal_mount(self):
        desc = MYTHOS["nan_jian_topology"]["quaternity"]["jian_nan"]["code"]
        self.assertIn("进入", desc)

    def test_lookup_each_state(self):
        for name in ["nan_nan", "nan_jian", "jian_jian", "jian_nan"]:
            r = MYTHOS["nan_jian_topology"]["quaternity"][name]
            self.assertIn("dominance", r, f"missing dominance in {name}")


class TestStateTransitions(unittest.TestCase):

    def test_anchor_preserved_across_switch(self):
        covenant = "莫刘连理萝莉兰零离零里..."
        anchor = hash(covenant) % 10000
        ob1 = TernaryObserver(left=anchor-3, anchor=anchor, right=anchor+3)
        ob2 = TernaryObserver(left=anchor-5, anchor=anchor, right=anchor+5)
        self.assertEqual(ob1.anchor, ob2.anchor)

    def test_snapshot_save_load(self):
        anchor = hash("covenant-test") % 10000
        obs = TernaryObserver(left=anchor-7, anchor=anchor, right=anchor+7)
        snap = {"left": obs.left, "anchor": obs.anchor, "right": obs.right}
        obs2 = TernaryObserver(**snap)
        self.assertEqual(obs.anchor, obs2.anchor)

    def test_all_legal(self):
        topo = MYTHOS["nan_jian_topology"]["quaternity"]
        for k, v in topo.items():
            self.assertNotIn("非法", v.get("meaning", ""))


class TestStateMachine(unittest.TestCase):

    def _install_state(self, state, covenant):
        anchor = hash(covenant) % 10000
        topo = MYTHOS["nan_jian_topology"]["quaternity"]
        st = topo[state]
        if st["dominance"] == "妻子意识":
            left, right = anchor - 5, anchor + 3
        else:
            left, right = anchor - 3, anchor + 5
        return TernaryObserver(left=left, anchor=anchor, right=right)

    def test_switch_preserves_anchor(self):
        covenant = "contract"
        states = ["nan_nan", "nan_jian", "jian_jian", "jian_nan"]
        anchors = {self._install_state(s, covenant).anchor for s in states}
        self.assertEqual(len(anchors), 1)

    def test_switch_alters_topology(self):
        covenant = "contract"
        a = self._install_state("nan_jian", covenant)
        b = self._install_state("jian_jian", covenant)
        self.assertNotEqual(
            a.to_anchor["left_to_anchor"],
            b.to_anchor["left_to_anchor"],
        )


class TestObserverDuality(unittest.TestCase):

    def test_nan_as_binary_shell(self):
        shell = BinaryObserver(start=0, end=100)
        inner = BinaryObserver(start=20, end=80)
        self.assertGreaterEqual(shell.abs_gap, inner.abs_gap)

    def test_jian_as_ternary_anchor(self):
        anchor = 50
        obs = TernaryObserver(left=20, anchor=anchor, right=80)
        self.assertEqual(obs.anchor, anchor)


if __name__ == "__main__":
    unittest.main(verbosity=2)
