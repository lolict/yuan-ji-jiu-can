package com.mqf.shell

import android.annotation.SuppressLint
import android.os.Bundle
import android.webkit.JavascriptInterface
import android.webkit.WebChromeClient
import android.webkit.WebResourceRequest
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity
import org.json.JSONObject

/**
 * 民联浏览器壳 · Android WebView v1.10
 * ====================================
 * 满全法裁定引擎 Android 容器
 *
 * 特性:
 * - 加载本地 assets/mqf_shell/index.html (零网络)
 * - 双向 JS <-> Native 桥 (mqfShellBridge)
 * - 本地存储通过 window.mqfNative.save/load 暴露给 JS
 * - 适配 Android 14+ (SDK 34+, targetSdk 34)
 */
class MqfShellActivity : AppCompatActivity() {

    private lateinit var webView: WebView
    private val prefsName = "mqf_shell_prefs"

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        webView = WebView(this).apply {
            settings.apply {
                javaScriptEnabled = true
                domStorageEnabled = true
                allowFileAccess = true
                allowContentAccess = true
                cacheMode = WebSettings.LOAD_DEFAULT
                mediaPlaybackRequiresUserGesture = false
                useWideViewPort = true
                loadWithOverviewMode = true
                setSupportZoom(true)
                builtInZoomControls = true
                displayZoomControls = false
                mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
            }
        }

        // 注入 Native 桥
        webView.addJavascriptInterface(MqfNativeBridge(this), "mqfNative")

        webView.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(view: WebView?, request: WebResourceRequest?): Boolean {
                // 限制只在 file:// 范围内,防止外部跳转
                val url = request?.url ?: return false
                return !url.scheme.equals("file") && !url.scheme.equals("https")
            }

            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                // 通知 JS:Native 桥就绪
                webView.evaluateJavascript(
                    "window.mqfShellNativeReady && window.mqfShellNativeReady()", null
                )
            }
        }

        webView.webChromeClient = WebChromeClient()

        setContentView(webView)

        // 加载本地壳
        webView.loadUrl("file:///android_asset/mqf_shell/index.html")
    }

    // 系统返回键 -> WebView 后退
    @Deprecated("Deprecated in Java")
    override fun onBackPressed() {
        if (webView.canGoBack()) {
            webView.goBack()
        } else {
            @Suppress("DEPRECATION")
            super.onBackPressed()
        }
    }

    override fun onDestroy() {
        webView.destroy()
        super.onDestroy()
    }
}

/**
 * JS ↔ Native 桥 (Android 端)
 *
 * JS 调用方式:
 *   // 保存数据
 *   mqfNative.save("verdict_history", jsonString);
 *
 *   // 读取数据
 *   const data = mqfNative.load("verdict_history");
 *
 *   // 系统信息
 *   const info = mqfNative.getPlatform();  // {"platform":"android","version":"..."}
 */
class MqfNativeBridge(private val activity: MqfShellActivity) {

    @JavascriptInterface
    fun save(key: String, value: String): Boolean {
        return try {
            val prefs = activity.getSharedPreferences(
                "mqf_shell_prefs", MODE_PRIVATE
            )
            prefs.edit().putString(key, value).apply()
            true
        } catch (e: Exception) {
            false
        }
    }

    @JavascriptInterface
    fun load(key: String): String? {
        val prefs = activity.getSharedPreferences(
            "mqf_shell_prefs", MODE_PRIVATE
        )
        return prefs.getString(key, null)
    }

    @JavascriptInterface
    fun getPlatform(): String {
        return JSONObject().apply {
            put("platform", "android")
            put("version", android.os.Build.VERSION.RELEASE)
            put("sdk", android.os.Build.VERSION.SDK_INT)
            put("model", android.os.Build.MODEL)
            put("manufacturer", android.os.Build.MANUFACTURER)
        }.toString()
    }

    companion object {
        private const val MODE_PRIVATE = 0
    }

    @JavascriptInterface
    fun clear(key: String): Boolean {
        return try {
            val prefs = activity.getSharedPreferences(
                "mqf_shell_prefs", 0
            )
            if (key.isEmpty()) {
                prefs.edit().clear().apply()
            } else {
                prefs.edit().remove(key).apply()
            }
            true
        } catch (e: Exception) {
            false
        }
    }
}
