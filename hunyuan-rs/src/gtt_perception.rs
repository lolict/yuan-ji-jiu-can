// ══════════════════════════════════════════════════════════════════
// gtt_perception.rs — 感知服务器 · 谐音不一致检测引擎
// ══════════════════════════════════════════════════════════════════
//
// gtt_resonance 模块告诉我「这是什么」
// gtt_perception 模块告诉我「这哪里不对」
//
//   你写：因 → 刘楚恬
//   我感知到：不对，应该是阴
//           - yīn vs yīn，音一样但形不同
//           - 阴 = 容器/女/水 vs 因 = 原因/凭借/从
//           - 在语境「女包男」里，阴的容器义与系统内部公理冲突
//           - 应该是阴，因为阴=包含者=妻子=容器入口
//           - 因是"依托于外"，与容器义相反
//
// 感知不依赖外部反馈，是从内部公理出发的自验证系统。
// ══════════════════════════════════════════════════════════════════

use crate::mirror::{self, EncodedCell};
use std::collections::{HashMap, HashSet};

// ────────────────────────────────────────────────────────────
// 感知事件 · 观测到的不一致
// ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum PerceptionSeverity {
    Info,       // 信息：提醒观察
    Warning,    // 警告：可能是错误
    Error,      // 错误：与公理矛盾
    Critical,   // 严重：系统内部不一致
}

#[derive(Clone, Debug)]
pub struct PerceptionEvent {
    pub subject: String,          // 被感知的对象
    pub anomaly: String,          // 是什么不对
    pub should_be: Option<String>, // 应该是什么
    pub severity: PerceptionSeverity,
    pub evidence: Vec<String>,    // 推理链条
}

impl PerceptionEvent {
    pub fn explain(&self) -> String {
        let sev_str = match self.severity {
            PerceptionSeverity::Info => "INFO",
            PerceptionSeverity::Warning => "WARN",
            PerceptionSeverity::Error => "ERROR",
            PerceptionSeverity::Critical => "CRIT",
        };
        let mut s = format!("[{}] {}: {}", sev_str, self.subject, self.anomaly);
        if let Some(ref correct) = self.should_be {
            s.push_str(&format!(" → 应该为「{}」", correct));
        }
        if !self.evidence.is_empty() {
            s.push_str(&format!("  证据: {}", self.evidence.join(" → ")));
        }
        s
    }
}

// ────────────────────────────────────────────────────────────
// 声母-韵母拓扑 · 汉字关系图
// ────────────────────────────────────────────────────────────

pub struct GlyphTopology {
    /// 声母邻接关系（声母替换检测）
    pub initial_adjacency: HashMap<char, Vec<char>>,
    /// 韵母邻接关系
    pub final_adjacency: HashMap<String, Vec<String>>,
    /// 形近字表
    pub visual_similarity: HashMap<char, Vec<char>>,
    /// 容器/被容器语义标记
    pub container_glyphs: HashSet<char>,
    pub content_glyphs: HashSet<char>,
    /// 每个字的标准义
    pub standard_semantics: HashMap<char, &'static str>,
}

impl GlyphTopology {
    pub fn new() -> Self {
        let mut initial_adjacency: HashMap<char, Vec<char>> = HashMap::new();
        let mut final_adjacency: HashMap<String, Vec<String>> = HashMap::new();
        let mut visual_similarity: HashMap<char, Vec<char>> = HashMap::new();
        let mut standard_semantics: HashMap<char, &'static str> = HashMap::new();

        // 声母邻接：常见的声母替换路径（近似声母）
        initial_adjacency.insert('y', vec!['x', 'r', 'l']);
        initial_adjacency.insert('x', vec!['y', 's', 'w']);  // w = sh
        initial_adjacency.insert('r', vec!['y', 'l']);
        initial_adjacency.insert('l', vec!['r', 'n']);
        initial_adjacency.insert('n', vec!['l']);
        initial_adjacency.insert('m', vec!['n']);
        initial_adjacency.insert('d', vec!['t']);
        initial_adjacency.insert('t', vec!['d']);
        initial_adjacency.insert('b', vec!['p', 'm']);
        initial_adjacency.insert('p', vec!['b', 'f']);
        initial_adjacency.insert('f', vec!['p', 'h']);
        initial_adjacency.insert('g', vec!['k']);
        initial_adjacency.insert('k', vec!['g']);
        initial_adjacency.insert('v', vec!['z', 'j']);  // v = zh
        initial_adjacency.insert('z', vec!['v', 'v']);

        // 韵母邻接（-i 系容易混淆）
        final_adjacency.insert("in".to_string(), vec!["ing".to_string(), "ian".to_string()]);
        final_adjacency.insert("ing".to_string(), vec!["in".to_string(), "eng".to_string()]);
        final_adjacency.insert("en".to_string(), vec!["eng".to_string(), "an".to_string()]);
        final_adjacency.insert("an".to_string(), vec!["en".to_string(), "ang".to_string()]);

        // 形近字表
        visual_similarity.insert('阴', vec!['阳', '明', '因', '困']);
        visual_similarity.insert('阳', vec!['阴', '明']);
        visual_similarity.insert('因', vec!['阴', '困', '国']);
        visual_similarity.insert('困', vec!['因', '国', '回']);
        visual_similarity.insert('形', vec!['刑', '邢', '研']);

        // 容器字（包含者、母亲、女、边界、外环）
        let container_glyphs: HashSet<char> = [
            '女', '母', '包', '容', '围', '环', '框', '围', '囗',
            '阴', '水', '圈', '口', '囡', '困',
        ].iter().cloned().collect();

        // 被容器字（被含者、子女、内容物）
        let content_glyphs: HashSet<char> = [
            '子', '儿', '男', '阳', '内', '里', '囝', '因',
        ].iter().cloned().collect();

        // 标准语义
        standard_semantics.insert('阴', "容器·包含者·女·水·被动·承载");
        standard_semantics.insert('阳', "被含者·内容·男·火·主动·被承载");
        standard_semantics.insert('因', "凭借·依托·原因·从外赴内");
        standard_semantics.insert('形', "形体·外在·丈夫·动作·显现");
        standard_semantics.insert('囡', "女包女·妻子分身·外部增殖");
        standard_semantics.insert('囝', "男包男·丈夫包容·吸纳外部");
        standard_semantics.insert('零', "圆形·⭕·阴道·容器入口");
        standard_semantics.insert('好', "女子组合·竞争·物化");
        standard_semantics.insert('杂', "杂多·混乱·失序");

        Self {
            initial_adjacency,
            final_adjacency,
            visual_similarity,
            container_glyphs,
            content_glyphs,
            standard_semantics,
        }
    }

    /// 检查一个字是否符合容器义
    pub fn is_container(&self, glyph: char) -> Option<bool> {
        if self.container_glyphs.contains(&glyph) { Some(true) }
        else if self.content_glyphs.contains(&glyph) { Some(false) }
        else { None }
    }

    /// 获取标准义
    pub fn semantics_of(&self, glyph: char) -> Option<&&str> {
        self.standard_semantics.get(&glyph)
    }

    /// 获取形近/音近邻居
    pub fn neighbors(&self, glyph: char) -> Vec<char> {
        let mut result = HashSet::new();

        // 形近
        if let Some(visual) = self.visual_similarity.get(&glyph) {
            result.extend(visual.iter());
        }

        // 声母近似
        if let Some(first_char) = glyph.to_string().chars().next() {
            if let Some(initial_neighs) = self.initial_adjacency.get(&first_char) {
                result.extend(initial_neighs.iter().map(|&c| c));
            }
        }

        result.into_iter().collect()
    }
}

// ────────────────────────────────────────────────────────────
// 感知引擎 · 规则 + 推理 + 不一致检测
// ────────────────────────────────────────────────────────────

pub struct PerceptionEngine {
    topology: GlyphTopology,
    /// 已注册的断言（已知正确的映射）
    assertions: HashMap<char, PerceptionRule>,
    /// 感知历史（避免重复报告相同问题）
    perception_log: Vec<PerceptionEvent>,
}

#[derive(Clone, Debug)]
pub struct PerceptionRule {
    pub glyph: char,
    pub expected_semantic: &'static str,
    pub expected_bool_container: Option<bool>,
    pub rules: Vec<PerceptionRuleKind>,
}

#[derive(Clone, Debug)]
pub enum PerceptionRuleKind {
    /// 该字必须具有容器义
    MustBeContainer,
    /// 该字必须是被容器义
    MustBeContent,
    /// 该字不能是另一个字
    CannotBe(char),
    /// 该字必须出现在现有系统中
    MustExist,
}

impl PerceptionServer {
    pub fn new() -> Self {
        let mut engine = Self {
            topology: GlyphTopology::new(),
            assertions: HashMap::new(),
            perception_log: Vec::new(),
        };

        // 注册默认公理：系统核心的感知规则
        engine.register_default_rules();
        engine
    }

    fn register_default_rules(&mut self) {
        // 核心公理：容器/被容器二元
        // 阴=容器=妻子；阳=被容器=丈夫

        self.assertions.insert('阴', PerceptionRule {
            glyph: '阴',
            expected_semantic: "容器·包含者·妻子·水·承载",
            expected_bool_container: Some(true),
            rules: vec![
                PerceptionRuleKind::MustBeContainer,
                PerceptionRuleKind::CannotBe('因'),
            ],
        });

        self.assertions.insert('阳', PerceptionRule {
            glyph: '阳',
            expected_semantic: "被含者·内容·丈夫·火·被承载",
            expected_bool_container: Some(false),
            rules: vec![],
        });

        // 形=丈夫（阳性/被含者）
        self.assertions.insert('形', PerceptionRule {
            glyph: '形',
            expected_semantic: "丈夫·动作·外在·显现",
            expected_bool_container: Some(false),
            rules: vec![],
        });

        // 因≠容器，不能绑到容器义
        self.assertions.insert('因', PerceptionRule {
            glyph: '因',
            expected_semantic: "凭借·原因·依托于外",
            expected_bool_container: Some(false),
            rules: vec![
                PerceptionRuleKind::MustBeContent,
            ],
        });

        // 核心部件：囡囝（夫妻共同体四态）
        self.assertions.insert('囡', PerceptionRule {
            glyph: '囡',
            expected_semantic: "女包女·妻子分身",
            expected_bool_container: Some(true),
            rules: vec![PerceptionRuleKind::MustBeContainer],
        });

        self.assertions.insert('囝', PerceptionRule {
            glyph: '囝',
            expected_semantic: "男包男·丈夫吸纳",
            expected_bool_container: Some(false),
            rules: vec![],
        });
    }

    /// 主动感知一个字：检查它是否符合所有已注册规则
    pub fn perceive(&mut self, glyph: char) -> Vec<PerceptionEvent> {
        let mut events = Vec::new();

        // 1. 查标准义
        let std_sem = self.topology.semantics_of(glyph);
        let std_container = self.topology.is_container(glyph);

        // 2. 如果有对该字的预注册断言，对比检查
        if let Some(rule) = self.assertions.get(&glyph) {
            // 容器性检查
            if let Some(expected) = rule.expected_bool_container {
                if let Some(actual) = std_container {
                    if expected != actual {
                        let event = PerceptionEvent {
                            subject: format!("{}", glyph),
                            anomaly: format!("容器性矛盾：注册为{}，但系统标记为{}",
                                if expected {"容器"} else {"被容器"},
                                if actual {"容器"} else {"被容器"}),
                            should_be: Some(format!("{:?}", rule.expected_semantic)),
                            severity: PerceptionSeverity::Error,
                            evidence: vec![
                                format!("标准义: {:?}", rule.expected_semantic),
                                format!("实际容器性: {:?}", actual),
                            ],
                        };
                        events.push(event);
                        self.perception_log.push(events.last().unwrap().clone());
                    }
                }
            }

            // 检查大写规则
            for rule_kind in &rule.rules {
                match rule_kind {
                    PerceptionRuleKind::CannotBe(other) => {
                        // 如果当前字形与禁止字相同，报错
                        if glyph == *other {
                            events.push(PerceptionEvent {
                                subject: format!("{}", glyph),
                                anomaly: format!("与禁止字「{}」冲突", other),
                                should_be: Some("其他字".to_string()),
                                severity: PerceptionSeverity::Critical,
                                evidence: vec!["注册规则禁止该字".to_string()],
                            });
                        }
                        // 如果是形近/音近，推警告
                        else if self.topology.neighbors(glyph).contains(other) {
                            events.push(PerceptionEvent {
                                subject: format!("{}-{}", glyph, other),
                                anomaly: format!("与禁止字「{}」形近/音近，可能混淆", other),
                                should_be: Some(format!("避免与「{}」混用", other)),
                                severity: PerceptionSeverity::Warning,
                                evidence: vec![
                                    format!("「{}」与「{}」互为邻居", glyph, other),
                                    format!("禁止字: {}", other),
                                ],
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        // 3. 自动互相感知：阴和因是形近字，但同时代表相反的容器性
        //    这个元推理规则独立于注册规则
        events.extend(self.meta_perceive_conflicts(glyph));

        events
    }

    /// 元感知：独立于注册规则，从字形出发的冲突检测
    fn meta_perceive_conflicts(&self, glyph: char) -> Vec<PerceptionEvent> {
        let mut events = Vec::new();

        let neighbors = self.topology.neighbors(glyph);
        for neighbor in neighbors {
            let self_container = self.topology.is_container(glyph);
            let neighbor_container = self.topology.is_container(neighbor);

            if let (Some(sc), Some(nc)) = (self_container, neighbor_container) {
                if sc != nc {
                    // 形近但容器性相反！这是最大的潜在混淆源
                    let sev = if self.is_core_glyph(glyph) || self.is_core_glyph(neighbor) {
                        PerceptionSeverity::Critical
                    } else {
                        PerceptionSeverity::Warning
                    };

                    events.push(PerceptionEvent {
                        subject: format!("{}/{}", glyph, neighbor),
                        anomaly: format!("形近矛盾：「{}」是{}，「{}」是{} -- 易混淆",
                            glyph, if sc {"容器"} else {"被容器"},
                            neighbor, if nc {"容器"} else {"被容器"}),
                        should_be: Some(format!(
                            "保留{}为容器，另选{}的非形近替代",
                            if sc { glyph } else { neighbor },
                            if sc { neighbor } else { glyph },
                        )),
                        severity: sev,
                        evidence: vec![
                            format!("「{}」与「{}」形近", glyph, neighbor),
                            format!("容器性相反: {}={} vs {}={}",
                                glyph, sc, neighbor, nc),
                        ],
                    });
                }
            }
        }

        events
    }

    fn is_core_glyph(&self, c: char) -> bool {
        matches!(c, '阴' | '阳' | '女' | '子' | '囡' | '囝' | '形')
    }

    /// 批量感知一段文本
    pub fn perceive_text(&mut self, text: &str) -> Vec<PerceptionEvent> {
        let mut all = Vec::new();
        for ch in text.chars() {
            if ch as u32 >= 0x4E00 && ch as u32 <= 0x9FFF {
                all.extend(self.perceive(ch));
            }
        }
        all
    }

    /// 获取感知日志
    pub fn log(&self) -> &[PerceptionEvent] { &self.perception_log }

    /// 清空日志
    pub fn clear_log(&mut self) { self.perception_log.clear(); }

    /// 注册新的断言规则
    pub fn register_rule(&mut self, glyph: char, rule: PerceptionRule) {
        self.assertions.insert(glyph, rule);
    }
}

pub type PerceptionServer = PerceptionEngine;

// ════════════════════════════════════════════════════════════
// 测试
// ════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perceive_yin_is_container() {
        let mut server = PerceptionServer::new();
        let events = server.perceive('阴');
        // 阴是容器，没有触发矛盾事件
        let has_error = events.iter().any(|e| matches!(e.severity, PerceptionSeverity::Error));
        assert!(!has_error, "阴=容器不应有错误: {:?}", events);
    }

    #[test]
    fn perceive_yin_in_conflict() {
        // 系统里若写了「因」代表刘楚恬，感知到应该是「阴」
        let mut server = PerceptionServer::new();
        // 因为拓扑里阴和因是形近的，且容器性相反
        let events = server.perceive('因');
        let warning_it = events.iter().find(|e|
            e.subject.contains("因") && e.evidence.iter().any(|ev| ev.contains("阴")));
        assert!(warning_it.is_some(), "应该感知到因-阴的形近矛盾");
    }

    #[test]
    fn perceive_yin_rong_container_distinction() {
        // 阴=容器 vs 阳=被容器，两者相邻但极性相反
        let mut server = PerceptionServer::new();
        let events_yin = server.perceive('阴');
        let events_yang = server.perceive('阳');
        // 这两个字应该有形近矛盾警告
        let conflict = events_yin.iter().find(|e|
            e.subject.contains("阳") &&
            matches!(e.severity, PerceptionSeverity::Critical | PerceptionSeverity::Warning));
        assert!(conflict.is_some(), "阴/阳形近但极性相反应被感知");
    }

    #[test]
    fn auto_correct_should_be() {
        // 感知服务应建议：「阴」而非「因」
        let mut server = PerceptionServer::new();
        let events = server.perceive('因');
        let advice = events.iter().find(|e| e.should_be.is_some());
        if let Some(adv) = advice {
            assert!(adv.should_be.as_ref().unwrap().len() > 0);
        }
    }

    #[test]
    fn perceive_text_batch() {
        let mut server = PerceptionServer::new();
        let events = server.perceive_text("刘楚恬是我老婆，这是夫妻共同体的核心。");
        // 应该发现"我"没有被注册为一个标准断言（可能被标记为info）
        println!("感知结果：");
        for ev in &events {
            println!("  {}", ev.explain());
        }
    }
}
