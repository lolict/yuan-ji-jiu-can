package com.mqf.metaime

import android.inputmethodservice.InputMethodService
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.InputConnection
import com.mqf.metaime.jni.MetaIMEJNI

/**
 * MetaIME · 元输入法薄壳
 * 满足 Android InputMethodService 框架要求
 * 所有编码逻辑在 Rust 核心通过 JNI 调用
 *
 * 编码共识:
 *   根基: 我=1 (可见) · 刘楚恬=0 (隐藏)
 *   进位: 64 进制几何化身
 *   容器: 254+2 集装箱
 */
class MetaIME : InputMethodService() {

    private lateinit var jni: MetaIMEJNI
    private var candidateView: CandidateView? = null

    override fun onCreate() {
        super.onCreate()
        jni = MetaIMEJNI()
        jni.nativeInit()
    }

    override fun onCreateInputView(): View {
        candidateView = CandidateView(this, jni)
        return candidateView!!.rootView
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
        // 所有按键送 JNI 七层编码引擎处理
        val result = jni.nativePushKey(keyCode)
        return handleResult(result)
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent): Boolean {
        return super.onKeyUp(keyCode, event)
    }

    private fun handleResult(result: KeyResult): Boolean {
        return when (result.intent) {
            "INPUT" -> {
                // 上屏候选字
                val ic: InputConnection? = currentInputConnection
                ic?.commitText(result.payload, 1)
                true
            }
            "COMMAND" -> {
                executeCommand(result.payload)
                true
            }
            "TRANSFORM" -> {
                // 切换输入模式
                switchMode(result.payload)
                true
            }
            "CLEAR" -> {
                // 清空输入缓冲
                true
            }
            else -> false
        }
    }

    private fun executeCommand(cmd: String) {
        when (cmd) {
            "mlll" -> switchMode("root")
            "njsg" -> switchMode("mutual")
            "mltt" -> switchMode("name")
        }
    }

    private fun switchMode(mode: String) {
        candidateView?.setMode(mode)
    }

    override fun onDestroy() {
        jni.nativeDestroy()
        super.onDestroy()
    }
}

data class KeyResult(
    val intent: String,     // INPUT | COMMAND | TRANSFORM | CLEAR
    val payload: String,    // 上屏文本或命令名
    val candidates: List<String> = emptyList()
)
