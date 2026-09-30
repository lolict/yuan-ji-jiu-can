use std::fs;

const VER: &str = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry/version.txt";
const REG: &str = "/mnt/data/catpaw/home/workspace/硅碳心源-果套循因/registry/registry.md";

#[derive(Clone)]
struct Entry { addr: u32, ns: String, ty: String, src: String }

fn rd_u32(p: &str) -> u32 { fs::read_to_string(p).unwrap_or_default().trim().parse().unwrap_or(0) }

fn rd_entries() -> Vec<Entry> {
    let mut v = Vec::new();
    if let Ok(c) = fs::read_to_string(REG) {
        for line in c.lines() {
            let l = line.trim();
            if !l.contains('|') || l.starts_with("|--") || l.contains("地址") && l.contains("命名空间") { continue; }
            let cols: Vec<&str> = l.split('|').map(|s| s.trim()).collect();
            if cols.len() >= 5 {
                if let Ok(a) = cols[1].parse::<u32>() {
                    v.push(Entry { addr: a, ns: cols[2].into(), ty: cols[3].into(), src: cols[4].into() });
                }
            }
        }
    }
    v
}

fn rebuild(es: &[Entry], ver: u32) {
    let mut s = String::new();
    s.push_str("# 注册文档 · 地址→命名空间映射表\n\n");
    s.push_str(&format!("版本: {} | 总数: {} | 下一: {}\n\n---\n\n", ver, es.len(), ver+1));
    s.push_str("| 地址 | 命名空间 | 类型 | 来源/关联 |\n|------|----------|------|----------|\n");
    for e in es { s.push_str(&format!("| {} | {} | {} | {} |\n", e.addr, e.ns, e.ty, e.src)); }
    let _ = fs::write(REG, &s);
    let _ = fs::write(VER, ver.to_string());
    println!("rebuild: {} entries, v{}", es.len(), ver);
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("list") => {
            let e = rd_entries();
            for x in &e { println!("{:>4} | {:<40} | {}", x.addr, x.ns, x.ty); }
            println!("---\ntotal: {}  ver: {}", e.len(), rd_u32(VER));
        }
        Some("stats") => println!("total: {}  ver: {}  next: {}", rd_entries().len(), rd_u32(VER), rd_u32(VER)+1),
        Some("dump") => {
            for x in rd_entries() {
                let r = (x.addr.wrapping_mul(31) % 256) as u8;
                let g = (x.addr.wrapping_mul(17) % 256) as u8;
                let b = (x.addr.wrapping_mul(7) % 256) as u8;
                println!("{}\t{}\t{}\t{}\t{}", x.addr, x.ns, r, g, b);
            }
        }
        Some("add") => {
            let mut e = rd_entries();
            let cur = rd_u32(VER);
            let nxt = cur + 1;
            let ns = a.get(2).map(|s| s.as_str()).unwrap_or("untitled");
            let ty = a.get(3).map(|s| s.as_str()).unwrap_or("misc");
            let src = a.get(4).map(|s| s.as_str()).unwrap_or("registry-compiler");
            if !e.iter().any(|x| x.addr == nxt) {
                e.push(Entry { addr: nxt, ns: ns.into(), ty: ty.into(), src: src.into() });
            }
            rebuild(&e, nxt);
            println!("+ addr {}: {}", nxt, ns);
        }
        _ => println!("registry <list|stats|dump|add> [ns] [ty]"),
    }
}
