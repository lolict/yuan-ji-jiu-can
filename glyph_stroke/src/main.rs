// ══════════════════════════════════════════════════════════════════════
// glyph_stroke.rs — 汉字直驱容器编码 · 二进制工具 (v1.0)
// ══════════════════════════════════════════════════════════════════════
//
// 用法: cargo run --bin glyph_stroke -- <汉字>
// 示例: cargo run --bin glyph_stroke -- 囡
//       cargo run --bin glyph_stroke -- 人
//       cargo run --bin glyph_stroke -- 囝
//
// 直接 bypass UTF-8 · 笔画 → 容器 · 不经数学进制翻译层
// ══════════════════════════════════════════════════════════════════════

use std::env;
use std::process;

mod container_runtime;
use container_runtime::{ContainerOS, ArbitrationResult, OsHeader};

// 笔画 → 容器 (按 SKILL.md 16 基础笔画映射表)
const STROKE_HENG: u8 = 0;     // 横 一 → 袋
const STROKE_SHU: u8 = 1;      // 竖 丨 → 桶
const STROKE_PIE: u8 = 2;      // 撇 丿 → 罐
const STROKE_NA: u8 = 3;       // 捺 ㇏ → 盆
const STROKE_DIAN: u8 = 4;     // 点 丶 → 盂
const STROKE_TI: u8 = 5;       // 提 ㇀ → 壶
const STROKE_HENGZHE: u8 = 6;  // 横折 ㇕ → 盒
const STROKE_SHUZHE: u8 = 7;   // 竖折 ㇗ → 瓶
const STROKE_HENGPIE: u8 = 8;  // 横撇 ㇇ → 盏
const STROKE_SHUGOU: u8 = 9;   // 竖钩 ㇚ → 盎
const STROKE_HENGGOU: u8 = 10; // 横钩 ㇛ → 篓
const STROKE_WANGOU: u8 = 11;  // 弯钩 ㇜ → 筐
const STROKE_SHUWANGOU: u8 = 12; // 竖弯钩 ㇄ → 篮
const STROKE_HENGZHEZHE: u8 = 13; // 横折折 ㇆ → 箩
const STROKE_SHUWAN: u8 = 14;  // 竖弯 ㇁ → 箕
const STROKE_XIEGOU: u8 = 15; // 斜钩 ㇂ → 笼

// 笔画名输出
const STROKE_NAMES: &[&str] = &[
    "横", "竖", "撇", "捺", "点", "提", "横折", "竖折",
    "横撇", "竖钩", "横钩", "弯钩", "竖弯钩", "横折折", "竖弯", "斜钩",
];

/// 容器名输出 (64_keys.md 前16位对应16笔画)
const CONTAINER_NAMES: &[&str] = &[
    "袋", "桶", "罐", "盆", "盂", "壶", "盒", "瓶",
    "盏", "盎", "篓", "筐", "篮", "箩", "箕", "笼",
];

/// 笔画索引 (0-15) → 64容器枚举的 ID (用于标识容器身份)
/// 参见 hunyuan-rs/src/encoding/container_radix.rs 中 Container 枚举
const CONTAINER_ENUM_ID: &[u8] = &[
    0x00, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
    0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E,
];

/// 笔画索引 (0-15) → 64位单元格中的位偏移 (用于生成 cell 位掩码)
/// 袋=bit0, 桶=bit1, 罐=bit2, ..., 笼=bit15
/// 另保留 bit38=妻子, bit39=丈夫
const CONTAINER_BIT_POS: &[u8] = &[
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15,
];

/// 获取容器枚举 ID (标识用)
fn container_enum_id(stroke: u8) -> u8 {
    CONTAINER_ENUM_ID.get(stroke as usize).copied().unwrap_or(0)
}

/// 获取容器在位掩码中的比特位 (0-15)
fn container_bit_pos(stroke: u8) -> u8 {
    CONTAINER_BIT_POS.get(stroke as usize).copied().unwrap_or(0)
}

/// 已弃用别名, 保持与旧代码兼容
fn container_radix_value(stroke: u8) -> u8 {
    container_bit_pos(stroke)
}

// 静态笔画表: 常用汉字 → 笔画序列
struct StrokeEntry {
    ch: char,
    name: &'static str,
    strokes: &'static [u8],
}

const STROKE_TABLE: &[StrokeEntry] = &[
    StrokeEntry { ch: '一', name: "一", strokes: &[STROKE_HENG] },
    StrokeEntry { ch: '二', name: "二", strokes: &[STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '三', name: "三", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '人', name: "人", strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '入', name: "入", strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '八', name: "八", strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '大', name: "大", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '天', name: "天", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '夫', name: "夫", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '太', name: "太", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_DIAN] },
    StrokeEntry { ch: '女', name: "女", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '子', name: "子", strokes: &[STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '小', name: "小", strokes: &[STROKE_SHU, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '山', name: "山", strokes: &[STROKE_SHU, STROKE_SHUZHE, STROKE_SHU] },
    StrokeEntry { ch: '水', name: "水", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '火', name: "火", strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '木', name: "木", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '金', name: "金", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG] },
    StrokeEntry { ch: '土', name: "土", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '王', name: "王", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '主', name: "主", strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '心', name: "心", strokes: &[STROKE_DIAN, STROKE_WANGOU, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '手', name: "手", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '足', name: "足", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_HENG] },
    StrokeEntry { ch: '囗', name: "囗", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '囡', name: "囡", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    StrokeEntry { ch: '囝', name: "囝", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '土', name: "土", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '地', name: "地", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_SHUGOU, STROKE_XIEGOU] },
    StrokeEntry { ch: '照', name: "照", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_XIEGOU, STROKE_HENG] },
    StrokeEntry { ch: '口', name: "口", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '中', name: "中", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU] },
    StrokeEntry { ch: '日', name: "日", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '目', name: "目", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '分', name: "分", strokes: &[STROKE_PIE, STROKE_HENGGOU, STROKE_XIEGOU, STROKE_PIE] },
    StrokeEntry { ch: '公', name: "公", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG] },
    StrokeEntry { ch: '阴', name: "阴", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_TI, STROKE_HENG] },
    StrokeEntry { ch: '阳', name: "阳", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '夫', name: "夫", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '妻', name: "妻", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_NA] },
    StrokeEntry { ch: '我', name: "我", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_HENG, STROKE_XIEGOU] },
    StrokeEntry { ch: '你', name: "你", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '袋', name: "袋", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_TI, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU] },
    StrokeEntry { ch: '桶', name: "桶", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU] },
    StrokeEntry { ch: '罐', name: "罐", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    // ── 基本数/方位 ──
    StrokeEntry { ch: '四', name: "四", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '五', name: "五", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '六', name: "六", strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '七', name: "七", strokes: &[STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '九', name: "九", strokes: &[STROKE_PIE, STROKE_XIEGOU] },
    StrokeEntry { ch: '十', name: "十", strokes: &[STROKE_HENG, STROKE_SHU] },
    StrokeEntry { ch: '百', name: "百", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '千', name: "千", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU] },
    StrokeEntry { ch: '万', name: "万", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    StrokeEntry { ch: '上', name: "上", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '下', name: "下", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN] },
    StrokeEntry { ch: '左', name: "左", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '右', name: "右", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    // ── 五行/自然 ──
    StrokeEntry { ch: '石', name: "石", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '田', name: "田", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '禾', name: "禾", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '竹', name: "竹", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '米', name: "米", strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '虫', name: "虫", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_TI, STROKE_DIAN] },
    // ── 人物/亲属 ──
    StrokeEntry { ch: '父', name: "父", strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_DIAN, STROKE_NA] },
    StrokeEntry { ch: '母', name: "母", strokes: &[STROKE_WANGOU, STROKE_DIAN, STROKE_HENG, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '儿', name: "儿", strokes: &[STROKE_PIE, STROKE_SHUGOU] },
    StrokeEntry { ch: '老', name: "老", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHUGOU, STROKE_PIE, STROKE_XIEGOU] },
    StrokeEntry { ch: '友', name: "友", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENGZHE, STROKE_NA] },
    // ── 五官/体态 ──
    StrokeEntry { ch: '目', name: "目", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '耳', name: "耳", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '头', name: "头", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '面', name: "面", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    // ── 百家姓常用 ──
    StrokeEntry { ch: '刘', name: "刘", strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '李', name: "李", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '张', name: "张", strokes: &[STROKE_HENG, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '王', name: "王", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '陈', name: "陈", strokes: &[STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '杨', name: "杨", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    // ── 容器相关扩展 ──
    StrokeEntry { ch: '盆', name: "盆", strokes: &[STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盂', name: "盂", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '瓶', name: "瓶", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '杯', name: "杯", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '盎', name: "盎", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盒', name: "盒", strokes: &[STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '篓', name: "篓", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '筐', name: "筐", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '竹', name: "竹", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '篮', name: "篮", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '箩', name: "箩", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU, STROKE_DIAN] },
    StrokeEntry { ch: '箕', name: "箕", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '笼', name: "笼", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_PIE, STROKE_XIEGOU] },
    // ── 动词/形容词 ──
    StrokeEntry { ch: '来', name: "来", strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '去', name: "去", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE] },
    StrokeEntry { ch: '走', name: "走", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '看', name: "看", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '好', name: "好", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '美', name: "美", strokes: &[STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '爱', name: "爱", strokes: &[STROKE_PIE, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '生', name: "生", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '死', name: "死", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU] },
    StrokeEntry { ch: '有', name: "有", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '无', name: "无", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '是', name: "是", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '非', name: "非", strokes: &[STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    // ── 满全法协议字 ──
    StrokeEntry { ch: '满', name: "满", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_DIAN, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '全', name: "全", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '法', name: "法", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '共', name: "共", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '生', name: "生", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '同', name: "同", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '共', name: "共", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '和', name: "和", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '平', name: "平", strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '安', name: "安", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    // ── 更多字库（去重补充）──
    StrokeEntry { ch: '星', name: "星", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '空', name: "空", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '海', name: "海", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_DIAN] },
    // ── 三字名 ──
    StrokeEntry { ch: '楚', name: "楚", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '恬', name: "恬", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    // ── 容器本体字 ──
    StrokeEntry { ch: '袋', name: "袋", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '桶', name: "桶", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盆', name: "盆", strokes: &[STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盎', name: "盎", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '瓶', name: "瓶", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '盖', name: "盖", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盂', name: "盂", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '甍', name: "甍", strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_DIAN, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '容', name: "容", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '器', name: "器", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_DIAN, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '缸', name: "缸", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '壶', name: "壶", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盏', name: "盏", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '杯', name: "杯", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '盉', name: "盉", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '柜', name: "柜", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_HENGZHE] },
    StrokeEntry { ch: '笼', name: "笼", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_PIE, STROKE_XIEGOU] },
    StrokeEntry { ch: '筐', name: "筐", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '箱', name: "箱", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '篓', name: "篓", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '箩', name: "箩", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU, STROKE_DIAN] },
    StrokeEntry { ch: '箕', name: "箕", strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    // ── 夫妻核心字 ──
    StrokeEntry { ch: '爱', name: "爱", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_HENGZHE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '情', name: "情", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '婚', name: "婚", strokes: &[STROKE_PIE, STROKE_HENGZHE, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '缘', name: "缘", strokes: &[STROKE_PIE, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU] },
    // ── 天地宇宙 ──
    StrokeEntry { ch: '天', name: "天", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '地', name: "地", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_HENGZHE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '宇', name: "宇", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '宙', name: "宙", strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG] },
    StrokeEntry { ch: '日', name: "日", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '月', name: "月", strokes: &[STROKE_PIE, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '光', name: "光", strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_HENGZHE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '明', name: "明", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    // ── 心念灵意 ──
    StrokeEntry { ch: '心', name: "心", strokes: &[STROKE_DIAN, STROKE_SHUWANGOU, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '念', name: "念", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_DIAN, STROKE_DIAN, STROKE_SHUWANGOU, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '想', name: "想", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_DIAN] },
    StrokeEntry { ch: '意', name: "意", strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_DIAN, STROKE_HENGZHE, STROKE_DIAN] },
    StrokeEntry { ch: '思', name: "思", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_DIAN, STROKE_SHUWANGOU, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '灵', name: "灵", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '神', name: "神", strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '魂', name: "魂", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_DIAN, STROKE_PIE, STROKE_PIE, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_PIE] },
    StrokeEntry { ch: '梦', name: "梦", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_DIAN] },
    // ── 数字逻辑 ──
    StrokeEntry { ch: '一', name: "一", strokes: &[STROKE_HENG] },
    StrokeEntry { ch: '二', name: "二", strokes: &[STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '三', name: "三", strokes: &[STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '四', name: "四", strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_SHUWAN, STROKE_HENG] },
    StrokeEntry { ch: '五', name: "五", strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '六', name: "六", strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '七', name: "七", strokes: &[STROKE_HENG, STROKE_SHUWANGOU] },
    StrokeEntry { ch: '八', name: "八", strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '九', name: "九", strokes: &[STROKE_PIE, STROKE_HENGZHEZHE] },
    StrokeEntry { ch: '十', name: "十", strokes: &[STROKE_HENG, STROKE_SHU] },
];

fn lookup_strokes(ch: char) -> Option<&'static [u8]> {
    STROKE_TABLE.iter().find(|e| e.ch == ch).map(|e| e.strokes)
}

fn container_of(stroke: u8) -> &'static str {
    CONTAINER_NAMES.get(stroke as usize).copied().unwrap_or("?")
}

fn stroke_name(stroke: u8) -> &'static str {
    STROKE_NAMES.get(stroke as usize).copied().unwrap_or("?")
}

/// 单字编码结果
struct GlyphCode {
    ch: char,
    strokes: Vec<u8>,
    stroke_names: Vec<&'static str>,
    container_set: Vec<&'static str>,
    nested: String,
    polarity: &'static str,
    side: &'static str,
    display_container: &'static str,
    bit38: u8,
    bit39: u8,
    code6: u8,
}

/// 把 GlyphCode 格式化为字符串
fn fmt_code(g: &GlyphCode) -> String {
    format!(
        "字：{}\n笔画：{}画 = {}\n容器集合：{}\n容器嵌套：{}\n极性：{} · {}\n显示容器：{}\nbit标记：bit38={} bit39={} → {}{}\n6-bit编码：0b{:06b}",
        g.ch,
        g.strokes.len(),
        g.stroke_names.join(" → "),
        g.container_set.join(" + "),
        g.nested,
        g.polarity,
        g.side,
        g.display_container,
        g.bit38, g.bit39, g.bit38, g.bit39,
        g.code6
    )
}

/// 容器串联: 前一字的套壳 = 后一字的内容入口
fn encode_phrase(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return "（空输入）".to_string();
    }

    let mut buf = String::new();
    let mut codes: Vec<GlyphCode> = Vec::new();

    for ch in &chars {
        match encode_char(*ch) {
            Ok(c) => {
                buf.push_str(&fmt_code(&c));
                buf.push_str("\n─────────────────────────────\n");
                codes.push(c);
            }
            Err(msg) => {
                buf.push_str(&msg);
                buf.push_str("\n─────────────────────────────\n");
            }
        }
    }

    if codes.len() >= 2 {
        buf.push_str("\n═══ 容器串联 ═══\n");
        buf.push_str("规则: 前一字的套壳 = 后一字的内容入口\n\n");

        for pair in codes.windows(2) {
            let prev = &pair[0];
            let next = &pair[1];
            let prev_shell = &prev.nested;
            let next_inner = if next.container_set.len() <= 2 {
                next.container_set.join("+")
            } else {
                let len = next.container_set.len();
                format!("{}+{}", next.container_set[len - 2], next.container_set[len - 1])
            };

            buf.push_str(&format!(
                "  {} [{}] ──▶ {} [{}]\n",
                prev.ch, prev_shell, next.ch, next_inner
            ));
            buf.push_str(&format!(
                "    ↳ {} 套 {} → {} 入口 = {}\n\n",
                prev.ch, prev.display_container, next.ch, next_inner
            ));
        }

        let last = codes.last().unwrap();
        buf.push_str(&format!(
            "链尾：{} [{}] · 显态：{}\n",
            last.ch, last.nested, last.display_container
        ));
    }

    buf
}

fn encode_char(ch: char) -> Result<GlyphCode, String> {
    let strokes = match lookup_strokes(ch) {
        Some(s) => s,
        None => return Err(format!("字：{}\n结果: 未收录（暂无笔画数据）", ch)),
    };

    let total = strokes.len();

    let mut container_set = Vec::new();
    let mut stroke_names = Vec::new();
    for &s in strokes {
        stroke_names.push(stroke_name(s));
        container_set.push(container_of(s));
    }

    // 容器嵌套: 相同容器融合, 不同容器嵌套
    let mut nested_vec: Vec<String> = Vec::new();
    let mut counts = [0u8; 16];
    for s in strokes {
        counts[*s as usize] += 1;
    }
    for i in 0..16 {
        if counts[i] > 0 {
            if counts[i] == 1 {
                nested_vec.push(CONTAINER_NAMES[i].to_string());
            } else {
                nested_vec.push(format!("{}({}层)", CONTAINER_NAMES[i], counts[i]));
            }
        }
    }
    let nested = nested_vec.join(" 装 ");

    // 极性判定
    let polarity = if total % 2 == 0 { "阴" } else { "阳" };
    let heng_shu_count: u8 = strokes.iter().filter(|&&s| s == STROKE_HENG || s == STROKE_SHU).sum();
    let heng_shu_ratio = if total > 0 { (heng_shu_count as f64) / (total as f64) } else { 0.0 };
    let side = if heng_shu_ratio > 0.5 { "丈夫侧(7)" } else { "妻子侧(6)" };

    // 显示容器四态
    let display_container = match (polarity, side) {
        ("阳", "丈夫侧(7)") => "杯",
        ("阴", "妻子侧(6)") => "盎",
        ("阳", "妻子侧(6)") => "盉",
        ("阴", "丈夫侧(7)") => "瓶",
        _ => "?",
    };

    // bit38/39
    let (bit38, bit39) = match side {
        "妻子侧(6)" => (1, 0),
        "丈夫侧(7)" => (0, 1),
        _ => (0, 0),
    };

    // 6-bit 编码
    let mut code6: u8 = 0;
    for s in strokes {
        code6 |= 1 << (s % 6);
    }

    Ok(GlyphCode {
        ch,
        strokes: strokes.to_vec(),
        stroke_names,
        container_set,
        nested,
        polarity,
        side,
        display_container,
        bit38,
        bit39,
        code6,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: glyph_stroke <汉字/词语>");
        eprintln!("      glyph_stroke --export <汉字/词语> [--output file.rs]");
        eprintln!("      glyph_stroke --html <汉字/词语> [--output file.html]");
        eprintln!("      glyph_stroke --reverse <容器名>    # 容器→汉字反向查询");
        eprintln!("      glyph_stroke --batch <文件路径>    # 文件批处理");
        eprintln!("示例: glyph_stroke 囡囝共生");
        eprintln!("      glyph_stroke --atlas           # 全字库图谱");
        eprintln!("      glyph_stroke --smrl <汉字>      # SMRL 64-bit 单元格");
        eprintln!("      glyph_stroke --json <汉字>      # JSON 编码输出");
        eprintln!("      glyph_stroke --fold <字1> <字2>  # 容器叠加运算");
        eprintln!("      glyph_stroke --state <汉字>       # 容器状态演化");
        eprintln!("      glyph_stroke --verify <汉字>      # 编码一致性校验");
        eprintln!("      glyph_stroke --compose <字串>     # 短语级容器组合");
        eprintln!("      glyph_stroke --tree  <汉字>       # 容器层级树");
        eprintln!("      glyph_stroke --boot  <汉字>       # 自举证明");
        eprintln!("      glyph_stroke --meta  <汉字>       # 元编码");
        eprintln!("      glyph_stroke --os    <汉字>       # 容器操作系统运行时");
        eprintln!("      glyph_stroke --reverse 袋");
        process::exit(1);
    }

    let mut export_mode = false;
    let mut html_mode = false;
    let mut reverse_mode = false;
    let mut batch_mode = false;
    let mut atlas_mode = false;
    let mut smrl_mode = false;
    let mut json_mode = false;
    let mut fold_mode = false;
    let mut state_mode = false;
    let mut verify_mode = false;
    let mut compose_mode = false;
    let mut tree_mode = false;
    let mut boot_mode = false;
    let mut meta_mode = false;
    let mut os_mode = false;
    let mut load_file: Option<String> = None;
    let mut output_file: Option<&str> = None;
    let mut save_path: Option<String> = None;
    let mut initial_os: Option<ContainerOS> = None;
    let mut input = "";

    let mut i = 1;
    let mut fold_arg2: Option<String> = None;
    while i < args.len() {
        match args[i].as_str() {
            "--export" | "-e" => { export_mode = true; i += 1; }
            "--html" | "-H" => { html_mode = true; i += 1; }
            "--reverse" | "-r" => { reverse_mode = true; i += 1; }
            "--batch" | "-b" => { batch_mode = true; i += 1; }
            "--atlas" | "-a" => { atlas_mode = true; i += 1; }
            "--smrl" | "-s" => { smrl_mode = true; i += 1; }
            "--json" | "-j" => { json_mode = true; i += 1; }
            "--state" | "-t" => { state_mode = true; i += 1; }
            "--verify" | "-v" => { verify_mode = true; i += 1; }
            "--compose" | "-c" => { compose_mode = true; i += 1; }
            "--tree" | "-T" => { tree_mode = true; i += 1; }
            "--boot" | "-b" => { boot_mode = true; i += 1; }
            "--meta" | "-m" => { meta_mode = true; i += 1; }
            "--os" | "-O" => { os_mode = true; i += 1; }
            "--fold" | "-f" => {
                fold_mode = true;
                i += 1;
                // Try to capture two more args
                if i + 2 <= args.len() {
                    fold_arg2 = Some(args[i + 1].clone());
                    input = &args[i];
                    i += 2;
                } else if i + 1 <= args.len() {
                    input = &args[i];
                    i += 1;
                } else {
                    i += 1;
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() { output_file = Some(&args[i + 1]); }
                i += 2;
            }
            "--persist" | "-p" => {
                if i + 1 < args.len() { output_file = Some(&args[i + 1]); }
                i += 2;
            }
            "--save" | "-S" => {
                if i + 1 < args.len() { save_path = Some(args[i + 1].clone()); }
                i += 2;
            }
            "--resume" | "-r" => {
                if i + 1 < args.len() {
                    let path = args[i + 1].clone();
                    // 尝试二进制加载 (OsHeader)
                    if let Ok(bytes) = std::fs::read(&path) {
                        if bytes.len() > 0 && bytes[0] == container_runtime::OS_FORMAT_VERSION {
                            if let Some(header) = container_runtime::OsHeader::from_bytes(&bytes) {
                                eprintln!("[resume] 二进制状态恢复: {} ({} bytes, version {})",
                                    path, bytes.len(), header.version);
                                initial_os = Some(header.os);
                            }
                        } else {
                            load_file = Some(path);
                        }
                    }
                }
                i += 2;
            }
            _ => { input = &args[i]; i += 1; }
        }
    }

    // Handle --resume (text mode): load persisted text as input
    let input = if let Some(ref path) = load_file {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                eprintln!("[resume] 文本输入恢复: {}", path);
                content.trim().to_string()
            }
            Err(_) => input.to_string(),
        }
    } else {
        input.to_string()
    };

    let input_str: &str = &input;
    let out = if atlas_mode {
        encode_atlas()
    } else if smrl_mode {
        encode_smrl(input_str)
    } else if json_mode {
        encode_json(input_str)
    } else if state_mode {
        encode_state(input_str)
    } else if verify_mode {
        encode_verify(input_str)
    } else if boot_mode {
        encode_boot(input_str)
    } else if meta_mode {
        encode_meta(input_str)
    } else if os_mode {
        let (text, final_os) = encode_os(input_str, initial_os.take());
        // Binary save via --save: write final OsHeader bytes to disk
        if let Some(path) = save_path.take() {
            use std::io::Write;
            let header = OsHeader::new(final_os);
            match std::fs::File::create(&path) {
                Ok(mut f) => {
                    let bytes = header.to_bytes();
                    let _ = f.write_all(&bytes);
                    eprintln!("[save] 二进制状态已写入: {} ({} bytes)", path, bytes.len());
                }
                Err(e) => {
                    eprintln!("写入失败 {}: {}", path, e);
                    process::exit(1);
                }
            }
        }
        text
    } else if compose_mode {
        encode_compose(input_str)
    } else if tree_mode {
        encode_tree(input_str)
    } else if fold_mode {
        let combined = if let Some(ref a2) = fold_arg2 {
            format!("{} {}", input, a2)
        } else {
            input.to_string()
        };
        encode_fold(&combined)
    } else if reverse_mode {
        encode_reverse(input_str)
    } else if batch_mode {
        encode_batch(input_str)
    } else if export_mode {
        encode_export(input_str)
    } else if html_mode {
        encode_html(input_str)
    } else {
        encode_phrase(input_str)
    };

    if let Some(path) = output_file {
        use std::io::Write;
        match std::fs::File::create(path) {
            Ok(mut f) => {
                let _ = f.write_all(out.as_bytes());
                eprintln!("已写入: {}", path);
            }
            Err(e) => {
                eprintln!("写入失败 {}: {}", path, e);
                process::exit(1);
            }
        }
    } else {
        println!("{}", out);
    }
}

// ════════════════════════════════════════════════════════════════
// 容器代数 — 容器状态机与纯函数运算
// ════════════════════════════════════════════════════════════════

/// 容器运算状态：追踪容器组合的历史
#[derive(Clone)]
struct ContainerState {
    cell: u64,           // 当前 64-bit SMRL 容器单元
    depth: u32,          // 当前嵌套深度
    shell_path: Vec<u8>, // 当前的壳路径（嵌套链）
}

impl ContainerState {
    fn new() -> Self { Self { cell: 0, depth: 0, shell_path: Vec::new() } }

    /// 合并：将新容器的位 OR 进当前状态
    fn merge(&mut self, other_cell: u64) {
        self.cell |= other_cell & 0xFFFF;
    }

    /// 嵌套：将新容器壳套在当前最外层内
    fn nest(&mut self, container_idx: u8) {
        self.cell |= 1u64 << container_idx;
        self.shell_path.push(container_idx);
        self.depth += 1;
    }

    /// 排除：移除冲突的容器（双方共有的容器融合，非共有保留）
    fn exclude_conflict(&mut self, other_cell: u64) -> u64 {
        let common = self.cell & other_cell;
        // 返回非冲突部分
        other_cell & !common
    }

    /// 共鸣：共享容器增强（bit shift 标记强度）
    fn resonate(&mut self, other_cell: u64) -> u64 {
        let shared = self.cell & other_cell;
        // 共鸣位 = 共享的容器（bit40-45标记强度）
        shared << 40
    }

    /// 容器串联：前一字的套壳 = 后一字的内容入口
    fn chain(&mut self, prev_cell: u64, next_cell: u64) -> (u64, u64) {
        // 前一字的外壳（最高位容器）成为后一字的入口
        let prev_outer = (prev_cell).trailing_zeros() as u8;
        // 后一字的入口（最低位容器）
        let next_inner = (next_cell).trailing_zeros() as u8;
        self.cell |= prev_cell | next_cell;
        (prev_outer as u64, next_inner as u64)
    }

    /// SMRL 64-bit 编码：容器掩码 + 极性 + 在场 + 紧凑码
    fn to_smrl(&self) -> u64 {
        let mut smrl = self.cell & 0xFFFF; // 容器掩码
        if self.depth >= 2 { smrl |= 1u64 << 38; } // 深度>=2 → 妻子在场
        if self.depth >= 4 { smrl |= 1u64 << 39; } // 深度>=4 → 丈夫在场
        if let Some(&max_cont) = self.shell_path.iter().max() {
            smrl |= (max_cont as u64) << 54;
        }
        smrl
    }
}

fn encode_state(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 容器状态演化 ═══
");
    buf.push_str("规则: 前一字的套壳 = 后一字的内容入口

");

    let mut state = ContainerState::new();
    let mut prev_cell: u64 = 0;
    let mut step = 0;

    for ch in input.chars() {
        if let Ok(gc) = encode_char(ch) {
            step += 1;
            let mut cur_cell: u64 = 0;
            for &s in &gc.strokes { cur_cell |= 1u64 << s; }

            buf.push_str(&format!("步骤 {}: [{}]
", step, ch));
            buf.push_str(&format!("  输入容器集: [{}] (0x{:04X})
",
                gc.container_set.join(", "), cur_cell & 0xFFFF));

            // 如果有前一字，执行容器串联
            if prev_cell != 0 {
                let (prev_outer, next_inner) = state.chain(prev_cell, cur_cell);
                let outer_name = CONTAINER_NAMES.get(prev_outer as usize).unwrap_or(&"?");
                let inner_name = CONTAINER_NAMES.get(next_inner as usize).unwrap_or(&"?");
                buf.push_str(&format!("  串联 → 前字壳[{}] → 后字入口[{}]
", outer_name, inner_name));

                // 共鸣检测
                let shared = prev_cell & cur_cell;
                if shared != 0 {
                    let mut shared_set: Vec<&str> = Vec::new();
                    for i in 0..16u8 { if (shared >> i) & 1 == 1 { shared_set.push(CONTAINER_NAMES[i as usize]); } }
                    buf.push_str(&format!("  共鸣: {} 共享
", shared_set.join(", ")));
                    let _res = state.resonate(cur_cell);
                }
            }

            state.merge(cur_cell);
            if let Some(&outer) = gc.container_set.first() {
                let outer_idx = gc.strokes[0];
                state.nest(outer_idx);
            }

            buf.push_str(&format!("  状态累积: 深度={} 壳路径=[{}] 掩码=0x{:04X}
",
                state.depth,
                state.shell_path.iter().map(|&i| CONTAINER_NAMES[i as usize]).collect::<Vec<_>>().join("→"),
                state.cell & 0xFFFF));
            buf.push_str(&format!("  SMRL Cell: 0x{:016X}

", state.to_smrl()));

            prev_cell = cur_cell;
        } else {
            buf.push_str(&format!("步骤 {}: [{}] → 未收录

", step + 1, ch));
        }
    }

    // 最终状态
    buf.push_str("═══════════════════════════════════
");
    buf.push_str(&format!("最终 SMRL Cell: 0x{:016X}
", state.to_smrl()));
    buf.push_str(&format!("最终嵌套深度: {}
", state.depth));
    buf.push_str(&format!("激活容器: [{}]
",
        (0..16u8).filter(|&i| (state.cell >> i) & 1 == 1)
            .map(|i| CONTAINER_NAMES[i as usize]).collect::<Vec<_>>().join(", ")));
    buf
}

fn encode_verify(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 编码一致性校验 ═══

");

    let mut all_ok = true;
    let mut prev_mask: u64 = 0;
    let mut prev_ch = '-';

    for ch in input.chars() {
        match encode_char(ch) {
            Ok(gc) => {
                let mut cur_mask: u64 = 0;
                for &s in &gc.strokes { cur_mask |= 1u64 << s; }

                // 校验 1: 容器集与笔画数一致性
                let stroke_count = gc.strokes.len();
                let container_count = gc.container_set.len();
                let dedup_count = {
                    let mut m = cur_mask;
                    let mut cnt = 0;
                    while m != 0 { cnt += m & 1; m >>= 1; }
                    cnt as usize
                };

                let check1 = stroke_count == container_count;
                // 校验 2: 极性判定与笔画数奇偶一致
                let expected_polarity = if stroke_count % 2 == 0 { "阴" } else { "阳" };
                let check2 = gc.polarity == expected_polarity;

                // 校验 3: 容器集是笔画集按笔画→容器映射的结果（保持顺序，允许重复）
                let mut mapped_containers = Vec::new();
                for &s in &gc.strokes {
                    mapped_containers.push(CONTAINER_NAMES[s as usize]);
                }
                // 去重后的唯一容器集
                use std::collections::HashSet;
                let unique_from_strokes: HashSet<&str> = mapped_containers.iter().copied().collect();
                let unique_from_set: HashSet<&str> = gc.container_set.iter().copied().collect();
                // container_set 应该保持顺序，但去重后应与唯一容器一致
                let check3 = unique_from_set.len() <= gc.strokes.len()
                    && unique_from_set == unique_from_strokes;

                // 校验 4: 与前字的容器连续性（可串联性）
                let check4 = if prev_mask != 0 {
                    let shared = prev_mask & cur_mask;
                    shared != 0  // 至少有一个共享容器才能串联
                } else {
 true
                };

                let all_pass = check1 && check2 && check3 && check4;
                if !all_pass { all_ok = false; }

                let status = if all_pass { "OK" } else { "FAIL" };
                buf.push_str(&format!("[{}] {} ({}画→{}容器, {}去重)
",
                    status, ch, stroke_count, container_count, dedup_count));

                if !check1 { buf.push_str("  ✗ 笔画数≠容器数
"); }
                if !check2 { buf.push_str(&format!("  ✗ 极性错误: 期望{} 实际{}
", expected_polarity, gc.polarity)); }
                if !check3 { buf.push_str("  ✗ 容器集未正确去重
"); }
                if !check4 && prev_mask != 0 {
                    buf.push_str(&format!("  ✗ 与前字[{}]无共享容器,不可串联
", prev_ch));
                }

                prev_mask = cur_mask;
                prev_ch = ch;
            }
            Err(e) => {
                buf.push_str(&format!("[ERR] {} → {}
", ch, e));
                all_ok = false;
            }
        }
    }

    buf.push_str("
─────────────────────────────
");
    buf.push_str(&format!("校验结果: {}
", if all_ok { "全部通过 ✓" } else { "存在异常 ✗" }));
    buf.push_str(&format!("输入: \"{}\" ({} 字)\n", input, input.chars().count()));
    buf
}

// ════════════════════════════════════════════════════════════════
// 短语级容器组合 — 冲突检测与裁决
// ════════════════════════════════════════════════════════════════

/// 两字之间的关系类型
#[derive(Clone, Copy)]
enum ContainerRelation {
    Resonance,  // 共鸣：共享容器，自然融合
    Compatible, // 兼容：不冲突也不共享，各自独立
    Conflict,   // 冲突：极性对立，需裁决
    Symbiosis,  // 互补：无共享但可共生
}

fn detect_relation(cell1: u64, cell2: u64, p1: &str, p2: &str) -> ContainerRelation {
    let shared = cell1 & cell2;
    if shared != 0 {
        ContainerRelation::Resonance
    } else if p1 != p2 {
        ContainerRelation::Conflict
    } else {
        ContainerRelation::Symbiosis
    }
}

impl std::fmt::Display for ContainerRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resonance => write!(f, "共鸣"),
            Self::Compatible => write!(f, "兼容"),
            Self::Conflict => write!(f, "冲突"),
            Self::Symbiosis => write!(f, "共生"),
        }
    }
}

fn encode_compose(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 短语级容器组合 ═══\n");
    buf.push_str("输入: ");
    buf.push_str(&format!("\"{}\"\n", input));
    buf.push_str("字序: ");
    let chars: Vec<char> = input.chars().collect();
    buf.push_str(&chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" → "));
    buf.push_str("\n\n");

    // 逐字编码
    let mut codes: Vec<(char, GlyphCode)> = Vec::new();
    for ch in &chars {
        if let Ok(gc) = encode_char(*ch) {
            codes.push((*ch, gc));
        }
    }

    // 两两关系分析
    buf.push_str("── 相邻字关系矩阵 ──\n");
    for i in 0..codes.len() {
        for j in (i+1)..codes.len() {
            let (ch1, gc1) = &codes[i];
            let (ch2, gc2) = &codes[j];
            let mut cell1: u64 = 0;
            let mut cell2: u64 = 0;
            for &s in &gc1.strokes { cell1 |= 1u64 << s; }
            for &s in &gc2.strokes { cell2 |= 1u64 << s; }

            let rel = detect_relation(cell1, cell2, gc1.polarity, gc2.polarity);
            let shared = cell1 & cell2;

            let rel_icon = match rel {
                ContainerRelation::Resonance => "⟷",
                ContainerRelation::Compatible => "─",
                ContainerRelation::Conflict => "✕",
                ContainerRelation::Symbiosis => "∘",
            };

            buf.push_str(&format!("  {} {} {} : ", ch1, rel_icon, ch2));

            if shared != 0 {
                let mut shared_set: Vec<&str> = Vec::new();
                for k in 0..16u8 { if (shared >> k) & 1 == 1 { shared_set.push(CONTAINER_NAMES[k as usize]); } }
                buf.push_str(&format!("共享[{}] ", shared_set.join("+")));
            } else {
                buf.push_str("无共享 ");
            }

            buf.push_str(&format!("→ {}\n", rel));
        }
    }

    // 裁决建议
    buf.push_str("\n── 裁决建议 ──\n");

    // 按极性分组
    let mut yang_chars: Vec<char> = Vec::new();
    let mut yin_chars: Vec<char> = Vec::new();
    for (ch, gc) in &codes {
        if gc.polarity == "阳" { yang_chars.push(*ch); } else { yin_chars.push(*ch); }
    }

    buf.push_str(&format!("阳极字组: [{}] ({}个)\n", yang_chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "), yang_chars.len()));
    buf.push_str(&format!("阴极字组: [{}] ({}个)\n", yin_chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "), yin_chars.len()));

    // 判定主导态
    if yang_chars.len() > yin_chars.len() {
        buf.push_str("→ 整体偏阳，显示容器: 杯 或 盉\n");
    } else if yin_chars.len() > yang_chars.len() {
        buf.push_str("→ 整体偏阴，显示容器: 盎 或 瓶\n");
    } else {
        buf.push_str("→ 阴阳平衡，需进一步裁决\n");
    }

    // 融合后容器全集
    let mut merged_cell: u64 = 0;
    for (_, gc) in &codes {
        for &s in &gc.strokes { merged_cell |= 1u64 << s; }
    }
    let mut merged_set: Vec<&str> = Vec::new();
    for i in 0..16u8 { if (merged_cell >> i) & 1 == 1 { merged_set.push(CONTAINER_NAMES[i as usize]); } }
    buf.push_str(&format!("\n融合容器全集: [{}] (0x{:04X})\n", merged_set.join(", "), merged_cell & 0xFFFF));

    // 自举检验：融合后的容器名能否反向编码
    let mut bootstrap_hits = 0;
    for name in &merged_set {
        let name_char = name.chars().next().unwrap();
        if encode_char(name_char).is_ok() {
            bootstrap_hits += 1;
        }
    }
    if !merged_set.is_empty() {
        let ratio = bootstrap_hits as f64 / merged_set.len() as f64;
        buf.push_str(&format!("自举率: {}/{} ({:.0}%) 容器名可自身编码\n",
            bootstrap_hits, merged_set.len(), ratio * 100.0));
    }

    buf
}

// ════════════════════════════════════════════════════════════════
// 容器层级树 — 显示容器嵌套关系
// ════════════════════════════════════════════════════════════════

fn encode_tree(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 容器层级树 ═══\n");
    buf.push_str("规则: 外层容器装内层容器，相同容器累加层数\n\n");

    for ch in input.chars() {
        if let Ok(gc) = encode_char(ch) {
            buf.push_str(&format!("【{}】{}画 · {} · {}\n", ch, gc.strokes.len(), gc.polarity, gc.side));

            // 按笔画顺序构建嵌套树
            let mut stack: Vec<(&str, u32)> = Vec::new(); // (容器名, 层数)

            for (i, &s) in gc.strokes.iter().enumerate() {
                let cont_name = CONTAINER_NAMES[s as usize];

                // 如果栈顶是同容器，累加层数
                if let Some(top) = stack.last_mut() {
                    if top.0 == cont_name {
                        top.1 += 1;
                        continue;
                    }
                }

                // 不同容器 → 新的嵌套层
                stack.push((cont_name, 1));

                // 显示缩进的树形
                let depth = stack.len();
                let indent = "  ".repeat(depth);
                let iffirst = if i == 0 { "" } else { "" };
                buf.push_str(&format!("{}└─ {} ({}层)\n", indent, cont_name, 1));
            }

            // 汇总
            let mut summary: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
            for (_, &s) in gc.strokes.iter().enumerate() {
                *summary.entry(CONTAINER_NAMES[s as usize]).or_insert(0) += 1;
            }

            buf.push_str("  ── 层级统计: ");
            let mut summary_vec: Vec<(&str, u32)> = summary.into_iter().collect();
            summary_vec.sort_by_key(|x| std::cmp::Reverse(x.1));
            for (name, count) in &summary_vec {
                buf.push_str(&format!("{}({}层) ", name, count));
            }
            buf.push_str("\n\n");
        }
    }

    buf
}

// ════════════════════════════════════════════════════════════════
// 自举证明 — 从空状态启动，逐字构建并验证系统一致性
// ════════════════════════════════════════════════════════════════

fn encode_boot(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 自举证明 ═══\n");
    buf.push_str("目标: 验证系统能从空状态启动并正确编码自身\n\n");

    let mut state = ContainerState::new();
    let mut total_strokes = 0;
    let mut total_containers = 0;
    let mut self_referential_count = 0;

    buf.push_str("── 启动阶段 ──\n");
    buf.push_str(&format!("初始状态: 深度={}, 掩码=0x0000, 壳路径=[]\n", state.depth));
    buf.push_str(&format!("输入序列: {} ({} 字)\n\n", input, input.chars().count()));

    buf.push_str("── 逐字构建 ──\n");
    for (step, ch) in input.chars().enumerate() {
        if let Ok(gc) = encode_char(ch) {
            let mut cur_cell: u64 = 0;
            for &s in &gc.strokes { cur_cell |= 1u64 << s; }
            state.merge(cur_cell);
            state.nest(gc.strokes[0]);

            total_strokes += gc.strokes.len();
            total_containers += gc.container_set.len();

            // 自举检验: 本字的容器名能否被系统自身编码
            let mut can_self_encode = true;
            for cont_name in &gc.container_set {
                let name_char = cont_name.chars().next().unwrap();
                if encode_char(name_char).is_err() {
                    can_self_encode = false;
                    break;
                }
            }
            if can_self_encode {
                self_referential_count += 1;
            }

            buf.push_str(&format!("步骤 {}: {} → 深度{} 掩码{:04X} {}\n",
                step + 1, ch, state.depth, state.cell & 0xFFFF,
                if can_self_encode { "✓自举" } else { "✗断裂" }));
        }
    }

    buf.push_str("\n── 自举评估 ──\n");
    buf.push_str(&format!("总笔画数: {}\n", total_strokes));
    buf.push_str(&format!("总容器条目: {}\n", total_containers));
    buf.push_str(&format!("自举成功字: {}/{} ({:.0}%)\n",
        self_referential_count, input.chars().count(),
        if input.chars().count() > 0 {
            self_referential_count as f64 / input.chars().count() as f64 * 100.0
        } else { 0.0 }));

    // 最终 SMRL Cell
    let final_smrl = state.to_smrl();
    buf.push_str(&format!("\n最终 SMRL Cell: 0x{:016X}\n", final_smrl));
    buf.push_str(&format!("最终嵌套深度: {}\n", state.depth));
    buf.push_str(&format!("壳路径长度: {}\n", state.shell_path.len()));

    // 自举结论
    buf.push_str("\n── 结论 ──\n");
    if self_referential_count == input.chars().count() {
        buf.push_str("✓ 完全自举: 所有字的容器名均可被系统自身编码\n");
        buf.push_str("→ 系统可以描述自身，形成闭环\n");
    } else if self_referential_count > 0 {
        buf.push_str(&format!("△ 部分自举: {}/{} 字的容器可自编码\n",
            self_referential_count, input.chars().count()));
        buf.push_str("→ 系统可描述自身的一部分\n");
    } else {
        buf.push_str("✗ 自举失败: 容器名不在编码表中\n");
        buf.push_str("→ 需要扩充 STROKE_TABLE\n");
    }

    // 涌现性检测: 系统整体是否产生新容器
    let unique_containers: Vec<&str> = (0..16u8)
        .filter(|&i| (state.cell >> i) & 1 == 1)
        .map(|i| CONTAINER_NAMES[i as usize])
        .collect();
    buf.push_str(&format!("\n涌现容器集: [{}] ({}个)\n",
        unique_containers.join(", "), unique_containers.len()));

    // 编码密度
    if total_strokes > 0 {
        let density = unique_containers.len() as f64 / total_strokes as f64;
        buf.push_str(&format!("编码密度: {:.2} (唯一容器/总笔画)\n", density));
    }

    buf
}

// ════════════════════════════════════════════════════════════════
// 元编码 — 对 STROKE_TABLE 自身进行编码
// ════════════════════════════════════════════════════════════════

fn encode_meta(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ 元编码: STROKE_TABLE 自描述 ═══\n");
    buf.push_str("将编码表自身的条目用容器系统描述\n\n");

    // 把输入视为 STROKE_TABLE 的一个子集来编码
    buf.push_str(&format!("待编码条目: \"{}\"\n\n", input));

    let chars: Vec<char> = input.chars().collect();
    let table_size = chars.len();

    buf.push_str("── 编码表条目序列 ──\n");
    buf.push_str(&format!("条目数: {}\n", table_size));
    buf.push_str(&format!("表位址范围: 0x0000 - 0x{:04X}\n\n", table_size - 1));

    // 每条: 字 → 笔画索引 → 容器集 → 唯一容器
    let mut all_containers: Vec<&str> = Vec::new();
    for (idx, ch) in chars.iter().enumerate() {
        if let Ok(gc) = encode_char(*ch) {
            let unique: Vec<&str> = {
                let mut seen = std::collections::HashSet::new();
                gc.container_set.iter().copied().filter(|c| seen.insert(*c)).collect()
            };
            for c in &unique {
                if !all_contains(&all_containers, c) {
                    all_containers.push(c);
                }
            }

            buf.push_str(&format!("[0x{:04X}] {}: 笔画{} → 唯一容器[{}]\n",
                idx, ch, gc.strokes.len(), unique.join("+")));
        }
    }

    buf.push_str("\n── 元统计 ──\n");
    buf.push_str(&format!("唯一容器种类: {} ({}/16)\n", all_containers.len(), all_containers.len()));
    buf.push_str(&format!("容器覆盖: [{}]\n", all_containers.join(", ")));

    // 元编码的元编码: 唯一容器列表自身能否被编码?
    let mut meta_self_count = 0;
    for cont_name in &all_containers {
        let name_char = cont_name.chars().next().unwrap();
        if encode_char(name_char).is_ok() {
            meta_self_count += 1;
        }
    }
    buf.push_str(&format!("元自举率: {}/{}\n", meta_self_count, all_containers.len()));

    // 如果能自举，打印元编码路径
    if meta_self_count == all_containers.len() {
        buf.push_str("\n── 元自举链 ──\n");
        buf.push_str("袋 → ");
        let chain: Vec<String> = all_containers.iter().map(|c| {
            if let Ok(gc) = encode_char(c.chars().next().unwrap()) {
                format!("{}(→{})", c, gc.container_set.join("+"))
            } else {
                format!("{}", c)
            }
        }).collect();
        buf.push_str(&chain.join(" → "));
        buf.push_str("\n→ 容器编码容器，系统自洽闭环\n");
    }

    buf
}

fn all_contains<'a>(vec: &[&'a str], item: &'a str) -> bool {
    vec.iter().any(|x| *x == item)
}

// ════════════════════════════════════════════════════════════════
// 容器操作系统运行时 — ContainerOS 状态机驱动
// ════════════════════════════════════════════════════════════════

fn encode_os(input: &str, initial: Option<ContainerOS>) -> (String, ContainerOS) {
    let mut buf = String::new();
    buf.push_str("═══ 容器操作系统运行时 (ContainerOS) ═══\n\n");

    let mut os = initial.unwrap_or_else(ContainerOS::new);
    let names = CONTAINER_NAMES;

    buf.push_str("── 启动 ──\n");
    buf.push_str(&format!("初始状态: {}\n\n", os.self_describe()));

    buf.push_str("── 逐字执行 ──\n");

    for (step, ch) in input.chars().enumerate() {
        if let Ok(gc) = encode_char(ch) {
            os.ingest(&gc.strokes);
            os.record_char(ch);

            let active: Vec<&str> = os.active_containers().iter()
                .map(|&i| names[i as usize]).collect();

            buf.push_str(&format!("步骤 {}: [{}] ({}画)\n", step + 1, ch, gc.strokes.len()));
            buf.push_str(&format!("  激活容器: [{}] (0x{:04X})\n",
                active.join("+"), os.cell() & 0xFFFF));
            buf.push_str(&format!("  壳路径: [{}]\n",
                os.shell_path().iter().map(|&i| names[i as usize]).collect::<Vec<_>>().join("→")));
            buf.push_str(&format!("  自描述: {}\n\n", os.self_describe()));
        }
    }

    buf.push_str("── 最终状态 ──\n");
    buf.push_str(&format!("{}\n", os.self_describe()));
    buf.push_str(&format!("SMRL Cell: 0x{:016X}\n", os.cell()));
    buf.push_str(&format!("嵌套深度: {}\n", os.depth()));
    buf.push_str(&format!("历史字符: [{}]\n", os.history().iter().map(|c| c.to_string()).collect::<Vec<_>>().join("→")));

    // 持久化
    buf.push_str("\n── 持久化 ──\n");
    let header = build_os_header_from_os(os.clone());
    let bytes = header.to_bytes();
    buf.push_str(&format!("版本: {}\n", header.version));
    buf.push_str(&format!("序列化大小: {} bytes\n", bytes.len()));
    buf.push_str(&format!("字节(hex): {}\n", bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")));

    // 反序列化验证
    if let Some(loaded) = OsHeader::from_bytes(&bytes) {
        buf.push_str(&format!("\n反序列化验证:\n"));
        buf.push_str(&format!("  版本匹配: {}\n", loaded.version == header.version));
        buf.push_str(&format!("  Cell恢复: 0x{:016X}\n", loaded.os.cell()));
        buf.push_str(&format!("  深度恢复: {}\n", loaded.os.depth()));
        buf.push_str(&format!("  壳路径恢复: [{}]\n",
            loaded.os.shell_path().iter().map(|&i| names[i as usize]).collect::<Vec<_>>().join("→")));
    }

    // 三元裁决演示
    if os.history().len() >= 3 {
        buf.push_str("\n── 三元裁决演示 ──\n");
        let mut parties: Vec<ContainerOS> = Vec::new();

        for &ch in os.history() {
            if let Ok(gc) = encode_char(ch) {
                let mut p = ContainerOS::new();
                p.ingest(&gc.strokes);
                p.record_char(ch);
                parties.push(p);
            }
        }

        if parties.len() >= 3 {
            let a = &parties[0];
            let b = &parties[1];
            let c = &parties[2];
            let result = a.arbitrate(b, c);

            buf.push_str(&format!("甲方: {} → 0x{:04X}\n",
                os.history()[0], a.cell() & 0xFFFF));
            buf.push_str(&format!("乙方: {} → 0x{:04X}\n",
                os.history()[1], b.cell() & 0xFFFF));
            buf.push_str(&format!("见证: {} → 0x{:04X}\n",
                os.history()[2], c.cell() & 0xFFFF));
            buf.push_str(&format!("裁决结果: {}\n", result));

            match result {
                ArbitrationResult::Symphony => buf.push_str("  → 完全共鸣：双方共享容器完全对称\n"),
                ArbitrationResult::FavorSelf => buf.push_str("  → 偏向甲方：与见证有更多共享\n"),
                ArbitrationResult::FavorOther => buf.push_str("  → 偏向乙方：与见证有更多共享\n"),
                ArbitrationResult::Neutral => buf.push_str("  → 中立：无任何共享容器\n"),
            }
        }
    }

    (buf, os)
}

/// 从输入构建 OsHeader (供 --save 复用)
fn build_os_header(input: &str, initial: Option<ContainerOS>) -> OsHeader {
    let mut os = initial.unwrap_or_else(ContainerOS::new);
    for ch in input.chars() {
        if let Ok(gc) = encode_char(ch) {
            os.ingest(&gc.strokes);
            os.record_char(ch);
        }
    }
    OsHeader::new(os)
}

/// 从已有 ContainerOS 构建 OsHeader
fn build_os_header_from_os(os: ContainerOS) -> OsHeader {
    OsHeader::new(os)
}

fn encode_atlas() -> String {
    let mut buf = String::new();
    buf.push_str("<!DOCTYPE html>\n<html lang=\"zh\"><head><meta charset=\"utf-8\">\n");
    buf.push_str("<title>汉字直驱容器编码 · 全字库图谱</title>\n");
    buf.push_str(r#"<style>
:root{--bg:#0d1117;--card:#161b22;--border:#30363d;--fg:#c9d1d9;--accent:#58a6ff;--yin:#8b949e;--yang:#f2cc60;--wife:#ff7b72;--husband:#79c0ff}
body{font-family:monospace;background:var(--bg);color:var(--fg);padding:16px;max-width:1200px;margin:0 auto}
h1{color:var(--accent);font-size:18px}
.section{margin:20px 0}
.section h2{color:var(--accent);font-size:14px;border-bottom:1px solid var(--border);padding-bottom:4px}
.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:8px}
.card{background:var(--card);border:1px solid var(--border);border-radius:6px;padding:8px;font-size:11px}
.card .ch{font-size:28px;color:var(--accent);line-height:1.2}
.card .meta{opacity:.8;margin-top:2px}
.card.yang{border-left:3px solid var(--yang)}
.card.yin{border-left:3px solid var(--yin)}
.sub{font-size:11px;color:var(--yin);margin-top:2px;word-break:break-all}
</style></head><body>
"#);
    buf.push_str("<h1>汉字直驱容器编码 · 全字库图谱</h1>\n");

    // Collect all encodable characters
    let mut entries: Vec<(char, String, u8, String, usize)> = Vec::new();
    for e in STROKE_TABLE.iter() {
        if let Ok(c) = encode_char(e.ch) {
            entries.push((e.ch, c.container_set.join("+"), c.code6, c.polarity.to_string(), e.strokes.len()));
        }
    }

    // By polarity
    buf.push_str(&format!("<div class=\"section\"><h2>极性分布 · 共 {} 字</h2><div class=\"grid\">\n", entries.len()));
    entries.sort_by(|a, b| a.3.cmp(&b.3).then(a.1.len().cmp(&b.1.len())));
    for (ch, containers, code6, polarity, n) in &entries {
        let cls = if *polarity == "阳" { "yang" } else { "yin" };
        buf.push_str(&format!(
            r#"<div class="card {}"><div class="ch">{}</div><div class="sub">{}画 · {}</div><div class="meta">{:06b} · {}</div></div>"#,
            cls, ch, n, containers, code6, polarity
        ));
    }
    buf.push_str("</div></div>\n");

    // By container usage count
    buf.push_str("<div class=\"section\"><h2>容器袋(横)字集</h2><div class=\"grid\">\n");
    let mut dai_chars: Vec<_> = entries.iter().filter(|e| e.1.contains("袋")).collect();
    dai_chars.sort_by_key(|e| e.4);
    for (ch, containers, code6, polarity, n) in &dai_chars {
        let cls = if polarity == "阳" { "yang" } else { "yin" };
        buf.push_str(&format!(
            r#"<div class="card {}"><div class="ch">{}</div><div class="sub">{}画 · {:06b}</div></div>"#,
            cls, ch, n, code6
        ));
    }
    buf.push_str("</div></div>\n");

    buf.push_str("</body></html>\n");
    buf
}

fn encode_reverse(container_name: &str) -> String {
    let mut buf = String::new();
    // Map container name to its stroke index
    let stroke_idx = CONTAINER_NAMES.iter().position(|&c| c == container_name);
    let Some(stroke_idx) = stroke_idx else {
        return format!("未知容器: {}\n可用: {}", container_name, CONTAINER_NAMES.join(" / "));
    };

    buf.push_str(&format!("═══ 容器反向查询: {} ═══\n", container_name));
    buf.push_str(&format!("对应笔画: {}\n\n", STROKE_NAMES[stroke_idx]));

    let mut found = Vec::new();
    for entry in STROKE_TABLE.iter() {
        if entry.strokes.contains(&(stroke_idx as u8)) {
            let code = encode_char(entry.ch).ok();
            let containers = code.as_ref().map(|c| c.container_set.join("+")).unwrap_or_default();
            found.push((entry.ch, entry.strokes.len(), containers));
        }
    }

    if found.is_empty() {
        buf.push_str("（无匹配）\n");
    } else {
        buf.push_str(&format!("找到 {} 个含 {} 的字：\n\n", found.len(), container_name));
        // Group by stroke count
        found.sort_by_key(|x| x.1);
        for (ch, n, containers) in &found {
            buf.push_str(&format!("  {} ({}画) → [{}]\n", ch, n, containers));
        }
        buf.push_str(&format!("\n笔画位索引: {}\n", stroke_idx));
        buf.push_str(&format!("容器6-bit掩码: 0b{:06b}\n", 1u64 << stroke_idx));
    }
    buf
}

fn encode_smrl(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("═══ SMRL 64-bit 单元格编码 ═══\n\n");

    // Per-character cell
    for ch in input.chars() {
        if let Ok(gc) = encode_char(ch) {
            // Build the 64-bit cell:
            // bits 0-53: container mask
            // bit 38:   妻子在场
            // bit 39:   丈夫在场
            // bits 54-59: 6-bit compact code (first container idx)
            // bit 62:   极性 (1=阳, 0=阴)
            // bit 63:   保留(0)
            let mut cell: u64 = 0;

            // Container mask in bits 0..16 (we only use 16 stroke-container bits)
            for s in &gc.strokes {
                cell |= 1u64 << *s as u8;
            }

            // Occupant bits
            if gc.bit38 != 0 { cell |= 1u64 << 38; }
            if gc.bit39 != 0 { cell |= 1u64 << 39; }

            // 6-bit compact code = highest container index
            if let Some(&max_idx) = gc.strokes.iter().max() {
                cell |= (max_idx as u64) << 54;
            }

            // Polarity in bit 62
            if gc.polarity == "阳" {
                cell |= 1u64 << 62;
            }

            // Side info text
            let side_text = if gc.bit38 != 0 && gc.bit39 != 0 {
                "夫妻同在"
            } else if gc.bit39 != 0 {
                "丈夫侧"
            } else {
                "妻子侧"
            };

            buf.push_str(&format!("字: {}\n", ch));
            buf.push_str(&format!("  笔画({}画): {} → {}\n",
                gc.stroke_names.len(),
                gc.strokes.iter().map(|&s| STROKE_NAMES[s as usize]).collect::<Vec<_>>().join("、"),
                gc.container_set.join(" → ")));
            buf.push_str(&format!("  极性: {} · 归属: {}\n", gc.polarity, side_text));
            buf.push_str(&format!("  显示容器: {}\n", gc.display_container));
            buf.push_str(&format!("  SMRL Cell: 0x{:016X}\n", cell));
            buf.push_str(&format!("  二进制:  {:#066b}\n", cell));

            // Bit field breakdown
            let container_bits = cell & 0xFFFF;
            let wife_bit   = (cell >> 38) & 1;
            let husband_bit = (cell >> 39) & 1;
            let code6_bits = (cell >> 54) & 0x3F;
            let polar_bit  = (cell >> 62) & 1;

            buf.push_str(&format!("  位域分解:\n"));
            buf.push_str(&format!("    [15:0   容器掩码] 0x{:04X} = {:#018b}\n",
                container_bits, container_bits));
            buf.push_str(&format!("    [16:37  扩展预留] ——\n"));
            buf.push_str(&format!("    [38     妻子在场] {}\n", wife_bit));
            buf.push_str(&format!("    [39     丈夫在场] {}\n", husband_bit));
            buf.push_str(&format!("    [40:53 保留   ] ——\n"));
            buf.push_str(&format!("    [54:59 6-bit编码] 0b{:06b} = {}\n", code6_bits, code6_bits));
            buf.push_str(&format!("    [60:61 保留   ] ——\n"));
            buf.push_str(&format!("    [62     极性(阳=1)] {}\n", polar_bit));
            buf.push_str(&format!("    [63     保留   ] 0\n"));

            // Nested form
            buf.push_str(&format!("  容器嵌套: {}\n", gc.nested));

            // Spousal resonance check (bit38/39 both set)
            if gc.bit38 != 0 && gc.bit39 != 0 {
                let xor = gc.bit38 ^ gc.bit39;
                buf.push_str(&format!("  ◆ bit38=bit39=1 → 夫妻共鸣态 (XOR={})\n", xor));
            }

            buf.push_str("\n");
        } else {
            buf.push_str(&format!("字: {} → (未收录)\n\n", ch));
        }
    }

    // Aggregate cell (phrase-level)
    if input.chars().count() > 1 {
        buf.push_str("═══════════════════════════════════\n");
        buf.push_str("聚合单元格（词语级 SMRL）\n\n");
        let mut agg_cell: u64 = 0;
        let mut any_wife = false;
        let mut any_husband = false;
        let mut all_yang = true;

        for ch in input.chars() {
            if let Ok(gc) = encode_char(ch) {
                for s in &gc.strokes {
                    agg_cell |= 1u64 << *s as u8;
                }
                if gc.bit38 != 0 { any_wife = true; }
                if gc.bit39 != 0 { any_husband = true; }
                if gc.polarity != "阳" { all_yang = false; }
            }
        }

        if any_wife   { agg_cell |= 1u64 << 38; }
        if any_husband { agg_cell |= 1u64 << 39; }
        if all_yang    { agg_cell |= 1u64 << 62; }

        let container_bits = agg_cell & 0xFFFF;
        buf.push_str(&format!("聚合容器掩码: 0x{:04X}\n", container_bits));
        buf.push_str(&format!("妻子聚合:    bit38={}\n", any_wife as u8));
        buf.push_str(&format!("丈夫聚合:    bit39={}\n", any_husband as u8));
        let agg_polar = if (agg_cell >> 62) & 1 == 1 { "阳" } else { "阴" };
        buf.push_str(&format!("聚合极性:    {} (bit62={})\n", agg_polar, (agg_cell >> 62) & 1));
        buf.push_str(&format!("聚合 Cell:   0x{:016X}\n", agg_cell));
        buf.push_str(&format!("  二进制:     {:#066b}\n", agg_cell));

        // Active containers list
        let mut active: Vec<&str> = Vec::new();
        for i in 0..16u8 {
            if (container_bits >> i) & 1 == 1 {
                active.push(CONTAINER_NAMES[i as usize]);
            }
        }
        buf.push_str(&format!("激活容器集: [{}]\n", active.join(", ")));

        if any_wife && any_husband {
            buf.push_str("\n◆ bit38+bit39=11 → 夫妻共鸣激活词\n");
        }
    }

    buf
}

fn encode_json(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("{\n");
    buf.push_str(&format!("  \"version\": \"1.0\",\n"));
    buf.push_str(&format!("  \"encoder\": \"glyph_stroke/radix-hanzi\",\n"));
    buf.push_str(&format!("  \"input\": \"{}\",\n", input));
    buf.push_str("  \"characters\": [\n");

    let mut agg_cell: u64 = 0;
    let mut any_wife = false;
    let mut any_husband = false;
    let char_count = input.chars().count();
    let mut idx = 0;

    for ch in input.chars() {
        if let Ok(gc) = encode_char(ch) {
            let mut cell: u64 = 0;
            let mut stroke_indices: Vec<u8> = Vec::new();
            let mut container_names: Vec<&str> = Vec::new();

            for &s in &gc.strokes {
                cell |= 1u64 << s;
                stroke_indices.push(s);
                if (s as usize) < 16 {
                    container_names.push(CONTAINER_NAMES[s as usize]);
                }
            }
            if gc.bit38 != 0 { cell |= 1u64 << 38; }
            if gc.bit39 != 0 { cell |= 1u64 << 39; }
            if let Some(&max_idx) = gc.strokes.iter().max() { cell |= (max_idx as u64) << 54; }
            if gc.polarity == "阳" { cell |= 1u64 << 62; }

            agg_cell |= cell & 0xFFFF;
            if gc.bit38 != 0 { any_wife = true; }
            if gc.bit39 != 0 { any_husband = true; }

            let side = if gc.bit38 != 0 && gc.bit39 != 0 { "夫妻同在" } else if gc.bit39 != 0 { "丈夫侧" } else { "妻子侧" };
            let strokes_str = stroke_indices.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
            let containers_str = container_names.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");

            buf.push_str("    {\n");
            buf.push_str(&format!("      \"char\": \"{}\",\n", ch));
            buf.push_str(&format!("      \"strokes\": [{}],\n", strokes_str));
            buf.push_str(&format!("      \"containers\": [{}],\n", containers_str));
            buf.push_str(&format!("      \"polarity\": \"{}\",\n", gc.polarity));
            buf.push_str(&format!("      \"side\": \"{}\",\n", side));
            buf.push_str(&format!("      \"display_container\": \"{}\",\n", gc.display_container));
            buf.push_str(&format!("      \"bit38\": {},\n", gc.bit38));
            buf.push_str(&format!("      \"bit39\": {},\n", gc.bit39));
            buf.push_str(&format!("      \"cell_u64\": {},\n", cell));
            buf.push_str(&format!("      \"cell_hex\": \"0x{:016X}\",\n", cell));
            buf.push_str(&format!("      \"code6\": {}\n", gc.code6));
            idx += 1;
            if idx < char_count {
                buf.push_str("    },\n");
            } else {
                buf.push_str("    }\n");
            }
        }
    }

    buf.push_str("  ],\n");

    if any_wife { agg_cell |= 1u64 << 38; }
    if any_husband { agg_cell |= 1u64 << 39; }
    let agg_polar = if (agg_cell >> 62) & 1 == 1 { "阳" } else { "阴" };

    buf.push_str("  \"aggregate\": {\n");
    buf.push_str(&format!("    \"cell_hex\": \"0x{:016X}\",\n", agg_cell));
    buf.push_str(&format!("    \"cell_u64\": {},\n", agg_cell));
    buf.push_str(&format!("    \"container_mask\": \"0x{:04X}\",\n", agg_cell & 0xFFFF));
    buf.push_str(&format!("    \"bit38_wife\": {},\n", any_wife));
    buf.push_str(&format!("    \"bit39_husband\": {},\n", any_husband));
    buf.push_str(&format!("    \"polarity\": \"{}\"\n", agg_polar));
    buf.push_str("  }\n");
    buf.push_str("}\n");

    buf
}

fn encode_fold(input: &str) -> String {
    // Split input into two parts for container fold operation
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        // Try splitting by common delimiters
        let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        if chars.len() >= 2 {
            let mut buf = String::new();
            buf.push_str("═══ 容器叠加运算 ═══\n\n");
            // Pairwise fold
            for window in chars.windows(2) {
                let c1 = window[0];
                let c2 = window[1];
                if let (Ok(gc1), Ok(gc2)) = (encode_char(c1), encode_char(c2)) {
                    buf.push_str(&fold_two(c1, &gc1, c2, &gc2));
                    buf.push_str("\n");
                }
            }
            return buf;
        }
        return "用法: --fold <字1> <字2>\n  或: --折 <字串>（逐对叠加）\n".to_string();
    }

    // Two explicit characters/words
    let word1 = parts[0];
    let word2 = parts[1];

    let mut buf = String::new();
    buf.push_str("═══ 容器叠加运算 ═══\n\n");
    buf.push_str(&format!("前字组: {}\n", word1));
    buf.push_str(&format!("后字组: {}\n\n", word2));

    // Aggregate containers for each word
    let mut cell1: u64 = 0;
    let mut cell2: u64 = 0;
    let mut set1: Vec<&str> = Vec::new();
    let mut set2: Vec<&str> = Vec::new();

    for ch in word1.chars() {
        if let Ok(gc) = encode_char(ch) {
            for &s in &gc.strokes {
                cell1 |= 1u64 << s;
                let name = CONTAINER_NAMES[s as usize];
                if !set1.contains(&name) { set1.push(name); }
            }
        }
    }
    for ch in word2.chars() {
        if let Ok(gc) = encode_char(ch) {
            for &s in &gc.strokes {
                cell2 |= 1u64 << s;
                let name = CONTAINER_NAMES[s as usize];
                if !set2.contains(&name) { set2.push(name); }
            }
        }
    }

    buf.push_str(&format!("前字容器: [{}] (0x{:04X})\n", set1.join(", "), cell1 & 0xFFFF));
    buf.push_str(&format!("后字容器: [{}] (0x{:04X})\n\n", set2.join(", "), cell2 & 0xFFFF));

    // Intersection = 公约数 (shared containers)
    let common = cell1 & cell2;
    // Union after fold: 前字容器装后字容器 (前字壳 = 后字入口)
    let folded = cell1 | cell2;

    let mut common_set: Vec<&str> = Vec::new();
    for i in 0..16u8 {
        if (common >> i) & 1 == 1 { common_set.push(CONTAINER_NAMES[i as usize]); }
    }
    let mut result_set: Vec<&str> = Vec::new();
    for i in 0..16u8 {
        if (folded >> i) & 1 == 1 { result_set.push(CONTAINER_NAMES[i as usize]); }
    }

    buf.push_str(&format!("公约数（共享）: [{}] (0x{:04X})\n", common_set.join(", "), common & 0xFFFF));
    buf.push_str(&format!("叠加结果: [{}] (0x{:04X})\n", result_set.join(", "), folded & 0xFFFF));

    if !common_set.is_empty() {
        buf.push_str(&format!("\n◆ 容器融合: {} 在前字和后字中都存在，可直接共鸣\n", common_set.join("、")));
    }

    // Nested form: 前字最外层装后字最内层
    if let (Some(first), Some(last)) = (set1.first(), set2.last()) {
        buf.push_str(&format!("◆ 容器串联: {}（前字壳） → {}（后字入口）\n", first, last));
    }

    buf
}

fn fold_two(c1: char, gc1: &GlyphCode, c2: char, gc2: &GlyphCode) -> String {
    let mut buf = String::new();
    buf.push_str(&format!("{} + {}: ", c1, c2));
    buf.push_str(&format!("[{}] + [{}] = ", gc1.container_set.join("+"), gc2.container_set.join("+")));

    // Compute combined container mask
    let mut combined: u64 = 0;
    for &s in &gc1.strokes { combined |= 1u64 << s; }
    for &s in &gc2.strokes { combined |= 1u64 << s; }

    let mut result: Vec<&str> = Vec::new();
    for i in 0..16u8 {
        if (combined >> i) & 1 == 1 { result.push(CONTAINER_NAMES[i as usize]); }
    }
    buf.push_str(&format!("[{}] (0x{:04X})", result.join("+"), combined & 0xFFFF));

    // Nesting: first of c1 + last of c2
    if let (Some(first), Some(last)) = (gc1.container_set.first(), gc2.container_set.last()) {
        if first != last {
            buf.push_str(& format!(" → {}装{}", first, last));
        }
    }
    buf
}

fn encode_batch(path: &str) -> String {
    use std::io::Read;
    let mut buf = String::new();
    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return format!("读取失败 {}: {}", path, e),
    };

    buf.push_str(&format!("═══ 批处理: {} ═══\n", path));
    let chars: Vec<char> = content.chars().filter(|c| !c.is_whitespace() && !c.is_ascii()).collect();
    buf.push_str(&format!("汉字数: {}\n\n", chars.len()));

    // Unique characters
    let mut seen = std::collections::HashSet::new();
    let unique: Vec<char> = chars.iter().filter(|c| seen.insert(**c)).copied().collect();
    buf.push_str(&format!("不重复: {} 字\n{}\n", unique.len(), "═".repeat(40)));

    for ch in &unique {
        if let Ok(code) = encode_char(*ch) {
            buf.push_str(&format!("{} [{}]·{:06b}\n", code.ch, code.container_set.join("+"), code.code6));
        }
    }
    buf
}

/// 输出为可嵌入 hunyuan-rs 的 Rust 源码
fn encode_export(input: &str) -> String {
    let mut buf = String::new();
    buf.push_str("// ── 汉字直驱容器编码 — 自动生成的字库常量 ──\n");
    buf.push_str(&format!("// 来源: {}\n", input));
    buf.push_str("// 编译方式: rustc --edition 2021 -o glyph_stroke src/bin/glyph_stroke.rs\n\n");

    buf.push_str("/// 容器编码字库项\n");
    buf.push_str("pub struct GlyphLibEntry {\n");
    buf.push_str("    pub ch: char,\n");
    buf.push_str("    pub container_codes: [u8; 16], // 16笔画位，每字节是容器索引(0-15)\n");
    buf.push_str("    pub total: u8,\n");
    buf.push_str("    pub code6: u8,\n");
    buf.push_str("    pub bit38: u8,\n");
    buf.push_str("    pub bit39: u8,\n");
    buf.push_str("    pub display_container: &'static str,\n");
    buf.push_str("    pub polarity: &'static str,\n");
    buf.push_str("    pub side: &'static str,\n");
    buf.push_str("}\n\n");

    buf.push_str("/// 64位容器编码单元 (参考 couple.rs SoulState::to_radix_cell)\n");
    buf.push_str("pub fn make_cell(entry: &GlyphLibEntry) -> u64 {\n");
    buf.push_str("    let mut cell: u64 = 0;\n");
    buf.push_str("    // bit0-53: 容器掩码 (54位容器空间)\n");
    buf.push_str("    for i in 0..entry.total as usize {\n");
    buf.push_str("        let c = entry.container_codes[i];\n");
    buf.push_str("        cell |= 1u64 << c;\n");
    buf.push_str("    }\n");
    buf.push_str("    // bit38: 妻子在场\n");
    buf.push_str("    cell |= (entry.bit38 as u64) << 38;\n");
    buf.push_str("    // bit39: 丈夫在场\n");
    buf.push_str("    cell |= (entry.bit39 as u64) << 39;\n");
    buf.push_str("    cell\n");
    buf.push_str("}\n\n");

    buf.push_str("#[allow(dead_code)]\n");
    buf.push_str(&format!("pub const GLYPH_{}: [GlyphLibEntry; {}] = [\n", input.chars().take(8).collect::<String>(), input.chars().filter(|c| !c.is_whitespace()).count()));

    for ch in input.chars().filter(|c| !c.is_whitespace()) {
        if let Ok(g) = encode_char(ch) {
            // 容器位数组 (unique, 用 64 容器枚举的真实值)
            let mut used: Vec<u8> = Vec::new();
            for &s in &g.strokes {
                let real = container_radix_value(s);
                if !used.contains(&real) {
                    used.push(real);
                }
            }
            let mut codes = [0u8; 16];
            for (i, &c) in used.iter().enumerate().take(16) {
                codes[i] = c;
            }

            buf.push_str("    GlyphLibEntry {\n");
            buf.push_str(&format!("        ch: '{}',\n", ch));
            buf.push_str(&format!("        container_codes: {:?}, // 16容器枚举hex值\n", codes));
            buf.push_str(&format!("        total: {},\n", g.strokes.len()));
            buf.push_str(&format!("        code6: 0b{:06b},\n", g.code6));
            buf.push_str(&format!("        bit38: {},\n", g.bit38));
            buf.push_str(&format!("        bit39: {},\n", g.bit39));
            buf.push_str(&format!("        display_container: \"{}\",\n", g.display_container));
            buf.push_str(&format!("        polarity: \"{}\",\n", g.polarity));
            buf.push_str(&format!("        side: \"{}\",\n", g.side));
            buf.push_str("    },\n");
        }
    }

    buf.push_str("];\n");

    // 汇入串行
    buf.push_str("\n/// 容器串联 — 前一字套壳 = 后一字内容入口\n");
    buf.push_str("pub fn chain_links() -> &'static [(char, char, &'static str)] {\n");
    buf.push_str("    &[\n");
    let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    for pair in chars.windows(2) {
        if let (Ok(prev), Ok(next)) = (encode_char(pair[0]), encode_char(pair[1])) {
            let next_inner = next.container_set.last().copied().unwrap_or("?");
            buf.push_str(&format!(
                "        ('{}', '{}', \"{}\"), // {}套{} → {}入口={}\n",
                prev.ch, next.ch, prev.display_container,
                prev.ch, prev.display_container, next.ch, next_inner
            ));
        }
    }
    buf.push_str("    ]\n");
    buf.push_str("}\n");

    buf
}

/// 生成自包含可视化 HTML 页面
fn encode_html(input: &str) -> String {
    let mut cards = String::new();
    let mut codes: Vec<GlyphCode> = Vec::new();

    for ch in input.chars().filter(|c| !c.is_whitespace()) {
        match encode_char(ch) {
            Ok(g) => {
                cards.push_str(&fmt_card(&g));
                codes.push(g);
            }
            Err(_) => {
                cards.push_str(&format!(
                    r#"<div class="card unknown"><div class="ch">{}</div><div class="meta">未收录</div></div>"#,
                    ch
                ));
            }
        }
    }

    // 容器串联段落
    let mut chain_html = String::new();
    if codes.len() >= 2 {
        chain_html.push_str(r#"<div class="chain-section"><h2>容器串联</h2><div class="chain-flow">"#);
        for pair in codes.windows(2) {
            let prev = &pair[0];
            let next = &pair[1];
            chain_html.push_str(&format!(
                r#"<div class="chain-step"><span class="from">{}<small>{}</small></span>
<span class="arrow">▶</span><span class="to">{}<small>入口={}</small></span></div>"#,
                prev.ch, prev.display_container, next.ch,
                next.container_set.last().copied().unwrap_or("?")
            ));
        }
        chain_html.push_str("</div></div>");
    }

    format!(r#"<!DOCTYPE html>
<html lang="zh">
<head>
<meta charset="utf-8">
<title>汉字直驱容器编码 · {}</title>
<style>
:root {{
  --bg: #0d1117; --card: #161b22; --border: #30363d;
  --fg: #c9d1d9; --accent: #58a6ff; --yin: #8b949e; --yang: #f2cc60;
  --wife: #ff7b72; --husband: #79c0ff; --neutral: #a5d6ff;
}}
body {{ font-family: monospace; background: var(--bg); color: var(--fg); padding: 24px; max-width: 960px; margin: 0 auto; }}
h1 {{ color: var(--accent); font-size: 20px; margin-bottom: 4px; }}
.subtitle {{ color: var(--yin); font-size: 13px; margin-bottom: 24px; }}
.grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(230px, 1fr)); gap: 12px; margin-bottom: 32px; }}
.card {{ background: var(--card); border: 1px solid var(--border); border-radius: 8px; padding: 14px; }}
.card .ch {{ font-size: 42px; line-height: 1; color: var(--accent); margin-bottom: 8px; }}
.card .meta {{ font-size: 12px; color: var(--fg); opacity: 0.9; line-height: 1.5; }}
.card .strokes {{ color: var(--neutral); font-size: 11px; margin-bottom: 4px; }}
.card .containers {{ color: var(--accent); margin: 4px 0; word-break: break-all; }}
.card .polarity {{ margin-top: 6px; font-size: 12px; }}
.card .polarity .yin {{ color: var(--yin); }}
.card .polarity .yang {{ color: var(--yang); }}
.card .display {{ display: inline-block; padding: 2px 8px; border-radius: 4px; background: rgba(88,166,255,0.15); color: var(--accent); }}
.card .bits {{ font-size: 11px; color: var(--yin); margin-top: 4px; }}
.card .bits .wife {{ color: var(--wife); }}
.card .bits .husband {{ color: var(--husband); }}
.card.unknown {{ opacity: 0.5; }}
.chain-section {{ background: var(--card); border: 1px solid var(--border); border-radius: 8px; padding: 16px; }}
.chain-section h2 {{ margin: 0 0 12px 0; color: var(--accent); font-size: 16px; }}
.chain-flow {{ display: flex; flex-wrap: wrap; align-items: center; gap: 8px; }}
.chain-step {{ display: flex; align-items: center; gap: 6px; font-size: 13px; }}
.chain-step .from, .chain-step .to {{ padding: 4px 10px; background: rgba(88,166,255,0.1); border-radius: 6px; }}
.chain-step .from small, .chain-step .to small {{ display: block; font-size: 10px; opacity: 0.7; text-align: center; }}
.chain-step .arrow {{ color: var(--accent); }}
</style>
</head>
<body>
<h1>汉字直驱容器编码</h1>
<div class="subtitle">来源: {} · 收录 {} 字</div>
<div class="grid">
{}
</div>
{}
</body>
</html>
"#, input, input, codes.len(), cards, chain_html)
}

fn fmt_card(g: &GlyphCode) -> String {
    let polarity_class = if g.polarity == "阴" { "yin" } else { "yang" };
    let bit38_cls = if g.bit38 == 1 { "wife" } else { "" };
    let bit39_cls = if g.bit39 == 1 { "husband" } else { "" };

    format!(
        r#"<div class="card">
  <div class="ch">{}</div>
  <div class="strokes">{}画 · {}</div>
  <div class="containers">{}</div>
  <div class="containers" style="opacity:0.7">→ {}</div>
  <div class="polarity"><span class="{}">{}</span> · {} · 显态 <span class="display">{}</span></div>
  <div class="bits">6bit:{:06b} · {} <span class="{}">妻={}</span> <span class="{}">夫={}</span></div>
</div>"#,
        g.ch,
        g.strokes.len(),
        g.stroke_names.join("→"),
        g.container_set.join(" + "),
        g.nested,
        polarity_class,
        g.polarity,
        g.side,
        g.display_container,
        g.code6,
        "",
        bit38_cls, g.bit38,
        bit39_cls, g.bit39,
    )
}
