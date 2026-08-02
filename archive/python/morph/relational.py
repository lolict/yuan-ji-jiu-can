"""
Relational Algebra
"""
from typing import List, Dict, Any, Callable, Tuple, Set
import enum


Name = str
Tuple_ = Dict[str, Any]
Relation_ = List[Tuple_]


def selection(rel: Relation_, predicate: Callable[[Tuple_], bool]) -> Relation_:
    """select: filter tuples"""
    return [t for t in rel if predicate(t)]


def projection(rel: Relation_, attrs: List[Name]) -> Relation_:
    """project: extract columns"""
    result = []
    for t in rel:
        row = {}
        for a in attrs:
            if a in t:
                row[a] = t[a]
        result.append(row)
    return result


def natural_join(r1: Relation_, r2: Relation_, on=None) -> Relation_:
    """join two relations"""
    if not r1 or not r2:
        return []
    if on is None:
        common = set(r1[0].keys()) & set(r2[0].keys())
        def _default(a, b):
            return all(a.get(k) == b.get(k) for k in common)
        on = _default
    result = []
    for a in r1:
        for b in r2:
            if on(a, b):
                merged = dict(a)
                merged.update(b)
                result.append(merged)
    return result


def division(r1: Relation_, r2: Relation_, r1_attrs: List[Name], r2_attrs: List[Name]) -> Relation_:
    """relational division"""
    if not r2:
        return r1
    r2_vals = set()
    for t in r2:
        key = tuple(t.get(a) for a in r2_attrs)
        r2_vals.add(key)
    r1_groups = {}
    for t in r1:
        key = tuple(t.get(a) for a in r1_attrs)
        if key not in r1_groups:
            r1_groups[key] = set()
        sub = tuple(t.get(a) for a in r2_attrs)
        r1_groups[key].add(sub)
    result = []
    for key, vals in r1_groups.items():
        if r2_vals.issubset(vals):
            result.append(dict(zip(r1_attrs, key)))
    return result


def transitive_closure(rel: Relation_, from_attr: Name, to_attr: Name) -> Relation_:
    """transitive closure: if A->B and B->C, add A->C"""
    closure = list(rel)
    changed = True
    safety = 0
    while changed and safety < 100:
        changed = False
        safety += 1
        existing = set()
        for t in closure:
            existing.add((t.get(from_attr), t.get(to_attr)))
        for a in closure:
            for b in closure:
                if a.get(to_attr) == b.get(from_attr):
                    pair = (a.get(from_attr), b.get(to_attr))
                    if pair not in existing:
                        closure.append({from_attr: pair[0], to_attr: pair[1]})
                        existing.add(pair)
                        changed = True
    return closure


class RelOp(enum.Enum):
    SELECT = "select"
    PROJECT = "project"
    JOIN = "join"
    DIVIDE = "divide"


class Query:
    """declarative query"""

    def __init__(self, rel: Relation_):
        self.rel = rel
        self.ops = []

    def select(self, pred: Callable[[Tuple_], bool]) -> 'Query':
        self.ops.append((RelOp.SELECT, pred))
        return self

    def project(self, attrs: List[Name]) -> 'Query':
        self.ops.append((RelOp.PROJECT, attrs))
        return self

    def join(self, other: Relation_, on=None) -> 'Query':
        self.ops.append((RelOp.JOIN, (other, on)))
        return self

    def exec(self) -> Relation_:
        result = list(self.rel)
        for op_name, arg in self.ops:
            if op_name == RelOp.SELECT:
                result = selection(result, arg)
            elif op_name == RelOp.PROJECT:
                result = projection(result, arg)
            elif op_name == RelOp.JOIN:
                other, on = arg
                result = natural_join(result, other, on)
        return result


def _test():
    bijection = [
        {"left": "v", "right": "y", "pair": "vy", "diff_type": "swap"},
        {"left": "y", "right": "v", "pair": "yv", "diff_type": "swap"},
        {"left": "t", "right": "f", "pair": "tf", "diff_type": "swap"},
        {"left": "f", "right": "t", "pair": "ft", "diff_type": "swap"},
        {"left": "c", "right": "k", "pair": "ck", "diff_type": "swap"},
        {"left": "l", "right": "r", "pair": "lr", "diff_type": "vibswap"},
    ]

    swaps = selection(bijection, lambda t: t["diff_type"] == "swap")
    print("[OK] select swaps: {}".format(len(swaps)))

    pairs = projection(swaps, ["pair"])
    print("[OK] project pairs: {}".format(len(pairs)))

    role_map = [
        {"initial": "v", "role": "Wei"},
        {"initial": "y", "role": "Xing"},
        {"initial": "t", "role": "Tai"},
        {"initial": "f", "role": "Feng"},
    ]

    joined = natural_join(swaps, role_map,
        on=lambda a, b: a["left"] == b["initial"] or a["right"] == b["initial"])
    print("[OK] join result: {} rows".format(len(joined)))

    chain = [
        {"from": "v", "to": "y"},
        {"from": "y", "to": "f"},
        {"from": "f", "to": "k"},
    ]
    closure = transitive_closure(chain, "from", "to")
    print("[OK] transitive closure: {} pairs".format(len(closure)))

    result = Query(bijection).select(lambda t: t["diff_type"] == "swap").project(["pair", "left", "right"]).exec()
    print("[OK] query result: {} rows".format(len(result)))


if __name__ == "__main__":
    _test()
