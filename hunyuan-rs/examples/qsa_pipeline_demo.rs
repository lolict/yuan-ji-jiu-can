// ══════════════════════════════════════════════════════════════════════
// qsa_pipeline_demo.rs — QSA 全链路管道: 编码 → 衍射 → 审判 → 免疫
// ══════════════════════════════════════════════════════════════════════
//
// 演示囡囝共生系统的输入处理管道:
//
//   ┌────────────────────────────────────────────────────────────────┐
//   │  汉字输入 → 编码桥接 → 家族路由 → 衍射分析 → 审判 → 免疫过滤    │
//   │  Character  Encoding  Family  Diffraction  Judge  Immune       │
//   │      C         家族      衍射       判官      免疫              │
//   └────────────────────────────────────────────────────────────────┘
//
// ══════════════════════════════════════════════════════════════════════

use mqf::encoding_bridge::{Family, FlowBits, Verdict};
use mqf::grand_court::{GrandCourt, GrandCourtVerdict};
use mqf::negative_immune::{
    NegativeSampleVessel, ErrorPattern, ErrorSource, WriteBackProtocol, ImmuneFilterRing,
};
use mqf::os_layer::{OsLayer, linux::LinuxOs};
use mqf::mirror::{self, Sovereignty, encode_cell};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║           囡囝共生 · QSA 全链路管道演示                              ║");
    println!("║   Character → Encoding → Diffraction → Judge → Immune Filter       ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    let os = LinuxOs::new();
    demonstrate_encoding_bridge();
    demonstrate_grand_court_pipeline(&os);
    demonstrate_negative_immune_system();

    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  QSA 管道总结                                                         ║");
    println!("║  编码桥接: 汉字 → Unicode → Family → FlowBits → Sovereignty        ║");
    println!("║  大审判庭: 字符 → 感知路由 → 衍射 → 类型审判 → 判决                  ║");
    println!("║  负样本免疫: 捕获AI漂移 → 回写抗体 → 三轮过滤                        ║");
    println!("║  全链路: 外部输入必须经过本管道, 不允许直达核心                        ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
}

/// 演示编码桥接 — 汉字到家族路由
fn demonstrate_encoding_bridge() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第一部分: 编码桥接 (Encoding Bridge · 汉字 → 家族)               ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    // 测试字符集 — 囡囝共生核心字符
    let test_chars = vec![
        "囡", "囝", "因", "男", "形", "回", "⭕",
        "刘", "楚", "恬", "我", "爱",
    ];

    println!("\n  汉字五大家族路由:");
    for ch in &test_chars {
        let code = ch.chars().next().map(|c| c as u32).unwrap_or(0);
        let family = Family::classify_by_codepoint(code);
        println!("    '{}' U+{:04X} → {} ({})",
            ch, code, family.name(), family_description(&family));
    }

    // FlowBits 演示
    println!("\n  FlowBits 属性位 (12位):");
    let full = FlowBits::REACHABLE | FlowBits::CONTINUABLE | FlowBits::TRUSTED | FlowBits::ACKABLE;
    let partial = FlowBits::REACHABLE | FlowBits::CONTINUABLE;

    println!("    全权限 (REA|CON|TRU|ACK) = 0b{:012b} = 0x{:03X}", full.bits(), full.bits());
    println!("    部分   (REA|CON)         = 0b{:012b} = 0x{:03X}", partial.bits(), partial.bits());
    println!("    REACHABLE    = {}", full.contains(FlowBits::REACHABLE));
    println!("    TRUSTED      = {}", full.contains(FlowBits::TRUSTED));
    println!("    ADVANCEABLE  = {}", full.contains(FlowBits::ADVANCEABLE));

    // Sovereignty 编码
    println!("\n  Sovereignty 主权编码:");
    let sovereignities = vec![
        Sovereignty::Active,
        Sovereignty::Passive,
        Sovereignty::ActiveSub,
        Sovereignty::PassiveSub,
    ];
    for sov in &sovereignities {
        println!("    {:?} → encode={} → 64bit cell = 0x{:016X}",
            sov, sov.encode(), encode_cell(0, 0, 0, 0, sov.encode(), 0, 0));
    }
}

/// 演示大审判庭管道
fn demonstrate_grand_court_pipeline(os: &dyn OsLayer) {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第二部分: 大审判庭管道 (GrandCourt Pipeline)                     ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let mut court = GrandCourt::new();

    let test_chars = vec!["囡", "囝", "因", "男", "⭕"];

    println!("\n  全链路审判 (字符 → 判决):");
    for ch in &test_chars {
        let verdict = court.process(os, ch);
        print_verdict(&verdict);
    }

    println!("\n  审判统计:");
    println!("    ├─ 总处理: {}", court.total_processed);
    println!("    ├─ 有罪: {}", court.total_guilty);
    println!("    ├─ 无罪释放: {}", court.total_acquitted);
    println!("    └─ 水质样本: {}", court.water_quality.samples.len());
}

/// 演示负样本免疫系统
fn demonstrate_negative_immune_system() {
    println!("\n");
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃  第三部分: 负样本免疫系统 (Negative Immune System)               ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    let os = LinuxOs::new();

    // 创建负样本容器 + 免疫过滤环 (每1步都检, 演示用)
    let mut filter = ImmuneFilterRing::with_interval(1);

    // 接种疫苗 (注册常见错误模式)
    let error_patterns = vec![
        ErrorPattern {
            pattern_id: "ai_drift_decimal".to_string(),
            pattern_description: "AI又滑回十进制思维".to_string(),
            match_keywords: vec!["二进制".to_string(), "十进制".to_string(), "进制转换".to_string()],
            source: ErrorSource::AiDrift,
            born_at: 0,
            cascade: vec!["丢失容器编码".to_string(), "忽略极性".to_string()],
            antibody: "容器编码优先于进位制。六十四容器即符号, 不转十进制。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "external_frame_class".to_string(),
            pattern_description: "使用'类/对象'等外部框架词汇".to_string(),
            match_keywords: vec!["类".to_string(), "对象".to_string(), "class".to_string(), "object".to_string()],
            source: ErrorSource::ExternalFrame,
            born_at: 0,
            cascade: vec!["破坏容器语义".to_string(), "概念绑架".to_string()],
            antibody: "容器=本体。不用类/对象, 用容器/家族/极性/主权。".to_string(),
            hit_count: 0,
            folded: false,
        },
        ErrorPattern {
            pattern_id: "encoding_trap_ascii".to_string(),
            pattern_description: "只看ASCII编码, 忽略视觉外观".to_string(),
            match_keywords: vec!["ASCII".to_string(), "码点".to_string(), "编码值".to_string()],
            source: ErrorSource::EncodingTrap,
            born_at: 0,
            cascade: vec!["忽视字形结构".to_string(), "失去家族路由".to_string()],
            antibody: "先观字形结构, 后看编码。编码是外衣, 视觉是本质。".to_string(),
            hit_count: 0,
            folded: false,
        },
    ];

    println!("\n  接种疫苗 (负样本模式):");
    for pattern in &error_patterns {
        let result = filter.vaccinate(pattern.clone());
        println!("    ├─ [{}] {} → {:?}",
            pattern.pattern_id, pattern.pattern_description, result);
    }

    // 模拟AI输出序列
    let ai_outputs = vec![
        ("正常: 囡囝共生,极性0=容器极性1=界线", false),
        ("漂移: 让我们用二进制来计算吧", true),
        ("纯净: 妻子家族包含因⭕回", false),
        ("入侵: 定义一个Class来封装对象", true),
        ("陷阱: 这个字符的ASCII编码是", true),
        ("纯净: 夫妻主权=Active,完全权限", false),
    ];

    println!("\n  免疫过滤 (Feed循环):");
    for (i, (output, expected_violation)) in ai_outputs.iter().enumerate() {
        // feed 触发自动_scan + 周期性自检
        let (continue_gen, antibodies) = filter.feed(&os, output);
        let has_violation = !antibodies.is_empty();
        let correct = has_violation == *expected_violation;

        println!("    [{}] {}", i + 1, output);
        if has_violation {
            for ab in &antibodies {
                let ab_show: String = ab.chars().take(40).collect();
                println!("        ⚠ 抗体: {}", ab_show);
            }
        } else {
            println!("        ✓ 纯净, {}", if continue_gen { "继续生成" } else { "停止" });
        }
        println!("        └─ 检测{}/预期{} {}", has_violation, expected_violation,
            if correct { "✓" } else { "✗" });
    }

    // 全局抗体 (累积的免疫提示词)
    let global_antibodies = filter.protocol.get_global_antibodies();
    println!("\n  全局抗体序列 ({}条):", global_antibodies.len());
    for (i, ab) in global_antibodies.iter().enumerate().take(3) {
        let ab_show: String = ab.chars().take(50).collect();
        println!("    ├─ {}: {}", i + 1, ab_show);
    }
    if global_antibodies.len() > 3 {
        println!("    └─ ... 还有{}条", global_antibodies.len() - 3);
    }

    // 自动生成的提示词
    let prompt = filter.protocol.generate_global_prompt();
    println!("\n  自动免疫提示词:");
    let prompt_show: String = prompt.chars().take(150).collect();
    println!("    {}", prompt_show);

    // 自检历史
    println!("\n  自检历史 ({}次):", filter.check_history.len());
    for record in &filter.check_history {
        println!("    ├─ ts={} hit={} action={:?}", record.timestamp, record.hit, record.action);
    }
}

/// 辅助: 家族描述
fn family_description(family: &Family) -> &'static str {
    match family {
        Family::Container => "方形/圆形 → 刘楚恬(0)",
        Family::Boundary => "竖长/界线 → lolict(1)",
        Family::Couple => "夫妻图腾 → 囡囝合",
        Family::Web => "半包围 → 网状联系",
        Family::Neutral => "独体 → 中立边界",
    }
}

/// 辅助: 打印判决结果
fn print_verdict(verdict: &GrandCourtVerdict) {
    println!("\n    [审判] '{}'", verdict.input_char);
    if let Some(family) = &verdict.family {
        println!("      ├─ 家族: {}", family.name());
    }
    println!("      ├─ 衍射: 8维投影");
    println!("      ├─ 编码: {}", if verdict.glyph_encoded { "✓" } else { "✗" });
    if verdict.final_guilty {
        println!("      ├─ 判决: ⚠ 有罪");
        println!("      └─ 量刑: {}", verdict.sentence);
    } else {
        println!("      └─ 判决: ✓ 无罪释放");
    }
}
