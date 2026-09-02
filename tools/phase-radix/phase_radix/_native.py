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


# ── 产权流 ────────────────────────────────────────────────

# ── 产权流结构体尺寸 ────────────────────────────────────
# sizeof(property_flow_t) = 19452 (由 C 编译时确定)
#   records[254]: 254 × 12 = 3048
#   requests[1024]: 1024 × 16 = 16384
#   + 头部字段: 20
#   = 19452
PROPERTY_FLOW_SIZE = 19452

PROPERTY_ALLOW             = 0
PROPERTY_DENY_UNREGISTERED = 1
PROPERTY_DENY_WRONG_OWNER  = 2
PROPERTY_DENY_HISTORY      = 3
PROPERTY_CONDITIONAL       = 4

USE_READ    = 0x01
USE_WRITE   = 0x02
USE_EXPORT  = 0x03
USE_DERIVE  = 0x04


class PropertyFlowRef:
    """产权流 C 核心的 Python 封装"""
    
    def __init__(self):
        self._buf = ctypes.create_string_buffer(PROPERTY_FLOW_SIZE)
        _lib.property_flow_init(self._buf)
    
    def register(self, symbol: int, owner: int, is_public: bool = False) -> bool:
        return _lib.property_flow_register(self._buf, symbol, owner, is_public) == 0
    
    def check(self, symbol: int, requester: int, use_type: int = USE_READ) -> int:
        return _lib.property_flow_check(self._buf, symbol, requester, use_type)
    
    def grant(self, symbol: int, grantee: int):
        _lib.property_flow_grant(self._buf, symbol, grantee)
    
    def verdict_str(self, verdict: int) -> str:
        names = {
            0: "ALLOW (允许)",
            1: "DENY (未登记)",
            2: "DENY (产权不符)",
            3: "DENY (侵权历史)",
            4: "CONDITIONAL (有条件)",
        }
        return names.get(verdict, f"UNKNOWN ({verdict})")


# ── WASM 内核 ──────────────────────────────────────────────

import struct as _struct

# sizeof(wk_kernel_t) ≈ 4+2+1+1+8+4+4 + 254*8 + 2 + 8 + 4+4+4 = ~20KB
# 精确值通过编译时 sizeof 获取
WK_KERNEL_SIZE = 2080   # sizeof(wk_kernel_t) = 2080 (编译时确定)

WK_MAGIC     = 0x574B4E4C
WK_VERSION   = 0x0003

# 囡囝四态
WK_STATE_NN   = 0x00
WK_STATE_NJSG = 0x01
WK_STATE_JJ   = 0x02
WK_STATE_JN   = 0x03

# 极性
WK_POLARITY_WIFE    = 0x0
WK_POLARITY_HUSBAND = 0x1
WK_POLARITY_SYSTEM  = 0x2

# 八流
WK_FLOW_PRELOAD    = 0x00
WK_FLOW_CATEGORY   = 0x01
WK_FLOW_CLUSTER    = 0x02
WK_FLOW_DISTINGUISH = 0x03
WK_FLOW_PROPERTY   = 0x04
WK_FLOW_INTERPRET  = 0x05
WK_FLOW_RELATION   = 0x06
WK_FLOW_CONCEPT    = 0x07

# Cell 编码辅助
def _wk_cell(mccp, sovereignty, polarity, witness):
    return ((witness & 0xF) << 60) | ((polarity & 0xF) << 56) | \
           ((sovereignty & 0xFFFFFF) << 32) | (mccp & 0xFFFFFFFF)

def _wk_cell_mccp(cell):
    return cell & 0xFFFFFFFF

def _wk_cell_sovereign(cell):
    return (cell >> 32) & 0xFFFFFF

def _wk_cell_polarity(cell):
    return (cell >> 56) & 0xF

def _wk_cell_witness(cell):
    return (cell >> 60) & 0xF


# WASM 内核函数签名
_lib.wk_init.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_uint32]
_lib.wk_init.restype = None

_lib.wk_add_cell.argtypes = [ctypes.c_void_p, ctypes.c_uint64]
_lib.wk_add_cell.restype = ctypes.c_int

_lib.wk_find_by_mccp.argtypes = [ctypes.c_void_p, ctypes.c_uint32]
_lib.wk_find_by_mccp.restype = ctypes.c_uint64

_lib.wk_set_state.argtypes = [ctypes.c_void_p, ctypes.c_uint8]
_lib.wk_set_state.restype = ctypes.c_int

_lib.wk_route.argtypes = [ctypes.c_void_p, ctypes.c_uint64, ctypes.c_uint8]
_lib.wk_route.restype = ctypes.c_int

_lib.wk_stats.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_size_t]
_lib.wk_stats.restype = None

_lib.wk_save.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
_lib.wk_save.restype = ctypes.c_int

_lib.wk_load.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
_lib.wk_load.restype = ctypes.c_int

_lib.wk_serialized_size.argtypes = []
_lib.wk_serialized_size.restype = ctypes.c_uint32

# ── 距离感知聚类 (mind_organ) ─────────────────────────────

# 距离类型
MO_DIST_GEOMETRIC = 0
MO_DIST_SOVEREIGN = 1
MO_DIST_POLARITY  = 2
MO_DIST_WITNESS   = 3
MO_DIST_MCCP      = 4

# 最大 cell 数 (与 MO_MAX_CELLS 对齐)
MO_MAX_CELLS = 254

class MoCluster(ctypes.Structure):
    """单个簇 (与 C mo_cluster_t 对齐)"""
    _fields_ = [
        ("member_ids", ctypes.c_uint16 * 254),  # MO_MAX_CELLS
        ("member_count", ctypes.c_uint16),
        ("centroid_id", ctypes.c_uint16),
        ("cohesion", ctypes.c_float),
    ]

class MoResult(ctypes.Structure):
    """聚类结果 (与 C mo_result_t 对齐)"""
    _fields_ = [
        ("clusters", MoCluster * 32),  # MO_MAX_CLUSTERS
        ("cluster_count", ctypes.c_uint16),
        ("noise_count", ctypes.c_uint16),
        ("avg_cohesion", ctypes.c_float),
        ("dist_type", ctypes.c_int),
    ]

_lib.mo_distance.argtypes = [ctypes.c_uint64, ctypes.c_uint64, ctypes.c_int]
_lib.mo_distance.restype = ctypes.c_float

_lib.mo_cluster.argtypes = [ctypes.POINTER(ctypes.c_uint64), ctypes.c_uint32,
                             ctypes.c_float, ctypes.c_uint16, ctypes.c_int,
                             ctypes.POINTER(MoResult)]
_lib.mo_cluster.restype = None

_lib.mo_nearest.argtypes = [ctypes.POINTER(ctypes.c_uint64), ctypes.c_uint32,
                             ctypes.c_uint32, ctypes.c_int]
_lib.mo_nearest.restype = ctypes.c_int

_lib.mo_stats.argtypes = [ctypes.POINTER(MoResult), ctypes.c_char_p, ctypes.c_size_t]
_lib.mo_stats.restype = None


class MindOrganRef:
    """距离感知聚类 Python 封装"""
    
    def __init__(self):
        self._cells = []
    
    def add_cell(self, mccp, sovereignty, polarity, witness):
        cell = _wk_cell(mccp, sovereignty, polarity, witness)
        self._cells.append(cell)
        return len(self._cells) - 1
    
    def distance(self, i, j, dist_type=MO_DIST_GEOMETRIC):
        if i >= len(self._cells) or j >= len(self._cells):
            return -1.0
        return _lib.mo_distance(self._cells[i], self._cells[j], dist_type)
    
    def nearest(self, idx, dist_type=MO_DIST_GEOMETRIC):
        if idx >= len(self._cells):
            return -1
        arr = (ctypes.c_uint64 * len(self._cells))(*self._cells)
        return _lib.mo_nearest(arr, len(self._cells), idx, dist_type)
    
    def cluster(self, eps=0.3, min_pts=2, dist_type=MO_DIST_GEOMETRIC):
        if not self._cells:
            return None
        arr = (ctypes.c_uint64 * len(self._cells))(*self._cells)
        result = MoResult()
        _lib.mo_cluster(arr, len(self._cells), eps, min_pts, dist_type, ctypes.byref(result))
        buf = ctypes.create_string_buffer(512)
        _lib.mo_stats(ctypes.byref(result), buf, 512)
        return buf.value.decode('utf-8', errors='replace')


class WasmKernelRef:
    """WASM 内核 C 核心的 Python 封装"""
    
    def __init__(self, mem_capacity=65536):
        self._buf = (ctypes.c_uint8 * WK_KERNEL_SIZE)()
        self._mem = (ctypes.c_uint8 * mem_capacity)()
        _lib.wk_init(ctypes.byref(self._buf), ctypes.byref(self._mem), mem_capacity)
    
    def add_cell(self, mccp, sovereignty, polarity, witness):
        """添加一个 64位 cell"""
        cell = _wk_cell(mccp, sovereignty, polarity, witness)
        rc = _lib.wk_add_cell(ctypes.byref(self._buf), cell)
        return rc == 0
    
    def find_mccp(self, mccp):
        """通过 MCCP 查找 cell"""
        return _lib.wk_find_by_mccp(ctypes.byref(self._buf), mccp)
    
    def set_state(self, state):
        """设置囡囝四态"""
        return _lib.wk_set_state(ctypes.byref(self._buf), state) == 0
    
    @property
    def state(self):
        return self._buf[6]
    
    @property
    def locked(self):
        return bool(self._buf[7] & 0x02)
    
    @locked.setter
    def locked(self, val):
        if val:
            self._buf[7] = self._buf[7] | 0x02
        else:
            self._buf[7] = self._buf[7] & ~0x02
    
    def activate_flow(self, flow_type):
        """激活一流"""
        flow_offset = 2058  # offsetof(wk_kernel_t, flow_state)
        self._buf[flow_offset + flow_type] = 1
    
    def route(self, mccp, sovereignty, polarity, witness, flow_type):
        """路由判决"""
        cell = _wk_cell(mccp, sovereignty, polarity, witness)
        return _lib.wk_route(ctypes.byref(self._buf), cell, flow_type)
    
    def route_str(self, rc):
        names = {0: "ALLOW", -1: "INVALID", -2: "DENY未登记", -3: "DENY主权不符", -4: "DENY流休眠"}
        return names.get(rc, f"UNKNOWN({rc})")
    
    def stats(self):
        """获取统计字符串"""
        buf = ctypes.create_string_buffer(256)
        _lib.wk_stats(ctypes.byref(self._buf), buf, 256)
        return buf.value.decode('utf-8', errors='replace')
    
    def save(self, filename: str):
        """保存内核状态到文件"""
        return _lib.wk_save(ctypes.byref(self._buf), filename.encode('utf-8')) == 0
    
    def load(self, filename: str):
        """从文件加载内核状态"""
        return _lib.wk_load(ctypes.byref(self._buf), filename.encode('utf-8')) == 0
    
    @staticmethod
    def cell_str(cell):
        return f"MCCP=0x{_wk_cell_mccp(cell):08X} SOV=0x{_wk_cell_sovereign(cell):05X} POL={_wk_cell_polarity(cell)} WIT={_wk_cell_witness(cell)}"


# ── 全局互锁横轴 ──────────────────────────────────────────

# ── 产权流函数签名 ──
_lib.property_flow_init.argtypes = [ctypes.c_void_p]
_lib.property_flow_init.restype = None

_lib.property_flow_register.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint8, ctypes.c_int]
_lib.property_flow_register.restype = ctypes.c_int

_lib.property_flow_check.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint8, ctypes.c_uint8]
_lib.property_flow_check.restype = ctypes.c_int

_lib.property_flow_grant.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint8]
_lib.property_flow_grant.restype = None

_lib.interlock_init.argtypes = []
_lib.interlock_init.restype = None

_lib.interlock_register_container.argtypes = [ctypes.c_uint8, ctypes.c_char_p]
_lib.interlock_register_container.restype = ctypes.c_int

_lib.interlock_register_symbol.argtypes = [ctypes.c_uint32, ctypes.c_uint8]
_lib.interlock_register_symbol.restype = ctypes.c_int

_lib.interlock_check_symbol.argtypes = [ctypes.c_uint32]
_lib.interlock_check_symbol.restype = ctypes.c_int

_lib.interlock_activate.argtypes = []
_lib.interlock_activate.restype = None


class InterlockRef:
    """全局互锁横轴的 Python 封装"""
    
    def __init__(self):
        _lib.interlock_init()
    
    def register_container(self, container_id: int, name: str) -> bool:
        return _lib.interlock_register_container(container_id, name.encode('utf-8')) == 0
    
    def register_symbol(self, symbol: int, container: int) -> bool:
        return _lib.interlock_register_symbol(symbol, container) == 0
    
    def check_symbol(self, symbol: int) -> bool:
        return _lib.interlock_check_symbol(symbol) == 1
    
    def activate(self):
        _lib.interlock_activate()
    
    def stats(self) -> dict:
        """获取横轴状态 (通过解析 stdout 不够优雅, 这里只返回激活状态) """
        return {"active": True}


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


# ═══════════════════════════════════════════════════════════════
# MetaIME 七层编码引擎 Python 绑定
# ═══════════════════════════════════════════════════════════════

METAIME_ENGINE_SIZE    = 24720
METAIME_PATTERN_SIZE   = 34
METAIME_ACTION_SIZE    = 48
METAIME_MATCH_SIZE     = 56

METAIME_POL_WIFE     = 0x0
METAIME_POL_HUSBAND  = 0x1
METAIME_POL_SYSTEM   = 0x2
METAIME_POL_OBSERVER = 0x3
METAIME_POL_ARBITER  = 0x4
METAIME_POL_SECRET   = 0x5
METAIME_POL_EMPTY    = 0x6

METAIME_FLOW_PRELOAD    = 0x0
METAIME_FLOW_CATEGORY   = 0x1
METAIME_FLOW_CLUSTER    = 0x2
METAIME_FLOW_DISTINGUISH = 0x3
METAIME_FLOW_PROPERTY   = 0x4
METAIME_FLOW_INTERPRET  = 0x5
METAIME_FLOW_RELATION   = 0x6
METAIME_FLOW_CONCEPT    = 0x7

SYLLABLE_WILDCARD = 0xFFFF


class _MetaImePattern(ctypes.Structure):
    _fields_ = [
        ("syllables", ctypes.c_uint16 * 16),
        ("len", ctypes.c_uint8),
        ("wildcard_only", ctypes.c_uint8),
    ]


class _MetaImeAction(ctypes.Structure):
    _fields_ = [
        ("text", ctypes.c_char * 32),
        ("cell", ctypes.c_uint64),
        ("flow", ctypes.c_uint8),
        ("priority", ctypes.c_uint8),
    ]


class _MetaImeMatch(ctypes.Structure):
    _fields_ = [
        ("rule_id", ctypes.c_uint16),
        ("rule_index", ctypes.c_uint8),
        ("confidence", ctypes.c_uint8),
        ("action", _MetaImeAction),
    ]


_lib.metaime_init.argtypes = [ctypes.c_void_p]
_lib.metaime_init.restype = None
_lib.metaime_add_rule.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_void_p]
_lib.metaime_add_rule.restype = ctypes.c_int
_lib.metaime_feed.argtypes = [ctypes.c_void_p, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8]
_lib.metaime_feed.restype = ctypes.c_int
_lib.metaime_try_match.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
_lib.metaime_try_match.restype = ctypes.c_int
_lib.metaime_autofire.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_size_t,
                                    ctypes.POINTER(ctypes.c_uint64)]
_lib.metaime_autofire.restype = ctypes.c_int
_lib.metaime_reset.argtypes = [ctypes.c_void_p]
_lib.metaime_reset.restype = None
_lib.metaime_stats.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_size_t]
_lib.metaime_stats.restype = None
_lib.metaime_encode.argtypes = [ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8,
                                 ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint32]
_lib.metaime_encode.restype = ctypes.c_uint64
_lib.metaime_decode.argtypes = [ctypes.c_uint64,
                                 ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8),
                                 ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8),
                                 ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8),
                                 ctypes.POINTER(ctypes.c_uint32)]
_lib.metaime_decode.restype = None


def _make_syllable(sm, ym, tone):
    return ((sm & 0x3F) << 0) | ((ym & 0x3F) << 6) | ((tone & 0x7) << 12)


def _make_token(sm, ym, tone, pol, wit, flow, sov):
    return ((sm & 0x3F) << 42) | ((ym & 0xFF) << 34) | \
           ((tone & 0x7) << 31) | ((pol & 0x7) << 28) | \
           ((wit & 0xF) << 24) | ((flow & 0x7) << 21) | (sov & 0x1FFFFF)


class MetaImeEngine:
    """MetaIME 七层编码引擎 Python 封装"""

    def __init__(self):
        self._buf = (ctypes.c_uint8 * METAIME_ENGINE_SIZE)()
        _lib.metaime_init(ctypes.byref(self._buf))

    @property
    def auto_fire(self):
        return self._buf[24652]

    @auto_fire.setter
    def auto_fire(self, val):
        self._buf[24652] = max(0, min(100, val))

    def add_rule(self, syllables, text, cell=0, flow=0, priority=0):
        pat = _MetaImePattern()
        for i, s in enumerate(syllables):
            if i >= 16:
                break
            if s == SYLLABLE_WILDCARD:
                pat.syllables[i] = SYLLABLE_WILDCARD
            elif isinstance(s, tuple):
                pat.syllables[i] = _make_syllable(s[0], s[1], s[2])
            else:
                pat.syllables[i] = int(s)
        pat.len = min(len(syllables), 16)

        act = _MetaImeAction()
        enc = text.encode('utf-8')[:31]
        # 将 bytes 复制到 c_char 数组
        raw = (ctypes.c_char * 32).from_address(ctypes.addressof(act))
        ctypes.memmove(raw, enc, len(enc))
        act.cell = cell
        act.flow = flow
        act.priority = priority

        return _lib.metaime_add_rule(ctypes.byref(self._buf),
                                       ctypes.byref(pat), ctypes.byref(act))

    def feed(self, sm, ym, tone):
        rc = _lib.metaime_feed(ctypes.byref(self._buf), sm, ym, tone)
        match = _MetaImeMatch()
        if rc == 1:
            _lib.metaime_try_match(ctypes.byref(self._buf), ctypes.byref(match))
            return {
                "matched": True,
                "rule_id": match.rule_id,
                "confidence": match.confidence,
                "text": match.action.text.decode('utf-8', errors='replace'),
                "cell": match.action.cell,
                "flow": match.action.flow,
                "priority": match.action.priority,
            }
        return {"matched": False}

    def try_match(self):
        match = _MetaImeMatch()
        rc = _lib.metaime_try_match(ctypes.byref(self._buf), ctypes.byref(match))
        if rc == 1:
            return {
                "matched": True,
                "rule_id": match.rule_id,
                "confidence": match.confidence,
                "text": match.action.text.decode('utf-8', errors='replace'),
                "cell": match.action.cell,
                "flow": match.action.flow,
            }
        return {"matched": False}

    def autofire(self):
        buf = ctypes.create_string_buffer(64)
        cell = ctypes.c_uint64(0)
        rc = _lib.metaime_autofire(ctypes.byref(self._buf), buf, 64, ctypes.byref(cell))
        if rc == 1:
            return {"fired": True, "text": buf.value.decode('utf-8', errors='replace'), "cell": cell.value}
        return {"fired": False}

    def reset(self):
        _lib.metaime_reset(ctypes.byref(self._buf))

    def stats(self):
        buf = ctypes.create_string_buffer(256)
        _lib.metaime_stats(ctypes.byref(self._buf), buf, 256)
        return buf.value.decode('utf-8', errors='replace')

    @staticmethod
    def encode(sm, ym, tone, pol=0, wit=0, flow=0, sovereignty=0):
        return _lib.metaime_encode(sm, ym, tone, pol, wit, flow, sovereignty)

    @staticmethod
    def decode(cell):
        sm = ctypes.c_uint8(); ym = ctypes.c_uint8(); tone = ctypes.c_uint8()
        pol = ctypes.c_uint8(); wit = ctypes.c_uint8(); flow = ctypes.c_uint8()
        sov = ctypes.c_uint32()
        _lib.metaime_decode(cell,
                             ctypes.byref(sm), ctypes.byref(ym), ctypes.byref(tone),
                             ctypes.byref(pol), ctypes.byref(wit), ctypes.byref(flow),
                             ctypes.byref(sov))
        return {
            "shengmu": sm.value, "yunmu": ym.value, "tone": tone.value,
            "polarity": pol.value, "wit": wit.value, "flow": flow.value,
            "sovereignty": sov.value,
        }

    @staticmethod
    def syllable(sm, ym, tone):
        return _make_syllable(sm, ym, tone)

    @staticmethod
    def token(sm, ym, tone, pol, wit, flow, sov):
        return _make_token(sm, ym, tone, pol, wit, flow, sov)


# ── 天庭空间 ──────────────────────────────────────────────

COURT_COURT_SIZE = 6440

# 囡囝四态 (与 wasm_kernel.h 一致)
COURT_STATE_NN   = 0x00
COURT_STATE_NJSG = 0x01
COURT_STATE_JJ   = 0x02
COURT_STATE_JN   = 0x03

# 裁决结果
COURT_MERGE     = 0x01
COURT_REPEL     = 0x02
COURT_WITNESS   = 0x03
COURT_TRANSFORM = 0x04
COURT_DEFER     = 0x05
COURT_ESCALATE  = 0x06

# 裁决事由
REASON_SOUL_OK       = 0x00
REASON_SOV_CONFLICT  = 0x01
REASON_POL_MISMATCH  = 0x02
REASON_WIT_INSUFFICIENT = 0x03
REASON_FLOW_SLEEP    = 0x04
REASON_DISTANCE_FAR  = 0x05
REASON_HISTORY_REJECT = 0x06
REASON_STATE_FORBID  = 0x07

# 天庭空间 C API
_lib.court_init.argtypes = [ctypes.c_void_p]
_lib.court_init.restype = None

_lib.court_set_state.argtypes = [ctypes.c_void_p, ctypes.c_uint8]
_lib.court_set_state.restype = None

_lib.court_activate_flow.argtypes = [ctypes.c_void_p, ctypes.c_uint8]
_lib.court_activate_flow.restype = None

_lib.court_judge.argtypes = [ctypes.c_void_p, ctypes.c_uint64, ctypes.c_uint64, ctypes.c_uint8,
                              ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8),
                              ctypes.POINTER(ctypes.c_uint64)]
_lib.court_judge.restype = ctypes.c_float

_lib.court_retry_pending.argtypes = [ctypes.c_void_p]
_lib.court_retry_pending.restype = None

_lib.court_route_metaime.argtypes = [ctypes.c_void_p, ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8,
                                      ctypes.c_uint8, ctypes.c_uint8,
                                      ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint64)]
_lib.court_route_metaime.restype = ctypes.c_float

_lib.court_stats.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_size_t]
_lib.court_stats.restype = None

_lib.court_verdict_str.argtypes = [ctypes.c_uint8]
_lib.court_verdict_str.restype = ctypes.c_char_p

_lib.court_reason_str.argtypes = [ctypes.c_uint8]
_lib.court_reason_str.restype = ctypes.c_char_p


class CourtEngine:
    """天庭空间 · 规则引擎 Python 封装"""

    def __init__(self):
        self._buf = ctypes.create_string_buffer(COURT_COURT_SIZE)
        _lib.court_init(self._buf)

    def set_state(self, state):
        _lib.court_set_state(self._buf, state)

    def activate_flow(self, flow):
        _lib.court_activate_flow(self._buf, flow)

    def judge(self, cell_a, cell_b, flow=0):
        verdict = ctypes.c_uint8(0)
        reason = ctypes.c_uint8(0)
        out_cell = ctypes.c_uint64(0)
        conf = _lib.court_judge(self._buf, cell_a, cell_b, flow,
                                ctypes.byref(verdict), ctypes.byref(reason),
                                ctypes.byref(out_cell))
        v = verdict.value
        r = reason.value
        vs = _lib.court_verdict_str(v)
        rs = _lib.court_reason_str(r)
        if isinstance(vs, bytes):
            vs = vs.decode('utf-8', errors='replace')
        if isinstance(rs, bytes):
            rs = rs.decode('utf-8', errors='replace')
        return {
            'confidence': conf,
            'verdict': v,
            'verdict_str': vs,
            'reason': r,
            'reason_str': rs,
            'output_cell': out_cell.value,
        }

    def retry_pending(self):
        _lib.court_retry_pending(self._buf)

    def route_metaime(self, sm, ym, tone, pol, wit):
        verdict = ctypes.c_uint8(0)
        out_cell = ctypes.c_uint64(0)
        conf = _lib.court_route_metaime(self._buf, sm, ym, tone, pol, wit,
                                         ctypes.byref(verdict), ctypes.byref(out_cell))
        v = verdict.value
        vs = _lib.court_verdict_str(v)
        if isinstance(vs, bytes):
            vs = vs.decode('utf-8', errors='replace')
        return {
            'confidence': conf,
            'verdict': v,
            'verdict_str': vs,
            'output_cell': out_cell.value,
        }

    def stats(self):
        buf = ctypes.create_string_buffer(2048)
        _lib.court_stats(self._buf, buf, 2048)
        return buf.value.decode('utf-8', errors='replace')
