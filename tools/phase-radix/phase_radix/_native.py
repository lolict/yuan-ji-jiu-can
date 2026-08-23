"""
_native.py — Python 薄壳 · 直接调用 C 核心 libcolor.so

这是颜色进制的真正本体。Python 层只是壳，
所有审判/衍射/统计全部由我们自己的 C 核心执行。

color_system_t 在 C 端占 35016 字节, Python 端作为不透明缓冲区分配。
"""

import ctypes
import os

# ── 加载共享库 ──────────────────────────────────────────────
_dir = os.path.dirname(os.path.abspath(__file__))
_lib_path = os.path.join(_dir, "..", "..", "..", "encoding", "libcolor.so")
_lib_path = os.path.normpath(_lib_path)

_lib = ctypes.CDLL(_lib_path)

# ── 结构体尺寸 (由 C 编译时确定) ────────────────────────────
COLOR_SYSTEM_SIZE = 35016
TYPE_DISCIPLINE_SIZE = 6152
DIFFRACTION_COURT_SIZE = 24680
WEAVING_GRAPH_SIZE = 4136

# ── 枚举常量 (与 C 头文件一致) ─────────────────────────────
TYPE_BASE      = 0x01
TYPE_TONE      = 0x02
TYPE_COMPOUND  = 0x03
TYPE_SYMBOL    = 0x04
TYPE_EXTRA     = 0x05
TYPE_SHENG_MU  = 0x06
TYPE_MODIFIER  = 0x07
TYPE_UNKNOWN   = 0xFF

LEVEL_L1 = 0x10
LEVEL_L2 = 0x20
LEVEL_L3 = 0x30

PAIR_PASS                = 0
PAIR_REJECT_CROSS_LEVEL  = 1
PAIR_REJECT_MODIFIER     = 2
PAIR_REJECT_CROSS_TONE   = 3
PAIR_REJECT_EXTRA        = 4
PAIR_REJECT_UNKNOWN      = 5
PAIR_REJECT_DIFFERENT    = 6

# ── 结构体映射 ─────────────────────────────────────────────

class DiffractionResult(ctypes.Structure):
    _fields_ = [
        ("name", ctypes.c_char_p),
        ("type_diffract", ctypes.c_int),       # symbol_type_t = 4 bytes
        ("anchor_diffract", ctypes.c_uint8),
        ("aesthetic_score", ctypes.c_float),
        ("balanced", ctypes.c_uint8),
        ("goal_code", ctypes.c_uint8),
        ("can_cooperate", ctypes.c_uint8),
        ("partner_count", ctypes.c_uint8),
        ("pressure_given", ctypes.c_float),
        ("pressure_received", ctypes.c_float),
        ("equilibrium_delta", ctypes.c_float),
    ]

class PairDiffraction(ctypes.Structure):
    _fields_ = [
        ("name_a", ctypes.c_char_p),
        ("name_b", ctypes.c_char_p),
        ("type_a", ctypes.c_int),          # symbol_type_t = 4 bytes
        ("type_b", ctypes.c_int),          # symbol_type_t = 4 bytes
        ("type_match", ctypes.c_uint8),
        ("anchor_match", ctypes.c_uint8),
        ("symmetry", ctypes.c_float),
        ("proportion", ctypes.c_float),
        ("density_balance", ctypes.c_float),
        ("overall_aesthetic", ctypes.c_float),
        ("weaving_tension", ctypes.c_float),
        ("weaving_balance", ctypes.c_float),
        ("is_weaving_ok", ctypes.c_uint8),
    ]

# ── 函数签名 ────────────────────────────────────────────────

_lib.color_system_init.argtypes = [ctypes.c_void_p]
_lib.color_system_init.restype = None

_lib.color_system_trial.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p]
_lib.color_system_trial.restype = ctypes.c_int

_lib.color_system_diffract.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(DiffractionResult)]
_lib.color_system_diffract.restype = None

_lib.color_system_pair_diffract.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(PairDiffraction)]
_lib.color_system_pair_diffract.restype = None

_lib.color_system_register.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int, ctypes.c_int, ctypes.c_uint8]
_lib.color_system_register.restype = ctypes.c_int

_lib.color_system_level_str.argtypes = [ctypes.c_int]
_lib.color_system_level_str.restype = ctypes.c_char_p

_lib.color_system_type_str.argtypes = [ctypes.c_int]
_lib.color_system_type_str.restype = ctypes.c_char_p

# ── 封装: ColorSystemRef ───────────────────────────────────

class ColorSystemRef:
    """颜色建制 C 核心的 Python 封装"""
    
    def __init__(self):
        """分配 C 结构体内存并初始化"""
        self._buf = ctypes.create_string_buffer(COLOR_SYSTEM_SIZE)
        self._keepalive = []  # 防止 Python bytes 被 GC (C 端存储指针而非复制)
        _lib.color_system_init(self._buf)
    
    def register(self, name: str, type: int, level: int, anchor: int = 0xFF) -> bool:
        """注册一个符号到系统"""
        n = name.encode('utf-8')
        self._keepalive.append(n)  # C 端存指针 · 必须保持 Python bytes 存活
        ret = _lib.color_system_register(self._buf, n, type, level, anchor)
        return ret == 0
    
    def register_shengmu_batch(self, names: list) -> int:
        """批量注册声母, 返回成功数"""
        ok = 0
        for name in names:
            if self.register(name, TYPE_SHENG_MU, LEVEL_L1, 0x30):
                ok += 1
        return ok
    
    def trial(self, name_a: str, name_b: str) -> int:
        """审判一对符号, 返回 PAIR_* 常量"""
        a = name_a.encode('utf-8')
        b = name_b.encode('utf-8')
        return _lib.color_system_trial(self._buf, a, b)
    
    def diffract(self, name: str) -> dict:
        """衍射一个符号, 返回属性字典"""
        n = name.encode('utf-8')
        result = DiffractionResult()
        _lib.color_system_diffract(self._buf, n, ctypes.byref(result))
        return {
            "name": result.name.decode('utf-8') if result.name else name,
            "type": result.type_diffract,
            "type_str": type_str(result.type_diffract),
            "anchor": result.anchor_diffract,
            "aesthetic": result.aesthetic_score,
            "balanced": bool(result.balanced),
            "pressure_given": result.pressure_given,
            "pressure_received": result.pressure_received,
            "equilibrium_delta": result.equilibrium_delta,
        }
    
    def pair_diffract(self, name_a: str, name_b: str) -> dict:
        """配对衍射"""
        a = name_a.encode('utf-8')
        b = name_b.encode('utf-8')
        result = PairDiffraction()
        _lib.color_system_pair_diffract(self._buf, a, b, ctypes.byref(result))
        return {
            "type_match": bool(result.type_match),
            "anchor_match": bool(result.anchor_match),
            "symmetry": result.symmetry,
            "proportion": result.proportion,
            "density_balance": result.density_balance,
            "overall_aesthetic": result.overall_aesthetic,
            "weaving_ok": bool(result.is_weaving_ok),
        }
    
    def verdict_str(self, verdict: int) -> str:
        names = {
            0: "PASS (通过)",
            1: "REJECT (跨层级)",
            2: "REJECT (修饰符)",
            3: "REJECT (跨基调)",
            4: "REJECT (未归类)",
            5: "REJECT (未分类)",
            6: "REJECT (类型不合)",
        }
        return names.get(verdict, f"UNKNOWN ({verdict})")


# ── 声音进制扩展 ──────────────────────────────────────────────

# 音色常量
SHENGMU_TIMBRE = 0x06
YUNMU_TIMBRE = 0x01
TONE_TIMBRE = 0x02

# 轮廓常量
CONTOUR_FLAT = 0
CONTOUR_RISE = 1
CONTOUR_FALL = 2
CONTOUR_DIP = 3
CONTOUR_PEAK = 4

SOUND_RADIX_MAGIC = 0x534E4452
SOUND_MAX_EVENTS = 254
SOUND_SYSTEM_SIZE = 8 + 256 * 26  # 估算 sound_radix_t 大小


class SoundEvent(ctypes.Structure):
    _fields_ = [
        ("symbol_id", ctypes.c_uint16),
        ("frequency", ctypes.c_uint16),
        ("timbre", ctypes.c_uint8),
        ("contour", ctypes.c_uint8),
        ("duration", ctypes.c_uint16),
        ("amplitude", ctypes.c_uint8),
        ("symbol", ctypes.c_char * 16),
    ]


SOUND_RADIX_SIZE = 6616


# 声音函数
_lib.sound_radix_init.argtypes = [ctypes.c_void_p]
_lib.sound_radix_init.restype = None

_lib.sound_radix_encode_syllable.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_int]
_lib.sound_radix_encode_syllable.restype = ctypes.c_int

_lib.sound_radix_get_duration.argtypes = [ctypes.c_void_p]
_lib.sound_radix_get_duration.restype = ctypes.c_uint32

_lib.sound_radix_encode.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_uint16, ctypes.c_uint8, ctypes.c_uint16, ctypes.c_uint8, ctypes.c_uint8]
_lib.sound_radix_encode.restype = ctypes.c_int

# ── 公约数地址容器 ──────────────────────────────────────────

GONGYUESHU_SIZE = 4 + 254 * 24  # 约 6100 bytes

_lib.gongyueshu_init.argtypes = [ctypes.c_void_p]
_lib.gongyueshu_init.restype = None

_lib.gongyueshu_encode.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint8]
_lib.gongyueshu_encode.restype = ctypes.c_int

_lib.gongyueshu_get_packed.argtypes = [ctypes.c_void_p, ctypes.c_uint16]
_lib.gongyueshu_get_packed.restype = ctypes.c_uint32

_lib.gongyueshu_get_bit_length.argtypes = [ctypes.c_void_p, ctypes.c_uint16]
_lib.gongyueshu_get_bit_length.restype = ctypes.c_uint8

_lib.gongyueshu_freq.argtypes = [ctypes.c_uint8]
_lib.gongyueshu_freq.restype = ctypes.c_uint16


class GongyueshuRef:
    """公约数地址容器的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(8 + 254 * 32)  # 足够大
        self._keepalive = []
        _lib.gongyueshu_init(self._buf)
    
    def encode(self, shengmu: str, yunmu: str, tone: int) -> bool:
        """编码一个音节为公约数地址"""
        s = (shengmu or "").encode('utf-8')
        y = yunmu.encode('utf-8')
        self._keepalive.extend([s, y])
        return _lib.gongyueshu_encode(self._buf, s, y, tone) == 0
    
    def get_packed(self, idx: int) -> tuple:
        """获取紧凑编码 (值, 位长)"""
        val = _lib.gongyueshu_get_packed(self._buf, idx)
        bits = _lib.gongyueshu_get_bit_length(self._buf, idx)
        return val, bits
    
    @property
    def count(self) -> int:
        return int.from_bytes(bytes(self._buf[4:6]), 'little')


# ── 多维几何容器 ──────────────────────────────────────────

DIMENSION_SIZE = 32168  # sizeof(dimension_space_t)

_lib.dimension_init.argtypes = [ctypes.c_void_p]
_lib.dimension_init.restype = None

_lib.dimension_register_container.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint16]
_lib.dimension_register_container.restype = ctypes.c_int

_lib.dimension_compute_border.argtypes = [ctypes.c_void_p]
_lib.dimension_compute_border.restype = ctypes.c_int

_lib.dimension_linear_addr.argtypes = [ctypes.c_void_p, ctypes.c_uint16, ctypes.c_uint16, ctypes.c_uint8]
_lib.dimension_linear_addr.restype = ctypes.c_uint32


class DimensionRef:
    """多维几何容器的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(DIMENSION_SIZE)
        _lib.dimension_init(self._buf)
        self._setup_default_containers()
    
    def _setup_default_containers(self):
        """注册默认的子容器 (声母48 韵母41 声调4)"""
        _lib.dimension_register_container(self._buf, 0, "声母".encode('utf-8'), 48)
        _lib.dimension_register_container(self._buf, 1, "韵母".encode('utf-8'), 41)
        _lib.dimension_register_container(self._buf, 2, "声调".encode('utf-8'), 4)
        # 维度尺寸直接写入 (跳过元素标签)
        import struct
        # 容器尺寸在 offsets: 4+48+8=60, 60+48+8=116, 116+48+8=172
        # 更简单: 直接通过结构体尺寸字段写入
        # containers[0].size at offset 4(magic) + 2(ver) + 1(dim) + 1(reserved) = 8
        # 每个 sub_container_t: name(8) + size(2) + cap(2) + labels(8) = 24
        base = 8
        for i in range(3):
            off = base + i * 24
            size_off = off + 8  # after name pointer
            struct.pack_into('<H', self._buf, size_off, [48, 41, 4][i])
            cap_off = off + 10
            struct.pack_into('<H', self._buf, cap_off, [48, 41, 4][i])
        _lib.dimension_compute_border(self._buf)
    
    def compute_border(self) -> int:
        return _lib.dimension_compute_border(self._buf)
    
    def linear_addr(self, sm: int, ym: int, tone: int) -> int:
        """多维坐标 → 一维线性地址"""
        return _lib.dimension_linear_addr(self._buf, sm, ym, tone)


# ── 汉字几何容器 ──────────────────────────────────────────

HANZI_GEOMETRY_SIZE = 17288  # sizeof(hanzi_geometry_t)

# 结构类型
STRUCT_LEFT_RIGHT = 1
STRUCT_TOP_BOTTOM = 2
STRUCT_SURROUND = 3
STRUCT_SEMI = 4
STRUCT_LCR = 5
STRUCT_TMB = 6
STRUCT_PINZI = 7
STRUCT_SINGLE = 8

# 笔画类型
STROKE_HENG = 0
STROKE_SHU = 1
STROKE_PIE = 2
STROKE_NA = 3
STROKE_ZHE = 4


_lib.hanzi_geom_init.argtypes = [ctypes.c_void_p]
_lib.hanzi_geom_init.restype = None

_lib.hanzi_geom_add_entry.argtypes = [ctypes.c_void_p, ctypes.c_uint16, ctypes.c_uint16, ctypes.c_uint16, ctypes.c_int, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8)]
_lib.hanzi_geom_add_entry.restype = ctypes.c_int

_lib.hanzi_geom_packed.argtypes = [ctypes.c_void_p, ctypes.c_uint16]
_lib.hanzi_geom_packed.restype = ctypes.c_uint32


class HanziGeomRef:
    """汉字几何容器的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(HANZI_GEOMETRY_SIZE)
        _lib.hanzi_geom_init(self._buf)
    
    def add(self, char_code: int, radical1: int, radical2: int,
            structure: int, stroke_count: int, strokes: list) -> bool:
        """添加一个汉字条目"""
        strokes_arr = (ctypes.c_uint8 * len(strokes))(*strokes)
        ret = _lib.hanzi_geom_add_entry(self._buf, char_code, radical1, radical2,
                                         structure, stroke_count, strokes_arr)
        return ret == 0
    
    def get_packed(self, idx: int) -> int:
        """获取紧凑编码"""
        return _lib.hanzi_geom_packed(self._buf, idx)


# ── 负样本容器 (免疫记忆库) ──────────────────────────────

NEGATIVE_SAMPLE_SIZE = 5128  # sizeof(negative_sample_t)

REASON_CROSS_LEVEL = 1
REASON_CROSS_TYPE = 2
REASON_CROSS_ANCHOR = 3
REASON_AESTHETIC_FAIL = 4
REASON_WEAVING_FAIL = 5
REASON_BLACKLIST = 6
REASON_REPEAT = 7
REASON_DEGRADATION = 8


_lib.negative_init.argtypes = [ctypes.c_void_p]
_lib.negative_init.restype = None

_lib.negative_judge.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32, ctypes.POINTER(ctypes.c_uint8)]
_lib.negative_judge.restype = ctypes.c_int

_lib.negative_writeback.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint8]
_lib.negative_writeback.restype = ctypes.c_int

_lib.negative_law_judge.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32]
_lib.negative_law_judge.restype = ctypes.c_int


class NegativeRef:
    """负样本容器的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(NEGATIVE_SAMPLE_SIZE + 100)
        _lib.negative_init(self._buf)
    
    def judge(self, sym_a: int, sym_b: int) -> tuple:
        """裁定一对符号:返回 (判定, 原因)"""
        reason = ctypes.c_uint8(0)
        v = _lib.negative_judge(self._buf, sym_a, sym_b, ctypes.byref(reason))
        return v, reason.value
    
    def writeback(self, sym_a: int, sym_b: int, reason: int) -> bool:
        """回写记忆库"""
        return _lib.negative_writeback(self._buf, sym_a, sym_b, reason) == 0
    
    def law_judge(self, vA: int, vB: int) -> bool:
        """判定树: vA + vB == 2·v_core"""
        return _lib.negative_law_judge(self._buf, vA, vB) == 1


class SoundRadixRef:
    """声音进制 C 核心的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(SOUND_RADIX_SIZE)
        _lib.sound_radix_init(self._buf)
        self._keepalive = []
    
    def encode_syllable(self, shengmu: str, yunmu: str, tone: int) -> bool:
        """编码一个音节 (声母+韵母+声调)"""
        s = (shengmu or "").encode('utf-8')
        y = yunmu.encode('utf-8')
        self._keepalive.append(s)
        self._keepalive.append(y)
        return _lib.sound_radix_encode_syllable(self._buf, s, y, tone) == 0
    
    def duration_ms(self) -> int:
        """获取总时长 (毫秒)"""
        return _lib.sound_radix_get_duration(self._buf)


def type_str(t: int) -> str:
    names = {
        0x01: "基韵锚",
        0x02: "声调韵",
        0x03: "复韵母",
        0x04: "符号",
        0x05: "溢出",
        0x06: "声母",
        0x07: "修饰",
        0xFF: "未知",
    }
    return names.get(t, f"?0x{t:02X}")


def level_str(l: int) -> str:
    names = {
        0x10: "L1-基元",
        0x20: "L2-修饰",
        0x30: "L3-装配",
        0xFF: "未知",
    }
    return names.get(l, f"?0x{l:02X}")
