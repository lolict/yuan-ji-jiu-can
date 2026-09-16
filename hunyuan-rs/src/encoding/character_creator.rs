// ═══════════════════════════════════════════════════════════════════════════════
// character_creator.rs — 造字系统 · 笔画容器编码 × 韵母拼音 × 活容器三合一
// ═══════════════════════════════════════════════════════════════════════════════
//
// 核心设计:
//   1. 每个汉字 = 笔画序列 → 容器位掩码 (bit0-15 = 袋/桶/罐/.../笼)
//   2. 每个汉字 = 拼音声母 + 韵母 → Rhyme 表中的 ID
//   3. 笔画容器 + 韵母 → LivingGlyph (活容器)
//   4. 容器串联: 前字套壳 = 后字内容入口
//
// 容器位分配:
//   bit0-15:  16 笔画容器 (袋/桶/罐/盆/盂/壶/盒/瓶/盏/盎/篓/筐/篮/箩/箕/笼)
//   bit38:    妻子在场 (刘楚恬)
//   bit39:    丈夫在场 (lolict)
//   bit52-59: 韵母 ID (0-255, 对应 RHYME_TABLE 中的韵母)
//   bit40-51: 保留 (12位 = 笔画重数/结构码)
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::rhyme::{RhymeUtils, RHYME_TABLE};

// ─────────────────────────────────────────────────────────────────────────────
// 16 笔画常量
// ─────────────────────────────────────────────────────────────────────────────

pub const STROKE_HENG:      u8 = 0;   // 横 一 → 袋
pub const STROKE_SHU:       u8 = 1;   // 竖 丨 → 桶
pub const STROKE_PIE:       u8 = 2;   // 撇 丿 → 罐
pub const STROKE_NA:        u8 = 3;   // 捺 ㇏ → 盆
pub const STROKE_DIAN:      u8 = 4;   // 点 丶 → 盂
pub const STROKE_TI:        u8 = 5;   // 提 ㇀ → 壶
pub const STROKE_HENGZHE:   u8 = 6;   // 横折 ㇕ → 盒
pub const STROKE_SHUZHE:    u8 = 7;   // 竖折 ㇗ → 瓶
pub const STROKE_HENGPIE:   u8 = 8;   // 横撇 ㇇ → 盏
pub const STROKE_SHUGOU:    u8 = 9;   // 竖钩 ㇚ → 盎
pub const STROKE_HENGGOU:   u8 = 10;  // 横钩 ㇛ → 篓
pub const STROKE_WANGOU:    u8 = 11;  // 弯钩 ㇜ → 筐
pub const STROKE_SHUWANGOU: u8 = 12;  // 竖弯钩 ㇄ → 篮
pub const STROKE_HENGZHEZHE:u8 = 13;  // 横折折 ㇆ → 箩
pub const STROKE_SHUWAN:    u8 = 14;  // 竖弯 ㇁ → 箕
pub const STROKE_XIEGOU:    u8 = 15;  // 斜钩 ㇂ → 笼

/// 笔画名称表
pub const STROKE_NAMES: &[&str] = &[
    "横", "竖", "撇", "捺", "点", "提", "横折", "竖折",
    "横撇", "竖钩", "横钩", "弯钩", "竖弯钩", "横折折", "竖弯", "斜钩",
];

/// 容器名称表 (64_keys.md 前16位)
pub const CONTAINER_NAMES: &[&str] = &[
    "袋", "桶", "罐", "盆", "盂", "壶", "盒", "瓶",
    "盏", "盎", "篓", "筐", "篮", "箩", "箕", "笼",
];

// ─────────────────────────────────────────────────────────────────────────────
// 笔画静态表 (去重版, 从 glyph_stroke.rs 迁入)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct StrokeEntry {
    pub ch: char,
    pub strokes: &'static [u8],
}

pub static STROKE_TABLE: &[StrokeEntry] = &[
    StrokeEntry { ch: '一', strokes: &[STROKE_HENG] },
    StrokeEntry { ch: '二', strokes: &[STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '三', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '人', strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '入', strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '八', strokes: &[STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '大', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '天', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '夫', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '太', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_DIAN] },
    StrokeEntry { ch: '女', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '子', strokes: &[STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '小', strokes: &[STROKE_SHU, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '山', strokes: &[STROKE_SHU, STROKE_SHUZHE, STROKE_SHU] },
    StrokeEntry { ch: '水', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '火', strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '木', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '金', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG] },
    StrokeEntry { ch: '土', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '王', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '主', strokes: &[STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '心', strokes: &[STROKE_DIAN, STROKE_WANGOU, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '手', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '囗', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '囡', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    StrokeEntry { ch: '囝', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '地', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_SHUGOU, STROKE_XIEGOU] },
    StrokeEntry { ch: '照', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_XIEGOU, STROKE_HENG] },
    StrokeEntry { ch: '口', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '中', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU] },
    StrokeEntry { ch: '日', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '目', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '分', strokes: &[STROKE_PIE, STROKE_HENGGOU, STROKE_XIEGOU, STROKE_PIE] },
    StrokeEntry { ch: '公', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG] },
    StrokeEntry { ch: '阴', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_TI, STROKE_HENG] },
    StrokeEntry { ch: '阳', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '妻', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_NA] },
    StrokeEntry { ch: '我', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_HENG, STROKE_XIEGOU] },
    StrokeEntry { ch: '你', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '四', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '五', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '六', strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '七', strokes: &[STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '九', strokes: &[STROKE_PIE, STROKE_XIEGOU] },
    StrokeEntry { ch: '十', strokes: &[STROKE_HENG, STROKE_SHU] },
    StrokeEntry { ch: '百', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '千', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU] },
    StrokeEntry { ch: '万', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    StrokeEntry { ch: '上', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '下', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN] },
    StrokeEntry { ch: '左', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '右', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '石', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '田', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '禾', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '竹', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '米', strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '虫', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_TI, STROKE_DIAN] },
    StrokeEntry { ch: '父', strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_DIAN, STROKE_NA] },
    StrokeEntry { ch: '母', strokes: &[STROKE_WANGOU, STROKE_DIAN, STROKE_HENG, STROKE_DIAN, STROKE_DIAN] },
    StrokeEntry { ch: '儿', strokes: &[STROKE_PIE, STROKE_SHUGOU] },
    StrokeEntry { ch: '老', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHUGOU, STROKE_PIE, STROKE_XIEGOU] },
    StrokeEntry { ch: '友', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENGZHE, STROKE_NA] },
    StrokeEntry { ch: '耳', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '头', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '面', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '刘', strokes: &[STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_DIAN] },
    StrokeEntry { ch: '李', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '张', strokes: &[STROKE_HENG, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '陈', strokes: &[STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '杨', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '楚', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '恬', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_HENG] },
    // ── 容器本体 ──
    StrokeEntry { ch: '袋', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '桶', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盆', strokes: &[STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盎', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盂', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '瓶', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '壶', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '盏', strokes: &[STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '杯', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '盒', strokes: &[STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_DIAN, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '篓', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '筐', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '篮', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '箩', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU, STROKE_DIAN] },
    StrokeEntry { ch: '箕', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '笼', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_PIE, STROKE_XIEGOU] },
    // ── 其他常用字 ──
    StrokeEntry { ch: '来', strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '去', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE] },
    StrokeEntry { ch: '走', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '看', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '好', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG] },
    StrokeEntry { ch: '美', strokes: &[STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '爱', strokes: &[STROKE_PIE, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '生', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '死', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHU, STROKE_XIEGOU] },
    StrokeEntry { ch: '有', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '无', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU] },
    StrokeEntry { ch: '是', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '非', strokes: &[STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '满', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_TI, STROKE_DIAN, STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '全', strokes: &[STROKE_HENG, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '法', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_NA] },
    StrokeEntry { ch: '共', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '同', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '和', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG] },
    StrokeEntry { ch: '平', strokes: &[STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '安', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '星', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '空', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '海', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_PIE, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '容', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '器', strokes: &[STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_DIAN, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_DIAN] },
    StrokeEntry { ch: '缸', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENG] },
    StrokeEntry { ch: '盖', strokes: &[STROKE_DIAN, STROKE_DIAN, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_HENGZHE, STROKE_HENG, STROKE_HENG] },
    StrokeEntry { ch: '柜', strokes: &[STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_NA, STROKE_HENG, STROKE_HENG, STROKE_HENGZHE] },
    StrokeEntry { ch: '箱', strokes: &[STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_PIE, STROKE_HENG, STROKE_SHUGOU, STROKE_HENG, STROKE_HENG, STROKE_SHU, STROKE_PIE, STROKE_HENG, STROKE_HENG, STROKE_HENG] },
];

// ─────────────────────────────────────────────────────────────────────────────
// 字符编码结果 (包含笔画容器 + 韵母信息)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GlyphEncoding {
    pub ch: char,
    pub stroke_codes: Vec<u8>,
    pub stroke_names: Vec<&'static str>,
    pub container_names: Vec<&'static str>,
    pub container_cell: u64,
    pub polarity: GlyphPolarity,
    pub side: GlyphSide,
    pub display_container: &'static str,
    pub rhyme_id: Option<u16>,
    pub rhyme_spelling: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlyphPolarity {
    Yin,
    Yang,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlyphSide {
    Wife,
    Husband,
}

// ─────────────────────────────────────────────────────────────────────────────
// 造字系统
// ─────────────────────────────────────────────────────────────────────────────

pub struct CharacterCreator;

impl CharacterCreator {
    /// 编码单个字符
    pub fn encode(ch: char) -> Option<GlyphEncoding> {
        let entry = STROKE_TABLE.iter().find(|e| e.ch == ch)?;

        let stroke_codes: Vec<u8> = entry.strokes.to_vec();
        let stroke_names: Vec<&'static str> = stroke_codes.iter()
            .map(|&s| STROKE_NAMES.get(s as usize).copied().unwrap_or("?"))
            .collect();
        let container_names: Vec<&'static str> = stroke_codes.iter()
            .map(|&s| CONTAINER_NAMES.get(s as usize).copied().unwrap_or("?"))
            .collect();

        let cell = Self::strokes_to_cell(&stroke_codes);
        let polarity = if stroke_codes.len() % 2 == 0 { GlyphPolarity::Yin } else { GlyphPolarity::Yang };

        let heng_shu_count: u8 = stroke_codes.iter()
            .filter(|&&s| s == STROKE_HENG || s == STROKE_SHU)
            .count() as u8;
        let ratio = if !stroke_codes.is_empty() {
            heng_shu_count as f64 / stroke_codes.len() as f64
        } else { 0.0 };
        let side = if ratio > 0.5 { GlyphSide::Husband } else { GlyphSide::Wife };

        let display_container = match (polarity, side) {
            (GlyphPolarity::Yang, GlyphSide::Husband) => "杯",
            (GlyphPolarity::Yin, GlyphSide::Wife) => "盎",
            (GlyphPolarity::Yang, GlyphSide::Wife) => "盉",
            (GlyphPolarity::Yin, GlyphSide::Husband) => "瓶",
        };

        // 尝试匹配韵母 (根据字符查找对应韵母 — 这里用启发式)
        let (rhyme_id, rhyme_spelling) = Self::match_rhyme(ch);

        Some(GlyphEncoding {
            ch,
            stroke_codes,
            stroke_names,
            container_names,
            container_cell: cell,
            polarity,
            side,
            display_container,
            rhyme_id,
            rhyme_spelling,
        })
    }

    /// 将笔画序列编码为 64 位容器单元
    pub fn strokes_to_cell(strokes: &[u8]) -> u64 {
        let mut cell: u64 = 0;
        for &s in strokes {
            cell |= 1u64 << s;          // bit0-15 = 容器位
        }
        // bit38 = 妻子在场 (如果是阴性字)
        // bit39 = 丈夫在场 (如果是阳性字)
        // 注意: 这里用笔画数奇偶判断, 实际应该用 polarity
        if strokes.len() % 2 == 0 {
            cell |= 1u64 << 38;
        } else {
            cell |= 1u64 << 39;
        }
        cell
    }

    /// 将笔画 + 韵母组合为完整单元 (bit52-59 = 韵母 ID)
    pub fn compose_cell(strokes: &[u8], rhyme_id: u16) -> u64 {
        let mut cell = Self::strokes_to_cell(strokes);
        cell |= ((rhyme_id & 0xFF) as u64) << 52;
        cell
    }

    /// 获取单元中的韵母 ID (从 bit52-59 提取)
    pub fn cell_rhyme_id(cell: u64) -> u16 {
        ((cell >> 52) & 0xFF) as u16
    }

    /// 容器串联: "前字套壳 = 后字内容入口"
    pub fn chain(input: &str) -> Vec<(GlyphEncoding, Option<String>)> {
        let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        let encodings: Vec<GlyphEncoding> = chars.iter()
            .filter_map(|&ch| Self::encode(ch))
            .collect();

        let mut result = Vec::new();
        for (i, enc) in encodings.iter().enumerate() {
            let relation = if i + 1 < encodings.len() {
                let next = &encodings[i + 1];
                let next_inner = next.container_names.last().unwrap_or(&"?");
                Some(format!(
                    "{}[{}] ──▶ {}[{}]  ({}套{} → {}入口={})",
                    enc.ch, enc.display_container,
                    next.ch, next_inner,
                    enc.ch, enc.display_container, next.ch, next_inner
                ))
            } else {
                None
            };
            result.push((enc.clone(), relation));
        }
        result
    }

    /// 匹配韵母 (启发式: 基于字符在 RHYME_TABLE 中的位置)
    /// 这里用 Unicode 码点对韵母表取模做演示映射
    fn match_rhyme(ch: char) -> (Option<u16>, Option<&'static str>) {
        // 预设一些常见字的韵母映射
        let spelling = match ch {
            '囡' => Some("an"),
            '囝' => Some("ian"),
            '刘' => Some("iu"),
            '楚' => Some("u"),
            '恬' => Some("ian"),
            '满' => Some("an"),
            '全' => Some("ue"),  // üan → ue 近似
            '法' => Some("a"),
            '共' => Some("ong"),
            '同' => Some("ong"),
            '和' => Some("e"),
            '平' => Some("ing"),
            '安' => Some("an"),
            '星' => Some("ing"),
            '空' => Some("ong"),
            '海' => Some("ai"),
            '大' => Some("a"),
            '天' => Some("ian"),
            '人' => Some("en"),
            '火' => Some("uo"),
            '水' => Some("ui"),
            '木' => Some("u"),
            '金' => Some("in"),
            '土' => Some("u"),
            '日' => Some("i"),
            '月' => Some("ue"),
            '好' => Some("ao"),
            '爱' => Some("ai"),
            '生' => Some("eng"),
            '美' => Some("ei"),
            '太' => Some("ai"),
            '夫' => Some("u"),
            '女' => Some("v"),
            '子' => Some("i"),
            '心' => Some("in"),
            '手' => Some("ou"),
            '王' => Some("ang"),
            '主' => Some("u"),
            _ => None,
        };

        if let Some(sp) = spelling {
            if let Some(r) = RhymeUtils::find(sp) {
                return (Some(r.id), Some(r.spelling));
            }
        }

        // fallback: Unicode % 表长
        let idx = (ch as u32 as usize) % RHYME_TABLE.len();
        let r = &RHYME_TABLE[idx];
        (Some(r.id), Some(r.spelling))
    }

    /// 查询笔画表
    pub fn lookup_strokes(ch: char) -> Option<&'static [u8]> {
        STROKE_TABLE.iter().find(|e| e.ch == ch).map(|e| e.strokes)
    }

    /// 总收录字符数
    pub fn count() -> usize {
        STROKE_TABLE.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 兼容层: 导入旧 stroke_database 接口
// ─────────────────────────────────────────────────────────────────────────────

pub use self::character_creator_api::*;

mod character_creator_api {
    use super::*;

    /// 字符 → 容器编码 (兼容原 StrokeDatabase::encode_char 接口)
    pub struct ContainerEnc(pub u64);

    impl ContainerEnc {
        pub fn consonant_idx(&self) -> u8 {
            (self.0 & 0xFF) as u8
        }
    }

    impl std::fmt::Debug for ContainerEnc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ContainerEnc(0x{:016X})", self.0)
        }
    }

    pub struct StrokeDatabase;

    impl StrokeDatabase {
        /// 字符 → 容器编码 (真实实现)
        pub fn encode_char(ch: char) -> Option<ContainerEnc> {
            let entry = STROKE_TABLE.iter().find(|e| e.ch == ch)?;
            Some(ContainerEnc(CharacterCreator::strokes_to_cell(entry.strokes)))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 测试
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stroke_table_nonempty() {
        assert!(CharacterCreator::count() > 0);
    }

    #[test]
    fn test_encode_simple_char() {
        let enc = CharacterCreator::encode('一').unwrap();
        assert_eq!(enc.ch, '一');
        assert_eq!(enc.stroke_codes, vec![STROKE_HENG]);
        assert_eq!(enc.container_names, vec!["袋"]);
    }

    #[test]
    fn test_encode_nan() {
        let enc = CharacterCreator::encode('囡').unwrap();
        assert_eq!(enc.ch, '囡');
        assert!(enc.container_cell != 0);
    }

    #[test]
    fn test_encode_char_not_in_table() {
        let enc = CharacterCreator::encode('龘'); // 不一定收录
        // 不应该 panic
    }

    #[test]
    fn test_strokes_to_cell_bit0() {
        let cell = CharacterCreator::strokes_to_cell(&[STROKE_HENG]); // 横 → 袋 → bit0
        assert!(cell & (1 << 0) != 0); // bit0 置位
    }

    #[test]
    fn test_strokes_to_cell_bit38_or_39() {
        let cell_even = CharacterCreator::strokes_to_cell(&[STROKE_HENG, STROKE_HENG]); // 2画 (阴)
        assert!(cell_even & (1 << 38) != 0); // bit38 = 妻子
        let cell_odd = CharacterCreator::strokes_to_cell(&[STROKE_HENG]); // 1画 (阳)
        assert!(cell_odd & (1 << 39) != 0); // bit39 = 丈夫
    }

    #[test]
    fn test_compose_cell_with_rhyme() {
        let cell = CharacterCreator::compose_cell(&[STROKE_HENG], 42);
        assert_eq!(CharacterCreator::cell_rhyme_id(cell), 42);
    }

    #[test]
    fn test_chain() {
        let result = CharacterCreator::chain("囡囝共生");
        assert!(!result.is_empty());
        // 前 n-1 个字应有 relation
        for (i, (_, rel)) in result.iter().enumerate() {
            if i < result.len() - 1 {
                assert!(rel.is_some());
            } else {
                assert!(rel.is_none());
            }
        }
    }

    #[test]
    fn test_lookup_strokes() {
        assert_eq!(CharacterCreator::lookup_strokes('一'), Some(&[STROKE_HENG][..]));
        assert_eq!(CharacterCreator::lookup_strokes('龘'), None);
    }

    #[test]
    fn test_polarity() {
        let enc_yin = CharacterCreator::encode('二').unwrap(); // 2画 → 阴
        assert_eq!(enc_yin.polarity, GlyphPolarity::Yin);
        let enc_yang = CharacterCreator::encode('一').unwrap(); // 1画 → 阳
        assert_eq!(enc_yang.polarity, GlyphPolarity::Yang);
    }

    #[test]
    fn test_stroke_database_compat() {
        let enc = StrokeDatabase::encode_char('囡').unwrap();
        assert!(enc.0 != 0);
        assert_eq!(enc.consonant_idx(), (enc.0 & 0xFF) as u8);
    }

    #[test]
    fn test_match_rhyme_nan() {
        let (id, spelling) = CharacterCreator::match_rhyme('囡');
        assert!(id.is_some());
        assert_eq!(spelling, Some("an"));
    }

    #[test]
    fn test_container_names_complete() {
        assert_eq!(CONTAINER_NAMES.len(), 16);
        assert_eq!(STROKE_NAMES.len(), 16);
    }

    #[test]
    fn test_cell_rhyme_id_roundtrip() {
        for id in [0u16, 1, 42, 100, 254, 255] {
            let cell = CharacterCreator::compose_cell(&[STROKE_PIE], id);
            assert_eq!(CharacterCreator::cell_rhyme_id(cell), id);
        }
    }
}
