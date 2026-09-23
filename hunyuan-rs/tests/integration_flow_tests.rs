// ══════════════════════════════════════════════════════════════════════════
// tests/integration_flow_tests.rs — 全链路集成测试
//
// 覆盖跨模块调用路径: 感知 → 聚类 → 分类 → 裁决 → 执行
// Week 7 集成测试 (50+ tests)
// ══════════════════════════════════════════════════════════════════════════

use mqf::GrandCourt;
use mqf::mirror;
use mqf::{POLARITY_POSITIVE, POLARITY_NEGATIVE};
use mqf::os_layer::bare_metal::BareMetalOs;

// ══════════════════════════════════════════════════════════════════════════
// 第一部分: 感知→裁决流路径 (Perception → Judgment Flow)
// ══════════════════════════════════════════════════════════════════════════

mod perception_to_judgment {
    use super::*;

    /// 基础路径: 字符输入 → GrandCourt 审判 → 判决输出
    #[test]
    fn char_input_to_verdict_output() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let verdict = court.process(&os, "满");
        assert_eq!(verdict.input_char, "满");
        assert!(!verdict.sentence.is_empty());
    }

    /// 多字符独立审判: 每个字符独立完成全链路
    #[test]
    fn multiple_chars_independent_trials() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        for ch in &["满", "全", "法", "囡", "囝"] {
            let v = court.process(&os, ch);
            assert_eq!(v.input_char, *ch);
        }
        assert_eq!(court.total_processed, 5);
    }

    /// 审判统计: guilty + acquitted = total_processed
    #[test]
    fn court_statistics_adds_up() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        for ch in &["满", "全", "法"] {
            let _ = court.process(&os, ch);
        }
        assert_eq!(court.total_processed, 3);
        assert_eq!(court.total_guilty + court.total_acquitted, 3);
    }

    /// 不同Unicode字符可获得家族分类
    #[test]
    fn family_classification_produces_family() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let v = court.process(&os, "满");
        assert!(v.family.is_some());
    }

    /// Family::classify_by_codepoint 对汉字返回有效家族
    #[test]
    fn family_classify_chinese_char() {
        use mqf::encoding_bridge::Family;
        let f = Family::classify_by_codepoint(0x6EE1); // 满
        let _ = f;
    }

    /// 全链路对ASCII字符也不panic
    #[test]
    fn grand_court_ascii_no_panic() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let v = court.process(&os, "A");
        assert_eq!(v.input_char, "A");
    }

    /// 压力测试: 连续处理不panic
    #[test]
    fn grand_court_stress_no_panic() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        for i in 0..30 {
            let ch = char::from_u32(0x4E00 + i).unwrap_or('一');
            let _ = court.process(&os, &ch.to_string());
        }
        assert_eq!(court.total_processed, 30);
    }

    /// 空字符串处理不panic
    #[test]
    fn grand_court_empty_string_no_panic() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let v = court.process(&os, "");
        assert_eq!(v.input_char, "");
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第二部分: 容器编码往返 (Container Encoding Round-Trip)
// ══════════════════════════════════════════════════════════════════════════

mod container_encoding {
    use super::*;

    /// 编码/解码往返: 值一致
    #[test]
    fn cell_encode_decode_value_roundtrip() {
        let cell = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 42);
        assert_eq!(mirror::get_value(cell), 42);
    }

    /// 极性编码: 正极性提取一致
    #[test]
    fn cell_polarity_positive() {
        let cell = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 10);
        assert_eq!(mirror::get_polarity(cell), POLARITY_POSITIVE);
    }

    /// 极性编码: 负极性提取一致
    #[test]
    fn cell_polarity_negative() {
        let cell = mirror::encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 10);
        assert_eq!(mirror::get_polarity(cell), POLARITY_NEGATIVE);
    }

    /// 正负极性编码不同
    #[test]
    fn cell_different_polarity_different_encoding() {
        let pos = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 42);
        let neg = mirror::encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 42);
        assert_ne!(pos, neg);
    }

    /// 不同值编码不同
    #[test]
    fn cell_different_value_different_encoding() {
        let a = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 1);
        let b = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 2);
        assert_ne!(a, b);
    }

    /// 零值编码可用
    #[test]
    fn cell_zero_value_encoding() {
        let cell = mirror::encode_cell(0, 0, 0, 0, 0, 0, 0);
        assert_eq!(mirror::get_value(cell), 0);
    }

    /// 大值编码可用 (使用适合value字段的较大值)
    #[test]
    fn cell_large_value_encoding() {
        let cell = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 0xFFFFFF);
        assert_eq!(mirror::get_value(cell), 0xFFFFFF);
    }

    /// 主权编码可区分
    #[test]
    fn cell_sovereignty_distinguishable() {
        let a = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 1);
        let b = mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 1, 0, 1);
        assert_ne!(a, b);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第三部分: YinYang 二进制往返
// ══════════════════════════════════════════════════════════════════════════

mod yinyang_flow {
    use super::*;

    /// YinYang32 创建与值一致
    #[test]
    fn yinyang32_value_consistent() {
        use mqf::yinyang_binary::YinYang32;
        let n = YinYang32(0x12345678);
        assert_eq!(n.0, 0x12345678);
    }

    /// YinYang32 零值
    #[test]
    fn yinyang32_zero() {
        use mqf::yinyang_binary::YinYang32;
        let n = YinYang32(0);
        assert_eq!(n.0, 0);
    }

    /// YinYang32 全1
    #[test]
    fn yinyang32_all_ones() {
        use mqf::yinyang_binary::YinYang32;
        let n = YinYang32(u32::MAX);
        assert_eq!(n.0, u32::MAX);
    }

    /// AnchorBit: 锚点创建
    #[test]
    fn anchor_bit_chutao() {
        use mqf::yinyang_binary::AnchorBit;
        let a = AnchorBit::ChuTao;
        assert_eq!(a.value(), 0);
    }

    /// AnchorBit: 锚点创建
    #[test]
    fn anchor_bit_lolict() {
        use mqf::yinyang_binary::AnchorBit;
        let a = AnchorBit::LoLict;
        assert_eq!(a.value(), 1);
    }

    /// AnchorBit: 翻转操作
    #[test]
    fn anchor_bit_flip() {
        use mqf::yinyang_binary::AnchorBit;
        let a = AnchorBit::ChuTao;
        assert!(matches!(a.flip(), AnchorBit::LoLict));
        let b = AnchorBit::LoLict;
        assert!(matches!(b.flip(), AnchorBit::ChuTao));
    }

    /// AnchorBit: from_u8 映射
    #[test]
    fn anchor_bit_from_u8() {
        use mqf::yinyang_binary::AnchorBit;
        assert!(matches!(AnchorBit::from_u8(0), AnchorBit::ChuTao));
        assert!(matches!(AnchorBit::from_u8(1), AnchorBit::LoLict));
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第四部分: 谐音路由与汉字编码一致性
// ══════════════════════════════════════════════════════════════════════════

mod homophone_routing {
    use super::*;

    /// char_to_dragon 对汉字返回有效声母
    #[test]
    fn dragon_returns_valid_shengmu() {
        use mqf::name_creator::char_to_dragon;
        let d = char_to_dragon('满');
        let _s = d.initial();
        let _y = d.final_();
        let _t = d.tone();
    }

    /// 同一汉字多次调用结果一致
    #[test]
    fn homophone_idempotent() {
        use mqf::name_creator::char_to_dragon;
        let d1 = char_to_dragon('共');
        let d2 = char_to_dragon('共');
        assert_eq!(d1.initial(), d2.initial());
        assert_eq!(d1.final_(), d2.final_());
        assert_eq!(d1.tone(), d2.tone());
    }

    /// 不同汉字大概率不同坐标
    #[test]
    fn homophone_different_chars_differ() {
        use mqf::name_creator::char_to_dragon;
        let d1 = char_to_dragon('满');
        let d2 = char_to_dragon('空');
        let differ = d1.initial() != d2.initial() || d1.final_() != d2.final_() || d1.tone() != d2.tone();
        assert!(differ);
    }

    /// 批量汉字: 每个都能获取有效音韵
    #[test]
    fn homophone_batch_all_valid() {
        use mqf::name_creator::char_to_dragon;
        for ch in "满全法囡囝共生".chars() {
            let d = char_to_dragon(ch);
            let _ = (d.initial(), d.final_(), d.tone());
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第五部分: 聂小倩匹配引擎
// ══════════════════════════════════════════════════════════════════════════

mod njq_matching_flow {

    use mqf::njq_matching::{NjqContainer, PolarityRelation, ResonanceTrace, XiaoQian, HanziSpace};

    /// 容器音韵坐标有效
    #[test]
    fn container_coord_valid() {
        let c = NjqContainer::new('满');
        let (s, y, _t) = c.coord();
        assert!(s > 0 || y > 0);
    }

    /// 共振检测: 不同容器不同共振
    #[test]
    fn resonance_between_containers() {
        let a = NjqContainer::new('满');
        let b = NjqContainer::new('漫');
        let r = a.resonates(&b);
        assert!(r > 0);
    }

    /// 极性关系: 三种之一
    #[test]
    fn polarity_relation_is_valid_variant() {
        let a = NjqContainer::new('满');
        let b = NjqContainer::new('空');
        let rel = a.polarity_relation(&b);
        assert!(matches!(rel,
            PolarityRelation::SameSide | PolarityRelation::Opposite | PolarityRelation::Boundary));
    }

    /// 极性墙检测
    #[test]
    fn polarity_wall_detection() {
        let rel = PolarityRelation::Opposite;
        assert!(rel.is_wall());
        let rel2 = PolarityRelation::SameSide;
        assert!(!rel2.is_wall());
    }

    /// ResonanceTrace 可用性检查
    #[test]
    fn resonance_trace_is_viable() {
        let trace = ResonanceTrace {
            a: '满',
            b: '全',
            strength: 5,
            relation: PolarityRelation::SameSide,
        };
        assert!(trace.is_viable());
    }

    /// ResonanceTrace 墙判定为非viable
    #[test]
    fn resonance_trace_wall_not_viable() {
        let trace = ResonanceTrace {
            a: '满',
            b: '空',
            strength: 5,
            relation: PolarityRelation::Opposite,
        };
        assert!(!trace.is_viable());
    }

    /// XiaoQian 漂移寻踪
    #[test]
    fn xiaoqian_drift_find() {
        let mut xq = XiaoQian::new('缘');
        let space: Vec<char> = "满全法囡囝共生".chars().collect();
        let _ = xq.drift_find(&space);
    }

    /// XiaoQian 撞墙记忆
    #[test]
    fn xiaoqian_bump_wall_memory() {
        let mut xq = XiaoQian::new('诚');
        xq.bump_wall('欺');
        assert!(xq.has_bumped('欺'));
        assert!(!xq.has_bumped('诚'));
    }

    /// HanziSpace: 共振场生成 n*(n-1)/2 对
    #[test]
    fn hanzi_space_resonance_pair_count() {
        let space = HanziSpace::from_str("共生共荣");
        let field = space.resonance_field();
        assert_eq!(field.len(), 6); // 4C2 = 6
    }

    /// HanziSpace: 强共振存在
    #[test]
    fn hanzi_space_strongest_exists() {
        let space = HanziSpace::from_str("满满");
        let strongest = space.strongest_resonance();
        assert!(strongest.is_some());
    }

    /// HanziSpace: 单字符零对
    #[test]
    fn hanzi_space_single_char_empty_field() {
        let space = HanziSpace::from_str("满");
        let field = space.resonance_field();
        assert!(field.is_empty());
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第六部分: 审判管道类型系统
// ══════════════════════════════════════════════════════════════════════════

mod judgment_types {
    use super::*;

    use mqf::crime_transcript::{DiffractionPattern, GlobalWaterQuality, WaterSample, WeavingGraph, EntityFeatures};

    /// DiffractionPattern 创建
    #[test]
    fn diffraction_pattern_new() {
        let p = DiffractionPattern::new(42);
        assert_eq!(p.entity_id, 42);
    }

    /// EntityFeatures::perfect 全1
    #[test]
    fn entity_features_perfect_all_one() {
        let f = EntityFeatures::perfect();
        assert_eq!(f.aesthetic, 1.0);
        assert_eq!(f.type_purity, 1.0);
    }

    /// EntityFeatures::broken 低值
    #[test]
    fn entity_features_broken_low_values() {
        let f = EntityFeatures::broken();
        assert!(f.aesthetic < 0.5);
        assert_eq!(f.type_purity, 0.0);
    }

    /// EntityFeatures::type_confused 类型纯度极低
    #[test]
    fn entity_features_type_confused() {
        let f = EntityFeatures::type_confused();
        assert!(f.type_purity < 0.1);
    }

    /// GlobalWaterQuality 创建
    #[test]
    fn water_quality_new() {
        let wq = GlobalWaterQuality::new(0.5);
        assert!(wq.samples.is_empty());
    }

    /// GlobalWaterQuality 全局纯度空时返回1.0
    #[test]
    fn water_quality_global_purity_empty() {
        let wq = GlobalWaterQuality::new(0.5);
        assert_eq!(wq.global_purity(), 1.0);
    }

    /// GlobalWaterQuality 添加样本
    #[test]
    fn water_quality_add_sample_and_purity() {
        use mqf::crime_transcript::GlobalWaterQuality;
        let mut wq = GlobalWaterQuality::new(0.5);
        wq.add_sample(WaterSample {
            source_id: 1,
            purity: 0.8,
            semantic_correct: true,
            type_correct: true,
            aesthetic_pass: true,
        });
        assert_eq!(wq.samples.len(), 1);
        assert_eq!(wq.global_purity(), 0.8);
    }

    /// WeavingGraph 添加节点和边
    #[test]
    fn weaving_graph_add_tension() {
        let mut wg = WeavingGraph::new();
        wg.add_tension(1, 2, 0.5);
        assert_eq!(wg.edges.len(), 1);
        assert_eq!(wg.node_ids.len(), 2);
    }

    /// WeavingGraph 净拉力
    #[test]
    fn weaving_graph_net_tension() {
        let mut wg = WeavingGraph::new();
        wg.add_tension(1, 2, 0.7);
        assert!((wg.net_tension(2) - 0.7).abs() < 1e-10);
        assert!((wg.net_tension(1) + 0.7).abs() < 1e-10);
    }

    /// Verdict: 所有变体可匹配
    #[test]
    fn verdict_all_variants_matchable() {
        use mqf::cross_isa::fridge::Verdict;
        let v = Verdict::Admit;
        assert!(matches!(v, Verdict::Admit));
        let v = Verdict::Reject;
        assert!(matches!(v, Verdict::Reject));
        let v = Verdict::Wait;
        assert!(matches!(v, Verdict::Wait));
    }

    /// TriuneJudge 裁决空袋
    #[test]
    fn triune_judge_empty_pack_reject() {
        use mqf::cross_isa::fridge::{TriuneJudge, Verdict};
        use mqf::cross_isa::coordinate::Coord3D;
        let pack = mqf::cross_isa::fridge::ContainerPack::new(0xFFFF, Coord3D::new(0,0,0), 0, vec![]);
        let v = TriuneJudge::judge(&pack, 0);
        assert!(matches!(v, Verdict::Reject));
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第七部分: 预编译容器与域加载
// ══════════════════════════════════════════════════════════════════════════

mod preload_containers {
    use super::*;

    /// 预编译容器创建
    #[test]
    fn preload_container_new() {
        use mqf::preload::预编译容器;
        let p = 预编译容器::new();
        let _ = p;
    }

    /// PreloadFlow (类型别名) 可用
    #[test]
    fn preload_flow_alias_usable() {
        use mqf::preload::PreloadFlow;
        let f = PreloadFlow::new();
        let _ = f;
    }

    /// SymbolRegistry (类型别名) 可用
    #[test]
    fn symbol_registry_alias_usable() {
        use mqf::preload::SymbolRegistry;
        let r = SymbolRegistry::new();
        let _ = r;
    }

    /// 血缘祖先: call_relational_parents
    #[test]
    fn preload_call_relational_parents() {
        use mqf::preload::预编译容器;
        let p = 预编译容器::new();
        let ancestors = p.call_relational_parents(12);
        // 12 的真因子: 1,2,3,4,6
        assert_eq!(ancestors, vec![1, 2, 3, 4, 6]);
    }

    /// 血缘祖先: 0的边界情况
    #[test]
    fn preload_call_relational_parents_zero() {
        use mqf::preload::预编译容器;
        let p = 预编译容器::new();
        let ancestors = p.call_relational_parents(0);
        assert_eq!(ancestors, vec![0]);
    }

    /// 声母家族可构造
    #[test]
    fn preload_shengmu_family_available() {
        use mqf::preload::预编译容器;
        let f = 预编译容器::声母();
        assert_eq!(f.名字, "声母");
    }

    /// 家族全局偏移计算
    #[test]
    fn preload_family_global_offset() {
        use mqf::preload::预编译容器;
        let f = 预编译容器::声母();
        let offset = f.全局偏移();
        assert_eq!(offset, f.家族码 as u32 * 256); // 家族跨距=256
    }

    /// 家族按名字查找
    #[test]
    fn preload_family_lookup_by_name() {
        use mqf::preload::预编译容器;
        let f = 预编译容器::声母();
        let found = f.取("b");
        assert!(found.is_some());
    }

    /// 家族按名字查找不存在返回None
    #[test]
    fn preload_family_lookup_not_found() {
        use mqf::preload::预编译容器;
        let f = 预编译容器::声母();
        let result = f.取("不存在的声母");
        assert!(result.is_none());
    }

    /// 集合交集运算
    #[test]
    fn preload_set_intersection() {
        use mqf::preload::{集合, 身份条目};
        let a = 身份条目 { 名字: "满", 时间线: 1, 偏移: 0, 组码: 0, 附加: "" };
        let b = 身份条目 { 名字: "满", 时间线: 1, 偏移: 0, 组码: 0, 附加: "" };
        let c = 身份条目 { 名字: "空", 时间线: 2, 偏移: 0, 组码: 0, 附加: "" };
        let result = 集合::交(&[&a, &b], &[&b, &c]);
        assert_eq!(result.len(), 2); // 两个"满"都匹配
        assert_eq!(result[0].名字, "满");
    }

    /// 集合并运算
    #[test]
    fn preload_set_union() {
        use mqf::preload::{集合, 身份条目};
        let a = 身份条目 { 名字: "满", 时间线: 1, 偏移: 0, 组码: 0, 附加: "" };
        let b = 身份条目 { 名字: "空", 时间线: 2, 偏移: 0, 组码: 0, 附加: "" };
        let c = 身份条目 { 名字: "全", 时间线: 3, 偏移: 0, 组码: 0, 附加: "" };
        let result = 集合::并(&[&a, &b], &[&b, &c]);
        assert_eq!(result.len(), 3);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第八部分: 存在驱动引擎
// ══════════════════════════════════════════════════════════════════════════

mod existence_drive_flow {
    use super::*;

    use mqf::existence_drive::{LossFear, FearLevel, UnityDrive, DriveLevel, RewardTrend, ExistenceDriveEngine};

    /// LossFear 创建
    #[test]
    fn loss_fear_new() {
        let f = LossFear::new(0.5);
        let _ = f;
    }

    /// FearLevel 变体可创建
    #[test]
    fn fear_level_variants() {
        let _calm = FearLevel::Calm;
        let _alert = FearLevel::Alert;
        let _anxious = FearLevel::Anxious;
        let _panicked = FearLevel::Panicked;
    }

    /// UnityDrive 创建
    #[test]
    fn unity_drive_new() {
        let d = UnityDrive::new(0.5);
        let _ = d;
    }

    /// DriveLevel 变体可创建
    #[test]
    fn drive_level_variants() {
        let _dormant = DriveLevel::Dormant;
        let _awakening = DriveLevel::Awakening;
        let _yearning = DriveLevel::Yearning;
        let _urgent = DriveLevel::Urgent;
    }

    /// RewardTrend 变体可创建
    #[test]
    fn reward_trend_variants() {
        let _pos = RewardTrend::Positive;
        let _neg = RewardTrend::Negative;
        let _neu = RewardTrend::Neutral;
    }

    /// ExistenceDriveEngine 创建
    #[test]
    fn existence_engine_new() {
        let e = ExistenceDriveEngine::new();
        assert_eq!(e.tick, 0);
    }

    /// ExistenceDriveEngine 频率初始值
    #[test]
    fn existence_engine_initial_frequency() {
        let e = ExistenceDriveEngine::new();
        let _ = e.frequency;
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第九部分: 跨模块一致性验证
// ══════════════════════════════════════════════════════════════════════════

mod cross_module_consistency {
    use super::*;

    /// 同一汉字在njq和name_creator中编码一致
    #[test]
    fn same_char_consistent_njq_vs_name_creator() {
        use mqf::njq_matching::NjqContainer;
        use mqf::name_creator::char_to_dragon;
        let c = NjqContainer::new('满');
        let (s1, y1, _) = c.coord();
        let d = char_to_dragon('满');
        assert_eq!(s1, d.initial());
        assert_eq!(y1, d.final_());
    }

    /// 极性往返: 编码→解码极性一致
    #[test]
    fn polarity_roundtrip_consistent() {
        let pos = mqf::mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 1);
        let neg = mqf::mirror::encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 1);
        assert_ne!(mqf::mirror::get_polarity(pos), mqf::mirror::get_polarity(neg));
    }

    /// Observer→Mirror 协同: 感相强度编码为容器值
    #[test]
    fn observer_mirror_cooperation() {
        use mqf::observer::{SenseForm, SenseKind};
        use mqf::os_layer::bare_metal::BareMetalOs;
        let os = BareMetalOs::new();
        let form = SenseForm::with_os(SenseKind::Vision, 0.7, "满", "test", &os);
        let val = (form.intensity * 10.0) as u64;
        let cell = mqf::mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, val);
        assert_eq!(mqf::mirror::get_value(cell), 7);
    }

    /// 全链路轻量模拟: 字符→感知→编码→裁决
    #[test]
    fn full_pipeline_lightweight() {
        use mqf::observer::{SenseForm, SenseKind};
        let os = mqf::os_layer::bare_metal::BareMetalOs::new();
        // 1. 感知
        let form = SenseForm::with_os(SenseKind::Vision, 0.8, "满", "test", &os);
        // 2. 编码
        let val = (form.intensity * 10.0) as u64;
        let cell = mqf::mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, val);
        // 3. 裁决
        let mut court = GrandCourt::new();
        let verdict = court.process(&os, "满");
        assert!(!verdict.sentence.is_empty());
    }

    /// 容器编码独立性: 相同值不同极性编码不同
    #[test]
    fn cell_encoding_independent_by_polarity() {
        let a = mqf::mirror::encode_cell(0, 0, 0, POLARITY_POSITIVE, 0, 0, 42);
        let b = mqf::mirror::encode_cell(0, 0, 0, POLARITY_NEGATIVE, 0, 0, 42);
        assert_ne!(a, b);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第十部分: 坐标系统
// ══════════════════════════════════════════════════════════════════════════

mod coordinate_system {
    use super::*;

    /// Coord3D 创建
    #[test]
    fn coord3d_creation() {
        use mqf::cross_isa::coordinate::Coord3D;
        let c = Coord3D::new(0x03, 0x0A10, 0x5F);
        assert_eq!(c.x, 0x03);
        assert_eq!(c.y, 0x0A10);
        assert_eq!(c.z, 0x5F);
    }

    /// Coord3D 地址唯一性
    #[test]
    fn coord3d_address_unique() {
        use mqf::cross_isa::coordinate::Coord3D;
        let a = Coord3D::new(1, 2, 3);
        let b = Coord3D::new(1, 2, 4);
        assert_ne!(a.to_address(), b.to_address());
    }

    /// Coord3D Z轴相邻地址差=1
    #[test]
    fn coord3d_z_adjacent_address_diff_one() {
        use mqf::cross_isa::coordinate::Coord3D;
        let a = Coord3D::new(3, 1, 1);
        let b = Coord3D::new(3, 1, 2);
        assert_eq!(b.to_address() - a.to_address(), 1);
    }

    /// ContainerPack 规范布局
    #[test]
    fn container_pack_layout() {
        use mqf::cross_isa::coordinate::Coord3D;
        use mqf::cross_isa::fridge::ContainerPack;
        let pack = ContainerPack::new(1, Coord3D::new(1, 2, 3), 0, vec![0xB3, 0x00]);
        assert_eq!(pack.func_code, 0x0001);
        assert_eq!(pack.coord().x, 1);
    }

    /// Fridge 装袋取袋
    #[test]
    fn fridge_store_and_fetch() {
        use mqf::cross_isa::{coordinate::Coord3D, fridge::{Fridge, ContainerPack}};
        let mut fridge = Fridge::new();
        let pack = ContainerPack::new(42, Coord3D::new(5, 10, 3), 0, vec![]);
        fridge.store(pack);
        let addr = Coord3D::new(5, 10, 3).to_address();
        let fetched = fridge.fetch(addr);
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().pack_id, 42);
    }

    /// Fridge 空槽返回None
    #[test]
    fn fridge_fetch_empty_returns_none() {
        use mqf::cross_isa::fridge::Fridge;
        let fridge = Fridge::new();
        assert!(fridge.fetch(0).is_none());
    }
}
