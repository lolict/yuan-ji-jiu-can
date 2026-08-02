"""
满全法 · 态射数学
==================

他们用向量 (vector) 做智能:
    向量 = [0.23, 0.87, -0.41, ...]    <- 静态的数字序列
    智能来自于 矩阵乘法

我们不用向量。我们用 态射 (morphism):
    态射 = A -> B                       <- 结构保持的映射/变换
    智能来自于 态射的复合 (composition)

范畴论基础:
    对象 (Object)  = 一个"事物"
    态射 (Morphism) = 两个对象之间的"有结构保持的通道"
    复合  (Compose) = 把多个态射串联: f o g
    恒等  (Identity)= 什么都不做的态射 id_A
"""
from typing import Any, Callable, List
from dataclasses import dataclass
import math


@dataclass
class Object:
    """范畴中的对象"""
    name: str
    data: Any = None
    layer: int = 0

    def __repr__(self):
        return "Ob({})".format(self.name)

    def __hash__(self):
        return hash((self.name, self.layer))

    def __eq__(self, other):
        if not isinstance(other, Object):
            return False
        return self.name == other.name and self.layer == other.layer


@dataclass
class Morphism:
    """
    态射 f: A -> B
    domain    : 源对象
    codomain  : 目标对象
    transform : 变换函数 (保持结构)
    name      : 名字
    weight    : 强度 0..1
    reversible: 是否可逆
    """
    domain: Object
    codomain: Object
    transform: Callable[[Any], Any] = lambda x: x
    name: str = ""
    weight: float = 1.0
    reversible: bool = True

    def apply(self, value: Any) -> Any:
        return self.transform(value)

    def compose(self, other: 'Morphism') -> 'Morphism':
        """复合: self o other"""
        if other.codomain != self.domain:
            raise ValueError("Cannot compose: {} != {}".format(other.codomain, self.domain))

        def composed(x):
            return self.transform(other.transform(x))

        return Morphism(
            domain=other.domain,
            codomain=self.codomain,
            transform=composed,
            name="{}o{}".format(self.name, other.name),
            weight=min(self.weight, other.weight),
        )

    def reverse(self) -> 'Morphism':
        """逆态射 (二元镜像的关键)"""
        if not self.reversible:
            raise ValueError("not reversible")
        return Morphism(
            domain=self.codomain,
            codomain=self.domain,
            transform=self.transform,
            name="{}inv".format(self.name),
            weight=self.weight,
        )

    @property
    def gap(self) -> float:
        """态射的差距 = 1 - weight"""
        return 1.0 - self.weight

    def __repr__(self):
        return "Mor({}->{})".format(self.domain, self.codomain)


def identity(obj: Object) -> Morphism:
    """恒等态射"""
    return Morphism(domain=obj, codomain=obj, name="id_{}".format(obj.name), weight=1.0)


class Path:
    """态射序列 = 一条路径"""

    def __init__(self, morphisms: List[Morphism] = None):
        self.morphisms = morphisms or []

    def extend(self, m: Morphism):
        if self.morphisms and self.morphisms[-1].codomain != m.domain:
            raise ValueError("Path discontinuity")
        self.morphisms.append(m)

    def compose_all(self) -> Morphism:
        if not self.morphisms:
            raise ValueError("Empty path")
        result = self.morphisms[0]
        for m in self.morphisms[1:]:
            result = m.compose(result)
        return result

    @property
    def length(self) -> int:
        return len(self.morphisms)

    @property
    def total_gap(self) -> float:
        if not self.morphisms:
            return 1.0
        return 1.0 - math.prod(m.weight for m in self.morphisms)

    def __repr__(self):
        if not self.morphisms:
            return "empty"
        return " -> ".join(str(m.domain.name) for m in self.morphisms) + " -> {}".format(self.morphisms[-1].codomain.name)


def _test():
    a = Object("刘", layer=0)
    b = Object("楚", layer=0)
    c = Object("恬", layer=0)

    f = Morphism(a, b, name="l->c", weight=0.8)
    g = Morphism(b, c, name="iu->ian", weight=0.9)

    path = Path([f, g])
    print("[OK] path={}, gap={:.3f}".format(path.length, path.total_gap))

    f_inv = f.reverse()
    composed = f.compose(f_inv)
    print("[OK] f o f^-1 weight={:.3f}".format(composed.weight))

    id_a = identity(a)
    print("[OK] id_a: {} -> {}".format(id_a.domain, id_a.codomain))


if __name__ == "__main__":
    _test()


def is_isomorphism(f: Morphism, g: Morphism) -> bool:
    """Test if f and g are mutually inverse"""
    try:
        composed = f.compose(g)
        return composed.weight > 0.99
    except ValueError:
        return False
