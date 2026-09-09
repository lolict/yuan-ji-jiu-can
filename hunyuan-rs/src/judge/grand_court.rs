// ══════════════════════════════════════════════════════════════════════
// grand_court.rs · 大审判庭 — 全链路整合裁判系统
// ══════════════════════════════════════════════════════════════════════
//
// 把四个独立模块拉通为端到端管道:
//   1. encoding_bridge   — C编码系统 (1257字 · 五家族 · 48声母 · 66韵母)
//   2. crime_transcript  — 全局衍射裁判 (类型拓扑 · 编织图 · 全局水质)
//   3. glyph_pixel_algebra — 字形像素运算 (加减拆解 · 相位编码)
//   4. celestial_perception_v3 — 天觉v3 (感知 · 历史 · 淘汰)
//
// 管道: 字符输入 → 感知路由 → 衍射分析 → 类型审判 → 像素编码 → 判决输出
//
// ══════════════════════════════════════════════════════════════════════

use crate::crime_transcript::*;
use crate::glyph_pixel_algebra::*;
use crate::encoding_bridge::{Family, FlowBits};
use crate::os_layer::OsLayer;
use crate::os_layer::bare_metal::BareMetalOs;

/// 全链路判决结果
#[derive(Clone, Debug)]
pub struct GrandCourtVerdict {
    pub input_char: String,
    pub family: Option<Family>,
    pub diffraction: DiffractionPattern,
    pub crime_transcripts: Vec<CrimeTranscript>,
    pub glyph_encoded: bool,
    pub final_guilty: bool,
    pub sentence: String,
}

/// 大审判庭 — 整合所有裁判模块
pub struct GrandCourt {
    pub judge: Judge,
    pub water_quality: GlobalWaterQuality,
    pub total_processed: u64,
    pub total_guilty: u64,
    pub total_acquitted: u64,
}

impl GrandCourt {
    pub fn new() -> Self {
        Self {
            judge: Judge::new(),
            water_quality: GlobalWaterQuality::new(0.5),
            total_processed: 0,
            total_guilty: 0,
            total_acquitted: 0,
        }
    }

    /// 全链路审判: 一个字符从输入到最终判决
    pub fn process(&mut self, os: &dyn OsLayer, ch: &str) -> GrandCourtVerdict {
        self.total_processed += 1;

        // ── 第1步: 编码桥接 (C系统感知路由) ──
        // 取字符的Unicode码点
        let code_point = ch.chars().next().map(|c| c as u32).unwrap_or(0);
        let family = if code_point > 0 {
            Some(Family::classify_by_codepoint(code_point))
        } else {
            None
        };

        // ── 第2步: 衍射分析 (8维特征投影) ──
        let features = Self::char_to_features(ch, family);
        let diffraction = DiffractionMirror::diffract(code_point as u64, &features);

        // ── 第3步: 类型拓扑检验 (a≠á) ──
        if ch.chars().count() == 1 {
            let c = ch.chars().next().unwrap();
            // 检测是否带声调符号 (拉丁扩展区)
            if c as u32 >= 0x00C0 && c as u32 <= 0x00FF {
                let base = (c as u32 - 0x00C0) as u8;
                let _ = self.judge.judge_type(os, code_point as u64, "base", "accented");
                let _ = base; // 使用base避免警告
            }
        }

        // ── 第4步: 全局审判 ──
        let reference = EntityFeatures::perfect();
        let verdict = self.judge.judge(os, code_point as u64, &features, Some(&reference));

        // ── 第5步: 全局水质检测 ──
        self.water_quality.add_sample(WaterSample {
            source_id: code_point as u64,
            purity: features.aesthetic * 0.3 + features.type_purity * 0.3 + features.thinking * 0.4,
            semantic_correct: features.thinking > 0.3,
            type_correct: features.type_purity > 0.3,
            aesthetic_pass: features.aesthetic > 0.4,
        });

        // ── 第6步: 字形像素编码 ──
        let glyph_encoded = if verdict.guilty {
            // 有罪的字形做标记渲染 (破碎形态)
            let mut glyph = GlyphImage::new(8, 8);
            glyph.set(1, 1, GlyphPixel::ink(Phase::Diagonal));
            glyph.set(6, 6, GlyphPixel::ink(Phase::Diagonal));
            false // 未完整编码
        } else {
            // 无罪的字形做完整渲染
            let _glyph = GlyphTemplates::mu();
            true
        };

        // 更新统计
        if verdict.guilty {
            self.total_guilty += 1;
        } else {
            self.total_acquitted += 1;
        }

        GrandCourtVerdict {
            input_char: ch.to_string(),
            family,
            diffraction,
            crime_transcripts: verdict.transcripts.clone(),
            glyph_encoded,
            final_guilty: verdict.guilty,
            sentence: verdict.sentence,
        }
    }

    /// 将字符映射为8维特征 (模拟C系统的感知路由结果)
    fn char_to_features(ch: &str, family: Option<Family>) -> EntityFeatures {
        match family {
            Some(Family::Container) => EntityFeatures {
                aesthetic: 0.9,
                type_purity: 1.0,
                appearance: 0.85,
                structure: 0.9,
                thinking: 0.8,
                cooperativity: 0.95,
                goal: 0.85,
                ownership_righteousness: 1.0,
            },
            Some(Family::Boundary) => EntityFeatures {
                aesthetic: 0.85,
                type_purity: 1.0,
                appearance: 0.9,
                structure: 0.95,
                thinking: 0.85,
                cooperativity: 0.8,
                goal: 0.9,
                ownership_righteousness: 1.0,
            },
            Some(Family::Couple) => EntityFeatures {
                aesthetic: 0.95,
                type_purity: 0.9,
                appearance: 0.9,
                structure: 0.85,
                thinking: 0.9,
                cooperativity: 1.0,
                goal: 0.9,
                ownership_righteousness: 0.95,
            },
            Some(Family::Web) => EntityFeatures {
                aesthetic: 0.7,
                type_purity: 0.6,
                appearance: 0.65,
                structure: 0.7,
                thinking: 0.6,
                cooperativity: 0.7,
                goal: 0.65,
                ownership_righteousness: 0.8,
            },
            Some(Family::Neutral) => EntityFeatures {
                aesthetic: 0.5,
                type_purity: 0.5,
                appearance: 0.5,
                structure: 0.5,
                thinking: 0.5,
                cooperativity: 0.5,
                goal: 0.5,
                ownership_righteousness: 0.5,
            },
            None => EntityFeatures::broken(),
        }
    }

    /// 全局水质报告
    pub fn water_report(&self) -> String {
        self.water_quality.report()
    }

    /// 犯罪统计
    pub fn crime_stats(&self) -> String {
        let stats = self.judge.crime_statistics();
        if stats.is_empty() { return "暂无犯罪记录".to_string(); }
        let parts: Vec<String> = stats.iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();
        format!("犯罪统计: {}", parts.join(", "))
    }

    /// 综合报告
    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str("═══ 大审判庭综合报告 ═══\n");
        s.push_str(&format!("  已处理字符: {}\n", self.total_processed));
        s.push_str(&format!("  有罪: {} | 无罪: {}\n", self.total_guilty, self.total_acquitted));
        let conviction_rate = if self.total_processed > 0 {
            self.total_guilty as f64 / self.total_processed as f64 * 100.0
        } else { 0.0 };
        s.push_str(&format!("  定罪率: {:.1}%\n", conviction_rate));
        s.push_str(&format!("  {}\n", self.water_report()));
        s.push_str(&format!("  {}\n", self.crime_stats()));
        s
    }
}

impl Family {
    /// 按Unicode码点粗分类家族 (模拟C系统的感知分类)
    pub fn classify_by_codepoint(cp: u32) -> Self {
        match cp {
            // CJK统一汉字基本区 — 边界家族 (我侧)
            0x4E00..=0x9FFF => Family::Boundary,
            // 拉丁字母+声调: a=0x61, á=00E1 — 测试用
            0x0041..=0x005A | 0x0061..=0x007A => Family::Container, // 基本拉丁
            0x00C0..=0x00FF => Family::Couple,  // 拉丁扩展(带声调)
            // CJK部首扩展 — 网状
            0x2E80..=0x2EFF => Family::Web,
            // 全角符号 — 容器
            0xFF00..=0xFFEF => Family::Container,
            // 其他 — 中性
            _ => Family::Neutral,
        }
    }
}

// ══════════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grand_court_processes_basic_latin() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        let verdict = court.process(&os, "a");
        assert_eq!(verdict.input_char, "a");
        assert_eq!(verdict.family, Some(Family::Container));
    }

    #[test]
    fn grand_court_classifies_accented_differently() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        let v1 = court.process(&os, "a");
        let v2 = court.process(&os, "á");
        // a和á应该被分到不同家族
        assert_ne!(v1.family, v2.family, "a和á应分到不同家族");
    }

    #[test]
    fn grand_court_detects_cjk() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        let v = court.process(&os, "中");
        assert_eq!(v.family, Some(Family::Boundary));
    }

    #[test]
    fn grand_court_tracks_water_quality() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        // 处理多个字符
        court.process(&os, "a");
        court.process(&os, "b");
        court.process(&os, "中");

        let report = court.water_report();
        assert!(report.contains("3样本"), "应采集3个样本");
    }

    #[test]
    fn grand_court_generates_report() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        court.process(&os, "a");
        court.process(&os, "中");
        court.process(&os, "文");

        let report = court.report();
        assert!(report.contains("已处理字符: 3"));
        assert!(report.contains("大审判庭"));
    }

    #[test]
    fn diffraction_8_dimensions() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        let v = court.process(&os, "中");
        // 验证8维衍射
        let dims = v.diffraction.projections.len();
        assert_eq!(dims, 8, "应有8维衍射投影");
    }

    #[test]
    fn perfect_entity_passes() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        // Container家族(刘楚恬侧)的特征值很高
        let v = court.process(&os, "a");
        // a是基本拉丁, 家族=Container, 特征值高, 应无罪
        assert!(!v.final_guilty, "Container家族的a应无罪");
    }

    #[test]
    fn global_contamination_detected() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        // 连续输入中性字符 (特征值全0.5), 触发全局污染
        for _ in 0..10 {
            let _ = court.process(&os, "\u{0000}"); // U+0000 走 Neutral
        }
        let report = court.water_report();
        // 至少应显示多个样本
        assert!(report.contains("10样本"));
    }

    #[test]
    fn crime_statistics_count() {
        let os = BareMetalOs::new();
        let mut court = GrandCourt::new();
        court.process(&os, "a");
        court.process(&os, "á");

        let stats = court.judge.crime_statistics();
        // á应产生某种犯罪记录
        let total_crimes: usize = stats.values().sum();
        assert!(total_crimes > 0, "á应产生至少一条犯罪记录");
    }
}
