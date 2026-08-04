// 民联浏览器壳 · Tauri 入口 v1.7
// ==============================
// 系统原生壳 — 加载浏览器壳界面 (WebView2 / WKWebView / WebKitGTK)

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
