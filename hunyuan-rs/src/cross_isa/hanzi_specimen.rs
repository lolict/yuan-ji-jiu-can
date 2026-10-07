// hanzi_specimen.rs — 汉字容器编码视觉样张渲染器
//
// 渲染效果：每个汉字一张大卡片
//   - 左侧大字（基于笔画的线段绘制）
//   - 顶部状态分类色条 + 类名
//   - 笔画 → 容器映射表
//   - 6-bit 二进制编码 + 群轨道分类
//   - 极性 + 显示容器
// ══════════════════════════════════════════════════════════════════════

use image::{Rgb, RgbImage, imageops};
use std::f64::consts::PI;

use super::state_enumerator::{StateEnumerator, StateCategory, Transform};

// 笔画类型常量
const HENG: u8 = 0;
const SHU: u8 = 1;
const PIE: u8 = 2;
const NA: u8 = 3;
const DIAN: u8 = 4;
const TI: u8 = 5;
const HENGZHE: u8 = 6;
const SHUZHE: u8 = 7;
const HENGPIE: u8 = 8;
const SHUGOU: u8 = 9;
const HENGGOU: u8 = 10;
const WANGOU: u8 = 11;
const SHUWANGOU: u8 = 12;
const HENGZHEZHE: u8 = 13;
const SHUWAN: u8 = 14;
const XIEGOU: u8 = 15;

pub struct HanziSpecimen {
    pub ch: char,
    pub strokes: Vec<&'static str>,
    pub container_set: Vec<&'static str>,
    pub nested: String,
    pub polarity: &'static str,
    pub display_container: &'static str,
    pub code6: u8,
    pub bit_width: u8,
    pub category: StateCategory,
    pub category_color: Rgb<u8>,
    pub orbit_len: u8,
}

const CONTAINER_COLOR: Rgb<u8> = Rgb([90, 160, 240]);

impl HanziSpecimen {
    pub fn category_name(&self) -> &'static str {
        match self.category {
            StateCategory::AxialContainer => "轴心容器态",
            StateCategory::ClosedLoop => "闭环态",
            StateCategory::MirrorEntangled => "镜像纠缠态",
            StateCategory::ExtendedEntangled => "扩展纠缠态",
        }
    }
    pub fn category_code(&self) -> &'static str {
        match self.category {
            StateCategory::AxialContainer => "AXIS",
            StateCategory::ClosedLoop => "LOOP",
            StateCategory::MirrorEntangled => "MIRROR",
            StateCategory::ExtendedEntangled => "EXT",
        }
    }
}

// 笔画定义：类型 + 起点(0-100) + 终点(0-100)，在 100x100 的田字格里
struct StrokeDef { typ: u8, x0: i32, y0: i32, x1: i32, y1: i32, cx: i32, cy: i32 }

fn stroke_defs(ch: char) -> Option<Vec<StrokeDef>> {
    let strokes: Vec<(u8, i32, i32, i32, i32, i32, i32)> = match ch {
        // 横: (typ, x0, y0, x1, y1, cx, cy) cx/cy 为曲枢点
        '一' => vec![(HENG, 10, 50, 90, 50, 0, 0)],
        '二' => vec![(HENG, 15, 30, 85, 30, 0, 0), (HENG, 10, 70, 90, 70, 0, 0)],
        '三' => vec![(HENG, 20, 25, 80, 25, 0, 0), (HENG, 15, 50, 85, 50, 0, 0), (HENG, 10, 75, 90, 75, 0, 0)],
        '人' => vec![(PIE, 50, 15, 20, 90, 0, 0), (NA, 50, 15, 80, 90, 0, 0)],
        '大' => vec![(HENG, 20, 35, 80, 35, 0, 0), (PIE, 50, 15, 18, 90, 0, 0), (NA, 50, 15, 82, 90, 0, 0)],
        '夫' => vec![(HENG, 25, 20, 75, 20, 0, 0), (HENG, 20, 40, 80, 40, 0, 0), (PIE, 50, 10, 15, 90, 0, 0), (NA, 50, 10, 85, 90, 0, 0)],
        '天' => vec![(HENG, 25, 20, 75, 20, 0, 0), (HENG, 20, 38, 80, 38, 0, 0), (PIE, 50, 12, 18, 92, 0, 0), (NA, 50, 12, 82, 92, 0, 0)],
        '太' => vec![(HENG, 25, 30, 75, 30, 0, 0), (PIE, 50, 15, 20, 90, 0, 0), (NA, 50, 15, 80, 90, 0, 0), (DIAN, 55, 72, 60, 78, 0, 0)],
        '女' => vec![(PIE, 30, 25, 50, 55, 0, 0), (HENG, 20, 50, 80, 50, 0, 0), (HENG, 25, 72, 75, 72, 0, 0)],
        '子' => vec![(HENGGOU, 25, 30, 50, 30, 45, 45), (SHUGOU, 50, 30, 50, 70, 42, 65), (HENG, 20, 78, 80, 78, 0, 0)],
        '小' => vec![(SHU, 50, 15, 50, 55, 0, 0), (PIE, 50, 40, 20, 90, 0, 0), (DIAN, 50, 40, 80, 90, 0, 0)],
        '山' => vec![(SHU, 50, 15, 50, 90, 0, 0), (SHUZHE, 20, 45, 50, 55, 50, 90), (SHUZHE, 80, 45, 50, 55, 50, 90)],
        '火' => vec![(DIAN, 40, 25, 38, 35, 0, 0), (PIE, 50, 15, 20, 75, 0, 0), (PIE, 50, 15, 80, 75, 0, 0), (NA, 50, 40, 70, 88, 0, 0)],
        '木' => vec![(HENG, 20, 40, 80, 40, 0, 0), (SHU, 50, 15, 50, 88, 0, 0), (PIE, 50, 40, 18, 90, 0, 0), (NA, 50, 40, 82, 90, 0, 0)],
        '土' => vec![(HENG, 25, 45, 75, 45, 0, 0), (SHU, 50, 18, 50, 88, 0, 0), (HENG, 18, 82, 82, 82, 0, 0)],
        '王' => vec![(HENG, 20, 28, 80, 28, 0, 0), (HENG, 18, 52, 82, 52, 0, 0), (SHU, 50, 18, 50, 85, 0, 0), (HENG, 15, 82, 85, 82, 0, 0)],
        '主' => vec![(DIAN, 50, 18, 52, 22, 0, 0), (HENG, 22, 30, 78, 30, 0, 0), (HENG, 20, 52, 80, 52, 0, 0), (SHU, 50, 22, 50, 88, 0, 0), (HENG, 18, 85, 82, 85, 0, 0)],
        '金' => vec![(PIE, 50, 15, 35, 18, 0, 0), (PIE, 50, 15, 65, 18, 0, 0), (HENG, 22, 30, 78, 30, 0, 0), (HENG, 20, 48, 80, 48, 0, 0), (SHU, 30, 32, 30, 68, 0, 0), (SHU, 70, 32, 70, 68, 0, 0), (HENG, 18, 78, 82, 78, 0, 0)],
        '日' => vec![(SHU, 30, 18, 30, 82, 0, 0), (HENGZHE, 30, 18, 70, 18, 70, 82), (HENG, 30, 82, 70, 82, 0, 0), (HENG, 30, 50, 70, 50, 0, 0)],
        '月' => vec![(PIE, 30, 18, 30, 82, 0, 0), (HENGZHE, 30, 18, 70, 18, 70, 82), (HENG, 30, 48, 70, 48, 0, 0), (HENG, 30, 82, 70, 82, 0, 0)],
        '目' => vec![(SHU, 28, 15, 28, 85, 0, 0), (HENGZHE, 28, 15, 72, 15, 72, 85), (HENG, 28, 85, 72, 85, 0, 0), (HENG, 28, 38, 72, 38, 0, 0), (HENG, 28, 62, 72, 62, 0, 0)],
        '口' => vec![(SHU, 32, 22, 32, 78, 0, 0), (HENGZHE, 32, 22, 68, 22, 68, 78), (HENG, 32, 78, 68, 78, 0, 0)],
        '囗' => vec![(SHU, 20, 15, 20, 85, 0, 0), (HENGZHE, 20, 15, 80, 15, 80, 85), (HENG, 20, 85, 80, 85, 0, 0)],
        '囡' => vec![(SHU, 22, 15, 22, 85, 0, 0), (HENGZHE, 22, 15, 78, 15, 78, 85), (HENG, 22, 85, 78, 85, 0, 0), (HENG, 28, 42, 72, 42, 0, 0), (PIE, 50, 42, 30, 80, 0, 0), (HENG, 28, 82, 72, 82, 0, 0)],
        '囝' => vec![(SHU, 22, 15, 22, 85, 0, 0), (HENGZHE, 22, 15, 78, 15, 78, 85), (HENG, 22, 85, 78, 85, 0, 0), (SHU, 50, 40, 50, 65, 0, 0), (HENG, 28, 65, 72, 65, 0, 0), (SHUGOU, 50, 65, 60, 82, 52, 78)],
        '田' => vec![(SHU, 28, 18, 28, 82, 0, 0), (HENGZHE, 28, 18, 72, 18, 72, 82), (HENG, 28, 82, 72, 82, 0, 0), (SHU, 50, 18, 50, 82, 0, 0), (HENG, 28, 50, 72, 50, 0, 0)],
        '中' => vec![(SHU, 30, 20, 30, 80, 0, 0), (HENGZHE, 30, 20, 70, 20, 70, 80), (HENG, 30, 80, 70, 80, 0, 0), (SHU, 50, 10, 50, 92, 0, 0)],
        '共' => vec![(HENG, 22, 25, 78, 25, 0, 0), (SHU, 35, 25, 35, 65, 0, 0), (PIE, 35, 45, 18, 80, 0, 0), (NA, 35, 45, 78, 80, 0, 0), (HENG, 20, 78, 80, 78, 0, 0), (DIAN, 75, 25, 78, 30, 0, 0)],
        '同' => vec![(SHU, 28, 15, 28, 85, 0, 0), (HENGZHE, 28, 15, 72, 15, 72, 85), (HENG, 28, 85, 72, 85, 0, 0), (SHU, 42, 30, 42, 70, 0, 0), (HENGZHE, 42, 30, 62, 30, 62, 70), (HENG, 42, 70, 62, 70, 0, 0)],
        '生' => vec![(PIE, 50, 18, 32, 25, 0, 0), (HENG, 28, 42, 75, 42, 0, 0), (SHU, 50, 42, 50, 88, 0, 0), (HENG, 25, 65, 78, 65, 0, 0), (HENG, 20, 88, 80, 88, 0, 0)],
        '和' => vec![(PIE, 38, 18, 28, 25, 0, 0), (HENG, 22, 35, 52, 35, 0, 0), (SHU, 38, 32, 38, 72, 0, 0), (DIAN, 20, 72, 35, 72, 0, 0), (SHU, 62, 22, 62, 78, 0, 0), (HENGZHE, 62, 22, 78, 22, 78, 78), (DIAN, 62, 50, 78, 50, 0, 0)],
        '平' => vec![(HENG, 20, 20, 80, 20, 0, 0), (DIAN, 50, 20, 50, 30, 0, 0), (PIE, 50, 30, 20, 80, 0, 0), (SHU, 50, 30, 50, 90, 0, 0), (HENG, 18, 90, 82, 90, 0, 0)],
        '安' => vec![(DIAN, 50, 15, 52, 18, 0, 0), (HENG, 25, 28, 75, 28, 0, 0), (PIE, 50, 28, 30, 75, 0, 0), (HENG, 25, 75, 75, 75, 0, 0), (HENG, 20, 90, 80, 90, 0, 0)],
        '满' => vec![(HENG, 15, 28, 45, 28, 0, 0), (SHU, 28, 18, 28, 78, 0, 0), (TI, 28, 55, 38, 52, 0, 0), (DIAN, 38, 28, 42, 32, 0, 0), (PIE, 65, 28, 55, 40, 0, 0), (NA, 65, 28, 82, 48, 0, 0), (HENG, 55, 48, 82, 48, 0, 0), (DIAN, 55, 48, 60, 54, 0, 0), (HENG, 55, 65, 82, 65, 0, 0), (HENG, 55, 82, 82, 82, 0, 0)],
        '全' => vec![(PIE, 50, 12, 35, 15, 0, 0), (PIE, 50, 12, 65, 15, 0, 0), (HENG, 22, 28, 78, 28, 0, 0), (HENG, 20, 52, 80, 52, 0, 0), (SHU, 50, 28, 50, 90, 0, 0), (HENG, 18, 90, 82, 90, 0, 0)],
        '法' => vec![(HENG, 15, 30, 38, 30, 0, 0), (SHU, 25, 18, 25, 82, 0, 0), (HENG, 15, 82, 38, 82, 0, 0), (DIAN, 60, 18, 62, 22, 0, 0), (HENG, 55, 32, 80, 32, 0, 0), (PIE, 68, 32, 52, 78, 0, 0), (NA, 68, 32, 82, 82, 0, 0)],
        '合' => vec![(PIE, 50, 12, 35, 15, 0, 0), (PIE, 50, 12, 65, 15, 0, 0), (HENG, 22, 25, 78, 25, 0, 0), (SHUZHE, 28, 25, 28, 75, 78, 75), (HENG, 28, 75, 78, 75, 0, 0)],
        '妻' => vec![(HENG, 20, 18, 80, 18, 0, 0), (SHU, 50, 18, 50, 42, 0, 0), (HENG, 25, 42, 75, 42, 0, 0), (SHU, 50, 42, 50, 65, 0, 0), (HENG, 25, 65, 75, 65, 0, 0), (SHU, 50, 65, 50, 88, 0, 0), (HENG, 22, 88, 78, 88, 0, 0), (NA, 50, 88, 78, 92, 0, 0)],
        '爱' => vec![(DIAN, 35, 15, 38, 18, 0, 0), (DIAN, 65, 15, 62, 18, 0, 0), (HENGZHE, 38, 18, 62, 18, 52, 30), (DIAN, 50, 30, 52, 33, 0, 0), (HENG, 25, 42, 75, 42, 0, 0), (DIAN, 50, 42, 52, 45, 0, 0), (HENG, 30, 55, 70, 55, 0, 0), (PIE, 50, 55, 25, 90, 0, 0), (HENGZHE, 50, 55, 75, 55, 75, 90), (PIE, 75, 55, 85, 90, 0, 0), (NA, 75, 55, 60, 90, 0, 0)],
        '婚' => vec![(PIE, 30, 15, 30, 85, 0, 0), (HENGZHE, 30, 15, 50, 15, 50, 85), (HENG, 30, 85, 50, 85, 0, 0), (HENG, 30, 50, 50, 50, 0, 0), (PIE, 50, 15, 42, 50, 0, 0), (HENG, 50, 15, 68, 15, 0, 0), (PIE, 50, 15, 62, 50, 0, 0), (HENG, 58, 50, 85, 50, 0, 0), (SHU, 70, 50, 70, 85, 0, 0), (HENGZHE, 70, 50, 85, 50, 85, 85), (HENG, 70, 85, 85, 85, 0, 0), (HENG, 58, 85, 85, 85, 0, 0)],
        '缘' => vec![(PIE, 28, 20, 45, 25, 0, 0), (HENGZHE, 28, 20, 50, 20, 50, 35), (PIE, 50, 20, 35, 55, 0, 0), (HENG, 25, 55, 55, 55, 0, 0), (PIE, 40, 55, 55, 85, 0, 0), (NA, 40, 55, 62, 92, 0, 0), (PIE, 55, 40, 48, 70, 0, 0), (HENG, 48, 70, 70, 70, 0, 0), (PIE, 62, 70, 55, 92, 0, 0), (HENG, 55, 82, 72, 82, 0, 0), (SHU, 68, 60, 68, 92, 0, 0)],
        '心' => vec![(DIAN, 25, 65, 30, 72, 0, 0), (WANGOU, 40, 48, 35, 30, 50, 55), (DIAN, 55, 28, 60, 35, 0, 0), (DIAN, 68, 55, 72, 62, 0, 0)],
        '念' => vec![(HENG, 25, 22, 38, 22, 0, 0), (SHU, 30, 22, 32, 42, 0, 0), (PIE, 25, 32, 15, 52, 0, 0), (DIAN, 38, 22, 42, 28, 0, 0), (SHUWANGOU, 50, 22, 45, 35, 55, 42), (DIAN, 42, 42, 48, 48, 0, 0), (DIAN, 62, 22, 68, 28, 0, 0), (HENG, 58, 28, 75, 28, 0, 0), (PIE, 68, 28, 52, 72, 0, 0), (DIAN, 75, 28, 80, 32, 0, 0)],
        '想' => vec![(HENG, 22, 22, 45, 22, 0, 0), (SHU, 28, 22, 28, 55, 0, 0), (PIE, 28, 22, 18, 45, 0, 0), (DIAN, 35, 22, 40, 28, 0, 0), (HENG, 20, 45, 45, 45, 0, 0), (PIE, 20, 45, 12, 65, 0, 0), (HENG, 12, 65, 42, 65, 0, 0), (DIAN, 25, 65, 30, 70, 0, 0), (DIAN, 50, 22, 52, 28, 0, 0), (HENG, 52, 28, 68, 28, 0, 0), (PIE, 52, 28, 42, 65, 0, 0), (DIAN, 62, 28, 68, 32, 0, 0)],
        '意' => vec![(DIAN, 45, 12, 48, 15, 0, 0), (HENG, 22, 22, 78, 22, 0, 0), (PIE, 50, 12, 25, 32, 0, 0), (NA, 50, 12, 75, 32, 0, 0), (HENG, 20, 32, 80, 32, 0, 0), (SHU, 50, 25, 50, 55, 0, 0), (HENG, 20, 55, 80, 55, 0, 0), (SHU, 50, 55, 50, 72, 0, 0), (DIAN, 35, 72, 40, 78, 0, 0), (DIAN, 50, 72, 55, 78, 0, 0), (HENGZHE, 50, 72, 65, 72, 58, 85), (DIAN, 55, 85, 60, 90, 0, 0)],
        '思' => vec![(SHU, 30, 18, 30, 65, 0, 0), (HENGZHE, 30, 18, 68, 18, 68, 65), (HENG, 30, 65, 68, 65, 0, 0), (HENG, 30, 42, 68, 42, 0, 0), (PIE, 30, 42, 20, 60, 0, 0), (NA, 30, 42, 55, 60, 0, 0), (DIAN, 50, 60, 55, 65, 0, 0), (SHUWANGOU, 30, 65, 25, 80, 38, 88), (DIAN, 42, 78, 48, 85, 0, 0), (DIAN, 58, 78, 62, 85, 0, 0)],
        '情' => vec![(DIAN, 28, 20, 32, 25, 0, 0), (DIAN, 28, 30, 32, 35, 0, 0), (SHU, 32, 18, 32, 78, 0, 0), (HENG, 18, 48, 38, 48, 0, 0), (SHU, 58, 18, 58, 78, 0, 0), (HENG, 52, 32, 78, 32, 0, 0), (HENG, 52, 50, 78, 50, 0, 0), (HENG, 52, 68, 78, 68, 0, 0), (PIE, 70, 18, 60, 55, 0, 0), (HENG, 58, 55, 82, 55, 0, 0)],
        '星' => vec![(SHU, 35, 12, 35, 55, 0, 0), (HENGZHE, 35, 12, 65, 12, 65, 55), (HENG, 35, 55, 65, 55, 0, 0), (HENG, 35, 32, 65, 32, 0, 0), (PIE, 50, 55, 30, 92, 0, 0), (HENG, 20, 72, 50, 72, 0, 0), (SHU, 50, 72, 50, 92, 0, 0), (NA, 50, 72, 78, 92, 0, 0)],
        '空' => vec![(DIAN, 50, 12, 52, 15, 0, 0), (HENG, 30, 20, 70, 20, 0, 0), (HENG, 22, 32, 78, 32, 0, 0), (SHU, 50, 32, 50, 65, 0, 0), (HENG, 20, 65, 80, 65, 0, 0), (SHU, 30, 65, 30, 92, 0, 0), (HENG, 30, 92, 70, 92, 0, 0), (SHU, 70, 65, 70, 92, 0, 0)],
        '宇' => vec![(DIAN, 50, 12, 52, 15, 0, 0), (HENG, 28, 18, 72, 18, 0, 0), (SHU, 50, 18, 50, 52, 0, 0), (HENG, 25, 52, 75, 52, 0, 0), (SHUZHE, 28, 52, 28, 85, 72, 85), (HENG, 28, 85, 72, 85, 0, 0)],
        '宙' => vec![(DIAN, 50, 15, 52, 18, 0, 0), (HENG, 28, 22, 72, 22, 0, 0), (SHU, 50, 22, 50, 65, 0, 0), (HENGZHE, 50, 42, 70, 42, 70, 65), (PIE, 70, 42, 60, 60, 0, 0), (HENG, 50, 65, 70, 65, 0, 0)],
        '光' => vec![(DIAN, 50, 15, 48, 22, 0, 0), (HENG, 35, 25, 65, 25, 0, 0), (PIE, 50, 25, 20, 72, 0, 0), (HENGZHE, 50, 25, 75, 25, 72, 55), (PIE, 72, 55, 55, 90, 0, 0), (NA, 72, 55, 88, 85, 0, 0)],
        '明' => vec![(SHU, 32, 15, 32, 85, 0, 0), (HENGZHE, 32, 15, 65, 15, 65, 85), (HENG, 32, 85, 65, 85, 0, 0), (HENG, 32, 48, 65, 48, 0, 0), (PIE, 72, 18, 72, 82, 0, 0), (HENGZHE, 72, 18, 88, 18, 88, 82), (HENG, 72, 82, 88, 82, 0, 0)],
        '地' => vec![(HENG, 18, 28, 38, 28, 0, 0), (SHU, 25, 18, 25, 82, 0, 0), (TI, 25, 55, 35, 52, 0, 0), (HENG, 18, 82, 38, 82, 0, 0), (SHU, 58, 22, 58, 82, 0, 0), (HENGZHE, 58, 22, 75, 22, 75, 82), (SHUWANGOU, 75, 55, 70, 40, 72, 58), (HENG, 58, 82, 78, 82, 0, 0)],
        '照' => vec![(SHU, 30, 15, 30, 80, 0, 0), (HENGZHE, 30, 15, 65, 15, 65, 80), (HENG, 30, 80, 65, 80, 0, 0), (HENG, 30, 45, 65, 45, 0, 0), (XIEGOU, 50, 45, 55, 30, 48, 32), (HENG, 72, 15, 88, 15, 0, 0), (DIAN, 78, 15, 82, 22, 0, 0), (HENG, 72, 28, 88, 28, 0, 0), (DIAN, 78, 28, 82, 35, 0, 0)],
        '李' => vec![(HENG, 22, 28, 78, 28, 0, 0), (SHU, 50, 12, 50, 65, 0, 0), (PIE, 50, 28, 20, 90, 0, 0), (NA, 50, 28, 80, 90, 0, 0), (HENG, 20, 65, 80, 65, 0, 0), (SHUGOU, 42, 65, 35, 82, 38, 78), (HENG, 25, 82, 58, 82, 0, 0)],
        '刘' => vec![(DIAN, 30, 22, 35, 28, 0, 0), (PIE, 40, 15, 25, 35, 0, 0), (HENG, 28, 35, 55, 35, 0, 0), (SHU, 70, 18, 70, 85, 0, 0), (PIE, 70, 18, 55, 55, 0, 0), (DIAN, 70, 55, 50, 90, 0, 0)],
        '张' => vec![(HENG, 18, 35, 22, 35, 0, 0), (HENGZHE, 18, 35, 35, 35, 35, 55), (PIE, 35, 35, 50, 55, 0, 0), (HENG, 20, 55, 48, 55, 0, 0), (HENG, 15, 75, 85, 75, 0, 0), (SHU, 50, 25, 50, 90, 0, 0), (HENG, 50, 55, 50, 75, 0, 0), (HENG, 50, 75, 65, 90, 0, 0)],
        '陈' => vec![(HENG, 18, 28, 22, 28, 0, 0), (SHUGOU, 20, 22, 18, 35, 15, 32), (HENG, 15, 35, 28, 35, 0, 0), (HENG, 15, 48, 28, 48, 0, 0), (HENG, 15, 65, 28, 65, 0, 0), (SHU, 20, 18, 20, 85, 0, 0), (HENG, 32, 18, 35, 18, 0, 0), (HENG, 32, 30, 35, 30, 0, 0), (PIE, 60, 18, 48, 90, 0, 0), (NA, 60, 18, 82, 90, 0, 0)],
        '杨' => vec![(HENG, 18, 28, 42, 28, 0, 0), (SHU, 25, 28, 25, 72, 0, 0), (PIE, 25, 28, 15, 62, 0, 0), (NA, 25, 28, 42, 72, 0, 0), (HENG, 18, 72, 42, 72, 0, 0), (PIE, 60, 22, 60, 78, 0, 0), (PIE, 60, 22, 48, 55, 0, 0), (PIE, 60, 22, 72, 55, 0, 0), (HENG, 48, 55, 72, 55, 0, 0), (SHUGOU, 60, 55, 75, 78, 68, 75)],
        '水' => vec![(SHU, 50, 18, 50, 85, 0, 0), (HENGZHE, 50, 35, 35, 25, 30, 50), (PIE, 30, 50, 20, 80, 0, 0), (NA, 50, 50, 75, 80, 0, 0)],
        '容' => vec![(DIAN, 50, 12, 52, 15, 0, 0), (HENG, 28, 18, 72, 18, 0, 0), (HENG, 25, 28, 75, 28, 0, 0), (SHU, 50, 28, 50, 72, 0, 0), (PIE, 50, 50, 20, 85, 0, 0), (HENG, 20, 85, 80, 85, 0, 0), (NA, 50, 50, 80, 85, 0, 0)],
        '器' => vec![(SHU, 22, 15, 22, 72, 0, 0), (HENGZHE, 22, 15, 38, 15, 38, 72), (HENG, 22, 72, 38, 72, 0, 0), (DIAN, 30, 25, 34, 30, 0, 0), (SHU, 62, 15, 62, 72, 0, 0), (HENGZHE, 62, 15, 78, 15, 78, 72), (HENG, 62, 72, 78, 72, 0, 0), (DIAN, 70, 25, 74, 30, 0, 0), (HENG, 18, 82, 82, 82, 0, 0), (DIAN, 35, 82, 40, 86, 0, 0), (DIAN, 65, 82, 70, 86, 0, 0), (DIAN, 50, 82, 55, 86, 0, 0)],
        '刘' => vec![(DIAN, 28, 22, 32, 28, 0, 0), (PIE, 38, 15, 22, 35, 0, 0), (HENG, 25, 35, 52, 35, 0, 0), (SHU, 68, 18, 68, 85, 0, 0), (PIE, 68, 18, 55, 55, 0, 0), (DIAN, 68, 55, 48, 90, 0, 0)],
        '楚' => vec![(HENG, 22, 18, 45, 18, 0, 0), (SHU, 28, 18, 28, 65, 0, 0), (PIE, 28, 18, 18, 55, 0, 0), (NA, 28, 18, 45, 65, 0, 0), (HENG, 20, 65, 45, 65, 0, 0), (HENG, 20, 45, 45, 45, 0, 0), (HENG, 18, 85, 55, 85, 0, 0), (SHU, 55, 55, 55, 85, 0, 0), (PIE, 55, 55, 42, 85, 0, 0), (NA, 55, 55, 68, 90, 0, 0), (HENG, 52, 68, 65, 68, 0, 0), (HENG, 50, 90, 55, 90, 0, 0)],
        '恬' => vec![(DIAN, 28, 18, 32, 22, 0, 0), (DIAN, 28, 28, 32, 32, 0, 0), (SHU, 32, 15, 32, 82, 0, 0), (PIE, 50, 15, 65, 15, 0, 0), (HENG, 50, 32, 78, 32, 0, 0), (HENG, 50, 32, 78, 32, 0, 0), (HENG, 50, 48, 78, 48, 0, 0), (HENG, 50, 62, 78, 62, 0, 0), (PIE, 58, 15, 52, 82, 0, 0), (HENG, 52, 82, 72, 82, 0, 0)],
        // 容器本体
        '袋' => vec![(PIE, 50, 12, 35, 15, 0, 0), (HENG, 22, 25, 78, 25, 0, 0), (HENG, 20, 42, 80, 42, 0, 0), (SHU, 50, 42, 50, 88, 0, 0), (TI, 50, 72, 62, 68, 0, 0), (PIE, 38, 42, 22, 82, 0, 0), (NA, 62, 42, 80, 82, 0, 0)],
        '桶' => vec![(HENG, 22, 28, 78, 28, 0, 0), (SHU, 50, 15, 50, 90, 0, 0), (PIE, 50, 28, 20, 75, 0, 0), (NA, 50, 28, 80, 75, 0, 0), (SHU, 25, 18, 25, 72, 0, 0), (HENGZHE, 25, 18, 38, 18, 38, 72), (HENG, 25, 72, 38, 72, 0, 0), (HENG, 25, 45, 38, 45, 0, 0)],
        '盎' => vec![(HENG, 22, 18, 78, 18, 0, 0), (PIE, 50, 18, 30, 55, 0, 0), (NA, 50, 18, 70, 55, 0, 0), (SHU, 25, 55, 25, 85, 0, 0), (HENGZHE, 25, 55, 75, 55, 75, 85), (HENG, 25, 85, 75, 85, 0, 0), (HENG, 25, 72, 75, 72, 0, 0), (PIE, 25, 72, 20, 85, 0, 0), (NA, 25, 72, 40, 88, 0, 0), (HENG, 18, 88, 72, 88, 0, 0)],
        '盆' => vec![(PIE, 50, 12, 35, 12, 0, 0), (NA, 50, 12, 65, 12, 0, 0), (SHU, 25, 25, 25, 88, 0, 0), (HENGZHE, 25, 25, 75, 25, 75, 88), (HENG, 25, 88, 75, 88, 0, 0), (HENG, 25, 55, 75, 55, 0, 0), (DIAN, 42, 55, 45, 58, 0, 0)],
        '盂' => vec![(HENG, 22, 15, 78, 15, 0, 0), (HENG, 20, 30, 80, 30, 0, 0), (SHU, 50, 15, 50, 65, 0, 0), (HENG, 22, 65, 78, 65, 0, 0), (HENG, 22, 82, 78, 82, 0, 0), (DIAN, 30, 82, 35, 86, 0, 0)],
        '釜' => vec![(PIE, 50, 12, 32, 15, 0, 0), (HENG, 22, 25, 78, 25, 0, 0), (SHU, 25, 25, 25, 78, 0, 0), (HENGZHE, 25, 25, 75, 25, 75, 78), (HENG, 32, 78, 78, 78, 0, 0), (HENG, 32, 55, 75, 55, 0, 0), (DIAN, 50, 55, 52, 58, 0, 0)],
        '盒' => vec![(PIE, 50, 12, 35, 12, 0, 0), (NA, 50, 12, 65, 12, 0, 0), (HENG, 22, 25, 78, 25, 0, 0), (SHU, 50, 25, 50, 88, 0, 0), (HENGZHE, 25, 55, 75, 55, 75, 88), (HENG, 25, 55, 75, 55, 0, 0), (DIAN, 40, 70, 45, 74, 0, 0)],
        '瓶' => vec![(HENG, 22, 22, 78, 22, 0, 0), (SHU, 50, 22, 50, 78, 0, 0), (PIE, 50, 22, 20, 72, 0, 0), (NA, 50, 22, 80, 72, 0, 0), (DIAN, 65, 22, 70, 25, 0, 0), (PIE, 70, 22, 78, 48, 0, 0), (HENG, 65, 48, 82, 48, 0, 0), (HENGZHE, 70, 48, 85, 78, 70, 78)],
        '盏' => vec![(HENG, 22, 18, 78, 18, 0, 0), (HENG, 20, 32, 80, 32, 0, 0), (DIAN, 50, 32, 55, 38, 0, 0), (PIE, 50, 32, 35, 65, 0, 0), (HENG, 25, 65, 50, 65, 0, 0), (PIE, 72, 15, 65, 45, 0, 0), (HENG, 65, 45, 82, 45, 0, 0), (PIE, 72, 45, 80, 85, 0, 0), (NA, 72, 45, 60, 88, 0, 0)],
        '杯' => vec![(HENG, 22, 28, 45, 28, 0, 0), (SHU, 28, 28, 28, 88, 0, 0), (PIE, 28, 28, 18, 78, 0, 0), (NA, 28, 28, 45, 88, 0, 0), (HENG, 22, 88, 45, 88, 0, 0), (HENG, 55, 18, 78, 18, 0, 0), (HENG, 52, 32, 82, 32, 0, 0), (DIAN, 68, 32, 72, 36, 0, 0)],
        '壶' => vec![(HENG, 22, 15, 78, 15, 0, 0), (SHU, 50, 15, 50, 52, 0, 0), (HENG, 22, 52, 78, 52, 0, 0), (HENG, 18, 72, 82, 72, 0, 0), (SHU, 25, 52, 25, 92, 0, 0), (HENGZHE, 25, 72, 75, 72, 75, 92), (HENG, 25, 92, 75, 92, 0, 0), (HENG, 75, 15, 75, 72, 0, 0)],
        '缸' => vec![(PIE, 50, 12, 35, 18, 0, 0), (HENG, 22, 25, 78, 25, 0, 0), (HENG, 20, 45, 80, 45, 0, 0), (SHU, 50, 20, 50, 88, 0, 0), (HENG, 22, 88, 78, 88, 0, 0)],
        '柜' => vec![(HENG, 25, 25, 75, 25, 0, 0), (SHU, 50, 15, 50, 88, 0, 0), (PIE, 50, 25, 20, 85, 0, 0), (NA, 50, 25, 80, 85, 0, 0), (HENG, 22, 88, 78, 88, 0, 0), (HENG, 25, 55, 75, 55, 0, 0), (HENGZHE, 75, 55, 88, 55, 88, 85)],
        '箱' => vec![(PIE, 22, 15, 38, 15, 0, 0), (HENGGOU, 22, 15, 22, 35, 18, 32), (HENG, 22, 35, 38, 35, 0, 0), (SHU, 50, 15, 50, 88, 0, 0), (PIE, 50, 15, 30, 55, 0, 0), (NA, 50, 15, 70, 55, 0, 0), (HENG, 30, 55, 70, 55, 0, 0), (SHU, 28, 55, 28, 88, 0, 0), (PIE, 30, 72, 20, 88, 0, 0), (HENG, 20, 88, 68, 88, 0, 0), (NA, 30, 72, 65, 88, 0, 0), (SHU, 68, 55, 68, 88, 0, 0)],
        '篮' => vec![(PIE, 22, 15, 38, 15, 0, 0), (HENGGOU, 22, 15, 22, 32, 18, 28), (HENG, 22, 32, 38, 32, 0, 0), (PIE, 22, 32, 38, 55, 0, 0), (HENGGOU, 22, 55, 22, 72, 18, 68), (HENG, 22, 72, 38, 72, 0, 0), (SHU, 50, 15, 50, 92, 0, 0), (HENG, 22, 48, 78, 48, 0, 0), (SHU, 28, 72, 28, 92, 0, 0), (HENGZHE, 28, 72, 72, 72, 72, 92), (HENG, 28, 92, 72, 92, 0, 0), (SHU, 72, 72, 72, 92, 0, 0)],
        _ => return None,
    };
    let defs: Vec<StrokeDef> = strokes.into_iter().map(|(typ, x0, y0, x1, y1, cx, cy)| {
        StrokeDef { typ, x0, y0, x1, y1, cx, cy }
    }).collect();
    Some(defs)
}

// 笔画绘制到图像（支持线宽）
fn draw_stroke(img: &mut RgbImage, s: &StrokeDef, ox: i32, oy: i32, scale: i32, color: Rgb<u8>) {
    let x0 = ox + s.x0 * scale / 100;
    let y0 = oy + s.y0 * scale / 100;
    let x1 = ox + s.x1 * scale / 100;
    let y1 = oy + s.y1 * scale / 100;
    let cx = ox + s.cx * scale / 100;
    let cy = oy + s.cy * scale / 100;
    let thickness = (scale / 40).max(2); // 线宽：scale=200 → 5px
    if s.cx == 0 && s.cy == 0 {
        draw_thick_line(img, x0, y0, x1, y1, thickness, color);
    } else if s.typ == 7 || s.typ == 6 || s.typ == 13 {
        draw_thick_arc(img, x0, y0, cx, cy, x1, y1, thickness, color);
    } else {
        let mx = (x0 + cx) / 2;
        let my = (y0 + cy) / 2;
        let nx = (cx + x1) / 2;
        let ny = (cy + y1) / 2;
        draw_thick_line(img, x0, y0, mx, my, thickness, color);
        draw_thick_line(img, mx, my, nx, ny, thickness, color);
        draw_thick_line(img, nx, ny, x1, y1, thickness, color);
    }
}

fn draw_thick_line(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, thickness: i32, color: Rgb<u8>) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let r = thickness / 2;
    loop {
        // 绘制粗点（方形刷）
        for dy in -r..=r {
            for dx in -r..=r {
                let px = x0 + dx;
                let py = y0 + dy;
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

fn draw_thick_arc(img: &mut RgbImage, x0: i32, y0: i32, cx: i32, cy: i32, x1: i32, y1: i32, thickness: i32, color: Rgb<u8>) {
    let steps = 32;
    let mut pts = Vec::new();
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let tt = 1.0 - t;
        let x = tt * tt * x0 as f64 + 2.0 * tt * t * cx as f64 + t * t * x1 as f64;
        let y = tt * tt * y0 as f64 + 2.0 * tt * t * cy as f64 + t * t * y1 as f64;
        pts.push((x.round() as i32, y.round() as i32));
    }
    for w in pts.windows(2) {
        draw_thick_line(img, w[0].0, w[0].1, w[1].0, w[1].1, thickness, color);
    }
}

#[allow(dead_code)]
fn _bresenham(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as u32) < img.width() && (y0 as u32) < img.height() {
            img.put_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

#[allow(dead_code)]
fn _draw_arc(img: &mut RgbImage, x0: i32, y0: i32, cx: i32, cy: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let steps = 24;
    let mut pts = Vec::new();
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let tt = 1.0 - t;
        let x = tt * tt * x0 as f64 + 2.0 * tt * t * cx as f64 + t * t * x1 as f64;
        let y = tt * tt * y0 as f64 + 2.0 * tt * t * cy as f64 + t * t * y1 as f64;
        pts.push((x.round() as i32, y.round() as i32));
    }
    for w in pts.windows(2) {
        _bresenham(img, w[0].0, w[0].1, w[1].0, w[1].1, color);
    }
}
// 5x7 ASCII bitmap
const FONT_5X7: [[u8; 5]; 37] = [
    [0x0E,0x11,0x13,0x15,0x0E],[0x04,0x0C,0x04,0x04,0x0E],[0x0E,0x01,0x0E,0x10,0x1F],
    [0x0E,0x01,0x06,0x01,0x0E],[0x11,0x11,0x1F,0x01,0x01],[0x1F,0x10,0x0E,0x01,0x0E],
    [0x0E,0x10,0x1E,0x11,0x0E],[0x1F,0x01,0x02,0x04,0x04],[0x0E,0x11,0x0E,0x11,0x0E],
    [0x0E,0x11,0x0F,0x01,0x0E],[0x0E,0x11,0x1F,0x11,0x11],[0x1E,0x11,0x1E,0x11,0x1E],
    [0x0E,0x10,0x10,0x10,0x0E],[0x1E,0x11,0x11,0x11,0x1E],[0x1F,0x10,0x1E,0x10,0x1F],
    [0x1F,0x10,0x1E,0x10,0x10],[0x0E,0x10,0x17,0x11,0x0E],[0x11,0x11,0x1F,0x11,0x11],
    [0x0E,0x04,0x04,0x04,0x0E],[0x07,0x02,0x02,0x12,0x0C],[0x11,0x12,0x14,0x12,0x11],
    [0x10,0x10,0x10,0x10,0x1F],[0x11,0x1B,0x15,0x11,0x11],[0x11,0x19,0x15,0x13,0x11],
    [0x0E,0x11,0x11,0x11,0x0E],[0x1E,0x11,0x1E,0x10,0x10],[0x0E,0x11,0x11,0x09,0x06],
    [0x1E,0x11,0x1E,0x12,0x11],[0x0F,0x10,0x0E,0x01,0x1E],[0x1F,0x04,0x04,0x04,0x04],
    [0x11,0x11,0x11,0x11,0x0E],[0x11,0x11,0x11,0x0A,0x04],[0x11,0x11,0x15,0x15,0x0A],
    [0x1F,0x02,0x04,0x08,0x1F],
    [0x00,0x00,0x00,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00],
    [0x00,0x00,0x00,0x00,0x00],
];

fn draw_char_ascii(img: &mut RgbImage, ch: char, x: i32, y: i32, scale: i32, color: Rgb<u8>) {
    let idx = match ch {
        '0'..='9' => ch as usize - '0' as usize,
        'a'..='z' => ch as usize - 'a' as usize + 10,
        'A'..='Z' => ch as usize - 'A' as usize + 10,
        ' ' => 36,
        _ => 36,
    };
    if idx >= 37 { return; }
    let glyph = &FONT_5X7[idx];
    for col in 0..5 {
        let col_data = glyph[col];
        for row in 0..7 {
            if (col_data >> row) & 1 == 1 {
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = x + (col as i32) * scale + dx;
                        let py = y + (row as i32) * scale + dy;
                        if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                            img.put_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }
        }
    }
}

fn draw_string(img: &mut RgbImage, s: &str, x: i32, y: i32, scale: i32, color: Rgb<u8>) {
    let mut cx = x;
    for ch in s.chars() {
        draw_char_ascii(img, ch, cx, y, scale, color);
        cx += 6 * scale;
    }
}

// 汉字到笔画名
fn ch_to_strokes(ch: char) -> Option<Vec<&'static str>> {
    let names = match ch {
        '一' => vec!["横"], '二' => vec!["横","横"], '三' => vec!["横","横","横"],
        '人' => vec!["撇","捺"], '大' => vec!["横","撇","捺"], '夫' => vec!["横","横","撇","捺"],
        '天' => vec!["横","横","撇","捺"], '太' => vec!["横","撇","捺","点"],
        '女' => vec!["撇","横","横"], '子' => vec!["横钩","竖钩","横"],
        '小' => vec!["竖","撇","点"], '山' => vec!["竖","竖折","竖"],
        '火' => vec!["点","撇","撇","捺"], '木' => vec!["横","竖","撇","捺"],
        '土' => vec!["横","竖","横"], '王' => vec!["横","横","竖","横"],
        '主' => vec!["点","横","横","竖","横"], '金' => vec!["撇","捺","横","横","竖","横"],
        '日' => vec!["竖","横折","横","横"], '月' => vec!["撇","横折","横","横"],
        '目' => vec!["竖","横折","横","横","横"], '口' => vec!["竖","横折","横"],
        '囗' => vec!["竖","横折","横"], '囡' => vec!["竖","横折","横","横","撇","横"],
        '囝' => vec!["竖","横折","横","竖","横","竖钩"], '田' => vec!["竖","横折","横","竖","横"],
        '中' => vec!["竖","横折","横","竖"], '共' => vec!["横","竖","撇","捺","横","点"],
        '同' => vec!["竖","横折","撇","横","横","横"], '生' => vec!["撇","横","横","竖","横"],
        '和' => vec!["撇","横","竖","点","竖","横折","横"], '平' => vec!["横","点","撇","竖","横"],
        '安' => vec!["点","点","横","撇","横","横"],
        '满' => vec!["横","竖","提","点","横","竖","点","撇","捺","横","横"],
        '全' => vec!["撇","捺","横","横","竖","横"], '法' => vec!["横","竖","横","横","点","撇","捺"],
        '合' => vec!["撇","捺","横","竖折","横"], '妻' => vec!["横","竖","横","竖","横","竖","横","捺"],
        '爱' => vec!["点","点","横折","点","横","点","横","撇","横折","撇","捺"],
        '婚' => vec!["撇","横折","横","横","撇","横","撇","竖","横折","横","横","横"],
        '缘' => vec!["撇","横折","撇","横","撇","捺","撇","横","撇","横","竖"],
        '心' => vec!["点","卧钩","点","点"], '念' => vec!["横","竖","撇","点","点","卧钩","点","点"],
        '想' => vec!["横","竖","撇","点","点","横","横","撇","捺","点"],
        '意' => vec!["点","横","撇","捺","横","横","竖","横","点","点","横折","点"],
        '思' => vec!["竖","横折","横","横","撇","捺","点","卧钩","点","点"],
        '情' => vec!["点","点","竖","横","竖","横","横","横","撇","横"],
        '星' => vec!["竖","横折","撇","横","横","竖","横"], '空' => vec!["点","点","横","横","竖","横"],
        '宇' => vec!["点","点","横","竖","横折","横"], '宙' => vec!["点","点","横","竖","横折","撇","横"],
        '光' => vec!["点","横","撇","横折","撇","捺"], '明' => vec!["竖","横折","横","撇","横折","横","横"],
        '地' => vec!["横","竖","提","横折","竖弯钩","横"], '照' => vec!["竖","横折","撇","横","横","横折","点","点","点","横"],
        '李' => vec!["横","竖","撇","捺","横","竖钩","横"], '刘' => vec!["点","撇","横","竖","撇","点"],
        '张' => vec!["横","横折","撇","横","横","横","竖横"], '陈' => vec!["横","竖钩","横","竖","横撇","撇","捺"],
        '杨' => vec!["横","竖","撇","捺","撇","撇","横","竖钩"],
        '水' => vec!["竖","横折","撇","捺"], '容' => vec!["点","点","横","横","竖","撇","捺"],
        '器' => vec!["竖","横折","横","点","竖","横折","横","点","横","点","点"],
        '楚' => vec!["横","竖","撇","捺","横","横","竖","撇","捺","横","横"],
        '恬' => vec!["点","点","竖","撇","横","横","横","撇","横"],
        '袋' => vec!["撇","横","横","竖","提","撇","捺"], '桶' => vec!["横","竖","撇","捺","竖","横折","横"],
        '盎' => vec!["横","撇","捺","竖","横折","横","横","撇","捺"],
        '盆' => vec!["撇","捺","竖","横折","横","点"], '盂' => vec!["横","横","竖","横折","横","横","点"],
        '盒' => vec!["撇","捺","横","竖","横折","点"], '瓶' => vec!["横","竖","撇","捺","点","撇","横折","横"],
        '盖' => vec!["点","点","横","竖","横折","横","竖","横折"],
        '杯' => vec!["横","竖","撇","捺","横","横","点"], '壶' => vec!["横","竖","点","横","横","竖","横折","横"],
        '缸' => vec!["撇","横","横","竖","横"], '柜' => vec!["横","竖","撇","捺","横","横折"],
        '箱' => vec!["撇","横钩","横","撇","捺","竖","撇","捺","横","竖"],
        '篮' => vec!["撇","横钩","撇","横钩","竖","横折","竖","横"],
        _ => return None,
    };
    Some(names)
}

// 容器名映射
fn stroke_container(s: &str) -> &str {
    match s {
        "横" => "袋", "竖" => "桶", "撇" => "罐", "捺" => "盆",
        "点" => "盂", "提" => "壶", "横折" => "盒", "竖折" => "瓶",
        "横撇" => "盏", "竖钩" => "盎", "横钩" => "篓", "弯钩" => "筐",
        "竖弯钩" => "篮", "横折折" => "箩", "竖弯" => "箕", "斜钩" => "笼",
        "卧钩" => "盎", _ => "?",
    }
}

// 容器嵌套：相同容器多层, 不同容器逐一套
fn compose_containers(strokes: &[&'static str]) -> (String, Vec<&'static str>) {
    let mut counts = std::collections::HashMap::new();
    for s in strokes {
        let c = stroke_container(s);
        *counts.entry(c).or_insert(0u8) += 1;
    }
    let mut parts = Vec::new();
    let mut names = Vec::new();
    for s in strokes {
        let c = stroke_container(s);
        if !names.contains(&c) { names.push(c); }
    }
    if names.len() <= 3 {
        for c in &names {
            let n = counts[c];
            if n == 1 { parts.push(format!("{}", c)); }
            else { parts.push(format!("{}({}层)", c, n)); }
        }
    } else {
        parts.push(format!("{}(多层)", names[0]));
    }
    (parts.join(" 装 "), names)
}

pub fn encode_hanzi(ch: char) -> Option<HanziSpecimen> {
    let strokes = ch_to_strokes(ch)?;
    let container_names: Vec<&str> = strokes.iter().map(|s| stroke_container(s)).collect();
    let (nested, _names) = compose_containers(&strokes);

    let total = strokes.len();
    let code6: u8 = strokes.iter().enumerate().fold(0, |acc, (i, _s)| {
        acc | (1 << (i % 6))
    });
    let polarity = if total % 2 == 0 { "阴" } else { "阳" };

    // 6-bit code → state classification via StateEnumerator
    let bw = 6u8;
    let enumerator = StateEnumerator::new(bw);
    let all = enumerator.enumerate();
    let target_state = code6 as u64;

    let classification = all.iter().find(|c| c.state == target_state)
        .or_else(|| all.iter().find(|c| c.orbit.representative == c.state && c.orbit.elements.contains(&target_state)));

    let cat = if let Some(c) = classification {
        match c.category {
            StateCategory::AxialContainer => 1u8,
            StateCategory::ClosedLoop => 2,
            StateCategory::MirrorEntangled => 3,
            StateCategory::ExtendedEntangled => 4,
        }
    } else {
        0
    };

    let cat_enum = match cat {
        1 => StateCategory::AxialContainer,
        2 => StateCategory::ClosedLoop,
        3 => StateCategory::MirrorEntangled,
        4 => StateCategory::ExtendedEntangled,
        _ => StateCategory::MirrorEntangled,
    };

    // display container
    let display = match (polarity, total % 3) {
        ("阴", 0) => "盎", ("阳", 0) => "杯",
        ("阴", 1) => "瓶", ("阳", 1) => "盉",
        _ => "盎",
    };

    let color = match cat {
        1 => Rgb([74, 144, 217]),
        2 => Rgb([80, 184, 108]),
        3 => Rgb([217, 140, 74]),
        4 => Rgb([160, 100, 200]),
        _ => Rgb([120, 120, 120]),
    };

    let orbit_len = if let Some(c) = classification {
        match c.category {
            StateCategory::AxialContainer => 2,
            StateCategory::ClosedLoop => 2,
            StateCategory::MirrorEntangled => 4,
            StateCategory::ExtendedEntangled => 4,
        }
    } else { 4 };

    Some(HanziSpecimen {
        ch,
        strokes,
        container_set: container_names,
        nested,
        polarity,
        display_container: display,
        code6,
        bit_width: 6,
        category: cat_enum,
        category_color: color,
        orbit_len: orbit_len as u8,
    })
}

pub fn render_hanzi_specimen(text: &str, output_path: &str) {
    let chars: Vec<char> = text.chars().collect();
    let specimens: Vec<HanziSpecimen> = chars.iter().filter_map(|&c| {
        encode_hanzi(c)
    }).collect();

    if specimens.is_empty() {
        eprintln!("error: no renderable characters");
        return;
    }

    let card_w: u32 = 520;
    let card_h: u32 = 680;
    let cols = 3usize;
    let cols = if specimens.len() < cols { specimens.len() } else { cols };
    let rows = (specimens.len() + cols - 1) / cols;
    let padding: u32 = 24;
    let total_w = cols as u32 * card_w + (cols as u32 + 1) * padding;
    let total_h = rows as u32 * card_h + (rows as u32 + 1) * padding + 60;

    let mut img = RgbImage::from_pixel(total_w, total_h, Rgb([10, 14, 23]));
    let white = Rgb([220, 225, 235]);
    let dim = Rgb([80, 90, 110]);
    let card_bg = Rgb([18, 22, 35]);

    // header
    draw_string(&mut img, "HANZI CONTAINER ENCODE" , (padding+8) as i32, 14, 2, Rgb([90, 160, 240]));
    draw_string(&mut img, &format!("\"{}\"", text) , (padding+8) as i32, 44, 1, dim);

    for (i, spec) in specimens.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let cx = padding + col as u32 * (card_w + padding);
        let cy = 60 + padding + row as u32 * (card_h + padding);

        // card background
        for yy in cy..cy+card_h {
            for xx in cx..cx+card_w {
                if xx < total_w && yy < total_h {
                    img.put_pixel(xx, yy, card_bg);
                }
            }
        }

        // color bar (left edge)
        let bar_color = spec.category_color;
        for yy in cy..cy+card_h {
            for xx in cx..cx+8 {
                if xx < total_w && yy < total_h {
                    img.put_pixel(xx, yy, bar_color);
                }
            }
        }

        // Draw character (big) - centered in left area
        let char_area_x = cx + 36;
        let char_area_y = cy + 40;
        let char_size: u32 = 200;

        // Draw strokes
        if let Some(strokes) = stroke_defs(spec.ch) {
            for s in &strokes {
                draw_stroke(&mut img, s, char_area_x as i32, char_area_y as i32, char_size as i32, white);
            }
        } else {
            // fallback: draw box
            for x in 0..char_size {
                img.put_pixel(char_area_x + x, char_area_y, dim);
                img.put_pixel(char_area_x + x, char_area_y + char_size, dim);
            }
            for y in 0..char_size {
                img.put_pixel(char_area_x, char_area_y + y, dim);
                img.put_pixel(char_area_x + char_size, char_area_y + y, dim);
            }
        }

        // char label
        let label_y = char_area_y + char_size + 16u32;
        draw_string(&mut img, &format!("U+{:04X}", spec.ch as u32), char_area_x as i32, label_y as i32, 1, dim);

        // Category bar
        let cat_bar_y = label_y + 22;
        for xx in char_area_x..char_area_x+180 {
            if xx < total_w && cat_bar_y < total_h {
                img.put_pixel(xx, cat_bar_y, bar_color);
                img.put_pixel(xx, cat_bar_y+24, bar_color);
            }
        }
        for yy in cat_bar_y..cat_bar_y+24 {
            if char_area_x < total_w && yy < total_h {
                img.put_pixel(char_area_x, yy, bar_color);
                img.put_pixel(char_area_x+180, yy, bar_color);
            }
        }
        // inside: category name
        draw_string(&mut img, spec.category_name(), char_area_x as i32 + 4, cat_bar_y as i32 + 2, 1, Rgb([0, 0, 0]));

        // Right column: metadata
        let rx = cx + 280;
        let mut ry = cy + 35;

        draw_string(&mut img, &format!("{}", spec.polarity), rx as i32, ry as i32, 2, white);
        ry += 36;

        draw_string(&mut img, &format!("orb={}", spec.orbit_len), rx as i32, ry as i32, 1, dim);
        ry += 16;
        draw_string(&mut img, &format!("dsp={}", spec.display_container), rx as i32, ry as i32, 1, CONTAINER_COLOR);
        ry += 16;
        draw_string(&mut img, &format!("bits={:06b}", spec.code6), rx as i32, ry as i32, 1, Rgb([200, 160, 80]));
        ry += 24;

        // Container composition (flow)
        draw_string(&mut img, "containers:", rx as i32, ry as i32, 1, dim);
        ry += 14;

        // Show containers line by line
        let mut line = String::new();
        for (j, c) in spec.container_set.iter().enumerate() {
            if !line.is_empty() { line.push('+'); }
            line.push_str(c);
            if line.len() >= 6 {
                draw_string(&mut img, &line, rx as i32, ry as i32, 1, CONTAINER_COLOR);
                ry += 13;
                line.clear();
            }
        }
        if !line.is_empty() {
            draw_string(&mut img, &line, rx as i32, ry as i32, 1, CONTAINER_COLOR);
            ry += 13;
        }
        ry += 6;

        // Nesting
        draw_string(&mut img, "nest:", rx as i32, ry as i32, 1, dim);
        ry += 13;
        // split nested string by " 装 " and wrap
        let nesting_parts: Vec<&str> = spec.nested.split(" 装 ").collect();
        for part in nesting_parts.iter().take(4) {
            draw_string(&mut img, part, rx as i32, ry as i32, 1, Rgb([140, 180, 140]));
            ry += 13;
        }
    }

    imageops::flip_vertical_in_place(&mut img);
    let _ = img.save(output_path);
    println!("[hanzi_specimen] rendered {} chars → {}", specimens.len(), output_path);
}
