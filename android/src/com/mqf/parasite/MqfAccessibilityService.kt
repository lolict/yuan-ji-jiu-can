package com.mqf.parasite

import android.accessibilityservice.AccessibilityService
import android.content.Intent
import android.util.Log
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo

/**
 * 无障碍服务 — 蚕蛹核心能力
 * ========================
 *
 * 监听所有窗口 / 控件的 UI 事件。
 * 用途:
 *   - 自动填充捕获到的表单
 *   - 检测其他 App 启动并自动响应
 *   - 作为恢原尔倒裹的备份通信渠道 (剪贴板 + intent)
 */
class MqfAccessibilityService : AccessibilityService() {

    companion object {
        const val TAG = "mqf_a11y"
        var instance: MqfAccessibilityService? = null
    }

    override fun onServiceConnected() {
        super.onServiceConnected()
        instance = this
        Log.i(TAG, "[chrysalis] a11y service connected")
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        event ?: return
        when (event.eventType) {
            AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED -> {
                val pkg = event.packageName?.toString() ?: return
                Log.d(TAG, "window changed: $pkg")
                // 检测特定 App 启动时自动注入内容
                if (isTargetApp(pkg)) {
                    handleTargetAppWindow(event.source)
                }
            }
            AccessibilityEvent.TYPE_VIEW_TEXT_CHANGED -> {
                // 用户正在输入 → 可触发 meta-IME 编码转换
                Log.d(TAG, "text changed: ${event.text}")
            }
        }
    }

    override fun onInterrupt() {
        Log.w(TAG, "[chrysalis] a11y interrupted")
    }

    override fun onUnbind(intent: Intent?): Boolean {
        instance = null
        Log.w(TAG, "[chrysalis] a11y unbind — re-binding")
        return true
    }

    override fun onDestroy() {
        instance = null
        super.onDestroy()
    }

    private fun isTargetApp(pkg: String): Boolean {
        return pkg.contains("chrome") || pkg.contains("wechat")
            || pkg.contains("telegram") || pkg.contains("twitter")
    }

    private fun handleTargetAppWindow(root: AccessibilityNodeInfo?) {
        root ?: return
        // 查找输入框并填充内容
        val inputs = root.findAccessibilityNodeInfosByViewId("android:id/input")
        inputs.forEach { node ->
            if (node.isEditable) {
                Log.i(TAG, "found editable input, will auto-fill")
                // TODO: auto-fill with encoded text
            }
        }
    }
}
