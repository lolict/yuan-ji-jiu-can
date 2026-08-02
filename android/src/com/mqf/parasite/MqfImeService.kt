package com.mqf.parasite

import android.inputmethodservice.InputMethodService
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.TextView

/**
 * 输入法服务 (InputMethodService) — 元输入法 (MetaIME) Android 端
 * ==============================================
 *
 * 每部手机允许且仅需一个 IME 默认开启。
 * 满全法接管后:
 *   - 拦截所有按键 → 转七层神龙编码
 *   - AI 意图识别 → 自动补全 / 自动执行
 *   - 通过 channel_router 路由到目标位置
 */
class MqfImeService : InputMethodService() {

    companion object {
        const val TAG = "mqf_ime"
        var instance: MqfImeService? = null
    }

    private var inputBuffer = StringBuilder()

    override fun onCreate() {
        super.onCreate()
        instance = this
        Log.i(TAG, "[chrysalis] IME service created")
    }

    override fun onCreateInputView(): View {
        // 极简浮层:仅一个透明的 TextView 接收输入
        return TextView(this).apply {
            text = "[mqf]"
            setTextColor(0xFF6B9D99.toInt())
            textSize = 14f
            setPadding(8, 4, 8, 4)
            Log.i(TAG, "[chrysalis] IME input view created")
        }
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        val c = event?.unicodeChar?.toChar() ?: return false
        inputBuffer.append(c)
        Log.i(TAG, "[chrysalis] key: $c buffer=$inputBuffer")
        // TODO: 触发意图识别 + 自动执行
        return super.onKeyUp(keyCode, event)
    }

    override fun onFinishInputView(finishingInput: Boolean) {
        // 用户切走 → 缓冲清空
        inputBuffer.clear()
        super.onFinishInputView(finishingInput)
    }

    override fun onDestroy() {
        instance = null
        super.onDestroy()
    }
}
