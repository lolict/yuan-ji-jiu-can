// ══════════════════════════════════════════════════════════════════════════════
// metaime — 元输入法核心 · 编译期内嵌字典 · 零运行时解析 · C-ABI 接口
// ══════════════════════════════════════════════════════════════════════════════
//
// 冰箱模型实现: 字典在编译时被"搅拌"进二进制
// 没有文件读取, 没有运行时分配, 只有预加载的静态查找表
//
// C-ABI 接口通过 JNI 被 Android Kotlin 壳调用:
//   metaime_init()             → 状态指针
//   metaime_query(state, shengmu, yunmu, tone) → *const c_char (候选词)
//   metaime_free(s: *mut c_char)
//
// ══════════════════════════════════════════════════════════════════════════════

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ──────────────────────────────────────────────────────────────────────────────
// 声母定义 (24 个, 对应传统 23 + 零声母)
// ──────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shengmu {
    None = 0,    // 零声母
    B = 1,  P = 2,  M = 3,  F = 4,
    D = 5,  T = 6,  N = 7,  L = 8,
    G = 9,  K = 10, H = 11,
    J = 12, Q = 13, X = 14,
    Zhi = 15, Chi = 16, Shi = 17, Ri = 18,
    Zi = 19, Ci = 20, Si = 21,
    Y = 22, W = 23,
}

impl Shengmu {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Shengmu::None),
            1 => Some(Shengmu::B),   2 => Some(Shengmu::P),
            3 => Some(Shengmu::M),   4 => Some(Shengmu::F),
            5 => Some(Shengmu::D),   6 => Some(Shengmu::T),
            7 => Some(Shengmu::N),   8 => Some(Shengmu::L),
            9 => Some(Shengmu::G),   10 => Some(Shengmu::K),
            11 => Some(Shengmu::H),
            12 => Some(Shengmu::J),  13 => Some(Shengmu::Q),
            14 => Some(Shengmu::X),
            15 => Some(Shengmu::Zhi), 16 => Some(Shengmu::Chi),
            17 => Some(Shengmu::Shi), 18 => Some(Shengmu::Ri),
            19 => Some(Shengmu::Zi),  20 => Some(Shengmu::Ci),
            21 => Some(Shengmu::Si),
            22 => Some(Shengmu::Y),   23 => Some(Shengmu::W),
            _ => None,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 韵母定义 (简化版, 覆盖普通话核心韵母)
// ──────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Yunmu {
    None = 0,
    A = 1, O = 2, E = 3, I = 4, U = 5, Vv = 6,
    Ai = 7, Ei = 8, Ao = 9, Ou = 10,
    An = 11, En = 12, Ang = 13, Eng = 14, Ong = 15,
    Ia = 16, Ie = 17, Iao = 18, Iu = 19,
    Ian = 20, In = 21, Iang = 22, Ing = 23, Ion = 24,
    Ua = 25, Uo = 26, Uai = 27, Ui = 28,
    Uan = 29, Un = 30, Ueng = 31,
    Vve = 32, Van = 33, Vn = 34,
}

impl Yunmu {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Yunmu::None), 1 => Some(Yunmu::A), 2 => Some(Yunmu::O),
            3 => Some(Yunmu::E), 4 => Some(Yunmu::I), 5 => Some(Yunmu::U),
            6 => Some(Yunmu::Vv), 7 => Some(Yunmu::Ai), 8 => Some(Yunmu::Ei),
            9 => Some(Yunmu::Ao), 10 => Some(Yunmu::Ou),
            11 => Some(Yunmu::An), 12 => Some(Yunmu::En), 13 => Some(Yunmu::Ang),
            14 => Some(Yunmu::Eng), 15 => Some(Yunmu::Ong),
            16 => Some(Yunmu::Ia), 17 => Some(Yunmu::Ie), 18 => Some(Yunmu::Iao),
            19 => Some(Yunmu::Iu), 20 => Some(Yunmu::Ian), 21 => Some(Yunmu::In),
            22 => Some(Yunmu::Iang), 23 => Some(Yunmu::Ing), 24 => Some(Yunmu::Ion),
            25 => Some(Yunmu::Ua), 26 => Some(Yunmu::Uo), 27 => Some(Yunmu::Uai),
            28 => Some(Yunmu::Ui), 29 => Some(Yunmu::Uan), 30 => Some(Yunmu::Un),
            31 => Some(Yunmu::Ueng), 32 => Some(Yunmu::Vve), 33 => Some(Yunmu::Van),
            34 => Some(Yunmu::Vn), _ => None,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 声调 (0 = 轻声, 1-4 = 四声)
// ──────────────────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Neutral = 0,
    Level = 1,    // 阴平 ˉ
    Rising = 2,   // 阳平 ˊ
    Low = 3,      // 上声 ˇ
    Falling = 4,  // 去声 ˋ
}

impl Tone {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Tone::Neutral), 1 => Some(Tone::Level),
            2 => Some(Tone::Rising),  3 => Some(Tone::Low),
            4 => Some(Tone::Falling), _ => None,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 拼音键 (声母 + 韵母 + 声调)
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PinyinKey {
    shengmu: u8,
    yunmu: u8,
    tone: u8,
}

// ──────────────────────────────────────────────────────────────────────────────
// 编译期内嵌字典
// 冰箱里的菜, 已经搅拌好, 直接吃
// ──────────────────────────────────────────────────────────────────────────────

struct DictEntry {
    key: PinyinKey,
    candidates: &'static str,
}

// 字典: 按拼音排序的静态数组 (未来增长时追加)
static DICT: &[DictEntry] = &[
    // —— b ——
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 1, tone: 1 }, candidates: "八芭" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 1, tone: 3 }, candidates: "把" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 1, tone: 4 }, candidates: "爸吧坝" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 7, tone: 3 }, candidates: "百" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 9, tone: 1 }, candidates: "包" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 11, tone: 1 }, candidates: "班般" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 11, tone: 4 }, candidates: "半办" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 13, tone: 1 }, candidates: "帮" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 13, tone: 4 }, candidates: "棒" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 18, tone: 1 }, candidates: "标表" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 2, tone: 1 }, candidates: "波播" },
    DictEntry { key: PinyinKey { shengmu: 1, yunmu: 2, tone: 4 }, candidates: "博伯" },
    // —— p ——
    DictEntry { key: PinyinKey { shengmu: 2, yunmu: 1, tone: 2 }, candidates: "爬" },
    DictEntry { key: PinyinKey { shengmu: 2, yunmu: 9, tone: 2 }, candidates: "跑" },
    DictEntry { key: PinyinKey { shengmu: 2, yunmu: 3, tone: 2 }, candidates: "陪" },
    // —— m ——
    DictEntry { key: PinyinKey { shengmu: 3, yunmu: 1, tone: 3 }, candidates: "马" },
    DictEntry { key: PinyinKey { shengmu: 3, yunmu: 3, tone: 2 }, candidates: "没美" },
    DictEntry { key: PinyinKey { shengmu: 3, yunmu: 4, tone: 4 }, candidates: "秘密" },
    // —— f ——
    DictEntry { key: PinyinKey { shengmu: 4, yunmu: 1, tone: 1 }, candidates: "发" },
    DictEntry { key: PinyinKey { shengmu: 4, yunmu: 3, tone: 2 }, candidates: "分" },
    DictEntry { key: PinyinKey { shengmu: 4, yunmu: 13, tone: 2 }, candidates: "房" },
    DictEntry { key: PinyinKey { shengmu: 4, yunmu: 14, tone: 4 }, candidates: "丰富" },
    // —— d ——
    DictEntry { key: PinyinKey { shengmu: 5, yunmu: 1, tone: 4 }, candidates: "大包" },
    DictEntry { key: PinyinKey { shengmu: 5, yunmu: 11, tone: 3 }, candidates: "胆" },
    DictEntry { key: PinyinKey { shengmu: 5, yunmu: 14, tone: 4 }, candidates: "定地" },
    DictEntry { key: PinyinKey { shengmu: 5, yunmu: 28, tone: 4 }, candidates: "对队" },
    DictEntry { key: PinyinKey { shengmu: 5, yunmu: 25, tone: 1 }, candidates: "多" },
    // —— t ——
    DictEntry { key: PinyinKey { shengmu: 6, yunmu: 1, tone: 1 }, candidates: "他她它" },
    DictEntry { key: PinyinKey { shengmu: 6, yunmu: 14, tone: 2 }, candidates: "抬头" },
    DictEntry { key: PinyinKey { shengmu: 6, yunmu: 26, tone: 1 }, candidates: "脱拖" },
    DictEntry { key: PinyinKey { shengmu: 6, yunmu: 28, tone: 4 }, candidates: "退" },
    // —— n ——
    DictEntry { key: PinyinKey { shengmu: 7, yunmu: 1, tone: 3 }, candidates: "那哪" },
    DictEntry { key: PinyinKey { shengmu: 7, yunmu: 11, tone: 2 }, candidates: "南" },
    DictEntry { key: PinyinKey { shengmu: 7, yunmu: 4, tone: 3 }, candidates: "你我" },
    DictEntry { key: PinyinKey { shengmu: 7, yunmu: 23, tone: 4 }, candidates: "宁静" },
    // —— l ——
    DictEntry { key: PinyinKey { shengmu: 8, yunmu: 1, tone: 4 }, candidates: "拉啦" },
    DictEntry { key: PinyinKey { shengmu: 8, yunmu: 18, tone: 3 }, candidates: "了" },
    DictEntry { key: PinyinKey { shengmu: 8, yunmu: 4, tone: 4 }, candidates: "里力利立" },
    DictEntry { key: PinyinKey { shengmu: 8, yunmu: 22, tone: 2 }, candidates: "两亮" },
    DictEntry { key: PinyinKey { shengmu: 8, yunmu: 23, tone: 2 }, candidates: "领零灵" },
    // —— g ——
    DictEntry { key: PinyinKey { shengmu: 9, yunmu: 3, tone: 3 }, candidates: "个各革" },
    DictEntry { key: PinyinKey { shengmu: 9, yunmu: 11, tone: 3 }, candidates: "感敢" },
    DictEntry { key: PinyinKey { shengmu: 9, yunmu: 13, tone: 1 }, candidates: "刚钢" },
    DictEntry { key: PinyinKey { shengmu: 9, yunmu: 25, tone: 1 }, candidates: "瓜" },
    DictEntry { key: PinyinKey { shengmu: 9, yunmu: 29, tone: 4 }, candidates: "关管馆" },
    // —— k ——
    DictEntry { key: PinyinKey { shengmu: 10, yunmu: 1, tone: 1 }, candidates: "咖" },
    DictEntry { key: PinyinKey { shengmu: 10, yunmu: 3, tone: 4 }, candidates: "可克刻客" },
    DictEntry { key: PinyinKey { shengmu: 10, yunmu: 5, tone: 1 }, candidates: "哭苦" },
    DictEntry { key: PinyinKey { shengmu: 10, yunmu: 12, tone: 3 }, candidates: "肯" },
    // —— h ——
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 1, tone: 1 }, candidates: "哈" },
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 3, tone: 3 }, candidates: "喝" },
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 9, tone: 3 }, candidates: "好号" },
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 11, tone: 1 }, candidates: "酣" },
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 25, tone: 1 }, candidates: "花" },
    DictEntry { key: PinyinKey { shengmu: 11, yunmu: 29, tone: 4 }, candidates: "换还" },
    // —— j ——
    DictEntry { key: PinyinKey { shengmu: 12, yunmu: 4, tone: 1 }, candidates: "机基几激" },
    DictEntry { key: PinyinKey { shengmu: 12, yunmu: 11, tone: 1 }, candidates: "间坚兼" },
    DictEntry { key: PinyinKey { shengmu: 12, yunmu: 17, tone: 4 }, candidates: "界价架驾" },
    DictEntry { key: PinyinKey { shengmu: 12, yunmu: 23, tone: 1 }, candidates: "京经精" },
    // —— q ——
    DictEntry { key: PinyinKey { shengmu: 13, yunmu: 3, tone: 1 }, candidates: "七期妻" },
    DictEntry { key: PinyinKey { shengmu: 13, yunmu: 17, tone: 3 }, candidates: "且" },
    DictEntry { key: PinyinKey { shengmu: 13, yunmu: 20, tone: 1 }, candidates: "千迁牵签" },
    DictEntry { key: PinyinKey { shengmu: 13, yunmu: 23, tone: 2 }, candidates: "情晴" },
    // —— x ——
    DictEntry { key: PinyinKey { shengmu: 14, yunmu: 1, tone: 1 }, candidates: "西席希吸" },
    DictEntry { key: PinyinKey { shengmu: 14, yunmu: 11, tone: 1 }, candidates: "先仙掀" },
    DictEntry { key: PinyinKey { shengmu: 14, yunmu: 17, tone: 4 }, candidates: "写下谢" },
    DictEntry { key: PinyinKey { shengmu: 14, yunmu: 23, tone: 1 }, candidates: "星形型" },
    // —— zh ——
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 1, tone: 1 }, candidates: "扎渣" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 3, tone: 1 }, candidates: "者这" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 11, tone: 1 }, candidates: "占沾盏" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 13, tone: 3 }, candidates: "长涨掌" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 13, tone: 4 }, candidates: "丈仗帐障" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 18, tone: 4 }, candidates: "注著主住" },
    DictEntry { key: PinyinKey { shengmu: 15, yunmu: 29, tone: 4 }, candidates: "转装" },
    // —— ch ——
    DictEntry { key: PinyinKey { shengmu: 16, yunmu: 1, tone: 1 }, candidates: "差插" },
    DictEntry { key: PinyinKey { shengmu: 16, yunmu: 3, tone: 2 }, candidates: "车彻" },
    DictEntry { key: PinyinKey { shengmu: 16, yunmu: 11, tone: 2 }, candidates: "缠" },
    DictEntry { key: PinyinKey { shengmu: 16, yunmu: 13, tone: 2 }, candidates: "场厂昌长" },
    DictEntry { key: PinyinKey { shengmu: 16, yunmu: 29, tone: 4 }, candidates: "传出串" },
    // —— sh ——
    DictEntry { key: PinyinKey { shengmu: 17, yunmu: 1, tone: 1 }, candidates: "杀沙纱" },
    DictEntry { key: PinyinKey { shengmu: 17, yunmu: 4, tone: 4 }, candidates: "是市事实视世" },
    DictEntry { key: PinyinKey { shengmu: 17, yunmu: 9, tone: 3 }, candidates: "少" },
    DictEntry { key: PinyinKey { shengmu: 17, yunmu: 12, tone: 3 }, candidates: "深" },
    DictEntry { key: PinyinKey { shengmu: 17, yunmu: 29, tone: 4 }, candidates: "拴双" },
    // —— r ——
    DictEntry { key: PinyinKey { shengmu: 18, yunmu: 1, tone: 3 }, candidates: "然燃" },
    DictEntry { key: PinyinKey { shengmu: 18, yunmu: 4, tone: 4 }, candidates: "日入" },
    DictEntry { key: PinyinKey { shengmu: 18, yunmu: 29, tone: 3 }, candidates: "软" },
    // —— z ——
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 3, tone: 4 }, candidates: "责则" },
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 4, tone: 1 }, candidates: "姿资滋知支枝之织" },
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 4, tone: 3 }, candidates: "只指纸止址子紫" },
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 4, tone: 4 }, candidates: "字自至制治志致质置智" },
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 11, tone: 1 }, candidates: "簪攒" },
    DictEntry { key: PinyinKey { shengmu: 19, yunmu: 13, tone: 3 }, candidates: "脏葬" },
    // —— c ——
    DictEntry { key: PinyinKey { shengmu: 20, yunmu: 3, tone: 4 }, candidates: "策册测侧" },
    DictEntry { key: PinyinKey { shengmu: 20, yunmu: 4, tone: 2 }, candidates: "词辞此次刺" },
    DictEntry { key: PinyinKey { shengmu: 20, yunmu: 4, tone: 4 }, candidates: "赐" },
    DictEntry { key: PinyinKey { shengmu: 20, yunmu: 29, tone: 2 }, candidates: "潺" },
    DictEntry { key: PinyinKey { shengmu: 20, yunmu: 29, tone: 4 }, candidates: "窜篡" },
    // —— s ——
    DictEntry { key: PinyinKey { shengmu: 21, yunmu: 4, tone: 1 }, candidates: "司思丝私斯撕" },
    DictEntry { key: PinyinKey { shengmu: 21, yunmu: 4, tone: 3 }, candidates: "死" },
    DictEntry { key: PinyinKey { shengmu: 21, yunmu: 4, tone: 4 }, candidates: "四似寺祀" },
    DictEntry { key: PinyinKey { shengmu: 21, yunmu: 11, tone: 3 }, candidates: "三散" },
    DictEntry { key: PinyinKey { shengmu: 21, yunmu: 29, tone: 3 }, candidates: "损" },
    // —— y ——
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 1, tone: 1 }, candidates: "压呀鸦丫" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 1, tone: 3 }, candidates: "雅" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 3, tone: 1 }, candidates: "爷也" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 4, tone: 1 }, candidates: "一衣医依伊壹" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 4, tone: 2 }, candidates: "移仪宜姨遗" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 4, tone: 3 }, candidates: "以已椅蚁倚" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 4, tone: 4 }, candidates: "意义易意议艺译" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 11, tone: 1 }, candidates: "烟雨" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 13, tone: 4 }, candidates: "样阳洋养" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 17, tone: 4 }, candidates: "夜叶业页" },
    DictEntry { key: PinyinKey { shengmu: 22, yunmu: 23, tone: 2 }, candidates: "赢迎营" },
    // —— w ——
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 1, tone: 3 }, candidates: "瓦" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 3, tone: 1 }, candidates: "威微危" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 3, tone: 2 }, candidates: "为围" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 3, tone: 3 }, candidates: "伟委尾" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 4, tone: 4 }, candidates: "五武午舞务物" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 11, tone: 3 }, candidates: "晚碗万" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 25, tone: 1 }, candidates: "蛙洼" },
    DictEntry { key: PinyinKey { shengmu: 23, yunmu: 26, tone: 1 }, candidates: "窝" },
    // —— 零声母 ——
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 1, tone: 1 }, candidates: "啊阿" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 2, tone: 4 }, candidates: "恶饿" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 3, tone: 4 }, candidates: "额恩" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 7, tone: 1 }, candidates: "哀挨埃" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 8, tone: 4 }, candidates: "被悲" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 9, tone: 3 }, candidates: "袄" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 10, tone: 4 }, candidates: "怄" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 11, tone: 1 }, candidates: "安氨" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 12, tone: 1 }, candidates: "恩" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 13, tone: 1 }, candidates: "肮脏" },
    DictEntry { key: PinyinKey { shengmu: 0, yunmu: 13, tone: 4 }, candidates: "盎" },
];

// ──────────────────────────────────────────────────────────────────────────────
// 查询函数 (二分查找)
// ──────────────────────────────────────────────────────────────────────────────

fn query_candidates(shengmu: u8, yunmu: u8, tone: u8) -> Option<&'static str> {
    let key = PinyinKey { shengmu, yunmu, tone };
    DICT.binary_search_by(|e| e.key.cmp(&key))
        .ok()
        .map(|idx| DICT[idx].candidates)
}

// ──────────────────────────────────────────────────────────────────────────────
// C-ABI 接口 (给 Android JNI 调用)
// ──────────────────────────────────────────────────────────────────────────────

/// Opaque 状态对象 (未来可扩展, 现在为最小)
pub struct MetaImeState;

impl MetaImeState {
    fn new() -> Self { MetaImeState }
}

/// 创建状态对象
#[no_mangle]
pub extern "C" fn metaime_init() -> *mut MetaImeState {
    Box::into_raw(Box::new(MetaImeState::new()))
}

/// 销毁状态对象
#[no_mangle]
pub extern "C" fn metaime_free(state: *mut MetaImeState) {
    if !state.is_null() {
        unsafe { Box::from_raw(state); }
    }
}

/// 核心查询接口
///
/// # Parameters
/// - state: 由 metaime_init() 返回的状态指针
/// - shengmu: 声母编码 (0-23)
/// - yunmu:   韵母编码 (0-34)
/// - tone:    声调 (0-4)
///
/// # Returns
/// 候选词的 UTF-8 字节序列 (以 \0 结尾), 若无结果返回 null
///
/// # Thread Safety
/// 状态只读, 多线程同时调用安全 (共享引用)
#[no_mangle]
pub extern "C" fn metaime_query_raw(
    _state: *mut MetaImeState,
    shengmu: u8,
    yunmu: u8,
    tone: u8,
) -> *const u8 {
    match query_candidates(shengmu, yunmu, tone) {
        Some(candidates) => {
            // 候选词已是 UTF-8, 直接返回 (需确保结尾 \0)
            static EMPTY: &[u8] = b"\0";
            if candidates.is_empty() {
                return EMPTY.as_ptr();
            }
            // 注意: 候选词不含 \0, 返回的指针指向 DICT 静态数据
            // 必须保证调用方立即复制, 不允许修改
            candidates.as_ptr()
        }
        None => std::ptr::null(),
    }
}

/// 释放 metaime_query_raw 返回的字符串 (空操作,因为是静态字典)
#[no_mangle]
pub extern "C" fn metaime_str_free(_s: *mut u8) {
    // 静态字典, 无需释放
}

/// 查询结果: 包含长度信息, 方便 JNI String 构造
#[repr(C)]
pub struct MetaImeResult {
    data: *const u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn metaime_query(
    _state: *mut MetaImeState,
    shengmu: u8,
    yunmu: u8,
    tone: u8,
) -> MetaImeResult {
    match query_candidates(shengmu, yunmu, tone) {
        Some(candidates) => MetaImeResult {
            data: candidates.as_ptr(),
            len: candidates.len(),
        },
        None => MetaImeResult {
            data: std::ptr::null(),
            len: 0,
        },
    }
}

/// 获取候选词的 UTF-8 长度 (返回字节数)
#[no_mangle]
pub extern "C" fn metaime_result_len(result: MetaImeResult) -> usize {
    result.len
}

/// 获取候选词的 UTF-8 指针 (不允许修改)
#[no_mangle]
pub extern "C" fn metaime_result_ptr(result: MetaImeResult) -> *const u8 {
    result.data
}

/// 销毁结果 (空操作, 静态字典)
#[no_mangle]
pub extern "C" fn metaime_result_free(_result: MetaImeResult) {
}

// ── CString 版本 FFI (推荐 JNI 使用) ──

/// 核心查询接口 (返回 CString, 由调用方用 metaime_string_free 释放)
#[no_mangle]
pub extern "C" fn metaime_query_cstr(
    _state: *mut MetaImeState,
    shengmu: u8,
    yunmu: u8,
    tone: u8,
) -> *mut c_char {
    match query_candidates(shengmu, yunmu, tone) {
        Some(candidates) => {
            match CString::new(candidates) {
                Ok(cs) => cs.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        None => std::ptr::null_mut(),
    }
}

/// 释放 metaime_query_cstr 返回的字符串
#[no_mangle]
pub extern "C" fn metaime_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s); }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 测试
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_ba() {
        // ba: b(1) a(1) 1声
        let r = query_candidates(1, 1, 1);
        assert!(r.is_some());
        assert!(r.unwrap().contains('八'));
    }

    #[test]
    fn test_query_ni() {
        // ni: n(7) i(4) 3声
        let r = query_candidates(7, 4, 3);
        assert!(r.is_some());
        let s = r.unwrap();
        assert!(s.contains('我') || s.contains('你'));
    }

    #[test]
    fn test_query_yi() {
        // yi: y(22) i(4) 1声
        let r = query_candidates(22, 4, 1);
        assert!(r.is_some());
        assert!(r.unwrap().contains('一'));
    }

    #[test]
    fn test_query_guo_not_found() {
        // guo: g(9) uo(26) 2声 (未收录)
        let r = query_candidates(9, 26, 2);
        assert!(r.is_none());
    }

    #[test]
    fn test_query_empty_invalid() {
        let r = query_candidates(255, 255, 255);
        assert!(r.is_none());
    }

    #[test]
    fn test_c_api() {
        let state = metaime_init();
        assert!(!state.is_null());

        let cstr_ptr = metaime_query_cstr(state, 7, 4, 3); // ni3
        assert!(!cstr_ptr.is_null());

        let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
        let s = cstr.to_str().unwrap();
        assert!(s.contains('我') || s.contains('你'));

        metaime_string_free(cstr_ptr);
        metaime_free(state);
    }

    #[test]
    fn test_c_api_not_found() {
        let state = metaime_init();
        let cstr_ptr = metaime_query_cstr(state, 9, 26, 2); // guo2 not found
        assert!(cstr_ptr.is_null());
        metaime_free(state);
    }

}
