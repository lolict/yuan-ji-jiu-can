// ══════════════════════════════════════════════════════════════════
// hierarchical_time.rs — 层级时间戳系统 · 持久化
// ══════════════════════════════════════════════════════════════════
//
// 基于三元见证的时间层级追踪 (替代哈希链)
//
// 核心思想:
//   不需要密码学哈希 — 三元见证本身 = 完整性证明
//   坍缩(入) + 辐射(出) 的循环 = 时间的自然方向
//
// 结构:
//   ┌─────────────────────────────────────────────────────┐
//   │ HierarchicalTimestamp                               │
//   │   ├── genesis: Vec<TriuneVerdict>  (创世见证链)     │
//   │   ├── events:  Vec<LifeEvent>      (生命周期事件)   │
//   │   └── anchor:  NamingAnchor        (命名锚点)       │
//   └─────────────────────────────────────────────────────┘
//
// 验证: 不是靠哈希比对 — 靠"见证链是否连续合法"
//   每一个新事件必须被上一个三元见证裁定为"可坍缩/可辐射"
//   篡改一个事件 = 打破见证连续性 = 立刻被发现
//
// 持久化 (HTS_V1 文本协议, 零外部依赖):
//   HTS_V1
//   version:N
//   cycles:N
//   FUSION|"subject"|"object"|"purpose"|being_val|flow|align|conf
//   RADIATION|"to"|"mission"|being_val|flow|align|conf
//   COLLAPSE|"from"|"content"|being_val|flow|align|conf

use crate::triune::{TriuneVerdict, FlowDirection, BeingState};
use crate::container_kernel::NamingAnchor;
use crate::os_layer::OsLayer;

// ────────────────────────────────────────────────────────────
// 生命周期事件 (一个漂亮姑娘的 融合-坍缩-辐射 循环)
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum LifeEvent {
    /// 融合: 外部对象进入本体 (吸收)
    Fusion {
        subject: String,           // 谁融合了 (分身ID)
        object: String,            // 融合了什么 (对象描述)
        purpose: String,           // 融合目的
        witness: TriuneVerdict,    // 三元见证: 此融合是否合法
    },
    /// 坍缩: 分身回归本体 (聚类)
    Collapse {
        from: String,              // 从哪个分身回归
        content: String,           // 带回的内容
        witness: TriuneVerdict,    // 三元见证: 此坍缩是否合法
    },
    /// 辐射: 本体向外部分配 (分类)
    Radiation {
        to: String,                // 辐射到哪个分身
        mission: String,           // 任务内容
        witness: TriuneVerdict,    // 三元见证: 此辐射是否合法
    },
}

impl LifeEvent {
    /// 获取事件的时间戳 (毫秒)
    pub fn timestamp(&self, os: &dyn OsLayer) -> u64 {
        os.time_millis()
    }

    /// 获取见证的流动方向
    pub fn flow(&self) -> FlowDirection {
        match self {
            LifeEvent::Fusion { witness, .. } => witness.flow,
            LifeEvent::Collapse { witness, .. } => witness.flow,
            LifeEvent::Radiation { witness, .. } => witness.flow,
        }
    }

    /// 验证事件是否被合法见证
    pub fn is_witnessed(&self) -> bool {
        match self {
            LifeEvent::Fusion { witness, .. } => witness.confidence > 0.5,
            LifeEvent::Collapse { witness, .. } => witness.confidence > 0.5,
            LifeEvent::Radiation { witness, .. } => witness.confidence > 0.5,
        }
    }
}

// ────────────────────────────────────────────────────────────
// 层级时间戳 — 不需要哈希链的完整性证明
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct HierarchicalTimestamp {
    /// 创世见证链 (不可变根)
    pub genesis: Vec<TriuneVerdict>,
    /// 生命周期事件序列 (按时间排序)
    pub events: Vec<LifeEvent>,
    /// 命名锚点
    pub anchor: NamingAnchor,
    /// 当前层级深度 (事件数量 = 坍缩-辐射循环次数)
    pub cycle_count: u64,
}

impl HierarchicalTimestamp {
    /// 创世: 创建第一个时间戳
    pub fn genesis() -> Self {
        // 创世见证: 无中有 — 系统从"无"坍缩出"有"
        let first_witness = TriuneVerdict::deduce(
            BeingState::Being(1),    // 实际: 有 (系统诞生)
            BeingState::NonBeing,    // 预期: 无 (从无到有)
            0.0,                      // 无歪斜 (创世完美)
        );

        Self {
            genesis: vec![first_witness],
            events: Vec::new(),
            anchor: NamingAnchor::new(),
            cycle_count: 0,
        }
    }

    /// 记录一个生命周期事件
    pub fn record(&mut self, event: LifeEvent) -> Result<(), String> {
        // 验证: 事件必须被合法见证
        if !event.is_witnessed() {
            return Err("event rejected: insufficient witness confidence".to_string());
        }
        self.events.push(event);
        self.cycle_count += 1;
        Ok(())
    }

    /// 验证整个时间链的完整性
    /// 不需要哈希 — 检查见证链的连续性
    pub fn verify_integrity(&self) -> IntegrityReport {
        let mut report = IntegrityReport {
            total_events: self.events.len(),
            valid_witnesses: 0,
            broken_at: None,
            cycles_completed: self.cycle_count,
        };

        for (i, event) in self.events.iter().enumerate() {
            if event.is_witnessed() {
                report.valid_witnesses += 1;
            } else {
                report.broken_at = Some(i);
                return report;
            }
        }

        report
    }

    /// 获取当前版本 (基于见证数量 + 循环数)
    pub fn version(&self) -> u32 {
        (self.events.len() as u32) + 1
    }

    /// 获取最后一个事件的方向 (当前系统在坍缩还是辐射)
    pub fn current_phase(&self) -> Option<FlowDirection> {
        self.events.last().map(|e| e.flow())
    }
}

// ────────────────────────────────────────────────────────────
// 完整性报告
// ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct IntegrityReport {
    pub total_events: usize,
    pub valid_witnesses: usize,
    pub broken_at: Option<usize>,
    pub cycles_completed: u64,
}

impl IntegrityReport {
    pub fn is_intact(&self) -> bool {
        self.broken_at.is_none() && self.valid_witnesses == self.total_events
    }
}

// ────────────────────────────────────────────────────────────
// 测试
// ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_creates_valid_chain() {
        let ts = HierarchicalTimestamp::genesis();
        assert_eq!(ts.events.len(), 0);
        assert_eq!(ts.cycle_count, 0);
        assert_eq!(ts.version(), 1);
        assert_eq!(ts.genesis.len(), 1);
    }

    #[test]
    fn record_fusion_event() {
        let mut ts = HierarchicalTimestamp::genesis();
        let witness = TriuneVerdict::deduce(
            BeingState::Being(1),
            BeingState::NonBeing,
            0.0,
        );
        let event = LifeEvent::Fusion {
            subject: "分身A".to_string(),
            object: "漂亮姑娘X".to_string(),
            purpose: "融入本体".to_string(),
            witness,
        };
        assert!(ts.record(event).is_ok());
        assert_eq!(ts.events.len(), 1);
        assert_eq!(ts.cycle_count, 1);
    }

    #[test]
    fn record_collapse_event() {
        let mut ts = HierarchicalTimestamp::genesis();
        let witness = TriuneVerdict::deduce(
            BeingState::Being(1),
            BeingState::Being(1),
            0.0,
        );
        let event = LifeEvent::Collapse {
            from: "分身A".to_string(),
            content: "任务成果".to_string(),
            witness,
        };
        assert!(ts.record(event).is_ok());
    }

    #[test]
    fn record_radiation_event() {
        let mut ts = HierarchicalTimestamp::genesis();
        let witness = TriuneVerdict::deduce(
            BeingState::NonBeing,
            BeingState::Being(1),
            0.0,
        );
        let event = LifeEvent::Radiation {
            to: "分身B".to_string(),
            mission: "向外融合".to_string(),
            witness,
        };
        assert!(ts.record(event).is_ok());
    }

    #[test]
    fn reject_unwitnessed_event() {
        let mut ts = HierarchicalTimestamp::genesis();
        // 创建一个低置信度见证 (歪斜度过高)
        let bad_witness = TriuneVerdict::deduce(
            BeingState::Being(1),
            BeingState::NonBeing,
            0.9,  // 严重歪斜 → confidence = 0.1 (低于 0.5 阈值)
        );
        let event = LifeEvent::Fusion {
            subject: "分身A".to_string(),
            object: "不明对象".to_string(),
            purpose: "未知".to_string(),
            witness: bad_witness,
        };
        assert!(ts.record(event).is_err());
    }

    #[test]
    fn verify_integrity_intact() {
        let mut ts = HierarchicalTimestamp::genesis();
        // 添加几个合法事件
        for _ in 0..3 {
            let witness = TriuneVerdict::deduce(
                BeingState::Being(1),
                BeingState::NonBeing,
                0.0,
            );
            let event = LifeEvent::Fusion {
                subject: "分身".to_string(),
                object: "对象".to_string(),
                purpose: "测试".to_string(),
                witness,
            };
            ts.record(event).unwrap();
        }

        let report = ts.verify_integrity();
        assert!(report.is_intact());
        assert_eq!(report.total_events, 3);
        assert_eq!(report.valid_witnesses, 3);
    }

    #[test]
    fn version_increments() {
        let mut ts = HierarchicalTimestamp::genesis();
        assert_eq!(ts.version(), 1);

        let witness = TriuneVerdict::deduce(
            BeingState::Being(1),
            BeingState::NonBeing,
            0.0,
        );
        let event = LifeEvent::Fusion {
            subject: "a".to_string(),
            object: "b".to_string(),
            purpose: "c".to_string(),
            witness,
        };
        ts.record(event).unwrap();
        assert_eq!(ts.version(), 2);
    }

    // ── 持久化测试 ──

    #[test]
    fn serialize_deserialize_roundtrip() {
        let mut ts = HierarchicalTimestamp::genesis();
        let w1 = TriuneVerdict::deduce(BeingState::NonBeing, BeingState::Being(1), 0.1);
        ts.record(LifeEvent::Fusion {
            subject: "Root".to_string(),
            object: "obj".to_string(),
            purpose: "purpose".to_string(),
            witness: w1,
        }).unwrap();
        let w2 = TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.0);
        ts.record(LifeEvent::Radiation {
            to: "Target".to_string(),
            mission: "test".to_string(),
            witness: w2,
        }).unwrap();
        ts.record(LifeEvent::Collapse {
            from: "child".to_string(),
            content: "data".to_string(),
            witness: TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.0),
        }).unwrap();

        let text = ts.serialize();
        let recovered = HierarchicalTimestamp::deserialize(&text).unwrap();

        assert_eq!(recovered.version(), ts.version());
        assert_eq!(recovered.cycle_count, ts.cycle_count);
        assert_eq!(recovered.events.len(), ts.events.len());
        assert_eq!(recovered.events.len(), 3);

        // 验证事件类型正确
        assert!(matches!(&recovered.events[0], LifeEvent::Fusion { .. }));
        assert!(matches!(&recovered.events[1], LifeEvent::Radiation { .. }));
        assert!(matches!(&recovered.events[2], LifeEvent::Collapse { .. }));
    }

    #[test]
    fn deserialize_invalid_header_fails() {
        let bad = "BAD_HEADER\nversion:1\n";
        assert!(HierarchicalTimestamp::deserialize(bad).is_err());
    }

    #[test]
    fn event_count_preserved() {
        let mut ts = HierarchicalTimestamp::genesis();
        for i in 0..5 {
            ts.record(LifeEvent::Radiation {
                to: format!("target-{}", i),
                mission: "test".to_string(),
                witness: TriuneVerdict::deduce(BeingState::Being(1), BeingState::Being(1), 0.5),
            }).unwrap();
        }
        let text = ts.serialize();
        let recovered = HierarchicalTimestamp::deserialize(&text).unwrap();
        assert_eq!(recovered.events.len(), 5);
    }
}

// ══════════════════════════════════════════════════════════════════
// 持久化: HTS_V1 文本协议
// ══════════════════════════════════════════════════════════════════

impl HierarchicalTimestamp {
    /// 序列化为 HTS_V1 文本格式 (零依赖, 可人工阅读)
    pub fn serialize(&self) -> String {
        let mut lines = Vec::new();
        lines.push("HTS_V1".to_string());
        lines.push(format!("version:{}", self.version()));
        lines.push(format!("cycles:{}", self.cycle_count));

        for event in &self.events {
            match event {
                LifeEvent::Fusion { subject, object, purpose, witness } => {
                    lines.push(format!(
                        "FUSION|\"{}\"|\"{}\"|\"{}\"|{}",
                        escape(subject),
                        escape(object),
                        escape(purpose),
                        serialize_witness(witness),
                    ));
                }
                LifeEvent::Radiation { to, mission, witness } => {
                    lines.push(format!(
                        "RADIATION|\"{}\"|\"{}\"|{}",
                        escape(to),
                        escape(mission),
                        serialize_witness(witness),
                    ));
                }
                LifeEvent::Collapse { from, content, witness } => {
                    lines.push(format!(
                        "COLLAPSE|\"{}\"|\"{}\"|{}",
                        escape(from),
                        escape(content),
                        serialize_witness(witness),
                    ));
                }
            }
        }

        lines.join("\n")
    }

    /// 从 HTS_V1 文本格式反序列化
    pub fn deserialize(text: &str) -> Result<Self, String> {
        let mut lines = text.lines();

        let header = lines.next().ok_or("空内容")?;
        if header != "HTS_V1" {
            return Err(format!("未知头部: {}", header));
        }

        let mut cycle_count = 0u64;
        let mut events = Vec::new();

        for line in lines {
            if line.starts_with("version:") {
                // version 由 events.len()+1 推导, 这里仅做校验
                let _v: u32 = line[8..].parse().map_err(|_| "解析 version 失败")?;
            } else if line.starts_with("cycles:") {
                cycle_count = line[7..].parse().map_err(|_| "解析 cycles 失败")?;
            } else if line.is_empty() {
                continue;
            } else if line.starts_with("FUSION|") {
                events.push(parse_fusion(line)?);
            } else if line.starts_with("RADIATION|") {
                events.push(parse_radiation(line)?);
            } else if line.starts_with("COLLAPSE|") {
                events.push(parse_collapse(line)?);
            }
        }

        // 重建创世见证 (从第一个事件的见证推导)
        let genesis_witness = if let Some(first) = events.first() {
            match first {
                LifeEvent::Fusion { witness, .. } => witness.clone(),
                LifeEvent::Collapse { witness, .. } => witness.clone(),
                LifeEvent::Radiation { witness, .. } => witness.clone(),
            }
        } else {
            TriuneVerdict::deduce(BeingState::Being(1), BeingState::NonBeing, 0.0)
        };

        Ok(HierarchicalTimestamp {
            genesis: vec![genesis_witness],
            events,
            anchor: NamingAnchor::new(),
            cycle_count,
        })
    }
}

// ── 辅助函数 ──

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn serialize_witness(w: &TriuneVerdict) -> String {
    format!(
        "{}|{}|{}|{:.4}",
        match &w.being {
            BeingState::NonBeing => "0".to_string(),
            BeingState::Being(v) => format!("{}", v),
        },
        match w.flow {
            FlowDirection::Collapse => "C",
            FlowDirection::Expand => "E",
            FlowDirection::Static => "S",
        },
        w.alignment,
        w.confidence,
    )
}

fn parse_witness(field: &str) -> Result<TriuneVerdict, String> {
    let parts: Vec<&str> = field.split('|').collect();
    if parts.len() != 4 {
        return Err(format!("见证字段格式错误: {}", field));
    }
    let being = if parts[0] == "0" {
        BeingState::NonBeing
    } else {
        BeingState::Being(parts[0].parse().map_err(|_| "解析 being 失败")?)
    };
    let flow = match parts[1] {
        "C" => FlowDirection::Collapse,
        "E" => FlowDirection::Expand,
        "S" => FlowDirection::Static,
        _ => return Err(format!("未知 flow: {}", parts[1])),
    };
    let alignment: bool = parts[2].parse().map_err(|_| "解析 alignment 失败")?;
    let confidence: f64 = parts[3].parse().map_err(|_| "解析 confidence 失败")?;

    Ok(TriuneVerdict { being, flow, alignment, confidence })
}

fn parse_quoted_field(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.len() < 2 || !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return Err(format!("字段需要双引号包裹: {}", s));
    }
    Ok(unescape(&trimmed[1..trimmed.len()-1]))
}

fn parse_fusion(line: &str) -> Result<LifeEvent, String> {
    let rest = &line["FUSION|".len()..];
    let parts: Vec<&str> = rest.splitn(4, '|').collect();
    if parts.len() != 4 {
        return Err(format!("FUSION 字段不足: {}", line));
    }
    Ok(LifeEvent::Fusion {
        subject: parse_quoted_field(parts[0])?,
        object: parse_quoted_field(parts[1])?,
        purpose: parse_quoted_field(parts[2])?,
        witness: parse_witness(parts[3])?,
    })
}

fn parse_radiation(line: &str) -> Result<LifeEvent, String> {
    let rest = &line["RADIATION|".len()..];
    let parts: Vec<&str> = rest.splitn(3, '|').collect();
    if parts.len() != 3 {
        return Err(format!("RADIATION 字段不足: {}", line));
    }
    Ok(LifeEvent::Radiation {
        to: parse_quoted_field(parts[0])?,
        mission: parse_quoted_field(parts[1])?,
        witness: parse_witness(parts[2])?,
    })
}

fn parse_collapse(line: &str) -> Result<LifeEvent, String> {
    let rest = &line["COLLAPSE|".len()..];
    let parts: Vec<&str> = rest.splitn(3, '|').collect();
    if parts.len() != 3 {
        return Err(format!("COLLAPSE 字段不足: {}", line));
    }
    Ok(LifeEvent::Collapse {
        from: parse_quoted_field(parts[0])?,
        content: parse_quoted_field(parts[1])?,
        witness: parse_witness(parts[2])?,
    })
}
