// ╔══════════════════════════════════════════════════════════════════╗
// ║  MLIL 框架 — 毛利兰/kndl/工藤新一/灰原哀                         ║
// ║  接入 knml☉l0l 统计文档聚合浏览器                               ║
// ║                                                                  ║
// ║  映射关系(基于用户提供的四个名字):                                ║
// ║  毛利兰 (Maori Lan)   = 模/立体  → 3D 坐标母体(方位/卦限/卦限进动)  ║
// ║  kndl (Kernel Delta) = 外壳/界壳 → 显示层(卡片/面板/搜索/包裹)     ║
// ║  工藤新一 (Kudo Shinichi) = 旋翼 → 抽取与索引(url抓取/关键词/折叠)   ║
// ║  灰原哀 (Haibara Ai) = 横/行      → 平铺统计排列(沿轴展开表格)      ║
// ║                                                                  ║
// ║  激活优先级: 毛利兰 → kndl → 工藤新一 → 灰原哀                    ║
// ║  每个内容项可以同时属于多个角色(多重分工)                          ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict | 囡囝凹月留                           ║
// ╚══════════════════════════════════════════════════════════════════╝

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum MlilRole {
    Maori,     // 毛利兰 — 立体/母体
    Kndl,      // kndl     — 外壳/界壳
    Kudo,      // 工藤新一  — 抽* /折叠
    Haibara,   // 灰原哀   — 横/行
}

impl MlilRole {
    pub fn label_cn(&self) -> &'static str {
        match self {
            MlilRole::Maori => "毛利兰",
            MlilRole::Kndl => "kndl",
            MlilRole::Kudo => "工藤新一",
            MlilRole::Haibara => "灰原哀",
        }
    }

    pub fn duty(&self) -> &'static str {
        match self {
            MlilRole::Maori => "模/立体/母体",
            MlilRole::Kndl => "外壳/界壳/显示",
            MlilRole::Kudo => "抽身/折叠/索引",
            MlilRole::Haibara => "横/行/表列",
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            MlilRole::Maori => "3D 坐标母体: 方位/卦限/卦限进动; 承载整体骨架",
            MlilRole::Kndl => "显示层与界面: 卡片/面板/搜索/包裹; 对外的可交互界壳",
            MlilRole::Kudo => "抽取与索引核心: url 抓取/关键词折叠/距离计算; 承担信息加工",
            MlilRole::Haibara => "横展表列: 平铺统计/沿轴展开表格; 作为统计锥的外视通道",
        }
    }

    pub fn layer_index(&self) -> u8 {
        match self {
            MlilRole::Maori => 0,
            MlilRole::Kndl => 1,
            MlilRole::Kudo => 2,
            MlilRole::Haibara => 3,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MlilRoleAssignment {
    pub role: MlilRole,
    pub activation: f64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MlilAnatomy {
    pub roles: Vec<MlilRoleAssignment>,
    pub summary: String,
}

/// 基于 (layer, coord, keywords, source) 四要素, 将一个内容项
/// 分解到四个角色. 激活度 0..1, 四项之和归一化为 1.
pub fn assign_roles(layer: u8, coord: &(i8, i8, i8), keywords: &[String], source: &str) -> MlilAnatomy {
    let mut maori: f64 = 0.0;
    let mut kndl: f64 = 0.0;
    let mut kudo: f64 = 0.0;
    let mut haibara: f64 = 0.0;

    // 毛利兰(立体/母体) — 坐标+z越浅立体度越高 + 源头层越浅越立体
    maori += ((3 - coord.2) as f64).min(2.0) * 0.05;
    maori += ((3 - layer) as f64).min(2.0) * 0.05;
    // 操作系统/共生/公有母题 → 毛利兰(立体母题)
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("OS") || k.contains("共生") || k.contains("隐*私") {
            maori += 0.08;
        }
        if k.contains("操作*系统") || k.contains("漏*洞") {
            maori += 0.12;
        }
    }

    // kndl(外壳/界壳) — 显示与通道: 表层条目 + 渠道 + 容器
    kndl += (layer as f64) * 0.05;
    if source == "minlian" {
        kndl += 0.14;
    }
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("浏览器") || k.contains("通道") || k.contains("kndl") {
            kndl += 0.14;
        }
        if k.contains("容*器") || k.contains("格*式") || k.contains("转换") {
            kndl += 0.10;
        }
        // 表层条目默认有外壳成分
        if k.contains("对*外") || k.contains("显*示") || k.contains("界*面") {
            kndl += 0.08;
        }
    }

    // 工藤新一(抽取 / 折叠 / 索引) — 信息抽取与加工
    kudo += 0.04; // 保底: 每个条目都需要某种索引
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("诊断") || k.contains("通*诊") || k.contains("索引") || k.contains("抽身") || k.contains("折叠") {
            kudo += 0.16;
        }
        if k.contains("古文") || k.contains("共生") {
            kudo += 0.08;
        }
        if k.contains("隐*私") || k.contains("搜索") || k.contains("关*键*词") {
            kudo += 0.08;
        }
    }
    if source == "deepseek-share" {
        kudo += 0.08;
    }

    // 灰原哀(横 / 行 / 平铺) — 每个条目都潜在地要展现给外部,所以有保底
    haibara += 0.06;
    haibara += (coord.1 as f64) * 0.04;
    for kw in keywords {
        let k = kw.to_lowercase();
        if k.contains("红*") || k.contains("标签") || k.contains("序列") {
            haibara += 0.16;
        }
        if k.contains("统*计") || k.contains("表*") || k.contains("平*铺") || k.contains("横*") {
            haibara += 0.10;
        }
        // 任何有关表现、展示、外宣的词都加
        if k.contains("展示") || k.contains("排*列") || k.contains("模*版") || k.contains("页*面") {
            haibara += 0.08;
        }
    }

    // 所有角色保底,避免 0 值(保证四个角色都有代表)
    maori = maori.max(0.05);
    kndl = kndl.max(0.05);
    kudo = kudo.max(0.05);
    haibara = haibara.max(0.05);

    // 归一化
    let total = maori + kndl + kudo + haibara;
    let norm = |v: f64| if total <= 0.0 { 0.25 } else { (v / total).clamp(0.0, 1.0) };

    let maori_n = norm(maori);
    let kndl_n = norm(kndl);
    let kudo_n = norm(kudo);
    let haibara_n = norm(haibara);

    // 选主角色(激活度最高者)
    let mut pairs = vec![
        (MlilRole::Maori, maori_n, "承载体".to_string()),
        (MlilRole::Kndl, kndl_n, "面向用户/外壳".to_string()),
        (MlilRole::Kudo, kudo_n, "抽取/折叠/索引".to_string()),
        (MlilRole::Haibara, haibara_n, "平铺统计排列".to_string()),
    ];
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let primary = &pairs[0];

    let summary = format!(
        "主角色: {}({}) {:.0}% · 毛利兰 {:.0}% · kndl {:.0}% · 工藤新一 {:.0}% · 灰原哀 {:.0}%",
        primary.0.label_cn(), primary.2,
        primary.1 * 100.0,
        maori_n * 100.0,
        kndl_n * 100.0,
        kudo_n * 100.0,
        haibara_n * 100.0,
    );

    let roles = pairs.into_iter().map(|(r, a, n)| MlilRoleAssignment {
        role: r,
        activation: (a * 100.0).round() / 100.0,
        note: n,
    }).collect();

    MlilAnatomy { roles, summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_roles_totals_near_one() {
        let kws = vec!["共生".to_string(), "古文".to_string()];
        let anatomy = assign_roles(1, &(1, 0, 0), &kws, "deepseek-share");
        let sum: f64 = anatomy.roles.iter().map(|r| r.activation).sum();
        assert!((sum - 1.0).abs() < 0.05, "total = {}", sum);
    }

    #[test]
    fn mnjian_is_mostly_maori() {
        // 源头 + z=0 → 毛利兰母体化
        let kws = vec!["源头".to_string()];
        let anatomy = assign_roles(0, &(0, 0, 0), &kws, "minlian");
        let maori = anatomy.roles.iter().find(|r| matches!(r.role, MlilRole::Maori)).unwrap();
        assert!(maori.activation > 0.2, "maori activation = {}", maori.activation);
    }

    #[test]
    fn browser_is_kndl() {
        let kws = vec!["浏览器".to_string(), "通道".to_string()];
        let anatomy = assign_roles(0, &(2, 2, 1), &kws, "minlian");
        let kndl = anatomy.roles.iter().find(|r| matches!(r.role, MlilRole::Kndl)).unwrap();
        // minlian + 通道 + 浏览器 → kndl 应该较高
        assert!(kndl.activation >= 0.25, "kndl activation = {}", kndl.activation);
    }
}
