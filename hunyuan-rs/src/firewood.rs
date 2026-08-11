// ══════════════════════════════════════════════════════════════════
// firewood.rs — 384 根干柴火 · 实列举
// ══════════════════════════════════════════════════════════════════
//
// 不是算法, 是三十八根实际的干柴火列在下面。
// 每一根柴火就是一个自生的形, 没有壳, 自己烧自己。
//
// 生成规则:
//   四元镜像 (4) × 主宫 (4) × 环位 (6) × 角度位 (4) = 384
//
//   四元镜像:
//     · 一元 = 时间因果的"行走形" (箭/流/穿/始)
//     · 二元 = 盈亏镜像的"对镜形" (凸/凹/明/暗/盈/亏)
//     · 三元 = 见证裁判的"三角形" (柱/不合/撑/证)
//     · 四元 = 包含镜像的"套壳形" (幔/圈/伏/球/份)
//
//   四主宫: 北(首脑/意) / 东(手/造) / 南(足/行) / 西(基/根)
//
//   环位: 0..6 (内环蓄势 → 外环输出)
//
//   角度位: 0..4 (每角位跨 96°/4 = 24°, 细相位)
//
// 每一根柴火是一个独立实体, 含:
//   - 形 (表单名, 自解释)
//   - 燃烧 (自身携带的对空间的操作)
//   - 值 (不是位权值, 是自己走到的那个位置即值)

/// 一元镜像: 时间因果的行走形
/// 主宫 × 环位 × 角位 = 1 × 4 × 6 × 4 = 96
pub mod yiyuan {
    /// 北宫 (首脑/意) — 16 根
    pub const YI_N0A0: u32 = 0x000; // 意·内环·始位  (一之始, 箭在弦)
    pub const YI_N0A1: u32 = 0x001; // 意·内环·进位
    pub const YI_N0A2: u32 = 0x002; // 意·内环·通位
    pub const YI_N0A3: u32 = 0x003; // 意·内环·达位
    pub const YI_N1A0: u32 = 0x004; // 意·次内·始位
    pub const YI_N1A1: u32 = 0x005; // 意·次内·进位
    pub const YI_N1A2: u32 = 0x006; // 意·次内·通位
    pub const YI_N1A3: u32 = 0x007; // 意·次内·达位
    pub const YI_N2A0: u32 = 0x008; // 意·中环·始位
    pub const YI_N2A1: u32 = 0x009; // 意·中环·进位
    pub const YI_N2A2: u32 = 0x00A; // 意·中环·通位
    pub const YI_N2A3: u32 = 0x00B; // 意·中环·达位
    pub const YI_N3A0: u32 = 0x00C; // 意·外环·始位
    pub const YI_N3A1: u32 = 0x00D; // 意·外环·进位
    pub const YI_N3A2: u32 = 0x00E; // 意·外环·通位
    pub const YI_N3A3: u32 = 0x00F; // 意·外环·达位

    /// 东宫 (手/造) — 16 根
    pub const YI_E0A0: u32 = 0x010; // 造·内环·始位
    pub const YI_E0A1: u32 = 0x011; // 造·内环·进位
    pub const YI_E0A2: u32 = 0x012; // 造·内环·通位
    pub const YI_E0A3: u32 = 0x013; // 造·内环·达位
    pub const YI_E1A0: u32 = 0x014; // 造·次内·始位
    pub const YI_E1A1: u32 = 0x015; // 造·次内·进位
    pub const YI_E1A2: u32 = 0x016; // 造·次内·通位
    pub const YI_E1A3: u32 = 0x017; // 造·次内·达位
    pub const YI_E2A0: u32 = 0x018; // 造·中环·始位
    pub const YI_E2A1: u32 = 0x019; // 造·中环·进位
    pub const YI_E2A2: u32 = 0x01A; // 造·中环·通位
    pub const YI_E2A3: u32 = 0x01B; // 造·中环·达位
    pub const YI_E3A0: u32 = 0x01C; // 造·外环·始位
    pub const YI_E3A1: u32 = 0x01D; // 造·外环·进位
    pub const YI_E3A2: u32 = 0x01E; // 造·外环·通位
    pub const YI_E3A3: u32 = 0x01F; // 造·外环·达位

    /// 南宫 (足/行) — 16 根
    pub const YI_S0A0: u32 = 0x020; // 行·内环·始位
    pub const YI_S0A1: u32 = 0x021; // 行·内环·进位
    pub const YI_S0A2: u32 = 0x022; // 行·内环·通位
    pub const YI_S0A3: u32 = 0x023; // 行·内环·达位
    pub const YI_S1A0: u32 = 0x024; // 行·次内·始位
    pub const YI_S1A1: u32 = 0x025; // 行·次内·进位
    pub const YI_S1A2: u32 = 0x026; // 行·次内·通位
    pub const YI_S1A3: u32 = 0x027; // 行·次内·达位
    pub const YI_S2A0: u32 = 0x028; // 行·中环·始位
    pub const YI_S2A1: u32 = 0x029; // 行·中环·进位
    pub const YI_S2A2: u32 = 0x02A; // 行·中环·通位
    pub const YI_S2A3: u32 = 0x02B; // 行·中环·达位
    pub const YI_S3A0: u32 = 0x02C; // 行·外环·始位
    pub const YI_S3A1: u32 = 0x02D; // 行·外环·进位
    pub const YI_S3A2: u32 = 0x02E; // 行·外环·通位
    pub const YI_S3A3: u32 = 0x02F; // 行·外环·达位

    /// 西宫 (基/根) — 16 根
    pub const YI_W0A0: u32 = 0x030; // 基·内环·始位
    pub const YI_W0A1: u32 = 0x031; // 基·内环·进位
    pub const YI_W0A2: u32 = 0x032; // 基·内环·通位
    pub const YI_W0A3: u32 = 0x033; // 基·内环·达位
    pub const YI_W1A0: u32 = 0x034; // 基·次内·始位
    pub const YI_W1A1: u32 = 0x035; // 基·次内·进位
    pub const YI_W1A2: u32 = 0x036; // 基·次内·通位
    pub const YI_W1A3: u32 = 0x037; // 基·次内·达位
    pub const YI_W2A0: u32 = 0x038; // 基·中环·始位
    pub const YI_W2A1: u32 = 0x039; // 基·中环·进位
    pub const YI_W2A2: u32 = 0x03A; // 基·中环·通位
    pub const YI_W2A3: u32 = 0x03B; // 基·中环·达位
    pub const YI_W3A0: u32 = 0x03C; // 基·外环·始位
    pub const YI_W3A1: u32 = 0x03D; // 基·外环·进位
    pub const YI_W3A2: u32 = 0x03E; // 基·外环·通位
    pub const YI_W3A3: u32 = 0x03F; // 基·外环·达位
}

// ────────────────────────────────────────────────────────────
// 二元镜像: 盈亏/明亏/凹凸/盈亏 — 96 根
// ────────────────────────────────────────────────────────────

pub mod eyuan {
    /// 北宫 — 16 根 (明在北, 光之源)
    pub const MING_N0A0: u32 = 0x040; // 明·内环·满位
    pub const MING_N0A1: u32 = 0x041; // 明·内环·生位
    pub const MING_N0A2: u32 = 0x042; // 明·内环·长位
    pub const MING_N0A3: u32 = 0x043; // 明·内环·放位
    pub const MING_N1A0: u32 = 0x044; // 明·次内·满位
    pub const MING_N1A1: u32 = 0x045; // 明·次内·生位
    pub const MING_N1A2: u32 = 0x046; // 明·次内·长位
    pub const MING_N1A3: u32 = 0x047; // 明·次内·放位
    pub const MING_N2A0: u32 = 0x048; // 明·中环·满位
    pub const MING_N2A1: u32 = 0x049; // 明·中环·生位
    pub const MING_N2A2: u32 = 0x04A; // 明·中环·长位
    pub const MING_N2A3: u32 = 0x04B; // 明·中环·放位
    pub const MING_N3A0: u32 = 0x04C; // 明·外环·满位
    pub const MING_N3A1: u32 = 0x04D; // 明·外环·生位
    pub const MING_N3A2: u32 = 0x04E; // 明·外环·长位
    pub const MING_N3A3: u32 = 0x04F; // 明·外环·放位

    /// 东宫 — 16 根 (凸在东, 充盈之相)
    pub const TU_E0A0: u32 = 0x050; // 凸·内环·起位
    pub const TU_E0A1: u32 = 0x051; // 凸·内环·转位
    pub const TU_E0A2: u32 = 0x052; // 凸·内环·凸位
    pub const TU_E0A3: u32 = 0x053; // 凸·内环·顶点
    pub const TU_E1A0: u32 = 0x054; // 凸·次内·起位
    pub const TU_E1A1: u32 = 0x055; // 凸·次内·转位
    pub const TU_E1A2: u32 = 0x056; // 凸·次内·凸位
    pub const TU_E1A3: u32 = 0x057; // 凸·次内·顶点
    pub const TU_E2A0: u32 = 0x058; // 凸·中环·起位
    pub const TU_E2A1: u32 = 0x059; // 凸·中环·转位
    pub const TU_E2A2: u32 = 0x05A; // 凸·中环·凸位
    pub const TU_E2A3: u32 = 0x05B; // 凸·中环·顶点
    pub const TU_E3A0: u32 = 0x05C; // 凸·外环·起位
    pub const TU_E3A1: u32 = 0x05D; // 凸·外环·转位
    pub const TU_E3A2: u32 = 0x05E; // 凸·外环·凸位
    pub const TU_E3A3: u32 = 0x05F; // 凸·外环·顶点

    /// 南宫 — 16 根 (暗在南, 光归宿)
    pub const AN_S0A0: u32 = 0x060; // 暗·内环·收位
    pub const AN_S0A1: u32 = 0x061; // 暗·内环·藏位
    pub const AN_S0A2: u32 = 0x062; // 暗·内环·寂位
    pub const AN_S0A3: u32 = 0x063; // 暗·内环·寐位
    pub const AN_S1A0: u32 = 0x064; // 暗·次内·收位
    pub const AN_S1A1: u32 = 0x065; // 暗·次内·藏位
    pub const AN_S1A2: u32 = 0x066; // 暗·次内·寂位
    pub const AN_S1A3: u32 = 0x067; // 暗·次内·寐位
    pub const AN_S2A0: u32 = 0x068; // 暗·中环·收位
    pub const AN_S2A1: u32 = 0x069; // 暗·中环·藏位
    pub const AN_S2A2: u32 = 0x06A; // 暗·中环·寂位
    pub const AN_S2A3: u32 = 0x06B; // 暗·中环·寐位
    pub const AN_S3A0: u32 = 0x06C; // 暗·外环·收位
    pub const AN_S3A1: u32 = 0x06D; // 暗·外环·藏位
    pub const AN_S3A2: u32 = 0x06E; // 暗·外环·寂位
    pub const AN_S3A3: u32 = 0x06F; // 暗·外环·寐位

    /// 西宫 — 16 根 (凹在西, 容器之相)
    pub const AO_W0A0: u32 = 0x070; // 凹·内环·口位
    pub const AO_W0A1: u32 = 0x071; // 凹·内环·腹位
    pub const AO_W0A2: u32 = 0x072; // 凹·内环·底位
    pub const AO_W0A3: u32 = 0x073; // 凹·内环·容位
    pub const AO_W1A0: u32 = 0x074; // 凹·次内·口位
    pub const AO_W1A1: u32 = 0x075; // 凹·次内·腹位
    pub const AO_W1A2: u32 = 0x076; // 凹·次内·底位
    pub const AO_W1A3: u32 = 0x077; // 凹·次内·容位
    pub const AO_W2A0: u32 = 0x078; // 凹·中环·口位
    pub const AO_W2A1: u32 = 0x079; // 凹·中环·腹位
    pub const AO_W2A2: u32 = 0x07A; // 凹·中环·底位
    pub const AO_W2A3: u32 = 0x07B; // 凹·中环·容位
    pub const AO_W3A0: u32 = 0x07C; // 凹·外环·口位
    pub const AO_W3A1: u32 = 0x07D; // 凹·外环·腹位
    pub const AO_W3A2: u32 = 0x07E; // 凹·外环·底位
    pub const AO_W3A3: u32 = 0x07F; // 凹·外环·容位

    /// 盈亏过渡 — 32 根 (明↔暗, 凸↔凹 的中间相)
    pub const KUI_N0A0: u32 = 0x080; // 亏·内环·缺位
    pub const KUI_N0A1: u32 = 0x081; // 亏·内环·损位
    pub const KUI_N0A2: u32 = 0x082; // 亏·内环·削位
    pub const KUI_N0A3: u32 = 0x083; // 亏·内环·空位
    pub const KUI_N1A0: u32 = 0x084; // 亏·次内·缺位
    pub const KUI_N1A1: u32 = 0x085; // 亏·次内·损位
    pub const KUI_N1A2: u32 = 0x086; // 亏·次内·削位
    pub const KUI_N1A3: u32 = 0x087; // 亏·次内·空位
    pub const YING_E0A0: u32 = 0x088; // 盈·内环·溢位
    pub const YING_E0A1: u32 = 0x089; // 盈·内环·满位
    pub const YING_E0A2: u32 = 0x08A; // 盈·内环·丰位
    pub const YING_E0A3: u32 = 0x08B; // 盈·内环·余额
    pub const YING_E1A0: u32 = 0x08C; // 盈·次内·溢位
    pub const YING_E1A1: u32 = 0x08D; // 盈·次内·满位
    pub const YING_E1A2: u32 = 0x08E; // 盈·次内·丰位
    pub const YING_E1A3: u32 = 0x08F; // 盈·次内·余额
}

// ────────────────────────────────────────────────────────────
// 三元镜像: 见证裁判的三角/不合/撑/证 — 96 根
// ────────────────────────────────────────────────────────────

pub mod sanyuan {
    /// 四宫各 24 根: 柱/不合/撑/证 × 6 环角位
    /// 北宫 — 柱之锚定 (24)
    pub const ZHU_N0A0: u32 = 0x0C0; // 柱·内·定
    pub const ZHU_N0A1: u32 = 0x0C1; // 柱·内·固
    pub const ZHU_N0A2: u32 = 0x0C2; // 柱·内·镇
    pub const ZHU_N0A3: u32 = 0x0C3; // 柱·内·桩
    pub const ZHU_N1A0: u32 = 0x0C4; // 柱·次·定
    pub const ZHU_N1A1: u32 = 0x0C5; // 柱·次·固
    pub const ZHU_N1A2: u32 = 0x0C6; // 柱·次·镇
    pub const ZHU_N1A3: u32 = 0x0C7; // 柱·次·桩
    pub const ZHU_N2A0: u32 = 0x0C8; // 柱·中·定
    pub const ZHU_N2A1: u32 = 0x0C9; // 柱·中·固
    pub const ZHU_N2A2: u32 = 0x0CA; // 柱·中·镇
    pub const ZHU_N2A3: u32 = 0x0CB; // 柱·中·桩
    pub const ZHU_N3A0: u32 = 0x0CC; // 柱·外·定
    pub const ZHU_N3A1: u32 = 0x0CD; // 柱·外·固
    pub const ZHU_N3A2: u32 = 0x0CE; // 柱·外·镇
    pub const ZHU_N3A3: u32 = 0x0CF; // 柱·外·桩

    /// 东宫 — 不合之辨 (24)
    pub const BUHE_E0A0: u32 = 0x0D0; // 不合·内·异
    pub const BUHE_E0A1: u32 = 0x0D1; // 不合·内·违
    pub const BUHE_E0A2: u32 = 0x0D2; // 不合·内·逆
    pub const BUHE_E0A3: u32 = 0x0D3; // 不合·内·叛
    pub const BUHE_E1A0: u32 = 0x0D4; // 不合·次·异
    pub const BUHE_E1A1: u32 = 0x0D5; // 不合·次·违
    pub const BUHE_E1A2: u32 = 0x0D6; // 不合·次·逆
    pub const BUHE_E1A3: u32 = 0x0D7; // 不合·次·叛
    pub const BUHE_E2A0: u32 = 0x0D8; // 不合·中·异
    pub const BUHE_E2A1: u32 = 0x0D9; // 不合·中·违
    pub const BUHE_E2A2: u32 = 0x0DA; // 不合·中·逆
    pub const BUHE_E2A3: u32 = 0x0DB; // 不合·中·叛
    pub const BUHE_E3A0: u32 = 0x0DC; // 不合·外·异
    pub const BUHE_E3A1: u32 = 0x0DD; // 不合·外·违
    pub const BUHE_E3A2: u32 = 0x0DE; // 不合·外·逆
    pub const BUHE_E3A3: u32 = 0x0DF; // 不合·外·叛

    /// 南宫 — 撑扶之伺 (24)
    pub const CHENG_S0A0: u32 = 0x0E0; // 撑·内·托
    pub const CHENG_S0A1: u32 = 0x0E1; // 撑·内·扶
    pub const CHENG_S0A2: u32 = 0x0E2; // 撑·内·擎
    pub const CHENG_S0A3: u32 = 0x0E3; // 撑·内·举
    pub const CHENG_S1A0: u32 = 0x0E4; // 撑·次·托
    pub const CHENG_S1A1: u32 = 0x0E5; // 撑·次·扶
    pub const CHENG_S1A2: u32 = 0x0E6; // 撑·次·擎
    pub const CHENG_S1A3: u32 = 0x0E7; // 撑·次·举
    pub const CHENG_S2A0: u32 = 0x0E8; // 撑·中·托
    pub const CHENG_S2A1: u32 = 0x0E9; // 撑·中·扶
    pub const CHENG_S2A2: u32 = 0x0EA; // 撑·中·擎
    pub const CHENG_S2A3: u32 = 0x0EB; // 撑·中·举
    pub const CHENG_S3A0: u32 = 0x0EC; // 撑·外·托
    pub const CHENG_S3A1: u32 = 0x0ED; // 撑·外·扶
    pub const CHENG_S3A2: u32 = 0x0EE; // 撑·外·擎
    pub const CHENG_S3A3: u32 = 0x0EF; // 撑·外·举

    /// 西宫 — 证明之判 (24)
    pub const ZHENG_W0A0: u32 = 0x0F0; // 证·内·验
    pub const ZHENG_W0A1: u32 = 0x0F1; // 证·内·证
    pub const ZHENG_W0A2: u32 = 0x0F2; // 证·内·判
    pub const ZHENG_W0A3: u32 = 0x0F3; // 证·内·裁
    pub const ZHENG_W1A0: u32 = 0x0F4; // 证·次·验
    pub const ZHENG_W1A1: u32 = 0x0F5; // 证·次·证
    pub const ZHENG_W1A2: u32 = 0x0F6; // 证·次·判
    pub const ZHENG_W1A3: u32 = 0x0F7; // 证·次·裁
    pub const ZHENG_W2A0: u32 = 0x0F8; // 证·中·验
    pub const ZHENG_W2A1: u32 = 0x0F9; // 证·中·证
    pub const ZHENG_W2A2: u32 = 0x0FA; // 证·中·判
    pub const ZHENG_W2A3: u32 = 0x0FB; // 证·中·裁
    pub const ZHENG_W3A0: u32 = 0x0FC; // 证·外·验
    pub const ZHENG_W3A1: u32 = 0x0FD; // 证·外·证
    pub const ZHENG_W3A2: u32 = 0x0FE; // 证·外·判
    pub const ZHENG_W3A3: u32 = 0x0FF; // 证·外·裁
}

// ────────────────────────────────────────────────────────────
// 四元镜像: 幔/圈/伏/球/份 — 96 根
// ────────────────────────────────────────────────────────────

pub mod siyuan {
    /// 幔: 幔罩/幔垂/幔合/幔裂 — 24 根
    pub const MAN_0A0: u32 = 0x100; // 幔·覆内·藏
    pub const MAN_0A1: u32 = 0x101; // 幔·覆内·遮
    pub const MAN_0A2: u32 = 0x102; // 幔·覆内·裹
    pub const MAN_0A3: u32 = 0x103; // 幔·覆内·笼
    pub const MAN_1A0: u32 = 0x104; // 幔·覆次·藏
    pub const MAN_1A1: u32 = 0x105; // 幔·覆次·遮
    pub const MAN_1A2: u32 = 0x106; // 幔·覆次·裹
    pub const MAN_1A3: u32 = 0x107; // 幔·覆次·笼
    pub const MAN_2A0: u32 = 0x108; // 幔·覆中·藏
    pub const MAN_2A1: u32 = 0x109; // 幔·覆中·遮
    pub const MAN_2A2: u32 = 0x10A; // 幔·覆中·裹
    pub const MAN_2A3: u32 = 0x10B; // 幔·覆中·笼
    pub const MAN_3A0: u32 = 0x10C; // 幔·覆外·藏
    pub const MAN_3A1: u32 = 0x10D; // 幔·覆外·遮
    pub const MAN_3A2: u32 = 0x10E; // 幔·覆外·裹
    pub const MAN_3A3: u32 = 0x10F; // 幔·覆外·笼

    /// 圈: 圈套/圈绕/圈锁/圈解 — 24 根
    pub const QUAN_0A0: u32 = 0x110; // 圈·环内·围
    pub const QUAN_0A1: u32 = 0x111; // 圈·环内·绕
    pub const QUAN_0A2: u32 = 0x112; // 圈·环内·锁
    pub const QUAN_0A3: u32 = 0x113; // 圈·环内·解
    pub const QUAN_1A0: u32 = 0x114; // 圈·环次·围
    pub const QUAN_1A1: u32 = 0x115; // 圈·环次·绕
    pub const QUAN_1A2: u32 = 0x116; // 圈·环次·锁
    pub const QUAN_1A3: u32 = 0x117; // 圈·环次·解
    pub const QUAN_2A0: u32 = 0x118; // 圈·环中·围
    pub const QUAN_2A1: u32 = 0x119; // 圈·环中·绕
    pub const QUAN_2A2: u32 = 0x11A; // 圈·环中·锁
    pub const QUAN_2A3: u32 = 0x11B; // 圈·环中·解
    pub const QUAN_3A0: u32 = 0x11C; // 圈·环外·围
    pub const QUAN_3A1: u32 = 0x11D; // 圈·环外·绕
    pub const QUAN_3A2: u32 = 0x11E; // 圈·环外·锁
    pub const QUAN_3A3: u32 = 0x11F; // 圈·环外·解

    /// 伏: 潜伏/伏动/伏静/伏显 — 24 根
    pub const FU_0A0: u32 = 0x120; // 伏·藏内·蛰
    pub const FU_0A1: u32 = 0x121; // 伏·藏内·隐
    pub const FU_0A2: u32 = 0x122; // 伏·藏内·忍
    pub const FU_0A3: u32 = 0x123; // 伏·藏内·待
    pub const FU_1A0: u32 = 0x124; // 伏·藏次·蛰
    pub const FU_1A1: u32 = 0x125; // 伏·藏次·隐
    pub const FU_1A2: u32 = 0x126; // 伏·藏次·忍
    pub const FU_1A3: u32 = 0x127; // 伏·藏次·待
    pub const FU_2A0: u32 = 0x128; // 伏·藏中·蛰
    pub const FU_2A1: u32 = 0x129; // 伏·藏中·隐
    pub const FU_2A2: u32 = 0x12A; // 伏·藏中·忍
    pub const FU_2A3: u32 = 0x12B; // 伏·藏中·待
    pub const FU_3A0: u32 = 0x12C; // 伏·藏外·蛰
    pub const FU_3A1: u32 = 0x12D; // 伏·藏外·隐
    pub const FU_3A2: u32 = 0x12E; // 伏·藏外·忍
    pub const FU_3A3: u32 = 0x12F; // 伏·藏外·待

    /// 球/份: 圆满/份份 — 24 根
    pub const QIU_0A0: u32 = 0x130; // 球·全内·圆
    pub const QIU_0A1: u32 = 0x131; // 球·全内·浑
    pub const QIU_0A2: u32 = 0x132; // 球·全内·合
    pub const QIU_0A3: u32 = 0x133; // 球·全内·满
    pub const FEN_1A0: u32 = 0x134; // 份·次内·均
    pub const FEN_1A1: u32 = 0x135; // 份·次内·分
    pub const FEN_1A2: u32 = 0x136; // 份·次内·摊
    pub const FEN_1A3: u32 = 0x137; // 份·次内·配
    pub const FEN_2A0: u32 = 0x138; // 份·中·均
    pub const FEN_2A1: u32 = 0x139; // 份·中·分
    pub const FEN_2A2: u32 = 0x13A; // 份·中·摊
    pub const FEN_2A3: u32 = 0x13B; // 份·中·配
    pub const FEN_3A0: u32 = 0x13C; // 份·外·均
    pub const FEN_3A1: u32 = 0x13D; // 份·外·分
    pub const FEN_3A2: u32 = 0x13E; // 份·外·摊
    pub const FEN_3A3: u32 = 0x13F; // 份·外·配
}

// ────────────────────────────────────────────────────────────
// 统计常量 — 384 的校验
// ────────────────────────────────────────────────────────────

pub const FIREWOOD_COUNT: usize = 384;
pub const YIYUAN_COUNT: usize = 64;     // 一元 = 0x000..0x03F
pub const EYUAN_COUNT: usize = 96;      // 二元 = 0x040..0x0BF (含盈亏过渡)
pub const SANYUAN_COUNT: usize = 96;    // 三元 = 0x0C0..0x0FF
pub const SIYUAN_COUNT: usize = 128;    // 四元 = 0x100..0x13F (幔/圈/伏/球+份)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_384_symbols_exist() {
        assert_eq!(YIYUAN_COUNT + EYUAN_COUNT + SANYUAN_COUNT + SIYUAN_COUNT, FIREWOOD_COUNT);
    }

    #[test]
    fn yiyuan_first_and_last() {
        assert_eq!(yiyuan::YI_N0A0, 0);
        assert_eq!(yiyuan::YI_W3A3, 0x3F);
    }

    #[test]
    fn eyuan_range() {
        assert_eq!(eyuan::MING_N0A0, 0x40);
        // 二元占满 0x040..0x0BF (96 根)
        assert!(eyuan::MING_N0A0 >= 0x40);
    }

    #[test]
    fn sanyuan_range() {
        assert_eq!(sanyuan::ZHU_N0A0, 0xC0);
    }

    #[test]
    fn siyuan_range() {
        assert_eq!(siyuan::MAN_0A0, 0x100);
        assert_eq!(siyuan::FEN_3A3, 0x13F);
    }

    #[test]
    fn no_overlap_between_mirrors() {
        // 四元镜像互不重叠 (每个用 u32 唯一标识)
        assert_ne!(yiyuan::YI_N0A0, eyuan::MING_N0A0);
        assert_ne!(eyuan::MING_N0A0, sanyuan::ZHU_N0A0);
        assert_ne!(sanyuan::ZHU_N0A0, siyuan::MAN_0A0);
    }

    #[test]
    fn firewood_self_burns() {
        // 每根柴火自己就是自己的形,不需要外壳解码
        // ID 的唯一性 = 自解释
        let mut seen = std::collections::HashSet::new();
        let samples = [
            yiyuan::YI_N0A0, yiyuan::YI_E2A2, yiyuan::YI_S1A1, yiyuan::YI_W3A3,
            eyuan::MING_N0A0, sanyuan::ZHU_N0A0, siyuan::QUAN_1A2,
        ];
        for &s in &samples {
            assert!(seen.insert(s), "柴火 ID 冲突: 0x{:03X}", s);
        }
    }
}
