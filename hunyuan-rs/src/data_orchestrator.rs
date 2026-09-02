// ══════════════════════════════════════════════════════════════════
// data_orchestrator.rs — 真实数据输入编排器 · Mode C
// ══════════════════════════════════════════════════════════════════
//
// 修复 Mode B 的路径问题, 使用绝对路径直接从项目根目录扫描。
// 输出 JSON 格式的聚类结果供前端消费。

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// FNV-1a 64-bit 哈希
pub fn fnv1a(data: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut h = FNV_OFFSET;
    for byte in data.bytes() {
        h ^= byte as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    format!("{:016X}", h)
}

/// 正向极性关键词
const POSITIVE_KEYS: &[&str] = &[
    "完成", "成功", "激活", "推进", "增长", "实现", "确认", "运行",
    "通过", "启动", "解决", "发现", "证明", "新建", "创建",
];

/// 负向极性关键词
const NEGATIVE_KEYS: &[&str] = &[
    "失败", "错误", "警告", "问题", "冲突", "禁止", "否定", "溢出",
    "不", "无", "未", "作废", "拒绝", "移除",
];

/// 极性估算结果
#[derive(Debug, Clone, PartialEq)]
pub enum PolarityScore {
    Positive(i32),
    Negative(i32),
    Neutral,
}

impl PolarityScore {
    /// 转整数: 1=阳, -1=阴, 0=中性
    pub fn to_int(&self) -> i32 {
        match self {
            PolarityScore::Positive(n) if *n >= 2 => 1,
            PolarityScore::Negative(n) if *n >= 2 => -1,
            _ => 0,
        }
    }
}

/// 估算文本极性
pub fn estimate_polarity(content: &str) -> PolarityScore {
    let mut score: i32 = 0;
    for kw in POSITIVE_KEYS {
        if content.contains(kw) {
            score += 1;
        }
    }
    for kw in NEGATIVE_KEYS {
        if content.contains(kw) {
            score -= 1;
        }
    }
    if score >= 2 {
        PolarityScore::Positive(score)
    } else if score <= -2 {
        PolarityScore::Negative(score.abs())
    } else {
        PolarityScore::Neutral
    }
}

/// 估算文档权重 (基于大小)
pub fn estimate_weight(content: &str) -> f64 {
    content.len() as f64 / 100.0
}

/// 文档条目
#[derive(Debug, Clone, PartialEq)]
pub struct DocEntry {
    pub path: String,
    pub hash: String,
    pub polarity: i32,
    pub weight: f64,
    pub size: usize,
}

impl DocEntry {
    pub fn new(path: &str, hash: &str, polarity: i32, weight: f64, size: usize) -> Self {
        DocEntry {
            path: path.to_string(),
            hash: hash.to_string(),
            polarity,
            weight,
            size,
        }
    }
}

/// 聚类结果
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterResult {
    pub total_docs: usize,
    pub polar_pos: usize,
    pub polar_neg: usize,
    pub polar_neu: usize,
    pub total_weight: f64,
    pub docs: Vec<DocEntry>,
}

impl ClusterResult {
    /// 构建聚类结果
    pub fn new(docs: Vec<DocEntry>) -> Self {
        let polar_pos = docs.iter().filter(|d| d.polarity > 0).count();
        let polar_neg = docs.iter().filter(|d| d.polarity < 0).count();
        let polar_neu = docs.iter().filter(|d| d.polarity == 0).count();
        let total_weight: f64 = docs.iter().map(|d| d.weight).sum();

        ClusterResult {
            total_docs: docs.len(),
            polar_pos,
            polar_neg,
            polar_neu,
            total_weight,
            docs,
        }
    }

    /// 打印简要报告
    pub fn print_report(&self) {
        println!("真实数据输入报告");
        println!("────────────────────────");
        println!("  总文档数:    {}", self.total_docs);
        println!("  阳性极性:    {}", self.polar_pos);
        println!("  阴性极性:    {}", self.polar_neg);
        println!("  中性极性:    {}", self.polar_neu);
        println!("  总权重:      {:.2}", self.total_weight);
        println!("────────────────────────");
    }
}

/// 扫描项目目录
pub fn scan_project(root: &str, extensions: &[&str], max_files: usize) -> Vec<DocEntry> {
    let ext_set: HashSet<String> = extensions.iter().map(|e| e.to_string()).collect();
    let mut docs = Vec::new();
    let path = Path::new(root);

    if let Ok(entries) = visit_dirs(path, &ext_set, &mut docs, max_files) {
        entries
    } else {
        docs
    }
}

/// 递归访问目录 (前 max_files 个匹配文件)
fn visit_dirs(
    dir: &Path,
    ext_set: &HashSet<String>,
    docs: &mut Vec<DocEntry>,
    max_files: usize,
) -> Result<Vec<DocEntry>, std::io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // 跳过 build 目录
            if let Some(name) = path.file_name() {
                let name = name.to_string_lossy();
                if name == "target" || name == "nimcache" || name == ".git" || name == "node_modules" {
                    continue;
                }
            }

            if path.is_dir() {
                visit_dirs(&path, ext_set, docs, max_files)?;
            } else if docs.len() < max_files {
                if let Some(ext) = path.extension() {
                    let ext_str = format!(".{}", ext.to_string_lossy());
                    if ext_set.contains(&ext_str) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if !content.is_empty() {
                                let pol = estimate_polarity(&content);
                                let w = estimate_weight(&content);
                                let rel_path = path.to_string_lossy()
                                    .replace(&format!("{}/", dir.to_string_lossy()), "");
                                docs.push(DocEntry::new(
                                    &rel_path,
                                    &fnv1a(&content),
                                    pol.to_int(),
                                    w,
                                    content.len(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(docs.clone())
}

// ══════════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a() {
        let h1 = fnv1a("hello world");
        let h2 = fnv1a("hello world");
        let h3 = fnv1a("different");
        assert_eq!(h1, h2);  // 相同输入 = 相同哈希
        assert_ne!(h1, h3);  // 不同输入 = 不同哈希
        assert_eq!(h1.len(), 16);  // 64-bit = 16 hex chars
    }

    #[test]
    fn test_estimate_polarity_positive() {
        let text = "完成 成功 激活 推进";
        let pol = estimate_polarity(text);
        assert_eq!(pol.to_int(), 1);
    }

    #[test]
    fn test_estimate_polarity_negative() {
        let text = "失败 错误 问题 警告 禁止";
        let pol = estimate_polarity(text);
        assert_eq!(pol.to_int(), -1);
    }

    #[test]
    fn test_estimate_polarity_neutral() {
        let text = "这是一个普通的文本 没有任何明显的极性";
        let pol = estimate_polarity(text);
        assert_eq!(pol.to_int(), 0);
    }

    #[test]
    fn test_estimate_weight() {
        assert_eq!(estimate_weight(""), 0.0);
        assert_eq(&estimate_weight("a".repeat(100).as_str()), &1.0);
        assert_eq(&estimate_weight("a".repeat(200).as_str()), &2.0);
    }

    #[test]
    fn test_doc_entry() {
        let doc = DocEntry::new("test.md", "ABCD", 1, 42.0, 100);
        assert_eq!(doc.path, "test.md");
        assert_eq!(doc.polarity, 1);
        assert_eq!(doc.size, 100);
    }

    #[test]
    fn test_cluster_result() {
        let docs = vec![
            DocEntry::new("a.md", "AAA", 1, 10.0, 1000),
            DocEntry::new("b.md", "BBB", -1, 20.0, 2000),
            DocEntry::new("c.md", "CCC", 0, 30.0, 3000),
        ];
        let cluster = ClusterResult::new(docs);
        assert_eq!(cluster.total_docs, 3);
        assert_eq!(cluster.polar_pos, 1);
        assert_eq!(cluster.polar_neg, 1);
        assert_eq!(cluster.polar_neu, 1);
        assert_eq!(cluster.total_weight, 60.0);
    }
}
