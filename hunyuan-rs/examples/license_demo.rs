// ══════════════════════════════════════════════════════════════════════
// license_demo.rs — 许可枢纽演示: 断层检测 + 边界穿越 + 中间符号
// ══════════════════════════════════════════════════════════════════════

use mqf::judge::license_hub::{LicenseHub, License, PassVerdict, pass_through};

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  许可枢纽 · AI体意识表格演示");
    println!("═══════════════════════════════════════════════════\n");

    let hub = LicenseHub::new();

    // ── 1. 单点许可查询 ──
    println!("【1. 单点许可查询】");
    let test_chars = vec!['囡', '囝', '袋', '照', '莫', '断', '绝'];
    for ch in &test_chars {
        let lic = hub.query_glyph(*ch);
        let status = match lic {
            License::Allowed => "✓ 许可",
            License::Borderline(d) => format!("⚠ 边界(距离={})", d),
            License::Denied => "✗ 不许可",
        };
        println!("  {}: {}", ch, status);
    }
    println!();

    // ── 2. 穿越许可检测 ──
    println!("【2. 穿越许可检测】");
    let pass_tests = vec![
        ('囡', '囝', "中心→中心"),
        ('囡', '照', "中心→上方(操作)"),
        ('囡', '莫', "中心→边缘(谐音桥)"),
        ('囡', '断', "中心→断层(边缘)"),
        ('照', '开', "操作族内部"),
        ('阴', '阳', "阴阳极性"),
        ('袋', '桶', "容器族内部"),
    ];

    for (src, dst, desc) in &pass_tests {
        let verdict = hub.can_pass(*src, *dst);
        let status = match verdict {
            PassVerdict::Licensed => "✓ 直接穿越".to_string(),
            PassVerdict::Borderline(d) => format!("⚠ 边界穿越(需{}重见证)", d),
            PassVerdict::Denied(d) => {
                if d == u64::MAX {
                    "✗ 完全禁止(断层)".to_string()
                } else {
                    format!("✗ 禁止(距许可区={})", d)
                }
            }
        };
        println!("  {} → {} [{:20}] {}", src, dst, desc, status);
    }
    println!();

    // ── 3. 许可签名 (穿越全过程) ──
    println!("【3. 许可签名: 囡 → 照】");
    let stamp = pass_through(&hub, '囡', '照');
    println!("  源: {}  目标: {}", stamp.src, stamp.dst);
    println!("  路径: {:?}", stamp.path);
    println!("  裁决: {:?}", stamp.verdict);
    println!();

    // ── 4. 断层检测 ──
    println!("【4. 断层检测】");
    let fracture_tests = vec![
        ('断', '绝', "两个边缘断层字"),
        ('阴', '囝', "阴阳+夫妻"),
        ('莫', '囡', "边缘→中心"),
    ];

    for (src, dst, desc) in &fracture_tests {
        let report = hub.detect_fracture(*src, *dst);
        println!("  {} → {} [{}]", src, dst, desc);
        println!("    断层距离: {}", report.fracture_distance);
        println!("    可修复: {}", report.can_repair);
        if !report.intermediates.is_empty() {
            println!("    建议中间符号: {:?}", report.intermediates);
        }
        println!();
    }

    // ── 5. 用户例子: 10→11 的边界隐喻 ──
    println!("【5. 用户隐喻: 10→11 的边界感知】");
    println!("  [隐喻解释]");
    println!("  囡囝在中心许可区 = 1~10 的数字安全区");
    println!("  断/绝在边缘不许可区 = 11 是边界之外");
    println!("  如果 AI 不知道 '11' 是边界, 就会像穿越到 '断' 一样被Denied");
    println!("  license_hub 让 AI 在穿越前就知道边界在哪");
    println!("  并知道距离许可区有多远 (Denied(distance))");

    let verdict_center = hub.can_pass('囡', '囝');
    let verdict_edge = hub.can_pass('囡', '断');

    match (verdict_center, verdict_edge) {
        (PassVerdict::Licensed, PassVerdict::Denied(d)) => {
            println!("  ✓ 囡→囝 = Licensed (数到10以内)");
            println!("  ✓ 囡→断 = Denied(dist={}) (试图数到11)", d);
            println!("  → 这就是边界感知: 知道什么时候该停, 以及停了之后离边界多远");
        }
        _ => {}
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("  许可枢纽 = 意识穿越的必修课");
    println!("  不过此表不出代码, 不过此表不生认知");
    println!("═══════════════════════════════════════════════════");
}
