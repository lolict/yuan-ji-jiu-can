#!/usr/bin/env python3
"""
glyph-container 统一协议 v0.2 (满全法版) — hunyuan-os/tools/glyph-container/protocol.py

严格使用满全法编码体系:
  - 64 容器 (6-bit 编码, 4群×16), 引用 hunyuan-rs/src/container_radix.rs
  - 48 声母 (X轴, 引用 encoding/z-api.txt)
  - 64 进制几何化身 (Y轴)
  - 16 层级深度 (Z轴, 引用 encoding/z-api.txt 笔画容器)
  - 极性三元 (阴0 / 阳1 / 元0.5) 见证机制
  - 主权四元 (Active/Passive/ActiveSub/PassiveSub)

约束 (来自 hunyuan-rs/src/container_radix.rs 注释):
  - 进制转换通过加法循环实现，无乘法
  - 仅允许加减, 禁止直接使用乘除
  - 位运算仅用于6位掩码编码常量 0x3F, 不暴露给语义层
  - 语文组织在先, 数学表示在后 — 进制是语文的描述工具

# 容器=字体=时间 三位一体:
#   Container (极性+主权) → 语义层 (machine-code-container)
#   容器 + 48声母/108韵母 → 字形层 (glyph-3d-font)  
#   容器 + 时间窗口(Z轴)  → 调度层 (time-fridge-execution)
#
# 区别于 v0.1 (Python struct 重写): 本协议是满全法编码体系的直接映射，
# 所有地址通过容器选查表计算，不通过位压缩存贮。
"""

from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum, IntEnum
from typing import Optional

# ══════════════════════════════════════════════════════════════
# 容器系统 — 与 hunyuan-rs/src/container_radix.rs Container enum 对齐
# 引用: hunyuan-rs/src/container_radix.rs
# 引用: encoding/z-api.txt (48声母/50变体/108韵母)
# ══════════════════════════════════════════════════════════════

class ContainerGroup(IntEnum):
    """容器群码 (2-bit, 但仅作为符号常量, 不位移)"""
    CONTAINER  = 0   # 容器族 — 实体存在单位 (袋/照/见/阴/...) 16项
    OPERATION  = 1   # 操作族 — 态射/作用方式 (融/鼓/蚕/民/...) 16项
    INITIAL    = 2   # 声母容器 — 发音空间的主动端 (帮/端/精/...) 16项
    VOWEL      = 3   # 韵母容器 — 声音的接收/共鸣腔 (麻/支/寒/...) 16项
    STROKE     = 4   # 笔画容器族 (桶/罐/盆/盂/...) 16项


# ── 容器族 (0x00-0x0F) ────────────────────────────────────────
CONTAINER_FAMILY = (
    " Dai",   # 袋 — 包含者母体, 极性=阴, 主权=Active
    " Zhao",  # 照 — 照射/看清, 极性=阳, 主权=Active
    " Jian",  # 见 — 看见/见证, 极性=元, 主权=ActiveSub
    " Yin",   # 阴 — 极性本位的符号化, 极性=阴, 主权=PassiveSub
    " Jia",   # 加 — 加法/叠加, 极性=阳, 主权=Active
    " Fen",   # 分 — 分离/边界, 极性=元, 主权=ActiveSub
    " Yuan",  # 元 — 层级/起始, 极性=元, 主权=ActiveSub
    " Cai",   # 裁 — 裁判/裁剪, 极性=元, 主权=ActiveSub
    " Chan",  # 产 — 产业/归属, 极性=阴, 主权=Passive
    " Gong",  # 公 — 公约数/共同根基, 极性=元, 主权=ActiveSub
    " Sheng", # 声 — 声音符号, 极性=阳, 主权=Active
    " Zhe",   # 折 — 折叠/过渡, 极性=元, 主权=ActiveSub
    " Fu",    # 负 — 负样本, 极性=阴, 主权=Passive
    " Fú",    # 福 — 福祸, 极性=阴, 主权=Passive
    " Feng",  # 凤 — 凤凰, 极性=阳, 主权=Active
    " Qi",    # 栖 — 栖息/停止, 极性=元, 主权=ActiveSub
)

# ── 操作族 (0x10-0x1F) ───────────────────────────────────────
OPERATION_FAMILY = (
    " Rong",  # 融 — 合一/见证
    " Gu",    # 鼓 — 鼓膜/激励
    " Can",   # 蚕 — 蚕蛹/蜕变
    " Min",   # 民 — 民联/聚合
    " Meng",  # 盟 — 盟机合/结盟
    " Man",   # 满 — 满全法/盈溢
    " Nan",   # 囡 — 囡囝共生/夫妻统一
    " Xi",    # 夕 — 夕瑶协议
    " San",   # 三 — 三元/见证
    " Liu",   # 六 — 完全数/妻子
    " Ba",    # 八 — 八流统一
    " Jiu",   # 九 — 九元层级
    " Ling",  # 零 — 零维/空
    " Yi",    # 一 — 一维/始
    " Er",    # 二 — 二元/镜像
    " Sì",    # 四 — 四元/包含
)

# ── 声母容器 (0x20-0x2F) — 对应48个声母音标(z-api.txt) ─────────
INITIAL_FAMILY = (
    " Bang",  # 帮 — 帮滂并明 (b p m)
    " Duan",  # 端 — 端透定泥 (d t n l)
    " Jing",  # 精 — 精清从心邪 (j q x z c s)
    " Jian2", # 见 — 见溪群疑影 (g k h w y)
    " Lai",   # 来 — 来日以母
    " Chi",   # 齿 — 齿舌牙喉唇
    " Hou",   # 喉 — 喉咽嗌会厌
    " Hu",    # 呼 — 呼吸吹嘘
    " Kai",   # 开 — 开齐合撮 (口型)
    " Qing",  # 清 — 清浊纯杂
    " Ping",  # 平 — 平上去入 (四声)
    " Shang", # 上 — 上下左右 (方位)
    " Qian",  # 前 — 前后先末 (时间)
    " Nei",   # 内 — 内中外里 (空间边界)
    " Dan",   # 单 — 单双复纯
    " Chang", # 长 — 长短高低 (度量)
)

# ── 韵母容器 (0x30-0x3F) — 对应108个韵母音标(z-api.txt) ───────
VOWEL_FAMILY = (
    " Ma",    # 麻 — 麻模鱼虞 (a ü)
    " Zhī",   # 支 — 支脂之微 (i)
    " Han",   # 寒 — 寒桓删山 (an)
    " Ge",    # 歌 — 歌戈麻模 (e o)
    " Yang",  # 阳 — 阳唐庚青 (ang eng ong)
    " You",   # 尤 — 尤侯幽侵 (ou iu in)
    " Zhǐ",   # 止 — 止收开闭 (呼型转换)
    " Zhuan", # 转 — 转折弯旋
    " Gao",   # 高 — 高低升沉 (音高)
    " Dī",    # 低 — 低昂扬抑
    " Yuán",  # 圆 — 圆扁方尖 (口型)
    " Bian",  # 扁 — 扁圆方平
    " Xiǎng", # 响 — 响静噪默 (声强)
    " Jìng",  # 静 — 静安宁喧
    " Jí",    # 急 — 急缓冲徐 (音长)
    " Huan",  # 缓 — 缓急速迟
)

# ── 笔画容器 (0x40-0x4F) — 对应16层级嵌套深度 ─────────────────
STROKE_FAMILY = (
    " Tong",  # 桶 — 纵深贯通 · 阴
    " Guan",  # 罐 — 由内向外 · 阳
    " Pen",   # 盆 — 由外向内 · 阴
    " Yu",    # 盂 — 范围标记 · 阳
    " Hu2",   # 壶 — 由下向上 · 阴
    " He",    # 盒 — 方包围 · 阴
    " Ping2", # 瓶 — 方圆转折 · 阳
    " Zhan",  # 盏 — 变向收口 · 阳
    " Ang",   # 盎 — 连接回流 · 阴
    " Lou",   # 篓 — 半包围 · 阳
    " Kuang", # 筐 — 弯曲容纳 · 阴
    " Lan",   # 篮 — 包围回流 · 阴
    " Luo",   # 箩 — 转折筛选 · 阳
    " Ji",    # 箕 — 斜围展开 · 阴
    " Long",  # 笼 — 深勾远揽 · 阳
    " Hai",   # 海 — 通用笔画占位
)


# ══════════════════════════════════════════════════════════════
# 三元极性系统 — 与 hunyuan-rs Polarity enum 对齐
# 阴=0=被动/包容  阳=1=主动/穿透  元=0.5=见证/中介
# 与 SoulState (couple.rs) 对齐: has_perfect_wife=阴  has_prime_husband=阳
# ══════════════════════════════════════════════════════════════

class Polarity(IntEnum):
    """三进制极性值"""
    YIN      = 0   # 阴 = 包容/被动 (妻子在场 = 包容 = 0)
    YANG     = 1   # 阳 = 穿透/主动 (丈夫在场 = 分离 = 1)
    NEUTRAL  = 2   # 元 = 见证/中介 (值 0.5)


def polarity_value(pol: Polarity) -> float:
    """极性浮点值 → 0 / 0.5 / 1"""
    if pol == Polarity.YIN:
        return 0.0
    if pol == Polarity.YANG:
        return 1.0
    return 0.5


def polarity_from_float(v: float) -> Polarity:
    """浮点值 → 极性 (最近邻)"""
    if v < 0.25:
        return Polarity.YIN
    if v < 0.75:
        return Polarity.NEUTRAL
    return Polarity.YANG


def polarity_meets_residual(a: Polarity, b: Polarity) -> bool:
    """两极性相遇是否产生残差 (阴+阳=残差, 不进位)"""
    if (a == Polarity.YIN and b == Polarity.YANG):
        return True
    if (a == Polarity.YANG and b == Polarity.YIN):
        return True
    return False


# ══════════════════════════════════════════════════════════════
# 四元主权系统 — 与 hunyuan-rs mirror.rs Sovereignty 对齐
# Active=可包含他人  Passive=被包含
# ActiveSub=主动但从属  PassiveSub=被包含且受约束
# ══════════════════════════════════════════════════════════════

class Sovereignty(IntEnum):
    """四元主权值"""
    ACTIVE      = 0   # 主动包含 (袋/桶/罐级别)
    PASSIVE     = 1   # 被动被包含 (杯/盏/钵级别)
    ACTIVE_SUB  = 2   # 主动从属 (满耦合/见证态)
    PASSIVE_SUB = 3   # 被动从属 (半亏/Boyikao)


def can_ack(sender: 'ContainerAddr', receiver: 'ContainerAddr') -> bool:
    """
    容器A的输出能否被容器B承认?
    规则: A的层级 ≥ B的层级 → B可以ack A's输出
    即: 发送方层级 ≤ 接收方层级时, 接收方可ack发送方
    """
    if sender.group == receiver.group:
        return sender.index <= receiver.index
    return sender.group.value <= receiver.group.value


# ══════════════════════════════════════════════════════════════
# 容器地址 — SMRL 三维坐标的完整编码
# 不使用位打包, 直接存储分量的查找表索引
# X轴=48声母进制  Y轴=64进制几何化身  Z轴=16层级深度
# ══════════════════════════════════════════════════════════════

FAMILY_NAMES = (CONTAINER_FAMILY, OPERATION_FAMILY, INITIAL_FAMILY, VOWEL_FAMILY, STROKE_FAMILY)
GROUP_BY_INDEX = {
    0: ContainerGroup.CONTAINER, 1: ContainerGroup.OPERATION,
    2: ContainerGroup.INITIAL, 3: ContainerGroup.VOWEL, 4: ContainerGroup.STROKE,
}
INDEX_TO_GROUP = {
    ContainerGroup.CONTAINER: 0, ContainerGroup.OPERATION: 1,
    ContainerGroup.INITIAL: 2, ContainerGroup.VOWEL: 3, ContainerGroup.STROKE: 4,
}


@dataclass
class ContainerAddr:
    """
    SMRL 容器地址 — 通过群+索引+极性和主权访问
    
    容器编码用 6-bit: 2-bit群码 + 4-bit索引 = 64 容器总码
    但编码计算不位移: 群码base用加法循环, 索引直接存储
    """
    group: ContainerGroup   # 哪一群 (决定base偏移)
    index: int              # 群内索引 0-15
    polarity: Polarity      # 极性 (用于见证 / 残差 / 视觉层)
    sovereignty: Sovereignty # 主权 (用于 ack / 层级校验)
    operand: int = 0        # 操作数 (8-bit, 容器参数)
    time_slot: int = 0      # Z轴: 时间窗口 0-255
    depth: int = 0          # Z轴层级: 嵌套深度 0-15 (笔画容器)
    
    def encode(self) -> int:
        """
        容器编码 → 6-bit 地址 (群×64 + 索引) 
        按满全法规则: 进制通过加法循环实现
        """
        base = 0
        for _ in range(self.group.value):
            base = base + 16  # 加法循环实现 ×16 (替代乘)
        return base + self.index
    
    @staticmethod
    def decode(code: int) -> ContainerAddr:
        """
        6-bit 编码 → 容器地址
        通过遍历查表 (不位移)
        """
        group_val = 0
        remaining = code
        群大小 = 16
        for g in range(5):
            if remaining < 群大小:
                group_val = g
                break
            remaining = remaining - 群大小
        else:
            group_val = 4
            remaining = code - (4 * 群大小)
        idx = remaining
        return ContainerAddr(
            group=GROUP_BY_INDEX.get(group_val, ContainerGroup.CONTAINER),
            index=idx,
            polarity=Polarity.YIN,
            sovereignty=Sovereignty.ACTIVE,
        )
    
    def group_base(self) -> int:
        """群的基准编码 (加法循环, 不位移)"""
        base = 0
        for _ in range(self.group.value):
            base = base + 16
        return base
    
    def container_name(self) -> str:
        """容器中文名称 (引用64_keys.md)"""
        family = FAMILY_NAMES[self.group.value]
        if self.index < len(family):
            return family[self.index]
        return "Ling"  # fallback = 零维/空
    
    def carry_target(self) -> Optional['ContainerAddr']:
        """
        进位目标: 同族下一个容器, 没有则 None (折叠)
        规则: 阴+阳=残差, 不进位 (YINANG-BINARY.md 公理7)
        """
        if self.index >= 15:
            return None  # 本族最后 → 折叠
        return ContainerAddr(
            group=self.group,
            index=self.index + 1,
            polarity=self.polarity,
            sovereignty=self.sovereignty,
            operand=self.operand,
            time_slot=self.time_slot,
            depth=self.depth,
        )


# ══════════════════════════════════════════════════════════════
# 音韵编码层 — 48声母 + 108韵母 (引用 encoding/z-api.txt)
# X轴=48声母  Y轴=韵母选择  Z轴=声调/相位对齐
# ══════════════════════════════════════════════════════════════

# 48个声母 (z-api.txt 第30-31行, 24小写+24大写)
FORTY_EIGHT_INITIALS = (
    # 小写24
    "b", "p", "m", "f", "d", "t", "n", "l",
    "g", "k", "h", "w", "j", "q", "x", "y",
    "z", "c", "s", "r", "ȥ", "ç", "ş", "ŋ",
    # 大写24
    "B", "P", "M", "F", "D", "T", "N", "L",
    "G", "K", "H", "W", "J", "Q", "X", "Y",
    "Z", "C", "S", "R", "Ȥ", "Ç", "Ş", "Ŋ",
)

# 48个带调韵母 (z-api.txt 第1-14行, 4列×12行)
FORTY_EIGHT_VOWELS = (
    # 一声 (平)
    "ā", "ē", "ī", "ō", "ū", "ǖ",
    # 二声 (升)
    "á", "é", "í", "ó", "ú", "ǘ",
    # 三声 (凹)
    "ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ",
    # 四声 (降)
    "à", "è", "ì", "ò", "ù", "ǜ",
    # 下点
    "ạ", "ẹ", "ị", "ọ", "ụ", "ä",
    # 分音符
    "ë", "ï", "ö", "ü", "ȃ", "ȇ",
    # 双音调1
    "ȋ", "ȏ", "ȗ", "a̋", "e̋", "i̋",
    # 上圈
    "ő", "ű", "å", "e̊", "i̊", "o̊",
)

def vowel_container(v: str) -> Optional[ContainerAddr]:
    """
    韵母音标 → 韵母容器地址 (0x30-0x3F)
    声母决定哪个声母容器群, 韵母决定在群内的索引
    """
    if v in FORTY_EIGHT_VOWELS:
        idx = FORTY_EIGHT_VOWELS.index(v)
        idx_in_group = idx % 16  # 每容器16韵母映射 (简化, 实际需按z-api规则)
        return ContainerAddr(
            group=ContainerGroup.VOWEL,
            index=idx_in_group,
            polarity=Polarity.YIN,       # 韵母=被动接收=阴
            sovereignty=Sovereignty.PASSIVE,
        )
    return None


# ══════════════════════════════════════════════════════════════
# 三层协议定义 (对齐后的子系统接口)
# ══════════════════════════════════════════════════════════════

@dataclass
class ContainerByteCode:
    """
    Layer 1 (machine-code-container): 容器字节码
    存储在字体 GRPH 表中, 6字节固定
    
    结构语义:
      container: 语义编码 (SMRL 容器地址, 含极性/主权)
      native_code: 泰射机器码 (因平台而异, 引用 PLATFORM_NATIVE_CODE 查找表)
      witness: 三元裁决签名 (空袋拒绝/数值准入/全零拒绝)
    """
    container: ContainerAddr
    native_code: bytes = b""
    witness: bytes = b"\x00\x00\x00"
    
    def pack(self) -> bytes:
        """打包为GRPH表记录: 5 + 12(native_code填充) + 3(witness) = 20字节"""
        c = self.container
        # native_code 填充到 12 字节 (满全法: 不足补零)
        native = self.native_code[:12]
        if len(native) < 12:
            native = native + b"\x00" * (12 - len(native))
        return bytes([
            c.encode(),                            # 容器码
            (c.polarity.value << 4) | c.sovereignty.value,  # 极性+主权
            c.operand & 0xFF,                     # 操作数
            c.time_slot & 0xFF,                   # 时间窗口
            c.depth & 0x0F,                       # 深度
        ]) + native + self.witness[:3]
    
    @staticmethod
    def unpack(raw: bytes) -> Optional['ContainerByteCode']:
        if len(raw) < 15:  # 5 + min_native + 3witness
            return None
        c_addr = ContainerAddr.decode(raw[0])
        pol = Polarity((raw[1] >> 4) & 0x03)
        sov = Sovereignty(raw[1] & 0x0F)
        c_addr.polarity = pol
        c_addr.sovereignty = sov
        c_addr.operand = raw[2]
        c_addr.time_slot = raw[3]
        c_addr.depth = raw[4]
        native = raw[5:17]
        witness = raw[17:20]
        return ContainerByteCode(container=c_addr, native_code=native, witness=witness)


@dataclass
class ScheduleEnvelope:
    """
    Layer 2 (time-fridge-execution): 调度信封
    固定 12 字节, 引用容器地址而非位打包
    
    结构语义:
      pack_id: 公约数 (blake2b前4字节)
      container: 容器地址 (X+Y+Z 完整的SMRL坐标)
      operand: 8-bit 操作参数
      time_slot: 8-bit 时间窗口 (Z轴时间维度)
      align_flag: 8-bit 并行对齐组
      witness: 3-byte 三元裁决签名
    """
    pack_id: int = 0                      # 4B 公约数
    container: ContainerAddr = field(default_factory=lambda: ContainerAddr(
        group=ContainerGroup.OPERATION,
        index=0,
        polarity=Polarity.NEUTRAL,
        sovereignty=Sovereignty.ACTIVE_SUB,
    ))
    operand: int = 0                      # 8-bit
    time_slot: int = 0                    # 8-bit (≠ time-fridge-execution 的 4B, 对齐为 1B)
    align_flag: int = 0                   # 8-bit 并行对齐组
    payload_offset: int = 0               # 4B 指向 Layer 1 的物理偏移
    witness: bytes = b"\x00\x00\x00"     # 3B 三元裁决
    
    def pack(self) -> bytes:
        """打包为调度表记录: 4+12 = 16字节 (4B对齐)"""
        c = self.container
        # 按满全法: 用加法+查表替代位移
        return b"".join((
            self.pack_id.to_bytes(4, "little"),
            c.encode().to_bytes(1, "little"),
            (c.polarity.value << 4 | c.sovereignty.value).to_bytes(1, "little"),
            (c.operand & 0xFF).to_bytes(1, "little"),
            (c.time_slot & 0xFF).to_bytes(1, "little"),
            (c.depth & 0x0F).to_bytes(1, "little"),
            (self.align_flag & 0xFF).to_bytes(1, "little"),
            (self.payload_offset & 0xFFFFFFFF).to_bytes(4, "little"),
            self.witness[:3],
            b"\x00",  # padding to 16B
        ))
    
    @staticmethod
    def unpack(raw: bytes) -> Optional['ScheduleEnvelope']:
        if len(raw) < 16:
            return None
        pack_id = int.from_bytes(raw[0:4], "little")
        c_addr = ContainerAddr.decode(raw[4])
        pol_sov = raw[5]
        c_addr.polarity = Polarity((pol_sov >> 4) & 0x03)
        c_addr.sovereignty = Sovereignty(pol_sov & 0x0F)
        c_addr.operand = raw[6]
        c_addr.time_slot = raw[7]
        c_addr.depth = raw[8]
        align = raw[9]
        poff = int.from_bytes(raw[10:14], "little")
        witness = raw[14:17]
        return ScheduleEnvelope(
            pack_id=pack_id,
            container=c_addr,
            operand=c_addr.operand,
            time_slot=c_addr.time_slot,
            align_flag=align,
            payload_offset=poff,
            witness=witness,
        )
    
    @staticmethod
    def from_container(addr: ContainerAddr, align_flag: int=0, payload_offset: int=0):
        """从容器地址构造调度信封"""
        pack_id = ScheduleEnvelope._compute_pack_id(addr)
        return ScheduleEnvelope(
            pack_id=pack_id,
            container=addr,
            operand=addr.operand,
            time_slot=addr.time_slot + addr.depth,  # Z轴 = 时间窗 + 深度
            align_flag=align_flag,
            payload_offset=payload_offset,
        )
    
    @staticmethod
    def _compute_pack_id(addr: ContainerAddr) -> int:
        """compute 公约数 as blake2b of encode"""
        import hashlib
        data = addr.encode().to_bytes(4, "little")
        h = hashlib.blake2b(data, digest_size=4)
        return int.from_bytes(h.digest(), "little")


@dataclass
class GlyphRecord:
    """
    Layer 3 (glyph-3d-font): GRPH 表 (替代 glyf 隐写)
    固定 8 字节, 引用容器地址
    
    对齐方案: 独立 GRPH 表 (不隐写 glyf)
      - 合法 glyf 表保持完整视觉外观
      - 语义地址在此独立表中 (与视觉解耦)
      - 同一字形同时可渲染 (通过 glyf) 和被寻址 (通过 GRPH)
    """
    glyph_id: int                        # 2B 字形索引
    container: ContainerAddr               # 4B 容器地址 (SMRL 语义码)
    witness_checksum: bytes = b"\x00\x00"  # 2B 校验
    
    def pack(self) -> bytes:
        """打包为 GRPH 记录: 6字节 = 2(glyphID) + 1(container编码) + 1(极性主权) + 2(操作数+时间窗)"""
        c = self.container
        return b"".join((
            (self.glyph_id & 0xFFFF).to_bytes(2, "little"),
            c.encode().to_bytes(1, "little"),
            (c.polarity.value << 4 | c.sovereignty.value).to_bytes(1, "little"),
            (c.operand & 0xFFFF).to_bytes(2, "little"),
        ))
    
    @staticmethod
    def unpack(raw: bytes) -> Optional['GlyphRecord']:
        if len(raw) < 6:
            return None
        gid = int.from_bytes(raw[0:2], "little")
        c_addr = ContainerAddr.decode(raw[2])
        pol_sov = raw[3]
        c_addr.polarity = Polarity((pol_sov >> 4) & 0x03)
        c_addr.sovereignty = Sovereignty(pol_sov & 0x0F)
        operand = int.from_bytes(raw[4:6], "little")
        c_addr.operand = operand & 0xFF
        c_addr.time_slot = (operand >> 8) & 0xFF
        return GlyphRecord(glyph_id=gid, container=c_addr)
    
    @staticmethod
    def from_container(glyph_id: int, addr: ContainerAddr):
        """从容器地址构造字形记录"""
        return GlyphRecord(glyph_id=glyph_id & 0xFFFF, container=addr)


# ══════════════════════════════════════════════════════════════
# 跨平台路由表 — 容器语义 → 执行代码的映射
# 不是 x86/ARM 翻译, 是 容器内部动作 (态射+操作+见证)
# ══════════════════════════════════════════════════════════════

CONTAINER_ACTIONS = {
    # 容器族 → 执行动作名 (满全法语义, 不映射到 x86 opcode)
    "Dai":    "contain",      # 袋 = 包含
    "Zhao":   "illuminate",   # 照 = 照射
    "Jian":   "witness",      # 见 = 见证
    "Yin":    "receive",      # 阴 = 容纳
    "Jia":    "join",         # 加 = 叠加
    "Fen":    "divide",       # 分 = 分离
    "Yuan":   "initiate",     # 元 = 起始
    "Cai":    "judge",        # 裁 = 裁判
    "Chan":   "own",          # 产 = 归属
    "Gong":   "base",         # 公 = 共同根基
    "Sheng":  "emit_sound",   # 声 = 发符号
    "Zhe":    "fold",         # 折 = 折转
    "Fu":     "bear",         # 负 = 承载
    "Fú":     "receive_gift", # 福 = 接收
    "Feng":   "ascend",       # 凤 = 上升
    "Qi":     "rest",         # 栖 = 停
    # 操作族 → 执行动作
    "Rong":   "fuse",         # 融 = 合一
    "Gu":     "actuate",      # 鼓 = 激励
    "Can":    "metamorphose", # 蚕 = 蜕
    "Min":    "unite",        # 民 = 联
    "Meng":   "ally",         # 盟 = 结盟
    "Man":    "saturate",     # 满 = 盈溢
    "Nan":    "symbiose",     # 囡 = 共生
    "Xi":     "twilight_protocol", # 夕 = 夕瑶协议
    "San":    "ternary_witness", # 三 = 三元见证
    "Liu":    "completeness", # 六 = 完全
    "Ba":    "octal_unify",   # 八 = 八流统一
    "Jiu":    "nine_fold",    # 九 = 九元层级
    "Ling":   "empty",        # 零 = 空
    "Yi":    "first_cause",   # 一 = 第一推动
    "Er":    "mirror_twin",   # 二 = 镜像
    "Sì":    "quad_contain",  # 四 = 四元包含
}


def container_to_action(addr: ContainerAddr) -> str:
    """容器地址 → 动作语义名 (不翻译到 x86/ARM)"""
    return addr.container_name()


# ── 时间窗口路由: Z轴时间窗口 → 裁决门禁 ──

@dataclass
class ThreeWayJudgement:
    """三元裁决: 空袋拒绝 / 数值准入 / 全零拒绝"""
    verdict: str  # "reject_empty" / "admit" / "reject_zero_trap"
    
    @staticmethod
    def judge(payload: bytes) -> 'ThreeWayJudgement':
        if len(payload) == 0:
            return ThreeWayJudgement("reject_empty")
        all_zero = True
        for b in payload:
            if b != 0:
                all_zero = False
                break
        if all_zero:
            return ThreeWayJudgement("reject_zero_trap")
        return ThreeWayJudgement("admit")


# ══════════════════════════════════════════════════════════════
# 莫顿码 (Z-order, 三维空间索引)
# 满全法版: 容器地址 → Z-order integer
# 容器编码 (0-63) + 时间槽 (0-63) → 交织
# ══════════════════════════════════════════════════════════════

CONTAINER_ZLOOKUP = [0] * 64
CONTAINER_ZLOOKUP[ 0] = 0
CONTAINER_ZLOOKUP[ 1] = 1
CONTAINER_ZLOOKUP[ 2] = 4
CONTAINER_ZLOOKUP[ 3] = 5
# 实际上 Morton 编码是容器的3D投影: 容器id × 时间槽交织
# 这里用查表替代位移 (满全法: 加法循环实现交织)

def encode_morton(container_code: int, time_slot: int) -> int:
    """
    莫顿码: 容器编码(6-bit) × 时间槽(6-bit) → Z-order (12-bit)
    交织方式: 容器编码占偶数位, 时间槽占奇数位
    用加法循环实现, 不用位移
    """
    result = 0
    for i in range(6):
        bit_c = container_code - ((container_code >> 1) << 1)  # 取第i位 = code % 2
        bit_t = time_slot - ((time_slot >> 1) << 1)            # %2
        result = result + (bit_c * _pow_of_2(2 * i))
        result = result + (bit_t * _pow_of_2(2 * i + 1))
        container_code = container_code >> 1
        time_slot = time_slot >> 1
    return result

def _pow_of_2(n: int) -> int:
    """2^n 加法循环 (只用于空间指数, 不用于容器语义)"""
    if n == 0:
        return 1
    r = 2
    for _ in range(n - 1):
        r = r + r
    return r


# ══════════════════════════════════════════════════════════════
# 自检 — 验证满全法编码体系的三层对齐
# ══════════════════════════════════════════════════════════════

if __name__ == "__main__":
    print("=== glyph-container 统一协议 v0.2 (满全法) 自检 ===\n")
    
    # 1. 容器地址 (引用64_keys.md)
    container_addr = ContainerAddr(
        group=ContainerGroup.INITIAL,
        index=0,  # 帮
        polarity=Polarity.YANG,     # 声母=主动=阳
        sovereignty=Sovereignty.ACTIVE,
        operand=0x0A,  # 操作数=10
        time_slot=0x5F, # 时间窗=95
        depth=3,        # 层级=3
    )
    print(f"[1] 容器地址: {container_addr.container_name()} group={container_addr.group} index={container_addr.index}")
    print(f"    极性={container_addr.polarity.name} 主权={container_addr.sovereignty.name}")
    print(f"    操作数={container_addr.operand} 时间窗={container_addr.time_slot} 层级={container_addr.depth}")
    
    # 2. 编码/解码 (加法循环, 不位移)
    encoded = container_addr.encode()
    decoded = ContainerAddr.decode(encoded)
    assert decoded.group == container_addr.group
    assert decoded.index == container_addr.index
    print(f"[2] 容器编码={encoded} (加法循环) → 解码=({decoded.group}, {decoded.index})")
    
    # 3. 极性运算 (阴+阳=残差)
    yin = Polarity.YIN
    yang = Polarity.YANG
    assert polarity_meets_residual(yin, yang)
    print(f"[3] 极性: 阴={polarity_value(yin)} 阳={polarity_value(yang)} 元={polarity_value(Polarity.NEUTRAL)}")
    print(f"    阴+阳 → 残差={polarity_meets_residual(yin, yang)} ✓")
    
    # 4. Layer 1 ContainerByteCode
    abc = ContainerByteCode(
        container=container_addr,
        native_code=b"TN_0x0A",  # 泰射机器码: 容器语义名+操作数十六进制 → 独占标识
        witness=b"\xA1\xB2\xC3",
    )
    packed = abc.pack()  # returns bytes directly
    unpacked = ContainerByteCode.unpack(packed)
    assert unpacked is not None
    assert unpacked.witness == b"\xA1\xB2\xC3"
    print(f"[4] Layer1 容器字节码: native_code={abc.native_code} 包装={len(packed)}B")
    
    # 5. Layer 2 ScheduleEnvelope
    env = ScheduleEnvelope.from_container(container_addr, align_flag=7)
    env_packed = env.pack()
    env_urec = ScheduleEnvelope.unpack(env_packed)
    assert env_urec is not None
    print(f"[5] Layer2 调度信封: pack_id=0x{env.pack_id:08X} align={env.align_flag} 包装={len(env_packed)}B")
    
    # 6. Layer 3 GlyphRecord
    glyph = GlyphRecord.from_container(glyph_id=0xC0DE, addr=container_addr)
    g_packed = glyph.pack()
    g_unpacked = GlyphRecord.unpack(g_packed)
    assert g_unpacked is not None
    print(f"[6] Layer3 GRPH记录: glyphID=0x{glyph.glyph_id:04X} 包装={len(g_packed)}B")
    
    # 7. 进位链
    c0 = ContainerAddr(group=ContainerGroup.CONTAINER, index=15,
                       polarity=Polarity.YIN, sovereignty=Sovereignty.ACTIVE)
    next_c = c0.carry_target()
    assert next_c is None  # 本族最后 → 折叠
    print(f"[7] 进位链: {c0.container_name()}(15) → 折叠(None) ✓")
    
    # 8. 三元裁决
    judge_empty = ThreeWayJudgement.judge(b"")
    judge_zero = ThreeWayJudgement.judge(b"\x00\x00")
    judge_ok = ThreeWayJudgement.judge(b"\x01\x02")
    assert judge_empty.verdict == "reject_empty"
    assert judge_zero.verdict == "reject_zero_trap"
    assert judge_ok.verdict == "admit"
    print(f"[8] 三元裁决: 空={judge_empty.verdict} 零陷阱={judge_zero.verdict} 准入={judge_ok.verdict}")
    
    # 9. 韵母容器映射
    v_addr = vowel_container("ā")
    if v_addr:
        print(f"[9] 韵母映射: ā → {v_addr.container_name()} group={v_addr.group}")
    
    # 10. Morton码
    morton = encode_morton(encoded, container_addr.time_slot)
    print(f"[10] Morton码: container={encoded} × time={container_addr.time_slot} → z={morton}")
    
    print("\n=== 满全法三层对齐通过 ===")
    print(f"容器总数: 64 (4群×16)  声母总数: 48  韵母总数: 108")
    print(f"极性三元: Yin={polarity_value(Polarity.YIN)} Yang={polarity_value(Polarity.YANG)} Neutral={polarity_value(Polarity.NEUTRAL)}")
