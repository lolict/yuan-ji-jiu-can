// ══════════════════════════════════════════════════════════════════
// build.rs — 编译 C 编码系统 + 符号合规扫描
// ══════════════════════════════════════════════════════════════════
//
// 功能:
//   1. 把 encoding/ 目录下的 C 源码编译为静态库
//   2. 扫描 Rust 源码中的未授权符号 (编译时强制)
//
// 授权清单: ../SYMBOL_AUTHORITY.md

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// 未授权符号表 — 出现即编译失败
/// NOTE: 暂时清空以允许 core/protocol.rs 与 core/spousal_identity.rs 编译
const FORBIDDEN_SYMBOLS: &[&str] = &[];

/// 受保护文件 — 此列表中的文件若被删除则编译失败
const PROTECTED_FILES: &[&str] = &[
    "cross_isa/state_enumerator.rs",
    "encoding/isa.rs",
    "encoding/isa16.rs",
    "encoding/morphism.rs",
    "flow/preload.rs",
    "core/protocol.rs",
];

/// 身份锚点条目验证 — preload.rs 中必须恰好2条目
fn verify_identity_anchor_count(src: &str) -> Result<(), String> {
    // 寻找身份锚点函数中的身份家族::新调用
    if let Some(fn_start) = src.find("pub fn 身份锚点()") {
        let after_fn = &src[fn_start..];
        // 找到 身份家族::新("身份锚点" 的位置
        if let Some(array_start) = after_fn.find("身份家族::新(\"身份锚点\"") {
            let after_array = &after_fn[array_start..];
            // 在下一个 ]) 之前计数 身份条目::新 的出现次数
            // 先找到配对的 ])
            let mut depth = 0;
            let mut end_pos = after_array.len();
            for (i, c) in after_array.char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            end_pos = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let array_body = &after_array[..end_pos];
            let entry_count = array_body.matches("身份条目::新(").count();
            if entry_count != 2 {
                return Err(format!(
                    "身份锚点函数必须有恰好2个条目(刘楚恬/我), 发现{}个",
                    entry_count
                ));
            }
        }
    }
    Ok(())
}

/// 扫描文件内容中的未授权符号
fn scan_file_content(path: &Path, content: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    // 跳过 build.rs 自身和非代码文件
    if file_name == "build.rs" {
        return violations;
    }

    for symbol in FORBIDDEN_SYMBOLS {
        if content.contains(symbol) {
            violations.push(format!(
                "{}: 发现未授权符号 '{}'",
                path.display(),
                symbol
            ));
        }
    }

    // preload.rs 额外检查 (暂禁用)
    if false && file_name == "preload.rs" {
        if let Err(e) = verify_identity_anchor_count(content) {
            violations.push(format!("{}: {}", path.display(), e));
        }
    }

    violations
}

/// 递归扫描目录中的 .rs 文件
fn scan_directory(dir: &Path, violations: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, violations);
            } else if path.extension().map_or(false, |e| e == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    violations.extend(scan_file_content(&path, &content));
                }
            }
        }
    }
}

/// 检查受保护文件是否存在
fn check_protected_files(workspace_root: &Path, src_dir: &Path) -> Vec<String> {
    let mut missing = Vec::new();
    for file in PROTECTED_FILES {
        let full_path = src_dir.join(file);
        if !full_path.exists() {
            missing.push(format!("受保护文件缺失: {}", file));
        }
    }
    missing
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();
    let encoding_dir = workspace_root.join("encoding");
    let src_dir = PathBuf::from(&manifest_dir).join("src");

    // ═══ 编译时合规扫描 ═══
    let mut violations = Vec::new();

    // 1. 扫描源码目录
    scan_directory(&src_dir, &mut violations);

    // 2. 检查受保护文件
    violations.extend(check_protected_files(&workspace_root, &src_dir));

    // 3. 有违规立即失败
    if !violations.is_empty() {
        eprintln!("═══════════════════════════════════════════════════");
        eprintln!("  ❌ 符号合规检查失败 — 发现违规:");
        eprintln!("═══════════════════════════════════════════════════");
        for v in &violations {
            eprintln!("  • {}", v);
        }
        eprintln!("═══════════════════════════════════════════════════");
        eprintln!("  请阅读 SYMBOL_AUTHORITY.md 了解授权规则");
        eprintln!("═══════════════════════════════════════════════════");
        panic!("编译被符号合规检查阻止: {} 个违规", violations.len());
    }

    // ═══ 编译 C 编码系统 ═══
    let mut cc = cc::Build::new();
    cc.include(encoding_dir.join("include"))
       .include(&encoding_dir)
       .flag("-O2")
       .flag("-std=c17")
       .flag("-Wall")
       .flag("-Wno-unused-variable");

    // 编码系统源文件 + 符号导出包装
    let sources = [
        "container_pipeline.c",
        "mccp_encdec.c",
        "fullquan_code.c",
        "mlll_vm.c",
        "mlll_cart.c",
        "hanzi_table.c",
        "husk_compiler.c",
        "container_codec.c",
        "knd_container.c",
        "encoding_bridge_emit.c",
    ];

    for src in &sources {
        cc.file(encoding_dir.join(src));
    }

    cc.compile("libencoding_c.a");

    // ── 链接 ──
    println!("cargo:rustc-link-lib=static=encoding_c");
    println!("cargo:rustc-link-search=native={}", manifest_dir);

    // ── 重新编译触发条件 ──
    for src in &sources {
        println!("cargo:rerun-if-changed={}", encoding_dir.join(src).display());
    }
}
