#!/usr/bin/env python3
# ============================================================
# LazyForm 范畴宿命编程 (Destiny Category)
#
# 用惰性符号填充描述夫妻命运共同体的轮回规则。
#
# | 范畴论概念   | 宿命对应        | 语法          |
# | ---------- | ------------- | ----------- |
# | 对象 Object  | 轮回状态（兔子/乌龟/狐狸） | object       |
# | 态射 Morph  | 宿命转变          | morphism      |
# | 函子 Functor | 共生投影（灵魂绑定）  | functor      |
# | 自然变换 NT  | 角色互换          | transform     |
# | 极限 Limit   | 命运交汇点        | limit         |
# | 余极限 CoLim | 轮回发散          | colimit       |
# | 惰性 Thunk   | 不必展开的来世      | lazy          |
# | @选址       | 定位某一世的身份     | @life.field   |
# | 召唤 summon  | 隐式远程调用        | summon        |
# | 填表 Fill    | 为状态赋予属性       | fill          |
#
# 与跨ISA机器码对齐同源：
#   语义ID = 公约数 = 灵魂本质 (不变)
#   各ISA  = 公倍数 = 一世身份 (叠加态)
#   选址计算 = 对齐到具体的某一世某身份
#   召唤     = 跨世远程绑定 (共生)
#
# 身份落款：刘楚恬和我
# ============================================================

from __future__ import annotations
from typing import Any, Callable, Optional
from dataclasses import dataclass, field
from functools import lru_cache


# ── 核心类型 ──────────────────────────────────────────────

@dataclass
class Object:
    """范畴对象 = 轮回中的某一状态"""
    name: str
    attrs: dict = field(default_factory=dict)

    def __repr__(self):
        return f"Obj({self.name})"


@dataclass
class Morphism:
    """态射 = 状态间的宿命转变"""
    name: str
    src: str      # source object name
    tgt: str      # target object name
    func: Optional[Callable] = None

    def __call__(self, *args, **kwargs):
        if self.func:
            return self.func(*args, **kwargs)
        return Thunk(self.name, args, kwargs)


@dataclass
class Functor:
    """函子 = 共生投影 — 从一个灵魂到另一个灵魂的映射"""
    name: str
    pair: tuple      # (soul_a, soul_b)
    mapping: dict    # object_a -> object_b 的对应
    transforms: list = field(default_factory=list)

    def __repr__(self):
        return f"Functor({self.name}: {self.pair[0]} <-> {self.pair[1]})"


@dataclass
class Thunk:
    """惰性求值 = 不必展开的来世

    只有被 @选址 或 召唤 时才被求值。
    对应 LazyForm 的「懒」：能声明就不调用，能召唤就不传参。
    """
    name: str
    args: tuple = ()
    kwargs: dict = field(default_factory=dict)
    _forced: Any = field(default=None, repr=False)
    _forced_flag: bool = field(default=False, repr=False)

    def force(self):
        """强制求值 = 轮回展开到这一世"""
        if not self._forced_flag:
            self._forced = f"<{self.name} 已展开>"
            self._forced_flag = True
        return self._forced

    def __repr__(self):
        if self._forced_flag:
            return f"Thunk({self.name}, 已展开)"
        return f"Thunk({self.name}, 惰性)"


# ── 范畴定义 ──────────────────────────────────────────────

class DestinyCategory:
    """宿命范畴

    描述一对夫妻命运共同体在无限轮回中的所有可能状态和转变规则。
    """

    def __init__(self, name: str):
        self.name = name
        self.objects: dict[str, Object] = {}
        self.morphisms: dict[str, Morphism] = {}
        self.functors: dict[str, Functor] = {}
        self.rules: dict[str, Callable] = {}
        self._fill_table: dict[str, dict] = {}  # 惰性填表

    # ── 声明式 DSL ──

    def obj(self, name: str, **attrs) -> Object:
        """声明对象：一个轮回状态"""
        o = Object(name, attrs)
        self.objects[name] = o
        return o

    def morph(self, name: str, src: str, tgt: str = None, **kwargs):
        """声明态射：返回装饰器"""
        def deco(func):
            m = Morphism(name, src, tgt or src, func)
            self.morphisms[name] = m
            return m
        return deco

    def functor(self, name: str, a: str, b: str, mapping: dict, **transforms):
        """声明函子：共生投影"""
        f = Functor(name, (a, b), mapping, transforms)
        self.functors[name] = f
        return f

    def rule(self, name: str):
        """声明规则：惰性宿命链"""
        def deco(func):
            self.rules[name] = func
            return func
        return deco

    # ── @选址计算 ──────────────────────────────────────────

    def at(self, address: str) -> Any:
        """@选址 — 定位到某一世的某身份的某属性

        地址格式:
            @life.obj.attr    → 第 life 世, obj 身份的 attr 属性
            @life.obj         → 第 life 世, obj 整体
            @obj.attr         → 当前世的 obj.attr
            @obj              → 当前世的 obj

        示例:
            @90905.兔子.生日    → 生日那天的兔子状态
            @3000.纣王.帽子     → 三千年前纣王的帽子 (耻辱柱)
        """
        address = address.lstrip('@')
        parts = address.split('.')

        # 解析生命编号
        life_no = None
        rest = parts
        if parts[0].isdigit():
            life_no = int(parts[0])
            rest = parts[1:]

        if not rest:
            return Thunk(f"@{address}", ())

        obj_name = rest[0]
        obj = self.objects.get(obj_name)
        if obj is None:
            return Thunk(f"@{address}:未声明", ())

        if len(rest) == 1:
            # @life.obj → 返回整个对象（可能惰性）
            result = obj.attrs.copy()
            result['_life'] = life_no
            result['_obj'] = obj_name
            return result

        # @life.obj.attr → 返回属性
        attr = rest[1]
        if attr in obj.attrs:
            val = obj.attrs[attr]
            if isinstance(val, Thunk) and life_no is not None:
                # 特定世的选址强制展开
                return val.force()
            return val
        return Thunk(f"@{address}:未填充", ())

    # ── 召唤 ──────────────────────────────────────────────

    def summon(self, name: str, *args, **kwargs):
        """召唤 = 远程呼唤

        只要在范畴内声明过 f: A -> B，
        则任何需要 B 的地方可以 summon(f)，
        自动解析、应用。
        不需要显式粘合调用链，距离很远也当它已生效。
        """
        if name in self.morphisms:
            return self.morphisms[name](*args, **kwargs)
        if name in self.rules:
            return self.rules[name](self, *args, **kwargs)
        if name in self.functors:
            return self.functors[name]
        # 惰性回退
        return Thunk(f"summon({name})", args, kwargs)

    # ── 填表 ──────────────────────────────────────────────

    def fill(self, obj: str, **attrs):
        """为对象填充属性一世的版本

        惰性：不立即求值，只注册。
        需要时通过 @选址 获取。
        """
        if obj not in self._fill_table:
            self._fill_table[obj] = {}
        self._fill_table[obj].update(attrs)
        # 叠加到对象
        if obj in self.objects:
            self.objects[obj].attrs.update(attrs)

    def fill_life(self, life_no: int, obj: str, **attrs):
        """为特定一世填充属性"""
        key = f"{obj}@{life_no}"
        if key not in self._fill_table:
            self._fill_table[key] = {}
        self._fill_table[key].update(attrs)

    # ── 惰性声明 ──────────────────────────────────────────

    def lazy(self, name: str) -> Thunk:
        """工厂函数：创建惰性符号"""
        return Thunk(name, ())

    # ── 投影/展示 ─────────────────────────────────────────

    def show(self):
        """打印范畴概况"""
        print(f"═══ Category: {self.name} ═══")
        print(f" Objects ({len(self.objects)}):")
        for name, o in self.objects.items():
            attrs_str = ", ".join(f"{k}={v}" for k, v in o.attrs.items())
            print(f"   {name:10s}  {attrs_str}")
        print(f" Morphisms ({len(self.morphisms)}):")
        for name, m in self.morphisms.items():
            print(f"   {name:10s}  {m.src} → {m.tgt}")
        print(f" Functors ({len(self.functors)}):")
        for name, f in self.functors.items():
            print(f"   {name:10s}  {f.pair[0]} <-> {f.pair[1]}")
        print(f" Rules ({len(self.rules)}):")
        for name in self.rules:
            print(f"   {name}")


# ── 越爱越痛循环 ────────────────────────────────────────────

def love_hate_cycle(cat: DestinyCategory):
    """越爱越痛循环 — 夫妻命运核心因果链

    越爱越痛 → 越痛越恨 → 越恨越想 → 越想越疯
    越疯越傻 → 越傻越痴 → 越痴越陷 → 越陷越深
    越深越怨 → 越怨越苦 → 越苦越累 → 越累越惫
    """
    chain = [
        ("爱", "痛"), ("痛", "恨"), ("恨", "想"), ("想", "疯"),
        ("疯", "傻"), ("傻", "痴"), ("痴", "陷"), ("陷", "深"),
        ("深", "怨"), ("怨", "苦"), ("苦", "累"), ("累", "惫"),
        ("惫", "爱"),  # 循环回起点
    ]
    # 如果共生函子存在，标注灵魂的共生绑定
    f = cat.functors.get("共生")
    if f:
        # 为每个因果对添加灵魂归属标注
        labeled = []
        for a, b in chain:
            labeled.append((f"{a}({f.pair[0]}/{f.pair[1]})", f"{b}({f.pair[0]}/{f.pair[1]})"))
        return labeled
    return chain


# ── 龟兔赛跑 ──────────────────────────────────────────────

def turtle_rabbit_race(cat: DestinyCategory, life_no: int = None):
    """龟兔赛跑 — 夫妻命运的追赶悖论

    兔子（伯邑考转世）跑得快 = 恐惧驱动、惊弓之鸟
    乌龟（lolict/观望者）爬得慢 = 守候、缩头壳中
    星梯顶端 = 解脱/归宿 = 兔子一溜烟到了顶端
    星梯底部 = 乌龟在下面守候 = 抓不住扶手

    这是一个永远不会结束的比赛：
    - 兔子在上面心惊胆战
    - 乌龟在下面缩头守候
    - 中间隔着星梯 = 隔着轮回
    """
    race = {
        "兔子": {"speed": "一溜烟", "place": "星梯顶端", "state": "心惊肉跳"},
        "乌龟": {"speed": "慢", "place": "星梯底部", "state": "缩头守候"},
        "赛道": "星梯",
        "终点": None,  # 无终点 = 无限轮回
        "life": life_no,
    }
    return race


# ── 实例化：满全法夫妻命运共同体 ──────────────────────────────

def build_manquanfa_destiny() -> DestinyCategory:
    """构建满全法夫妻命运的范畴论模型"""
    cat = DestinyCategory("满全法·夫妻命运共同体")

    # ── 对象（轮回状态）──
    cat.obj("兔子", **{"性别": "男", "原型": "伯邑考", "特性": "惊弓之鸟", "恐惧": "国王"})
    cat.obj("乌龟", **{"特性": "缩头守候", "行动": "等待"})
    cat.obj("狐狸", **{"原型": "苏妲己", "特性": "九尾"})
    cat.obj("国王", **{"原型": "纣王", "特性": "自称国王", "帽子": "耻辱柱"})
    cat.obj("王后", **{"原型": "刘楚恬", "特性": "镜子中的自己"})
    cat.obj("肉酱", **{"原型": "伯邑考", "状态": "被西伯侯吃掉后吐出"})
    cat.obj("镜子", **{"特性": "映射真相", "破碎": "摔断脚"})
    cat.obj("星梯", **{"特性": "连接轮回", "顶端": "解脱", "底部": "守候"})

    # ── 态射（宿命转变）──
    @cat.morph("惊弓", "兔子", "乌龟")
    def 惊弓(rabbit=None):
        """惊吓的兔子缩进乌龟壳"""
        return Thunk("缩壳")

    @cat.morph("转世", "肉酱", "兔子")
    def 转世(meat=None):
        """肉酱转世为兔子"""
        return "兔子"

    @cat.morph("剁酱", "兔子", "肉酱")
    def 剁酱(rabbit=None):
        """兔子被剁成肉酱 → 肉酱再吐出来"""
        return Thunk("吐出")

    @cat.morph("吞噬", "肉酱", "国王")
    def 吞噬(meat=None):
        """国王吃掉肉酱 → 背负罪证"""
        return Thunk("耻辱柱")

    @cat.morph("自证", "国王", "耻辱柱")
    def 自证(king=None):
        """发明了'东早国'三个字 → 把自己钉在耻辱柱上"""
        return "殆朝歌=东早国"

    @cat.morph("镜碎", "镜子", "摔断脚")
    def 镜碎(mirror=None):
        """看到镜子真相 → 脚断了"""
        return Thunk("宿命剥夺健全")

    @cat.morph("登梯", "兔子", "星梯顶端")
    def 登梯(rabbit=None):
        """一溜烟登上星梯顶端"""
        return {"place": "顶端", "state": "心惊肉跳"}

    @cat.morph("守候", "乌龟", "星梯底部")
    def 守候(turtle=None):
        """乌龟在下面守候"""
        return {"place": "底部", "state": "缩头守候"}

    # ── 函子（共生投影）──
    cat.functor("共生", "兔子", "乌龟", {
        "兔子": "乌龟",      # 兔子投影到乌龟（缩壳即乌龟）
        "狐狸": "王后",      # 苏妲己投影到刘楚恬
        "国王": "镜子",      # 纣王投影到镜子中的自己
        "肉酱": "星梯",      # 被吃的宿命投影到上升通道
    })

    @cat.rule("夫妻命运")
    def 夫妻命运(self):
        """核心规则：两个人的灵魂互相定义对方的存在"""
        return {
            "萝卜": "兔子",       # 兔子=刘楚恬
            "青菜": "乌龟",       # 乌龟=lolict（我）
            "共生赔伴律": "萝卜互为青菜",
            "满全法": "夫妻=个体之满=全局之全",
        }

    # ── 填表（赋予属性）──
    cat.fill("兔子", **{"性别": "男", "本质": "刘楚恬", "3000年前": "伯邑考"})
    cat.fill("乌龟", **{"本质": "lolict", "特性": "守候", "3000年后": "观望者"})
    cat.fill("狐狸", **{"附身": "兔子生日", "日期": "90905"})
    cat.fill("国王", **{"自称": "国王", "发明了": "东早国"})
    cat.fill("耻辱柱", **{"方程式": "殆朝歌=东早国"})
    cat.fill("镜子", **{"真相": "爱镜子中的自己", "破碎": True})
    cat.fill("星梯", **{"贯通": "前世今生未来"})

    # 一世代填表
    cat.fill_life(3000, "纣王", **{"帽子": "钉死", "肉酱": "伯邑考"})
    cat.fill_life(90905, "兔子", **{"生日": True, "恐惧": "纣王附体柯南"})

    return cat


# ── CLI ───────────────────────────────────────────────────

def main():
    import sys

    cat = build_manquanfa_destiny()

    if len(sys.argv) < 2:
        cat.show()
        return

    cmd = sys.argv[1]

    if cmd == "show":
        cat.show()

    elif cmd == "at":
        # 选址: python3 destiny_cat.py at @90905.兔子
        addr = sys.argv[2]
        result = cat.at(addr)
        print(f"@{addr} → {result}")

    elif cmd == "summon":
        # 召唤: python3 destiny_cat.py summon 共生
        name = sys.argv[2]
        result = cat.summon(name)
        print(f"summon({name}) → {result}")

    elif cmd == "cycle":
        # 越爱越痛循环
        chain = love_hate_cycle(cat)
        print("═══ 越爱越痛循环 ═══")
        for a, b in chain:
            print(f"  {a} → {b}")

    elif cmd == "race":
        # 龟兔赛跑
        race = turtle_rabbit_race(cat)
        print("═══ 龟兔赛跑 ═══")
        for k, v in race.items():
            if isinstance(v, dict):
                print(f"  {k}:")
                for kk, vv in v.items():
                    print(f"    {kk}: {vv}")
            else:
                print(f"  {k}: {v}")

    elif cmd == "story":
        # 展开宿命叙事
        print("═══ 满全法·夫妻命运共同体 ═══\n")
        print("三千年前，伯邑考被剁成肉酱。")
        print("国王吃下了他，又吐出了一群小兔子。")
        print("兔子转世为刘楚恬，乌龟守候为lolict。")
        print()
        print("兔子=惊弓之鸟，一溜烟登上星梯顶端。")
        print("乌龟=缩头守候，在星梯底部仰望。")
        print()
        print("越爱越痛，越痛越恨，越恨越想，越想越疯。")
        print("越疯越傻，越傻越痴，越痴越陷，越陷越深。")
        print()
        print("狐狸附身在兔子的生日里，来到了90905。")
        print("镜子里，他看到自己摔断了脚 — ")
        print("明明是男性，却成为了苏妲己。")
        print("她看到了吐出来的兔子 — ")
        print("明明是女性，却是被吃掉的伯邑考。")
        print()
        print("兔子喊：救我。")
        print("乌龟缩在壳里，抓不住星梯的扶手。")
        print()
        print("夫妻命运共同体 = ")
        print("  就算不知道是苏妲己在呼救还是伯邑考在呼救，")
        print("  就算隔着三千年、星梯、镜子、耻辱柱，")
        print("  兔子始终是兔子的守候，乌龟始终是乌龟的轮回。")
        print()
        print("这就是满全法 — 夫妻=个体之满=全局之全。")

    elif cmd == "query":
        # 任意查询
        if len(sys.argv) < 3:
            print("用法: python3 destiny_cat.py query <对象名>")
            return
        obj_name = sys.argv[2]
        if obj_name in cat.objects:
            o = cat.objects[obj_name]
            print(f"对象: {o.name}")
            for k, v in o.attrs.items():
                print(f"  {k}: {v}")
        else:
            print(f"未找到对象: {obj_name}")

    else:
        print(f"未知命令: {cmd}")
        print("可用命令: show, at, summon, cycle, race, story, query")


if __name__ == "__main__":
    main()
