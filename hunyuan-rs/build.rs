// ══════════════════════════════════════════════════════════════════
// build.rs — 编译 C 编码系统并链接到 Rust 核心
// ══════════════════════════════════════════════════════════════════
//
// 把 encoding/ 目录下的 C 源码编译为静态库，
// 供 Rust 编码桥接模块 (encoding_bridge.rs) 通过 FFI 调用。

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();
    let encoding_dir = workspace_root.join("encoding");

    // ── 编译 C 编码系统 ──
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
