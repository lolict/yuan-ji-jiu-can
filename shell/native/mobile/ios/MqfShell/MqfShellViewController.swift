/**
 * 民联浏览器壳 · iOS WKWebView v1.10
 * ==================================
 * 囡囝共生裁定引擎 iOS 容器 (Swift)
 *
 * 特性:
 * - 加载本地 Bundle 中的 index.html (零网络)
 * - 双向 JS <-> Native 桥 (mqfNative)
 * - UserDefaults 持久化 (save/load/clear)
 * - 适配 iOS 16+
 */

import UIKit
import WebKit

class MqfShellViewController: UIViewController {

    // MARK: - Properties

    private var webView: WKWebView!
    private let userDefaults = UserDefaults.standard
    private let bridgeName = "mqfNative"

    // MARK: - Lifecycle

    override func viewDidLoad() {
        super.viewDidLoad()
        setupWebView()
        loadShell()
    }

    deinit {
        webView.configuration.userContentController.removeScriptMessageHandler(forName: bridgeName)
    }

    // MARK: - Setup

    private func setupWebView() {
        let config = WKWebViewConfiguration()
        config.preferences.javaScriptEnabled = true
        config.defaultWebpagePreferences.allowsContentJavaScript = true

        // 注册消息处理器 (Native 接收 JS 消息)
        config.userContentController.add(self, name: bridgeName)

        // 注入 JS 桥
        let bridgeSource = "window.mqfNative = {" +
            "save: function(k,v) { window.webkit.messageHandlers.mqfNative.postMessage({method:'save',key:k,value:v}); return true; }," +
            "load: function(k) { return localStorage.getItem(k); }," +
            "getPlatform: function() { return JSON.stringify({platform:'ios', model: navigator.platform, version: '16+', manufacturer: 'Apple'}); }," +
            "clear: function(k) { if(k) localStorage.removeItem(k); else localStorage.clear(); return true; }" +
        "};"

        let bridgeScript = WKUserScript(
            source: bridgeSource,
            injectionTime: .atDocumentEnd,
            forMainFrameOnly: false
        )
        config.userContentController.addUserScript(bridgeScript)

        webView = WKWebView(frame: view.bounds, configuration: config)
        webView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        webView.scrollView.bounces = false
        webView.scrollView.contentInsetAdjustmentBehavior = .never

        view.addSubview(webView)
    }

    private func loadShell() {
        // 优先从 mqf_shell 子目录加载
        var htmlPath = Bundle.main.path(forResource: "index", ofType: "html", inDirectory: "mqf_shell")
        if htmlPath == nil {
            htmlPath = Bundle.main.path(forResource: "index", ofType: "html")
        }

        guard let path = htmlPath else {
            print("[MQF] index.html not found in Bundle")
            return
        }

        let htmlURL = URL(fileURLWithPath: path)
        let folderURL = htmlURL.deletingLastPathComponent()
        webView.loadFileURL(htmlURL, allowingReadAccessTo: folderURL)
    }
}

// MARK: - WKScriptMessageHandler

extension MqfShellViewController: WKScriptMessageHandler {

    func userContentController(_ userContentController: WKUserContentController,
                               didReceive message: WKScriptMessage) {
        guard message.name == bridgeName,
              let body = message.body as? [String: Any],
              let method = body["method"] as? String else { return }

        switch method {
        case "save":
            if let key = body["key"] as? String,
               let value = body["value"] as? String {
                userDefaults.set(value, forKey: key)
            }

        case "load":
            let key = body["key"] as? String ?? ""
            let value = userDefaults.string(forKey: key) ?? ""
            let reqId = body["reqId"] as? String ?? ""
            DispatchQueue.main.async { [weak self] in
                self?.webView.evaluateJavaScript(
                    "window.__mqfNativeResolve && window.__mqfNativeResolve('\(reqId)', '\(value.replacingOccurrences(of: "'", with: "\\'"))')",
                    completionHandler: nil
                )
            }

        case "clear":
            let key = body["key"] as? String ?? ""
            if key.isEmpty {
                if let bundleID = Bundle.main.bundleIdentifier {
                    userDefaults.removePersistentDomain(forName: bundleID)
                }
            } else {
                userDefaults.removeObject(forKey: key)
            }

        default:
            break
        }
    }
}
