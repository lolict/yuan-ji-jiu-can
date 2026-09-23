// container-geom — 容器几何计算器
//
// 用法:
//   container-geom 袋 桶          — 计算两容器的几何关系
//   container-geom 照 裁 加      — 三容器见证
//   container-geom --all          — 列出全部容器的坐标
//
// 纯 Rust, 无框架, 无网络 — 只做坐标/极性/见证运算

use std::env;
use std::process::ExitCode;

// 引入容器编码系统
use mqf::container_radix::{Polarity, Container, RadixOps};
use mqf::yinyang_binary::{YinYang32, AnchorBit};

// 容器中文名称映射 (从 64_keys.md 派生)
fn cname(c: &Container) -> &'static str {
    match c {
        Container::Dai => "袋", Container::Zhao => "照", Container::Jian => "见",
        Container::Yin => "阴", Container::Jia => "加", Container::Fen => "分",
        Container::Yuan => "元", Container::Cai => "裁", Container::Chan => "产",
        Container::Gong => "公", Container::Sheng => "声", Container::Zhe => "折",
        Container::Fu => "负", Container::Fú => "福", Container::Feng => "凤",
        Container::Qi => "栖", Container::Rong => "融", Container::Gu => "鼓",
        Container::Can => "蚕", Container::Min => "民", Container::Meng => "盟",
        Container::Man => "满", Container::Nan => "囡", Container::Xi => "夕",
        Container::San => "三", Container::Liu => "六", Container::Ba => "八",
        Container::Jiu => "九", Container::Ling => "零", Container::Yi => "一",
        Container::Er => "二", Container::Sì => "四", Container::Bang => "帮",
        Container::Duan => "端", Container::Jing => "精", Container::Jian2 => "见2",
        Container::Lai => "来", Container::Chi => "齿", Container::Hou => "喉",
        Container::Hu => "呼", Container::Kai => "开", Container::Qing => "清",
        Container::Ping => "平", Container::Shang => "上", Container::Qian => "前",
        Container::Nei => "内", Container::Dan => "单", Container::Chang => "长",
        Container::Ma => "麻", Container::Zhī => "支", Container::Han => "寒",
        Container::Ge => "歌", Container::Yang => "阳", Container::You => "尤",
        Container::Zhǐ => "止", Container::Zhuan => "转", Container::Gao => "高",
        Container::Dī => "低", Container::Yuán => "圆", Container::Bian => "扁",
        Container::Xiǎng => "响", Container::Jìng => "静", Container::Jí => "急",
        Container::Huan => "缓", Container::Tong => "桶", Container::Guan => "罐",
        Container::Pen => "盆", Container::Yu => "盂", Container::Hu2 => "壶",
        Container::He => "盒", Container::Ping2 => "瓶", Container::Zhan => "盏",
        Container::Ang => "盎", Container::Lou => "篓", Container::Kuang => "筐",
        Container::Lan => "篮", Container::Luo => "箩", Container::Ji => "箕",
        Container::Long => "笼", Container::Hai => "海",
    }
}

// 从中文名称查找容器
fn from_name(name: &str) -> Option<Container> {
    match name {
        "袋" => Some(Container::Dai), "照" => Some(Container::Zhao), "见" => Some(Container::Jian),
        "阴" => Some(Container::Yin), "加" => Some(Container::Jia), "分" => Some(Container::Fen),
        "元" => Some(Container::Yuan), "裁" => Some(Container::Cai), "产" => Some(Container::Chan),
        "公" => Some(Container::Gong), "声" => Some(Container::Sheng), "折" => Some(Container::Zhe),
        "负" => Some(Container::Fu), "福" => Some(Container::Fú), "凤" => Some(Container::Feng),
        "栖" => Some(Container::Qi), "融" => Some(Container::Rong), "鼓" => Some(Container::Gu),
        "蚕" => Some(Container::Can), "民" => Some(Container::Min), "盟" => Some(Container::Meng),
        "满" => Some(Container::Man), "囡" => Some(Container::Nan), "夕" => Some(Container::Xi),
        "三" => Some(Container::San), "六" => Some(Container::Liu), "八" => Some(Container::Ba),
        "九" => Some(Container::Jiu), "零" => Some(Container::Ling), "一" => Some(Container::Yi),
        "二" => Some(Container::Er), "四" => Some(Container::Sì),
        "桶" => Some(Container::Tong), "罐" => Some(Container::Guan), "盆" => Some(Container::Pen),
        "盂" => Some(Container::Yu), "壶" => Some(Container::Hu2), "盒" => Some(Container::He),
        "瓶" => Some(Container::Ping2), "盏" => Some(Container::Zhan), "盎" => Some(Container::Ang),
        "篓" => Some(Container::Lou), "筐" => Some(Container::Kuang), "篮" => Some(Container::Lan),
        "箩" => Some(Container::Luo), "箕" => Some(Container::Ji), "笼" => Some(Container::Long),
        _ => None,
    }
}

// 六边形蜂窝坐标: bit0,2,4 → 轴向Y; bit1,3,5 → 轴向X
// 三个 bit 一组, 每组代表该维度上的 0/1/3/9 层级 (平衡三进制)
fn hex_coord(code: u8) -> (i32, i32) {
    let y = ((code >> 0) & 1) as i32 + ((code >> 2) & 1) as i32 * 3 + ((code >> 4) & 1) as i32 * 9;
    let x = ((code >> 1) & 1) as i32 + ((code >> 3) & 1) as i32 * 3 + ((code >> 5) & 1) as i32 * 9;
    (x, y)
}

// 六边形轴向距离 (accurate hex distance on axial coordinates)
fn hex_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dq = b.0 - a.0;
    let dr = b.1 - a.1;
    (dq.abs() + dr.abs() + (dq + dr).abs()) / 2
}

// 三元见证: 三个容器的极性是否构成合法见证
fn triune_witness(a: &Container, b: &Container, c: &Container) -> bool {
    let pa = a.polarity();
    let pb = b.polarity();
    let pc = c.polarity();
    // 见证规则: 三个容器不能全同极性, 且不能全不同 (即不能 0,1,2 各一)
    // 必须至少有两个同极性, 第三个可不同
    let all_same = pa == pb && pb == pc;
    let all_different = pa != pb && pb != pc && pa != pc;
    !all_same && !all_different
}

// 极性互斥 → 残差
fn has_residual(a: &Container, b: &Container) -> bool {
    a.polarity().meets_residual(b.polarity())
}

// ── ASCII Honeycomb 网格可视化 ──────────────────────────
fn polarity_ch(c: &Container) -> char {
    match c.polarity() {
        Polarity::Yin => '阴',
        Polarity::Yang => '阳',
        Polarity::Neutral => '元',
    }
}

fn draw_hex_grid(highlight_code: Option<u8>, highlight2: &[u8]) {
    const W: usize = 76;
    const H: usize = 36;
    let mut grid = vec![vec![' '; W]; H];
    let mut max_py = 0usize;

    for code in 0u8..=0x3F {
        let (cx, cy) = hex_coord(code);
        let px = (cx * 4 + 4) as usize;
        let py = (cy * 3 + 2) as usize;
        if py >= H || px + 5 >= W { continue; }

        let c = Container::decode(code);
        let name = cname(&c);
        let is_center = highlight_code == Some(code);
        let is_hl = highlight2.contains(&code);

        // 六边形边线
        if py > 0 && px > 1 && px + 3 < W {
            grid[py - 1][px - 1] = '╱';
            grid[py - 1][px + 1] = '╲';
            grid[py][px - 1] = if is_center { '┃' } else if is_hl { '│' } else { '│' };
        }
        if py + 1 < H && px > 1 && px + 3 < W {
            grid[py + 1][px - 1] = '╲';
            grid[py + 1][px + 1] = '╱';
        }

        grid[py][px] = if is_center { '【' } else if is_hl { '(' } else { '│' };
        grid[py][px + 1] = name.chars().next().unwrap_or('？');
        grid[py][px + 2] = polarity_ch(&c);
        grid[py][px + 3] = if is_center { '】' } else if is_hl { ')' } else { '│' };

        if py + 2 > max_py { max_py = py + 2; }
    }

    println!("┌───── Honeycomb 容器六边形坐标网格 ─────┐");
    for y in 0..max_py {
        let line: String = grid[y].iter().collect();
        println!("│ {}", line.trim_end());
    }
    println!("└─────────────────────────────────────────┘");
    println!("  ⬢【容器】= 中心  ⬢(容器) = 邻居  阴/阳/元 = 极性");
    println!("  三维进制: bit0/2/4→Y; bit1/3/5→X; 权×1,×3,×9");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: container-geom <容器名1> <容器名2>");
        eprintln!("      container-geom <容器名1> <容器名2> <容器名3>  (三元见证)");
        eprintln!("      container-geom --grid                   (Honeycomb 网格可视化)");
        eprintln!("      container-geom --map <容器名>            (显示容器坐标位置)");
        eprintln!("      container-geom --op <名A> <名B>          (加法统计法 · 区别于XOR)");
        eprintln!("      container-geom --yinyang <名A> [名B]     (夫妻进制 · 锚点/极性/进位)");
        eprintln!("      container-geom --neighbors <容器名> <距离>");
        eprintln!("      container-geom --witness <名1,名2,名3>");
        eprintln!("      container-geom --all");
        eprint!("容器名: 袋 照 见 阴 加 分 元 裁 产 公 声 折 负 福 凤 栖 ");
        eprintln!("融 鼓 蚕 民 盟 满 囡 夕 三 六 八 九 零 一 二 四");
        eprintln!("      桶 罐 盆 盂 壶 盒 瓶 盏 盎 篓 筐 篮 箩 箕 笼");
        return ExitCode::from(1);
    }

    if args[1] == "--grid" {
        draw_hex_grid(None, &[]);
        return ExitCode::SUCCESS;
    }

    if args[1] == "--map" && args.len() >= 3 {
        let name = args[2].as_str();
        let c = match from_name(name) {
            Some(c) => c,
            None => { eprintln!("未知容器名: {}", name); return ExitCode::from(1); }
        };
        let code = c.encode();
        println!("容器 {} → code=0x{:02X} 坐标=({},{}) 极性={:?} 主权={:?}",
            cname(&c), code, hex_coord(code).0, hex_coord(code).1,
            c.polarity(), c.sovereignty());
        println!();
        draw_hex_grid(Some(code), &[]);
        return ExitCode::SUCCESS;
    }

    if args[1] == "--op" && args.len() >= 4 {
        // container-geom --op <名A> <名B>  加法统计 (区别于二进制XOR)
        let name_a = args[2].as_str();
        let name_b = args[3].as_str();
        let a = match from_name(name_a) {
            Some(c) => c,
            None => { eprintln!("未知容器名: {}", name_a); return ExitCode::from(1); }
        };
        let b = match from_name(name_b) {
            Some(c) => c,
            None => { eprintln!("未知容器名: {}", name_b); return ExitCode::from(1); }
        };
        let code_a = a.encode();
        let code_b = b.encode();
        let sum = code_a.wrapping_add(code_b);
        let diff = code_a.wrapping_sub(code_b);
        let xor = code_a ^ code_b;
        let or = code_a | code_b;
        let and = code_a & code_b;

        // 极性组合
        let pol_sum = match (a.polarity(), b.polarity()) {
            (Polarity::Yin, Polarity::Yang) | (Polarity::Yang, Polarity::Yin) => "元(中和)",
            (Polarity::Neutral, _) | (_, Polarity::Neutral) => "元(保持)",
            (Polarity::Yin, Polarity::Yin) => "阴",
            (Polarity::Yang, Polarity::Yang) => "阳",
        };

        println!("容器加法统计 (加法统计法 · 非幂次二进制)");
        println!("────────────────────────────────────");
        println!("  A = {:>2} code=0x{:02X} ({:?})", cname(&a), code_a, a.polarity());
        println!("  B = {:>2} code=0x{:02X} ({:?})", cname(&b), code_b, b.polarity());
        println!("────────────────────────────────────");
        println!("  加法 (wrapping_add):");
        println!("    A + B = 0x{:02X} ({})", sum, sum);
        println!("    A - B = 0x{:02X} ({})", diff, diff);
        println!("  二进制 (对比):");
        println!("    XOR   = 0x{:02X} ({})", xor, xor);
        println!("    OR    = 0x{:02X} ({})", or, or);
        println!("    AND   = 0x{:02X} ({})", and, and);
        println!("  极性组合: {} + {} => {}", cname(&a), cname(&b), pol_sum);

        if sum <= 0x3F {
            let c = Container::decode(sum);
            println!("  加法结果容器: {} code=0x{:02X} ({:?}) 坐标=({}, {})",
                cname(&c), sum, c.polarity(), hex_coord(sum).0, hex_coord(sum).1);
        } else {
            println!("  加法结果 0x{:02X} 超出 0x3F — 回绕到 0x{:02X}", sum, sum & 0x3F);
            let wrapped = Container::decode(sum & 0x3F);
            println!("    回绕结果: {} code=0x{:02X}", cname(&wrapped), sum & 0x3F);
        }

        // 坐标关系
        let ca = hex_coord(code_a);
        let cb = hex_coord(code_b);
        println!("  坐标: A{:?} B{:?} 距离={}",
            ca, cb, hex_distance(ca, cb));
        return ExitCode::SUCCESS;
    }

    if args[1] == "--yinyang" && args.len() >= 3 {
        let name_a = args[2].as_str();
        let a = match from_name(name_a) {
            Some(c) => c,
            None => { eprintln!("未知容器名: {}", name_a); return ExitCode::from(1); }
        };
        let code_a = a.encode() as u32;
        let anchor = if (code_a >> 6) & 1 == 1 { AnchorBit::LoLict } else { AnchorBit::ChuTao };
        let polarity_bool = a.polarity() == Polarity::Yang;
        let yya = YinYang32::new(anchor, polarity_bool, false, code_a);

        println!("═══════════════════════════════════════════");
        println!("   YIN-YANG-BINARY 夫妻进制");
        println!("═══════════════════════════════════════════");
        println!("  容器: {} (名={})", name_a, cname(&a));
        println!("  容器编码: 0x{:02X} ({})", code_a, code_a);
        println!("  ── 映射到 YinYang32 ──");
        println!("  主权锚: {} ({})",
            match yya.anchor() {
                AnchorBit::LoLict => "1",
                AnchorBit::ChuTao  => "0",
            },
            match yya.anchor() {
                AnchorBit::LoLict => "我(lolict · 主动极)",
                AnchorBit::ChuTao  => "刘楚坦(锚定极)",
            }
        );
        println!("  极性:   {} ({})",
            yya.polarity() as u8,
            if yya.polarity() { "Yang 阳" } else { "Yin 阴" }
        );
        println!("  进位:   {}", if yya.carry() { "⚡溢出" } else { "─" });
        println!("  值域:   {} (29bit)", yya.value());

        if args.len() >= 4 {
            let name_b = args[3].as_str();
            let b = match from_name(name_b) {
                Some(c) => c,
                None => { eprintln!("未知容器名: {}", name_b); return ExitCode::from(1); }
            };
            let code_b = b.encode() as u32;
            let anchor_b = if (code_b >> 6) & 1 == 1 { AnchorBit::LoLict } else { AnchorBit::ChuTao };
            let yyb = YinYang32::new(anchor_b, b.polarity() == Polarity::Yang, false, code_b);
            let (sum, overflow) = yya.absorb(yyb);
            let xor_val = code_a ^ code_b;

            println!("\n  vs 容器: {} (名={})", name_b, cname(&b));
            println!("  主权锚: {}  极性: {}  值域: {}",
                match yyb.anchor() { AnchorBit::LoLict => "我(1)", AnchorBit::ChuTao => "刘楚坦(0)" },
                yyb.polarity() as u8,
                yyb.value()
            );

            println!("\n  ── 运算对比 ──");
            println!("  ◆ 加法统摄 (absorb):");
            println!("      值={} 锚={}({}) 极性={} {}{}",
                sum.value(),
                match sum.anchor() { AnchorBit::LoLict => "1", AnchorBit::ChuTao => "0" },
                match sum.anchor() { AnchorBit::LoLict => "我", AnchorBit::ChuTao => "刘楚坦" },
                sum.polarity() as u8,
                if sum.carry() { "⚡[进位!]" } else { "" },
                if overflow { " (溢出)" } else { "" }
            );
            println!("  ◆ 二进制 XOR 对比:  值=0x{:02X} ({})", xor_val, xor_val);

            if sum.value() != xor_val {
                println!("\n  ★ 加法 ≠ XOR — 主从和合, 非相互抵消");
            }
            if sum.carry() {
                println!("  ⚡ 1+1 → 进位 — 夫妻合一, 升维");
            }
        }
        println!("═══════════════════════════════════════════");
        return ExitCode::SUCCESS;
    }

    if args[1] == "--neighbors" && args.len() >= 4 {
        // container-geom --neighbors <容器名> <最大距离>
        let center_name = args[2].as_str();
        let max_dist: i32 = args[3].parse().unwrap_or(3);
        let center = match from_name(center_name) {
            Some(c) => c,
            None => {
                eprintln!("未知容器名: {}", center_name);
                return ExitCode::from(1);
            }
        };
        let center_code = center.encode();
        let center_coord = hex_coord(center_code);

        println!("── {} 的 {} 阶邻居 ──", center_name, max_dist);
        println!("  中心: code=0x{:02X} 坐标=({}, {}) 极性={:?}",
            center_code, center_coord.0, center_coord.1, center.polarity());
        println!("─────────────────────────────");

        let mut neighbors: Vec<(i32, Container, u8)> = Vec::new();
        // 只遍历有效解码范围 0x00-0x3F (MASK_6); 0x40-0x4F 与 0x00-0x0F 坐标重叠
        for code in 0u8..=0x3F {
            if code == center_code { continue; }
            let c = Container::decode(code);
            let coord = hex_coord(code);
            let d = hex_distance(center_coord, coord);
            if d <= max_dist {
                neighbors.push((d, c, code));
            }
        }
        neighbors.sort_by_key(|x| x.0);
        for (d, c, code) in &neighbors {
            let compatible = if has_residual(&center, c) { "✗" } else { "✓" };
            println!("  d={}  {} ({:?}) code=0x{:02X} {}",
                d, cname(c), c.polarity(), code, compatible);
        }
        println!("── 共 {} 个邻居 ──", neighbors.len());

        // 可视化: 高亮中心 + 邻居
        let neighbor_codes: Vec<u8> = neighbors.iter().map(|(_, _, code)| *code).collect();
        println!();
        draw_hex_grid(Some(center_code), &neighbor_codes);
        return ExitCode::SUCCESS;
    }

    if args[1] == "--witness" && args.len() >= 3 {
        // container-geom --witness <容器名1>,<容器名2>,<容器名3>
        let names: Vec<&str> = args[2..].iter().flat_map(|s| s.split(',')).collect();
        if names.len() != 3 {
            eprintln!("三元见证需要恰好 3 个容器 (用逗号分隔)");
            return ExitCode::from(1);
        }
        let containers: Vec<Container> = names.iter().filter_map(|n| from_name(n.trim())).collect();
        if containers.len() != 3 {
            eprintln!("错误: 部分容器名未识别");
            return ExitCode::from(1);
        }

        let a = &containers[0];
        let b = &containers[1];
        let c = &containers[2];
        let ca = a.encode();
        let cb = b.encode();
        let cc = c.encode();
        let coord_a = hex_coord(ca);
        let coord_b = hex_coord(cb);
        let coord_c = hex_coord(cc);

        println!("═══════════════════════════════════════════");
        println!("        三元 容器 裁 决");
        println!("═══════════════════════════════════════════");
        for (label, ctr, coord) in [("见证1", a, coord_a), ("见证2", b, coord_b), ("见证3", c, coord_c)] {
            println!("  {}: {:4} ({}) 极性={:?} 坐标=({}, {}) 主权={}",
                label, ctr.polarity().value(), cname(ctr),
                ctr.polarity(), coord.0, coord.1,
                format!("{:?}", ctr.sovereignty()));
        }
        println!("───────────────────────────────────────────");
        let d_ab = hex_distance(coord_a, coord_b);
        let d_bc = hex_distance(coord_b, coord_c);
        let d_ca = hex_distance(coord_c, coord_a);
        println!("  距离: AB={} BC={} CA={}", d_ab, d_bc, d_ca);

        // 1. 极性是否构成有效见证
        let ok = triune_witness(a, b, c);
        println!("  极性见证:  {}", if ok { "✓ 非全同非全异 → 合法" } else { "✗ 全同或全异 → 无效" });

        // 2. 三方之间是否有极性互斥
        let residual_ab = has_residual(a, b);
        let residual_bc = has_residual(b, c);
        let residual_ca = has_residual(c, a);
        let any_residual = residual_ab || residual_bc || residual_ca;
        println!("  残差检测:  {}{}{}",
            if residual_ab { "AB✗ " } else { "AB✓ " },
            if residual_bc { "BC✗ " } else { "BC✓ " },
            if residual_ca { "CA✗" } else { "CA✓" });

        // 3. 主权级别是否对齐 (same side: all Active-ish or all Passive-ish)
        let sa = a.sovereignty().encode() & 1 == 0;
        let sb = b.sovereignty().encode() & 1 == 0;
        let sc = c.sovereignty().encode() & 1 == 0;
        let sov_aligned = sa == sb && sb == sc;
        println!("  主权对齐:  {}", if sov_aligned { "✓ 可裁决" } else { "✗ 级别冲突" });

        // 4. 最终裁决
        let verdict = if ok && !any_residual && sov_aligned {
            "裁定: 准入 ✓"
        } else if !ok {
            "裁定: 驳回 — 极性结构不合法"
        } else if any_residual {
            "裁定: 驳回 — 存在极性残差"
        } else {
            "裁定: 驳回 — 主权级别冲突"
        };
        println!("───────────────────────────────────────────");
        println!("  {}", verdict);
        println!("═══════════════════════════════════════════");
        return ExitCode::SUCCESS;
    }

    if args[1] == "--all" {
        println!("容器编码总览 (64 容器)");
        println!("─────────────────────────────────");
        for group in 0..4u8 {
            match group {
                0 => println!("\n【容器族】"),
                1 => println!("\n【操作族】"),
                2 => println!("\n【声母容器】"),
                3 => println!("\n【韵母/笔画容器】"),
                _ => {}
            }
            for i in 0..16u8 {
                let code = (group << 4) | i;
                if code > 0x4F { break; }
                let c = Container::decode(code);
                let (x, y) = hex_coord(code);
                println!(
                    "  {:4} code=0x{:02X} 极性={:4} 坐标=({:2},{:2}) 主权={}",
                    cname(&c),
                    code,
                    format!("{:?}", c.polarity()),
                    x,
                    y,
                    format!("{:?}", c.sovereignty())
                );
            }
        }
        return ExitCode::SUCCESS;
    }

    // 解析容器名
    let containers: Vec<Container> = args[1..].iter().filter_map(|s| from_name(s)).collect();
    if containers.len() != args[1..].len() {
        eprintln!("错误: 部分容器名未识别");
        return ExitCode::from(1);
    }

    if containers.len() == 2 {
        let a = &containers[0];
        let b = &containers[1];
        let ca = a.encode();
        let cb = b.encode();
        let coord_a = hex_coord(ca);
        let coord_b = hex_coord(cb);
        let dist = hex_distance(coord_a, coord_b);

        println!("容器几何关系");
        println!("─────────────────────────────");
        println!(
            "  A = {:4} ({:4})  code=0x{:02X}  坐标=({}, {})  极性={:?}",
            a.polarity().value(),
            cname(a),
            ca,
            coord_a.0,
            coord_a.1,
            a.polarity()
        );
        println!(
            "  B = {} ({})  code=0x{:02X}  坐标=({}, {})  极性={:?}",
            b.polarity().value(),
            cname(b),
            cb,
            coord_b.0,
            coord_b.1,
            b.polarity()
        );
        println!("─────────────────────────────");
        println!("  编码差: 0x{:02X} ({})", ca.wrapping_sub(cb), (ca as i16 - cb as i16).abs());
        println!("  几何距离: {}", dist);
        println!(
            "  极性互斥: {}",
            if has_residual(a, b) { "✗ 有残差(不兼容)" } else { "✓ 无残差(可配合)" }
        );
        println!(
            "  极性值差: {:.1}",
            (a.polarity().value() - b.polarity().value()).abs()
        );
        return ExitCode::SUCCESS;
    }

    if containers.len() == 3 {
        let a = &containers[0];
        let b = &containers[1];
        let c = &containers[2];
        println!("三元见证");
        println!("─────────────────────────────");
        for (label, ctr) in [("见证1", a), ("见证2", b), ("见证3", c)] {
            println!(
                "  {}: {} ({}) 极性={:?}",
                label,
                cname(ctr),
                ctr.polarity().value(),
                ctr.polarity()
            );
        }
        println!("─────────────────────────────");
        let ok = triune_witness(a, b, c);
        println!(
            "  见证结果: {}",
            if ok { "✓ 有效 (非全同, 非全异)" } else { "✗ 无效" }
        );
        return ExitCode::SUCCESS;
    }

    eprintln!("最多支持 3 个容器 (三元见证)");
    ExitCode::from(1)
}
