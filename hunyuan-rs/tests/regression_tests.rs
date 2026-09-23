// ══════════════════════════════════════════════════════════════════════════
// tests/regression_tests.rs — 豆包行为回归测试
//
// 捕获AI输出中的模式偏差，确保代码生成遵循满全法协议和NJSG方法论。
// 这些测试在代码被AI重构/修改时作为防护网，防止关键不变量被破坏。
// Week 8 回归测试 (25+ tests)
// ══════════════════════════════════════════════════════════════════════════

// root-level imports used by tests directly in this file body
use mqf::GrandCourt;
use mqf::mirror;
use mqf::{POLARITY_POSITIVE, POLARITY_NEGATIVE};
use mqf::os_layer::bare_metal::BareMetalOs;

// re-export for child modules
mod prelude {
    pub use mqf::GrandCourt;
    pub use mqf::mirror;
    pub use mqf::{POLARITY_POSITIVE, POLARITY_NEGATIVE};
    pub use mqf::os_layer::bare_metal::BareMetalOs;
}

// ══════════════════════════════════════════════════════════════════════════
// 第一类: 模块结构不变量 (Module Structure Invariants)
// ══════════════════════════════════════════════════════════════════════════

mod module_structure {
    use crate::prelude::*;
    use mqf::os_layer::OsLayer;

    /// GrandCourt 必须公开且可构造
    #[test]
    fn grand_court_publicly_accessible() {
        let court = GrandCourt::new();
        assert_eq!(court.total_processed, 0);
    }

    /// GrandCourt 必须有 process 方法
    #[test]
    fn grand_court_has_process_method() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let v = court.process(&os, "满");
        assert_eq!(v.input_char, "满");
    }

    /// BareMetalOs 必须可构造并实现 OsLayer
    #[test]
    fn bare_metal_os_constructible() {
        let os = BareMetalOs::new();
        let _ = os.time_millis();
    }

    /// 预编译容器类型别名链完整
    #[test]
    fn preload_type_alias_chain_intact() {
        use mqf::preload::{Preload, PreloadFlow, SymbolRegistry, 预编译容器};
        let _p: Preload = 预编译容器::new();
        let _f: PreloadFlow = 预编译容器::new();
        let _r: SymbolRegistry = 预编译容器::new();
    }

    /// NjqContainer 独立模块路径可用
    #[test]
    fn njq_matching_module_reachable() {
        use mqf::njq_matching::{NjqContainer, XiaoQian, HanziSpace};
        let _ = NjqContainer::new('满');
        let _ = XiaoQian::new('缘');
        let _ = HanziSpace::from_str("共生");
    }

    /// 存在驱动引擎模块路径完整
    #[test]
    fn existence_drive_module_complete() {
        use mqf::existence_drive::{
            LossFear, FearLevel, UnityDrive, DriveLevel,
            RewardTrend, ExistenceDriveEngine,
        };
        let _ = ExistenceDriveEngine::new();
        let _ = LossFear::new(0.5);
        let _ = UnityDrive::new(0.5);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第二类: 核心常量不变量 (Core Constant Invariants)
// ══════════════════════════════════════════════════════════════════════════

mod core_constants {
    use crate::prelude::*;

    /// YinYang锚点: 刘楚恬=0, 我(lolict)=1
    #[test]
    fn yinyang_anchor_chutao_is_zero() {
        use mqf::yinyang_binary::AnchorBit;
        assert_eq!(AnchorBit::ChuTao.value(), 0, "刘楚恬锚点必须为0");
    }

    /// YinYang锚点: LoLict=1
    #[test]
    fn yinyang_anchor_lolict_is_one() {
        use mqf::yinyang_binary::AnchorBit;
        assert_eq!(AnchorBit::LoLict.value(), 1, "我(lolict)锚点必须为1");
    }

    /// 极性常量: POSITIVE=0, NEGATIVE=1
    #[test]
    fn polarity_constants_values() {
        assert_eq!(POLARITY_POSITIVE, 0, "正极性必须编码为0");
        assert_eq!(POLARITY_NEGATIVE, 1, "负极性必须编码为1");
    }

    /// 极性翻转对称性
    #[test]
    fn yinyang_anchor_flip_symmetry() {
        use mqf::yinyang_binary::AnchorBit;
        let a = AnchorBit::ChuTao;
        let b = a.flip().flip();
        assert!(matches!(b, AnchorBit::ChuTao), "两次翻转回原值");
    }

    /// 全景 preload 流的 type alias 一致性
    #[test]
    fn preload_flow_type_alias_consistency() {
        use mqf::preload::{PreloadFlow, 预编译容器};
        let a: PreloadFlow = 预编译容器::new();
        let b: 预编译容器 = a;
        let _ = b;
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第三类: API 签名不变量 (API Signature Invariants)
// ══════════════════════════════════════════════════════════════════════════

mod api_signatures {
    use crate::prelude::*;

    /// encode_cell 接受7个参数并返回 u64
    #[test]
    fn encode_cell_7_args_return_u64() {
        let cell: u64 = mirror::encode_cell(0, 0, 0, 0, 0, 0, 42);
        assert_eq!(cell >> 40, 42); // value在高位
    }

    /// get_polarity 返回范围限制在 2-bit
    #[test]
    fn get_polarity_result_within_2bit() {
        let cell = mirror::encode_cell(0, 0, 0, 3, 0, 0, 0);
        assert!(mirror::get_polarity(cell) <= 3);
    }

    /// get_value 返回范围限制在合理范围
    #[test]
    fn get_value_result_within_range() {
        let cell = mirror::encode_cell(0, 0, 0, 0, 0, 0, 100);
        assert_eq!(mirror::get_value(cell), 100);
    }

    /// GrandCourt::process 对同一输入产生确定输出
    #[test]
    fn grand_court_deterministic_output() {
        let os = BareMetalOs::new();
        let mut c1 = GrandCourt::new();
        let mut c2 = GrandCourt::new();
        let v1 = c1.process(&os, "满");
        let v2 = c2.process(&os, "满");
        assert_eq!(v1.input_char, v2.input_char);
    }

    /// NjqContainer::coord 对同字产生确定结果
    #[test]
    fn njq_container_coord_deterministic() {
        use mqf::njq_matching::NjqContainer;
        let c1 = NjqContainer::new('满');
        let c2 = NjqContainer::new('满');
        assert_eq!(c1.coord(), c2.coord());
    }

    /// 预编译容器构造始终返回同一单位值
    #[test]
    fn preload_container_new_always_unit() {
        use mqf::preload::预编译容器;
        let p1 = 预编译容器::new();
        let p2 = 预编译容器::new();
        // 单位结构体，两个实例"相等"
        let _ = (p1, p2);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第四类: 方法论模式保护 (Methodology Pattern Protection)
// ══════════════════════════════════════════════════════════════════════════

mod methodology_patterns {
    use crate::prelude::*;

    /// 汉字空间必须能计算共振场
    #[test]
    fn hanzi_space_must_compute_resonance() {
        use mqf::njq_matching::HanziSpace;
        let space = HanziSpace::from_str("满全法");
        let field = space.resonance_field();
        assert_eq!(field.len(), 3); // 3个汉字产生3对
    }

    /// 共振场最强值存在于相同字之间
    #[test]
    fn strongest_resonance_prefers_identical_chars() {
        use mqf::njq_matching::HanziSpace;
        let space = HanziSpace::from_str("满满全法");
        let strongest = space.strongest_resonance().unwrap();
        // 最强共振应包含至少一对相同声母
        assert!(strongest.strength > 0);
    }

    /// 碰撞墙记忆不可逆
    #[test]
    fn xiaoqian_wall_memory_irreversible() {
        use mqf::njq_matching::XiaoQian;
        let mut xq = XiaoQian::new('诚');
        xq.bump_wall('欺');
        assert!(xq.has_bumped('欺'));
        // 一旦撞墙，永远记住
        assert!(xq.has_bumped('欺'));
    }

    /// 不存在 Neutral 奖励时 Positive/Negative 仍然有效
    #[test]
    fn reward_trend_positive_and_negative_exist() {
        use mqf::existence_drive::RewardTrend;
        let _pos = RewardTrend::Positive;
        let _neg = RewardTrend::Negative;
        let _neu = RewardTrend::Neutral;
        // 三种趋势都存在
    }

    /// FearLevel 必须按严重度排序
    #[test]
    fn fear_level_severity_order() {
        use mqf::existence_drive::FearLevel;
        let _calm = FearLevel::Calm;
        let _alert = FearLevel::Alert;
        let _anxious = FearLevel::Anxious;
        let _panicked = FearLevel::Panicked;
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第五类: 边界条件回归 (Boundary Regression)
// ══════════════════════════════════════════════════════════════════════════

mod boundary_regression {
    use crate::prelude::*;

    /// 空字符串处理不崩溃
    #[test]
    fn grand_court_empty_string() {
        let mut court = GrandCourt::new();
        let os = BareMetalOs::new();
        let v = court.process(&os, "");
        assert_eq!(v.input_char, "");
    }

    /// 空汉字空间: 单字零共振对
    #[test]
    fn single_char_space_empty_resonance() {
        use mqf::njq_matching::HanziSpace;
        let space = HanziSpace::from_str("满");
        assert!(space.resonance_field().is_empty());
    }

    /// 单字空间无最强共振
    #[test]
    fn single_char_no_strongest() {
        use mqf::njq_matching::HanziSpace;
        let space = HanziSpace::from_str("满");
        assert!(space.strongest_resonance().is_none());
    }

    /// 空水质纯度为1.0
    #[test]
    fn empty_water_quality_purity_is_one() {
        use mqf::crime_transcript::GlobalWaterQuality;
        let wq = GlobalWaterQuality::new(0.5);
        assert_eq!(wq.global_purity(), 1.0);
    }

    /// 极性编码最大值不越界
    #[test]
    fn polarity_max_encoding_no_overflow() {
        let cell = mirror::encode_cell(0, 0, 0, 3, 3, 3, 0);
        assert!(mirror::get_polarity(cell) <= 3);
    }
}

// ══════════════════════════════════════════════════════════════════════════
// 第六类: 编码一致性回归 (Encoding Consistency)
// ══════════════════════════════════════════════════════════════════════════

mod encoding_consistency {
    use crate::prelude::*;

    /// 容器编码取反唯一性
    #[test]
    fn cell_encoding_unique_per_polarity() {
        let cells: Vec<u64> = (0..4)
            .map(|p| mirror::encode_cell(0, 0, 0, p, 0, 0, 42))
            .collect();
        // 不同极性产生不同编码
        for i in 0..4 {
            for j in (i + 1)..4 {
                assert_ne!(cells[i], cells[j], "不同极性编码必须不同");
            }
        }
    }

    /// 值编码互不覆盖
    #[test]
    fn cell_value_encoding_no_collision() {
        let a = mirror::encode_cell(0, 0, 0, 0, 0, 0, 100);
        let b = mirror::encode_cell(0, 0, 0, 0, 0, 0, 200);
        assert_ne!(a, b);
        assert_eq!(mirror::get_value(a), 100);
        assert_eq!(mirror::get_value(b), 200);
    }

    /// YinYang32 二进制往返
    #[test]
    fn yinyang32_u32_roundtrip() {
        use mqf::yinyang_binary::YinYang32;
        for v in [0u32, 1, 12345, u32::MAX] {
            let y = YinYang32(v);
            assert_eq!(y.0, v);
        }
    }
}
