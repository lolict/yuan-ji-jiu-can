// ╔══════════════════════════════════════════════════════════════════╗
// ║  NJSG 统计文档聚合浏览器 · 后端 (std::net)                       ║
// ║  对外公开代号: 援技救残                                            ║
// ║  仅本地(127.0.0.1) — 无外部依赖                                    ║
// ║                                                                  ║
// ║  启动:                                                          ║
// ║    cargo run --bin njsg-server -- --port 8000                   ║
// ║                                                                  ║
// ║ 终点:                                                           ║
// ║   GET /                                                         → 聚合 HTML                                        ║
// ║   GET /api/index                                                → 全 JSON                                          ║
// ║   GET /api/items                                                → 所有条目                                          ║
// ║   GET /api/items/{id}                                           → 单个条目                                          ║
// ║   GET /api/items/{id}/neighbors                                 → 邻居(Chebyshev ≤ 1)                               ║
// ║   GET /api/packages                                              → 所有包裹                                          ║
// ║   GET /api/snapshot                                             → 统计锥                                            ║
// ║   GET /api/mlil_summary                                         → 四角色激活度汇总                                  ║
// ║   GET /health                                                   → 可达性检查                                        ║
// ║                                                                  ║
// ║  Copyright © 刘楚恬 & lolict | 囡囝凹月留                           ║
// ╚══════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

/// Parse --key value out of args, return None if absent
fn pick(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = pick(&args, "--port").and_then(|p| p.parse().ok()).unwrap_or(8000);

    // Browser dir resolution order:
    //   1) --dir=... or --dir ...
    //   2) NJSG_BROWSER_DIR env
    //   3) $CARGO_MANIFEST_DIR/../njsg-browser  (when run via cargo; CARGO_MANIFEST_DIR is set by cargo)
    //   4) ./njsg-browser (relative to CWD — run from hunyuan-os/)
    let dir = pick(&args, "--dir")
        .or_else(|| std::env::var("NJSG_BROWSER_DIR").ok())
        .or_else(|| {
            std::env::var("CARGO_MANIFEST_DIR").ok().map(|m| format!("{}/../njsg-browser", m))
        })
        .unwrap_or_else(|| {
            // CWD-based fallback
            let cwd = std::env::current_dir().unwrap_or_default();
            format!("{}/njsg-browser", cwd.display())
        });
    let static_path = format!("{}/index.html", dir);
    eprintln!("[njsg-server] 浏览器目录: {}", dir);
    eprintln!("[njsg-server] 静态文件: {}", static_path);

    let html_template = fs::read_to_string(&static_path).unwrap_or_else(|_| {
        eprintln!("[njsg-server] 警告: 未找到 {}, 使用 fallback", static_path);
        String::from("<!DOCTYPE html><body><h1>NJSG server (no index.html)</h1></body></html>")
    });

    let json_path = format!("{}/njsg_index.json", dir);
    let index_json: String = fs::read_to_string(&json_path).unwrap_or_else(|_| {
        eprintln!("[njsg-server] 警告: 未找到 {}", json_path);
        "{}".to_string()
    });

    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).expect("bind failed");
    eprintln!("[njsg-server] 监听于 http://{} (仅本地)", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let html = html_template.clone();
                let json = index_json.clone();
                // 单线程顺序处理 — 本地够用
                handle_conn(&mut stream, &html, &json);
            }
            Err(e) => eprintln!("[njsg-server] 连接错误: {}", e),
        }
    }
}

fn handle_conn(stream: &mut TcpStream, html: &str, index_json: &str) {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if buf_reader.read_line(&mut request_line).is_err() {
        return;
    }
    // 请求行: GET /api/items/3 HTTP/1.1
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path = parts[1];

    if method != "GET" {
        write_resp(stream, 405, "text/plain", b"ONLY GET");
        return;
    }
    if path == "/health" {
        write_resp(stream, 200, "application/json", br#"{"ok":true,"code":"NJSG-OK"}"#);
        return;
    }
    if path == "/" || path == "/index.html" {
        write_resp(stream, 200, "text/html", html.as_bytes());
        return;
    }
    if path == "/api/index" {
        write_resp(stream, 200, "application/json", index_json.as_bytes());
        return;
    }
    // /api/items, /api/items/3, /api/items/3/neighbors, /api/snapshot, /api/packages, /api/mlil_summary
    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segs.len() >= 2 && segs[0] == "api" {
        let out = route_api(&segs[1..], index_json);
        write_resp(stream, out.0, out.1, out.2.as_bytes());
        return;
    }
    write_resp(stream, 404, "text/plain", b"Not Found");
}

fn route_api(segs: &[&str], index_json: &str) -> (u16, &'static str, String) {
    let v: serde_json::Value = match serde_json::from_str(index_json) {
        Ok(v) => v,
        Err(e) => return (500, "text/plain", e.to_string()),
    };
    match segs[0] {
        "snapshot" => {
            let body = v.get("snapshot").cloned().unwrap_or(serde_json::Value::Null);
            (200, "application/json", serde_json::to_string_pretty(&body).unwrap_or_default())
        }
        "packages" => {
            let body = v.get("packages").cloned().unwrap_or(serde_json::Value::Null);
            (200, "application/json", serde_json::to_string_pretty(&body).unwrap_or_default())
        }
        "items" => {
            if segs.len() == 1 {
                let body = v.get("items").cloned().unwrap_or(serde_json::Value::Null);
                return (200, "application/json", serde_json::to_string_pretty(&body).unwrap_or_default());
            }
            let id: u32 = match segs[1].parse() {
                Ok(n) => n,
                Err(_) => return (400, "text/plain", "id 必须为整数".to_string()),
            };
            if segs.len() == 2 {
                let found = v.get("items").and_then(|i| i.as_array()).and_then(|arr| {
                    arr.iter().find(|x| x.get("id").and_then(|i| i.as_u64()) == Some(id as u64))
                });
                match found {
                    Some(it) => (200, "application/json", serde_json::to_string_pretty(it).unwrap_or_default()),
                    None => (404, "text/plain", "id 不存在".to_string()),
                }
            } else if segs.get(2) == Some(&"neighbors") {
                let max_cheb = 1u32;
                let target_coord = v.get("items").and_then(|i| i.as_array()).and_then(|arr| {
                    arr.iter().find(|x| x.get("id").and_then(|i| i.as_u64()) == Some(id as u64))
                }).and_then(|t| {
                    let c = t.get("coord")?;
                    Some((c.get("x")?.as_i64().unwrap_or(0),
                         c.get("y")?.as_i64().unwrap_or(0),
                         c.get("z")?.as_i64().unwrap_or(0)))
                });
                let (tx, ty, tz) = match target_coord {
                    Some(c) => c,
                    None => return (404, "text/plain", "target 不存在".to_string()),
                };
                let neighbors = v.get("neighbors").and_then(|i| i.as_array());
                let result = serde_json::json!({
                    "target_id": id,
                    "target_coord": {"x": tx, "y": ty, "z": tz},
                    "neighbor_entries": neighbors.map(|ns| {
                        ns.iter().filter_map(|e| {
                            let cheb = e.get("chebyshev")?.as_u64()?;
                            if cheb <= max_cheb as u64 { Some(e.clone()) } else { None }
                        }).collect::<Vec<_>>()
                    }).unwrap_or_default()
                });
                (200, "application/json", serde_json::to_string_pretty(&result).unwrap_or_default())
            } else {
                (404, "text/plain", "No such path".to_string())
            }
        }
        "mlil_summary" => {
            let mut counter: HashMap<String, usize> = HashMap::new();
            if let Some(arr) = v.get("items").and_then(|i| i.as_array()) {
                for it in arr {
                    let primary = it.get("mlil_primary").and_then(|p| p.as_str()).unwrap_or("未知");
                    *counter.entry(primary.to_string()).or_insert(0) += 1;
                }
            }
            let out = serde_json::json!({
                "snapshot": v.get("snapshot"),
                "mlil_distribution": counter,
                "note": "毛利兰=模/立体/母体; kndl=外壳/界壳/显示; 工藤新一=抽取/折叠/索引; 灰原哀=横/行/表列"
            });
            (200, "application/json", serde_json::to_string_pretty(&out).unwrap_or_default())
        }
        _ => (404, "text/plain", "Not Found".to_string()),
    }
}

fn write_resp(stream: &mut TcpStream, status: u16, ctype: &str, body: &[u8]) {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Unknown",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status, status_text, ctype, body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
    let _ = stream.flush();
}
