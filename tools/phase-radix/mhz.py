#!/usr/bin/env python3
"""
mhz — 满全法颜色进制 CLI

直接调用 C 核心 libcolor.so。
Python 只是壳，审判/衍射/注册全部由 C 执行。

用法:
  mhz trial <a> <b>     审判一对符号
  mhz diffract <name>   衍射分析一个符号
  mhz pair <a> <b>      配对衍射分析
  mhz register <name> <type> <level>  注册符号
  mhz demo              运行演示
  mhz status            系统状态
"""

import sys as _sys
import os

# 确保能 import 包
_sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from phase_radix._native import (
    ColorSystemRef,
    TYPE_BASE, TYPE_TONE, TYPE_COMPOUND, TYPE_SYMBOL,
    TYPE_EXTRA, TYPE_SHENG_MU, TYPE_MODIFIER,
    LEVEL_L1, LEVEL_L2, LEVEL_L3,
    type_str,
    SoundRadixRef,
    GongyueshuRef,
    DimensionRef,
    HanziGeomRef,
    NegativeRef,
    InterlockRef,
    PropertyFlowRef,
    PROPERTY_ALLOW, PROPERTY_DENY_UNREGISTERED, PROPERTY_DENY_WRONG_OWNER,
    PROPERTY_DENY_HISTORY, PROPERTY_CONDITIONAL,
    USE_READ, USE_WRITE, USE_EXPORT, USE_DERIVE,
    STRUCT_LEFT_RIGHT, STRUCT_TOP_BOTTOM, STRUCT_SURROUND, STRUCT_SEMI,
    STRUCT_LCR, STRUCT_TMB, STRUCT_PINZI, STRUCT_SINGLE,
    STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_ZHE,
    REASON_CROSS_LEVEL, REASON_CROSS_TYPE, REASON_CROSS_ANCHOR,
    CONTOUR_FLAT, CONTOUR_RISE, CONTOUR_FALL, CONTOUR_DIP,
    WasmKernelRef,
    WK_STATE_NN, WK_STATE_NJSG, WK_STATE_JJ, WK_STATE_JN,
    WK_POLARITY_WIFE, WK_POLARITY_HUSBAND, WK_POLARITY_SYSTEM,
    WK_FLOW_PRELOAD, WK_FLOW_CATEGORY, WK_FLOW_CLUSTER, WK_FLOW_DISTINGUISH,
    WK_FLOW_PROPERTY, WK_FLOW_INTERPRET, WK_FLOW_RELATION, WK_FLOW_CONCEPT,
)

# 容器标识 (与 global_interlock.h 一致)
CONT_COLOR_SYSTEM = 0x01
CONT_SOUND_RADIX  = 0x02
CONT_GONGYUESHU   = 0x03
CONT_DIMENSION    = 0x04
CONT_HANZI_GEOM   = 0x05
CONT_NEGATIVE     = 0x06

CONT_NAMES = {
    0x01: "颜色进制",
    0x02: "声音进制",
    0x03: "公约数地址",
    0x04: "多维几何",
    0x05: "汉字几何",
    0x06: "负样本",
}


# ── 声母表 ──────────────────────────────────────────────────
SHENGMU_48 = [
    "b","p","m","f","d","t","n","l",
    "g","k","h","j","q","x","zh","ch",
    "sh","r","z","c","s","y","w","y",
    "y","w","ȵ","ŋ","ɕ","ʈʂ","ʈʂʰ","ʂ",
    "ts","tsʰ","tɕ","tɕʰ","x","ɣ","ɲ","ʑ",
    "tʂ","tʂʰ","ɖ","ɳ","ɭ","ʐ","ɻ","ʔ",
]


def cmd_trial(sys: ColorSystemRef, a: str, b: str):
    """审判一对符号"""
    v = sys.trial(a, b)
    label = sys.verdict_str(v)
    print(f"{a} vs {b}: {label}")
    return v


def cmd_diffract(sys: ColorSystemRef, name: str):
    """衍射分析"""
    d = sys.diffract(name)
    print(f"符号: {d['name']}")
    print(f"  类型: {d['type_str']}")
    print(f"  锚点: 0x{d['anchor']:02X}")
    print(f"  审美分: {d['aesthetic']:.2f}")
    print(f"  出力: {d['pressure_given']:.2f}")
    print(f"  受力: {d['pressure_received']:.2f}")
    print(f"  失衡: {d['equilibrium_delta']:.2f}")
    print(f"  平衡: {'是' if d['balanced'] else '否'}")


def cmd_pair(sys: ColorSystemRef, a: str, b: str):
    """配对衍射"""
    pd = sys.pair_diffract(a, b)
    print(f"配对: {a} ↔ {b}")
    print(f"  类型匹配: {'是' if pd['type_match'] else '否'}")
    print(f"  锚点匹配: {'是' if pd['anchor_match'] else '否'}")
    print(f"  对称度: {pd['symmetry']:.2f}")
    print(f"  比例: {pd['proportion']:.2f}")
    print(f"  密度平衡: {pd['density_balance']:.2f}")
    print(f"  整体审美: {pd['overall_aesthetic']:.2f}")
    print(f"  编织合格: {'是' if pd['weaving_ok'] else '否'}")


def cmd_register(sys: ColorSystemRef, name: str, type_name: str, level_name: str):
    """注册符号"""
    type_map = {
        "base": TYPE_BASE, "tone": TYPE_TONE,
        "compound": TYPE_COMPOUND, "symbol": TYPE_SYMBOL,
        "extra": TYPE_EXTRA, "shengmu": TYPE_SHENG_MU,
        "modifier": TYPE_MODIFIER,
    }
    level_map = {"L1": LEVEL_L1, "L2": LEVEL_L2, "L3": LEVEL_L3}
    
    t = type_map.get(type_name.lower())
    l = level_map.get(level_name.upper())
    
    if t is None:
        print(f"未知类型: {type_name}")
        print(f"可用: {', '.join(type_map.keys())}")
        return
    if l is None:
        print(f"未知层级: {level_name}")
        print(f"可用: {', '.join(level_map.keys())}")
        return
    
    ok = sys.register(name, t, l)
    print(f"注册 {'成功' if ok else '失败'}: {name} = {type_name}/{level_name}")


def cmd_demo(sys: ColorSystemRef):
    """运行演示"""
    print("=" * 50)
    print("  满全法 · 颜色进制 — 演示")
    print("=" * 50)
    
    # 注册声母
    count = sys.register_shengmu_batch(SHENGMU_48)
    print(f"\n声母注册: {count}/48")
    
    # 审判测试
    print("\n─── 审判测试 ───")
    tests = [
        ("a", "á", "基韵锚 vs 声调 → 跨层级"),
        ("a", "A", "基韵锚 vs 大写 → 通过"),
        ("a", "o", "基韵锚 vs 基韵锚 → 跨基调"),
        ("b", "p", "声母 vs 声母 → 通过"),
        ("d", "t", "声母 vs 声母 → 通过"),
        ("zh", "ch", "翘舌音 vs 翘舌音 → 通过"),
        ("g", "k", "舌根音 vs 舌根音 → 通过"),
        ("m", "f", "唇音 vs 唇音 → 通过"),
    ]
    for a, b, desc in tests:
        v = sys.trial(a, b)
        label = sys.verdict_str(v)
        print(f"  {a:4} vs {b:4} → {label:20} [{desc}]")
    
    # 衍射
    print("\n─── 衍射分析 ───")
    for name in ["a", "á", "b", "zh"]:
        d = sys.diffract(name)
        print(f"  {name:4}: 类型={d['type_str']:6} 出力={d['pressure_given']:.2f} 失衡={d['equilibrium_delta']:.2f}")
    
    # 配对衍射
    print("\n─── 配对衍射 ───")
    for a, b in [("a", "A"), ("b", "p"), ("a", "á")]:
        pd = sys.pair_diffract(a, b)
        print(f"  {a} ↔ {b}: 类型匹配={'是' if pd['type_match'] else '否'} 审美={pd['overall_aesthetic']:.2f}")
    
    print("\n" + "=" * 50)
    print("  演示完成")
    print("=" * 50)


def create_system() -> ColorSystemRef:
    """创建并初始化完整系统 (声母+韵母+声调)"""
    s = ColorSystemRef()
    s.register_shengmu_batch(SHENGMU_48)
    return s


def cmd_addr(ga: GongyueshuRef, args: list):
    """公约数地址编码: mhz addr <声母> <韵母> <声调>"""
    if len(args) < 3:
        print("用法: mhz addr <声母> <韵母> <声调>")
        print("示例: mhz addr b a 1    → bā 的公约数地址")
        print("      mhz addr sh uang 1 → shuāng 的缩写地址")
        print("      mhz addr \"\" i 4      → ì")
        return
    shengmu = args[0] if args[0] else ""
    yunmu = args[1]
    tone = int(args[2])
    
    ok = ga.encode(shengmu, yunmu, tone)
    if ok:
        idx = ga.count - 1
        val, bits = ga.get_packed(idx)
        full_chars = len(shengmu) + len(yunmu)
        full_bits = full_chars * 8 + 2
        saving = (100 - bits * 100 // full_bits) if full_bits > 0 else 0
        print(f"编码: {shengmu}+{yunmu}+声{tone}")
        print(f"  全长: {full_chars}字符 = {full_bits}bit")
        print(f"  公约数地址: {bits}bit (0x{val:04X})")
        print(f"  压缩: 省{saving}%")
    else:
        print(f"编码失败: {shengmu}+{yunmu}")


def cmd_dim(ds: DimensionRef, args: list):
    """多维几何: mhz dim <声母索引> <韵母索引> <声调>"""
    if len(args) < 3:
        print("用法: mhz dim <声母索引0-47> <韵母索引0-40> <声调1-4>")
        print("示例: mhz dim 0 0 1    → (b, a, 一声) 的线性地址")
        print("      mhz dim 14 30 2  → (zh, ang, 二声)")
        return
    sm = int(args[0])
    ym = int(args[1])
    tone = int(args[2])
    
    addr = ds.linear_addr(sm, ym, tone)
    shengmu_names = ["b","p","m","f","d","t","n","l","g","k","h","j","q","x","zh","ch",
                     "sh","r","z","c","s","y","w","y"]
    yunmu_names = ["a","o","e","i","u","ü","ai","ei","ao","ou","ia","ie","io","iu",
                   "ua","uo","ue","üe","iao","iou","uai","uei","an","en","in","un",
                   "ün","ian","uan","üan","uen","ang","eng","ing","ong","iang",
                   "iong","uang","ueng"]
    
    sm_name = shengmu_names[sm] if sm < len(shengmu_names) else f"?{sm}"
    ym_name = yunmu_names[ym] if ym < len(yunmu_names) else f"?{ym}"
    
    print(f"坐标: ({sm_name}, {ym_name}, 声{tone})")
    print(f"  → 线性地址: {addr}")
    print(f"  → 这是从0维起点到3维空间的一个点")
    print(f"  → 遍历路径: 声母{sm_name} → 遍历韵母{ym_name} → 叠加声调{tone}")


def cmd_negative(ns: NegativeRef, args: list):
    """负样本: mhz neg <符号A> <符号B> [reason]"""
    if len(args) < 2:
        print("用法: mhz neg <符号A hex> <符号B hex> [拒绝原因]")
        print("示例: mhz neg 0x61 0xE1    → 裁定 a vs á")
        print("      mhz neg 0x61 0xE1 1  → 回写 a vs á 到记忆库(原因=跨层级)")
        return
    sym_a = int(args[0], 0)
    sym_b = int(args[1], 0)
    
    if len(args) >= 3:
        # 直接回写
        reason = int(args[2])
        ns.writeback(sym_a, sym_b, reason)
        reason_names = ["无","跨层级","跨类型","跨锚点","审美失败",
                        "编织失败","黑名单","重复","降解"]
        print(f"回写记忆库: ({hex(sym_a)},{hex(sym_b)}) 原因={reason_names[reason]}")
    else:
        # 裁定
        v, reason = ns.judge(sym_a, sym_b)
        verdict_names = ["通过","已知负样本","新拒绝","申诉"]
        reason_names = ["无","跨层级","跨类型","跨锚点","审美失败",
                        "编织失败","黑名单","重复","降解"]
        print(f"裁定 ({hex(sym_a)} vs {hex(sym_b)}):")
        print(f"  判定: {verdict_names[v]}")
        if v > 0:
            print(f"  原因: {reason_names[reason] if reason < len(reason_names) else '?'}")


def cmd_law(ns: NegativeRef, args: list):
    """判定树: mhz law <vA> <vB>"""
    if len(args) < 2:
        print("用法: mhz law <vA> <vB>")
        print("示例: mhz law 100 100  → vA+vB == 2·v_core ?")
        return
    vA = int(args[0])
    vB = int(args[1])
    result = ns.law_judge(vA, vB)
    print(f"{vA} + {vB} = {vA+vB}")
    print(f"  2·v_core = 200")
    print(f"  → {'通过 (产权合法)' if result else '负样本 (侵权)'}")


def cmd_hanzi(hg: HanziGeomRef, args: list):
    """汉字几何: mhz hanzi <Unicode> <部首1> <部首2> <结构> <笔画数> <笔顺...>"""
    if len(args) < 5:
        print("用法: mhz hanzi <Unicode> <部首1> <部首2> <结构> <笔画数> <笔顺...>")
        print("  结构: 1=左右 2=上下 3=全包围 4=半包围 5=左中右 6=上中下 7=品字 8=独体")
        print("  笔顺: 0=横 1=竖 2=撇 3=捺 4=折")
        print("示例: mhz hanzi 0x660E 72 74 1 8 2 1 2 1 1 1 1 1")
        return
    char_code = int(args[0], 0)
    radical1 = int(args[1])
    radical2 = int(args[2])
    structure = int(args[3])
    stroke_count = int(args[4])
    strokes = [int(x) for x in args[5:5+stroke_count]] if stroke_count > 0 else []
    
    ok = hg.add(char_code, radical1, radical2, structure, stroke_count, strokes)
    if ok:
        idx = len(strokes)  # 简化
        packed = hg.get_packed(0)  # 获取第一个
        struct_names = ["?","左右","上下","全包围","半包围","左中右","上中下","品字","独体"]
        print(f"编码: U+{char_code:04X}")
        print(f"  部首: {radical1} + {radical2}")
        print(f"  结构: {struct_names[structure]}")
        print(f"  笔画: {stroke_count}画")
        print(f"  紧凑编码: {packed:08X}")
    else:
        print("编码失败")


def cmd_sound(sound: SoundRadixRef, args: list):
    """编码音节: mhz sound <声母> <韵母> <声调>"""
    if len(args) < 3:
        print("用法: mhz sound <声母> <韵母> <声调1-4>")
        print("示例: mhz sound b a 1    → bā")
        print("      mhz sound zh ang 3  → ǎng")
        print("      mhz sound \"\" i 4       → ì (零声母)")
        return
    shengmu = args[0] if args[0] else ""
    yunmu = args[1]
    tone = int(args[2])
    
    ok = sound.encode_syllable(shengmu, yunmu, tone)
    dur = sound.duration_ms()
    contour_names = ["平(一声)", "升(二声)", "凹(三声)", "降(四声)"]
    print(f"编码 {'成功' if ok else '失败'}: {shengmu}+{yunmu}+声{tone}")
    print(f"  声调轮廓: {contour_names[tone-1] if 1<=tone<=4 else '?'}")
    print(f"  总时长: {dur}ms")


def cmd_sing(sound: SoundRadixRef, args: list):
    """编码多个音节并输出 WAV: mhz sing <音节1> <音节2> ..."""
    if len(args) < 1:
        print("用法: mhz sing <声母:韵母:声调> ...")
        print("示例: mhz sing b:a:1 m:a:3 zh:ang:2")
        return
    
    for syllable in args:
        parts = syllable.split(":")
        if len(parts) != 3:
            print(f"忽略无效音节: {syllable}")
            continue
        s, y, t = parts[0], parts[1], int(parts[2])
        sound.encode_syllable(s, y, t)
    
    dur = sound.duration_ms()
    print(f"编码 {len(args)} 个音节, 总时长 {dur}ms")
    print("(渲染到 WAV 需要更多内存, 请使用 Python API)")


def cmd_property(pf: PropertyFlowRef, args: list):
    """产权流: mhz property <子命令> [参数]"""
    if not args:
        print("用法: mhz property <register|check|grant|stats>")
        print("  register <symbol> <owner> [public]   — 登记产权")
        print("  check <symbol> <requester> [type]    — 侵权检查")
        print("  grant <symbol> <grantee>             — 授予使用权")
        print("  stats                                — 产权统计")
        return
    
    subcmd = args[0]
    
    if subcmd == "register":
        if len(args) < 3:
            print("用法: mhz property register <symbol> <owner> [public]")
            return
        sym = int(args[1], 0)
        owner = int(args[2], 0)
        is_public = len(args) > 3 and args[3].lower() in ("1", "true", "public")
        ok = pf.register(sym, owner, is_public)
        visibility = "公共" if is_public else "私有"
        print(f"产权登记 {'成功' if ok else '失败'}: 0x{sym:02X} → 容器{owner} ({visibility})")
    
    elif subcmd == "check":
        if len(args) < 3:
            print("用法: mhz property check <symbol> <requester> [use_type]")
            print("  use_type: read/write/export/derive (默认 read)")
            return
        sym = int(args[1], 0)
        requester = int(args[2], 0)
        use_type = USE_READ
        if len(args) > 3:
            type_map = {"read": USE_READ, "write": USE_WRITE, "export": USE_EXPORT, "derive": USE_DERIVE}
            use_type = type_map.get(args[3].lower(), USE_READ)
        verdict = pf.check(sym, requester, use_type)
        print(f"产权判定: 0x{sym:02X} ← 容器{requester} → {pf.verdict_str(verdict)}")
    
    elif subcmd == "grant":
        if len(args) < 3:
            print("用法: mhz property grant <symbol> <grantee>")
            return
        sym = int(args[1], 0)
        grantee = int(args[2], 0)
        pf.grant(sym, grantee)
        print(f"使用权授予: 0x{sym:02X} → 容器{grantee} (已标记为公共)")
    
    elif subcmd == "stats":
        import io
        buf = io.StringIO()
        # 直接调用 C 端的 stats (通过 FILE* 比较复杂, 用 Python 端替代)
        print("─── 产权流 ───")
        print(f"产权记录: 已登记")
        print(f"判定结果: ALLOW/DENY/CONDITIONAL")
        print(f"三级裁决: 登记检查→容器权限→历史追溯")
    
    else:
        print(f"未知子命令: {subcmd}")


def cmd_swarm(args: list):
    """njsg-swarm: mhz swarm <子命令>"""
    import sys
    from pathlib import Path
    if str(Path(__file__).parent) not in sys.path:
        sys.path.insert(0, str(Path(__file__).parent))
    from njsg_swarm import SwarmCoordinator, AGENT_IDENTITIES, AgentRole
    
    if not args:
        print("用法: mhz swarm <status|plan|scan|agents>")
        print("  status   — swarm 状态")
        print("  plan     — 生成开发计划")
        print("  scan     — 扫描系统缺口")
        print("  agents   — 列出代理身份")
        return
    
    subcmd = args[0]
    coord = SwarmCoordinator()
    
    if subcmd == "status":
        status = coord.status()
        print("═══ njsg-swarm 状态 ═══")
        print(f"头文件: {status['system']['headers']}")
        print(f"源文件: {status['system']['sources']}")
        print(f"符号链覆盖: {status['system']['chain_coverage']}")
        print(f"测试文件: {status['system']['tests']}")
        print(f"待办任务: {status['pending_tasks']}")
    elif subcmd == "plan":
        tasks = coord.generate_plan()
        print("═══ 开发计划 ═══")
        print(f"共 {len(tasks)} 个任务")
        for t in tasks:
            prio = "!" * (4 - t.priority.value)
            print(f"  [{t.role.value:8s}] {prio} {t.title}")
    elif subcmd == "scan":
        gaps = coord.growth.analyze_gaps()
        print("═══ 系统缺口 ═══")
        for g in gaps:
            print(f"  [{g['type']:15s}] {g['target']:30s} {g['description']}")
    elif subcmd == "agents":
        print("═══ 代理身份 (名称锁定) ═══")
        for role, identity in AGENT_IDENTITIES.items():
            caps = ', '.join(c.value for c in identity['capabilities'])
            print(f"  {identity['name']:10s} 极性={identity['polarity']:2d} 能力=[{caps}]")
    else:
        print(f"未知子命令: {subcmd}")


def cmd_wk(wk: WasmKernelRef, args: list):
    """WASM 内核: mhz wk <子命令> [参数]"""
    if not args:
        print("用法: mhz wk <init|add|find|state|lock|flow|route|stats|save|load>")
        print("  init                              — 初始化内核")
        print("  add <mccp> <sov> <pol> <wit>      — 添加 cell")
        print("  find <mccp>                       — 通过 MCCP 查找 cell")
        print("  state <nn|njsg|jj|jn>             — 设置囡囝四态")
        print("  lock <0|1>                        — 锁定/解锁")
        print("  flow <0-7>                        — 激活一流")
        print("  route <mccp> <sov> <pol> <wit> <flow> — 路由判决")
        print("  stats                             — 内核统计")
        print("  save <file>                       — 保存状态到文件")
        print("  load <file>                       — 从文件加载状态")
        return
    
    subcmd = args[0]
    
    if subcmd == "init":
        wk = WasmKernelRef()
        print("WASM 内核初始化完成 (WKNL v0.3)")
    
    elif subcmd == "add":
        if len(args) < 5:
            print("用法: mhz wk add <mccp> <sovereignty> <polarity> <witness>")
            return
        mccp = int(args[1], 0)
        sov = int(args[2], 0)
        pol = int(args[3], 0)
        wit = int(args[4], 0)
        ok = wk.add_cell(mccp, sov, pol, wit)
        print(f"添加 cell 0x{mccp:04X}: {'成功' if ok else '失败'}")
    
    elif subcmd == "find":
        if len(args) < 2:
            print("用法: mhz wk find <mccp>")
            return
        mccp = int(args[1], 0)
        cell = wk.find_mccp(mccp)
        if cell:
            print(f"找到: {WasmKernelRef.cell_str(cell)}")
        else:
            print(f"未找到 MCCP=0x{mccp:04X}")
    
    elif subcmd == "state":
        if len(args) < 2:
            print("用法: mhz wk state <nn|njsg|jj|jn>")
            return
        state_map = {"nn": WK_STATE_NN, "njsg": WK_STATE_NJSG, "jj": WK_STATE_JJ, "jn": WK_STATE_JN}
        s = state_map.get(args[1].lower())
        if s is None:
            print(f"无效状态: {args[1]}")
            return
        ok = wk.set_state(s)
        print(f"状态切换: {'成功' if ok else '失败'} (state={wk.state})")
    
    elif subcmd == "lock":
        if len(args) < 2:
            print("用法: mhz wk lock <0|1>")
            return
        wk.locked = args[1] == "1"
        print(f"互锁: {'锁定' if wk.locked else '解锁'}")
    
    elif subcmd == "flow":
        if len(args) < 2:
            print("用法: mhz wk flow <0-7> (0=PRELOAD 1=CATEGORY ... 7=CONCEPT)")
            return
        flow = int(args[1], 0)
        if flow < 0 or flow > 7:
            print(f"无效流: {flow}")
            return
        wk.activate_flow(flow)
        print(f"流 {flow} 已激活")
    
    elif subcmd == "route":
        if len(args) < 6:
            print("用法: mhz wk route <mccp> <sov> <pol> <wit> <flow>")
            return
        mccp = int(args[1], 0)
        sov = int(args[2], 0)
        pol = int(args[3], 0)
        wit = int(args[4], 0)
        flow = int(args[5], 0)
        rc = wk.route(mccp, sov, pol, wit, flow)
        print(f"路由判决: {wk.route_str(rc)}")
    
    elif subcmd == "stats":
        print(wk.stats())
    
    elif subcmd == "save":
        if len(args) < 2:
            print("用法: mhz wk save <文件名>")
            return
        ok = wk.save(args[1])
        print(f"保存: {'成功' if ok else '失败'} → {args[1]}")
    
    elif subcmd == "load":
        if len(args) < 2:
            print("用法: mhz wk load <文件名>")
            return
        ok = wk.load(args[1])
        print(f"加载: {'成功' if ok else '失败'} ← {args[1]}")
    
    else:
        print(f"未知子命令: {subcmd}")


def cmd_bootstrap(args: list):
    """自举编译器: mhz bootstrap <子命令> [参数]"""
    # 导入自举编译器
    import sys
    from pathlib import Path
    # 确保 tools/phase-radix 在 path 中
    if str(Path(__file__).parent) not in sys.path:
        sys.path.insert(0, str(Path(__file__).parent))
    from bootstrap_compiler import BootstrapEngine, DriftDetector
    
    if not args:
        print("用法: mhz bootstrap <stats|registry|check|manifest>")
        print("  stats     — 符号统计")
        print("  registry  — 生成符号注册表 (JSON)")
        print("  check     — 漂移检测 (C 头文件 vs Python 绑定)")
        print("  manifest  — 生成系统清单 (自举快照)")
        return
    
    subcmd = args[0]
    
    if subcmd == "stats":
        engine = BootstrapEngine()
        engine.load_all()
        stats = engine.stats()
        print("═══ 符号统计 ═══")
        for k, v in stats.items():
            print(f"  {k}: {v}")
        print()
        for name, parser in sorted(engine.parsers.items()):
            nf = len(parser.functions())
            nc = len(parser.constants())
            if nf + nc > 0:
                print(f"  {name}: {nf} functions, {nc} constants")
    
    elif subcmd == "registry":
        engine = BootstrapEngine()
        engine.load_all()
        print(engine.generate_registry())
    
    elif subcmd == "check":
        d = DriftDetector()
        result = d.check()
        print("═══ 漂移检测 ═══")
        print(f"C 函数总数: {len(result['c_functions'])}")
        print(f"Python 绑定函数: {len(result['py_functions'])}")
        if result['missing_in_python']:
            print(f"  ✗ 缺失绑定 ({len(result['missing_in_python'])}):")
            for f in result['missing_in_python'][:20]:
                print(f"    - {f}")
            if len(result['missing_in_python']) > 20:
                print(f"    ... 共 {len(result['missing_in_python'])} 个")
        if not result['drift_detected']:
            print("  ✓ 无漂移")
    
    elif subcmd == "manifest":
        engine = BootstrapEngine()
        engine.load_all()
        import hashlib
        import time
        manifest = {
            'version': '1.0',
            'generated': time.strftime('%Y-%m-%dT%H:%M:%S'),
            'headers': {},
            'stats': engine.stats(),
        }
        for name, parser in engine.parsers.items():
            # 计算头文件哈希
            h = hashlib.sha256(parser.text.encode('utf-8', errors='replace')).hexdigest()[:16]
            manifest['headers'][name] = {
                'hash': h,
                'functions': len(parser.functions()),
                'constants': len(parser.constants()),
            }
        import json
        print(json.dumps(manifest, indent=2, ensure_ascii=False))
    
    else:
        print(f"未知子命令: {subcmd}")


def cmd_interlock(il: InterlockRef, args: list):
    """全局互锁横轴: mhz interlock <子命令> [参数]"""
    if not args:
        print("用法: mhz interlock <init|register|check|activate|stats>")
        print("  init                          — 初始化横轴")
        print("  register container <id> <名>  — 注册容器齿轮")
        print("  register symbol <sym> <cont>  — 登记符号圆珠")
        print("  check <symbol>                — 检查符号是否已登记")
        print("  activate                      — 激活互锁")
        print("  stats                         — 横轴状态")
        return
    
    subcmd = args[0]
    
    if subcmd == "init":
        il = InterlockRef()
        print("横轴初始化完成 (FQM v0003)")
    
    elif subcmd == "register":
        if len(args) < 4:
            print("用法: mhz interlock register <container|symbol> <id> <name>")
            return
        if args[1] == "container":
            cid = int(args[2], 0)
            name = args[3]
            ok = il.register_container(cid, name)
            print(f"容器齿轮 {'已套上' if ok else '失败'}: {name} (0x{cid:02X})")
        elif args[1] == "symbol":
            sym = int(args[2], 0)
            cont = int(args[3], 0)
            ok = il.register_symbol(sym, cont)
            result_str = "已入圈" if ok else "失败"
            print(f"符号圆珠 {result_str}: 0x{sym:02X} → {CONT_NAMES.get(cont, '?')}")
    
    elif subcmd == "check":
        if len(args) < 2:
            print("用法: mhz interlock check <symbol>")
            return
        sym = int(args[1], 0)
        ok = il.check_symbol(sym)
        print(f"符号 0x{sym:02X}: {'已登记' if ok else '未登记'}")
    
    elif subcmd == "activate":
        il.activate()
        print("横轴互锁已激活 · 所有齿轮必须遵守")
    
    elif subcmd == "stats":
        s = il.stats()
        print("─── 全局互锁横轴 ───")
        print(f"状态: {'已激活' if s['active'] else '未激活'}")
        print(f"容器: {', '.join(CONT_NAMES.values())}")
    
    else:
        print(f"未知子命令: {subcmd}")


def cmd_status(sys: ColorSystemRef):
    """系统状态"""
    print("─── 满全法系统状态 ───")
    print(f"C 核心: libcolor.so")
    print(f"类型系统: 7类 × 3层")
    print(f"审判五关: 层级→类型→锚→审美→编织")
    print(f"衍射维度: 6维")
    print(f"集装箱: 1bit/16字 × 254框")
    print(f"声音进制: 事件编码 (频率+音色+时长+轮廓+强度)")
    print(f"全局互锁: 横轴 (6个容器齿轮已套上)")


def main():
    if len(_sys.argv) < 2:
        print(__doc__)
        return
    
    command = _sys.argv[1]
    args = _sys.argv[2:]
    
    # 需要系统实例的命令
    if command == "trial":
        if len(args) < 2:
            print("用法: mhz trial <a> <b>")
            return
        sys = create_system()
        cmd_trial(sys, args[0], args[1])
    
    elif command == "diffract":
        if len(args) < 1:
            print("用法: mhz diffract <name>")
            return
        sys = create_system()
        cmd_diffract(sys, args[0])
    
    elif command == "pair":
        if len(args) < 2:
            print("用法: mhz pair <a> <b>")
            return
        sys = create_system()
        cmd_pair(sys, args[0], args[1])
    
    elif command == "register":
        if len(args) < 3:
            print("用法: mhz register <name> <type> <level>")
            print("  type: base/tone/compound/symbol/extra/shengmu/modifier")
            print("  level: L1/L2/L3")
            return
        sys = create_system()
        cmd_register(sys, args[0], args[1], args[2])
    
    elif command == "demo":
        sys = create_system()
        cmd_demo(sys)
    
    elif command == "status":
        sys = create_system()
        cmd_status(sys)
    
    elif command == "addr":
        ga = GongyueshuRef()
        cmd_addr(ga, args)
    
    elif command == "dim":
        ds = DimensionRef()
        cmd_dim(ds, args)
    
    elif command == "hanzi":
        hg = HanziGeomRef()
        cmd_hanzi(hg, args)
    
    elif command == "neg":
        ns = NegativeRef()
        cmd_negative(ns, args)
    
    elif command == "law":
        ns = NegativeRef()
        cmd_law(ns, args)
    
    elif command == "sound":
        sound = SoundRadixRef()
        cmd_sound(sound, args)
    
    elif command == "sing":
        sound = SoundRadixRef()
        cmd_sing(sound, args)
    
    elif command == "interlock":
        il = InterlockRef()
        cmd_interlock(il, args)
    
    elif command == "property":
        pf = PropertyFlowRef()
        cmd_property(pf, args)
    
    elif command == "wk":
        wk = WasmKernelRef()
        # 自动从默认状态文件加载
        _wk_autoload(wk)
        cmd_wk(wk, args)
        # 自动保存到默认状态文件
        _wk_autosave(wk)
    
    elif command == "bootstrap":
        cmd_bootstrap(args)
    
    elif command == "swarm":
        cmd_swarm(args)
    
    elif command == "cocoon":
        cmd_cocoon(args)
    
    elif command == "docs":
        cmd_docs(args)
    
    elif command == "verify":
        import subprocess
        # __file__ = .../tools/phase-radix/mhz.py
        # target:   .../tools/phase-radix/verify_chain.py (同目录)
        verify_script = os.path.join(os.path.dirname(__file__), "verify_chain.py")
        verify_script = os.path.normpath(verify_script)
        if not os.path.exists(verify_script):
            print(f"验证脚本不存在: {verify_script}")
        else:
            result = subprocess.run([_sys.executable, verify_script] + args)
            return result.returncode
    
    elif command == "history":
        import subprocess
        history_script = os.path.join(os.path.dirname(__file__), "register_history.py")
        history_script = os.path.normpath(history_script)
        if not os.path.exists(history_script):
            print(f"历史登记脚本不存在: {history_script}")
        else:
            result = subprocess.run([_sys.executable, history_script] + args)
            return result.returncode
    
    else:
        print(f"未知命令: {command}")
        print(__doc__)


# ── 文档聚合浏览器 ────────────────────────────────────────

def cmd_docs(args: list):
    """统计文档聚合浏览器: mhz docs <子命令>"""
    import sys
    from pathlib import Path
    if str(Path(__file__).parent) not in sys.path:
        sys.path.insert(0, str(Path(__file__).parent))
    from doc_aggregator import DocumentScanner, SituationalClusterer, WrappingTree, WORKSPACE_DIR
    
    if not args:
        print("用法: mhz docs <scan|clusters|tree>")
        print("  scan      — 扫描文档")
        print("  clusters  — 态势聚类")
        print("  tree      — 连接包裹树")
        return
    
    subcmd = args[0]
    scanner = DocumentScanner(WORKSPACE_DIR)
    
    if subcmd == "scan":
        print("═══ 扫描文档 ═══")
        docs = scanner.scan()
        print(f"文档总数: {len(docs)}")
        print(f"独立关键词: {len(scanner.inverted_index)}")
    elif subcmd == "clusters":
        docs = scanner.scan()
        clusterer = SituationalClusterer(docs, scanner.inverted_index)
        clusters = clusterer.cluster()
        print("═══ 态势聚类 ═══")
        for i, c in enumerate(clusters):
            kws = clusterer.cluster_keywords(c)
            kw_str = ', '.join(f'{kw}({cnt})' for kw, cnt in kws[:5])
            print(f"  簇 {i}: {len(c)} docs | {kw_str}")
    elif subcmd == "tree":
        docs = scanner.scan()
        clusterer = SituationalClusterer(docs, scanner.inverted_index)
        clusterer.cluster()
        tree = WrappingTree(clusterer)
        tree.build()
        print("═══ 连接包裹树 ═══")
        print(tree.render_text())
    else:
        print(f"未知子命令: {subcmd}")


# ── 蚕蛹反向寄生壳 ────────────────────────────────────────

def cmd_cocoon(args: list):
    """蚕蛹反向寄生壳: mhz cocoon <子命令>"""
    import sys
    from pathlib import Path
    if str(Path(__file__).parent) not in sys.path:
        sys.path.insert(0, str(Path(__file__).parent))
    from cocoon_shell import ShellCatalog, ShellMatcher, ShedWearEngine, ConfigGenerator, KernelMounter, ShellType
    
    if not args:
        print("用法: mhz cocoon <catalog|match|wear|demo>")
        print("  catalog   — 列出可用寄生壳")
        print("  match <type> — 匹配最佳寄生壳")
        print("  wear <shell_id> — 执行穿衣演示")
        print("  demo      — 完整流程演示")
        return
    
    subcmd = args[0]
    catalog = ShellCatalog()
    
    if subcmd == "catalog":
        print("═══ 寄生壳目录 ═══")
        for shell in catalog.list_shells():
            caps = ', '.join(c.value for c in shell.capabilities)
            print(f"  [{shell.id:12s}] {shell.name:20s} 能力=[{caps}]")
    elif subcmd == "match":
        if len(args) < 2:
            print("用法: mhz cocoon match <type>")
            return
        type_map = {t.value: t for t in ShellType}
        st = type_map.get(args[1])
        if not st:
            print(f"未知类型: {args[1]}")
            return
        matcher = ShellMatcher(catalog)
        shell = matcher.match(st)
        if shell:
            print(f"匹配: {shell.name} ({shell.id})")
        else:
            print("未找到匹配的壳")
    elif subcmd == "demo":
        print("═══ 蚕蛹反向寄生壳 · 完整流程 ═══")
        matcher = ShellMatcher(catalog)
        shell = matcher.match(ShellType.HTTP_API)
        engine = ShedWearEngine(shell)
        external = {"GET": "/api/users", "Content-Type": "application/json"}
        internal = engine.shed(external)
        gen = ConfigGenerator(shell)
        config = gen.generate(mount_point="/cocoon/api")
        mounter = KernelMounter(config)
        ok = mounter.mount()
        print(f"壳: {shell.name}")
        print(f"转换: {external} → {internal}")
        print(f"挂载: {'成功' if ok else '失败'} @ {config['mount_point']}")
    else:
        print(f"未知子命令: {subcmd}")


# ── WASM 内核持久化辅助 ──────────────────────────────────

def _wk_default_path():
    """获取默认状态文件路径 (当前工作目录)"""
    import os
    return os.path.join(os.getcwd(), "wk_state.bin")

def _wk_autoload(wk: WasmKernelRef):
    """自动加载默认状态文件 (不存在则跳过)"""
    import os
    path = _wk_default_path()
    if os.path.exists(path):
        wk.load(path)

def _wk_autosave(wk: WasmKernelRef):
    """自动保存到默认状态文件"""
    import os
    path = _wk_default_path()
    wk.save(path)


if __name__ == "__main__":
    main()
