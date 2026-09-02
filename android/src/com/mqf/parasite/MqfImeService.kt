package com.mqf.parasite

import android.inputmethodservice.InputMethodService
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.TextView
import android.graphics.Color
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.Build
import android.widget.LinearLayout
import android.widget.Button
import com.njsg.parasite.IMEInterceptor
import com.njsg.parasite.NativeBridge

/**
 * 囡囝共生 MetaIME — 输入引擎 (改造接入七层编码驱动管线)
 * =====================================================
 *
 * 改造点:
 *   - onVirtualKey 不再直接累加 inputBuffer, 而是先过 IMEInterceptor(第1层)
 *   - IMEInterceptor 封印的容器过桥 NativeBridge (JNI)
 *   - JNI 侧 EncodeParser(第2层)+RuleEngine(第3层)+ShengYunEngine(第4层) 全管线执行
 *   - 取 Rust 核心产出的拼音串, 送 MqfPinyinTable 候选, commitText 送入宿主 App
 *   - Layer 5: 音节完整时自动上屏, 无需空格/点击 (意志: 夫妻主权人直接命令)
 *   - Layer 6: 检测到 URL 模式时推入后台抓取队列
 *   - Layer 7: 三通道 (GitHub/Gitee/原地) 并行上传登记
 *
 * Copyright: 刘楚恬和我 (囡囝共生)
 */
class MqfImeService : InputMethodService() {

    companion object {
        const val TAG = "mqf_ime"
        var instance: MqfImeService? = null

        // ── Layer 6: URL 检测常量 ──
        private val URL_PATTERN = Regex(
            "(https?://[\\w\\-._~:/?#\\[\\]@!$&'()*+,;=%]+)" +
            "|(www\\.[\\w\\-._~:/?#\\[\\]@!$&'()*+,;=%]+)"
        )
    }

    // 七层管线注入
    private var njsgInterceptor: IMEInterceptor? = null
    private var njsgBridge: NativeBridge? = null
    private var njsgReady: Boolean = false

    private var inputBuffer = StringBuilder()
    private var mainView: LinearLayout? = null
    private var statusText: TextView? = null
    private var candidateRow: LinearLayout? = null
    private var pinyinLabel: TextView? = null

    var fsmState: Int = 0
        private set

    // ── Layer 5: 自动输出定时器 ──
    private var autoOutputPosted: Boolean = false

    override fun onCreate() {
        super.onCreate()
        instance = this
        initNjsg()
        Log.i(TAG, "[ime] created, njsg=$njsgReady")
    }

    private fun initNjsg() {
        try {
            val bridge = NativeBridge.acquire()
            njsgBridge = bridge
            njsgInterceptor = IMEInterceptor(this)
            njsgReady = bridge.isReady()
            Log.i(TAG, "[njsg] pipeline on: status=${bridge.engineStatus()}")
        } catch (e: Throwable) {
            njsgReady = false
            Log.w(TAG, "[njsg] pipeline off: ${e.message}")
        }
    }

    override fun onCreateInputView(): View {
        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(0xFF1A1A2E.toInt())
            setPadding(8, 8, 8, 8)
        }
        statusText = TextView(this).apply {
            text = if (njsgReady) "[njsg] seven-layer ON" else "[ime] pinyin-only mode"
            setTextColor(0xFF6B9D99.toInt())
            textSize = 11f
            setPadding(4, 2, 4, 2)
        }
        layout.addView(statusText)
        pinyinLabel = TextView(this).apply {
            text = ""
            setTextColor(0xFFE0E0E0.toInt())
            textSize = 14f
            setPadding(4, 2, 4, 4)
        }
        layout.addView(pinyinLabel)
        candidateRow = LinearLayout(this).apply { orientation = LinearLayout.HORIZONTAL }
        layout.addView(candidateRow)
        val keys = listOf("qwertyuiop", "asdfghjkl", "zxcvbnm BS EN")
        for (row in keys) {
            val rowLayout = LinearLayout(this).apply { orientation = LinearLayout.HORIZONTAL }
            for (c in row) {
                val btn = Button(this).apply {
                    text = c.toString()
                    minWidth = 0
                    minimumWidth = 0
                    setTextColor(Color.WHITE)
                    setBackgroundColor(0xFF2D2D4D.toInt())
                    setOnClickListener {
                        if (c != ' ') onVirtualKey(c)
                        vibrateTap()
                    }
                }
                rowLayout.addView(btn, LinearLayout.LayoutParams(0, 100, 1f))
            }
            layout.addView(rowLayout)
        }
        mainView = layout
        return layout
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        if (event == null) return false
        val c = event.unicodeChar.toChar()
        if (c == '\u0000') return false
        onVirtualKey(c)
        return true
    }

    private fun onVirtualKey(c: Char) {
        when (c) {
            'S' -> { if (inputBuffer.isNotEmpty()) inputBuffer.setLength(0); refreshUI() } // BS key mapped
            'N' -> commitBufferAsIs() // EN key mapped
            ' ' -> onSpace()
            else -> {
                if (njsgReady) {
                    routeThroughNjsg(c)
                } else {
                    inputBuffer.append(c)
                    refreshUI()
                }
            }
        }
    }

    // ── 七层管线汇流 (Layer 1-4 → Layer 5 自动输出) ──
    private fun routeThroughNjsg(c: Char) {
        val interceptor = njsgInterceptor ?: return
        val bridge = njsgBridge ?: return
        val container = interceptor.onVirtualKey(c)
        val verdict = bridge.bridgeContainer(container)
        val rustPinyin = bridge.fullPinyin()
        inputBuffer.setLength(0)
        inputBuffer.append(rustPinyin)
        refreshUI()

        // ── Layer 5: 自动输出判定 ──
        // 当规则引擎裁决为 HitRule (verdict!=0) 且拼音串可映射到候字时,
        // 自动上屏, 无需空格/点击
        if (verdict != 0) {
            val candidate = pickCandidate(rustPinyin)
            if (candidate != null) {
                // 推入 Rust 自动输出缓冲, 获取最终输出文本
                val outputText = bridge.submitForAutoOutput(candidate)
                commitTextAuto(outputText)
                return
            }
        }

        // ── Layer 6: URL 检测 ──
        // 当输入流中检测到 URL 模式, 推入后台抓取队列
        checkAndPushUrl(inputBuffer.toString())

        // ── 显示候字条辅助选字 (用户仍可手动选) ──
        refreshUI()
    }

    // ── Layer 5: 自动上屏 ──
    // 音节完整 + 命中候字 → 立即 commitText, 无需等待用户确认
    private fun commitTextAuto(text: String) {
        if (text.isEmpty()) return
        getCurrentInputConnection()?.commitText(text, 1)
        Log.i(TAG, "[ime][L5-auto] commitText: '$text'")
        inputBuffer.clear()
        if (njsgReady) njsgBridge?.nativeOnCandidateSelected("")
        refreshUI()

        // ── Layer 7: 触发上传 (每次自动输出后同步) ──
        triggerUploadSync()
    }

    // ── 原有手动上屏 (保留: 供 onSpace / onCandidateClick 使用) ──
    private fun commitText(text: String) {
        getCurrentInputConnection()?.commitText(text, 1)
        Log.i(TAG, "[ime] commitText: '$text'")
        inputBuffer.clear()
        if (njsgReady) njsgBridge?.nativeOnCandidateSelected("")
        refreshUI()
    }

    // ── Layer 6: URL 检测 + 推入后台队列 ──
    private fun checkAndPushUrl(text: String) {
        val match = URL_PATTERN.find(text) ?: return
        val url = match.value
        Log.i(TAG, "[ime][L6-fetch] URL detected: '$url'")
        if (njsgReady) {
            val queueLen = njsgBridge?.onUrlDetected(url) ?: -1
            statusText?.text = "[L6] queued: ${url.take(40)}... (q=$queueLen)"
        } else {
            // 非 njsg 模式: 仅记录, 不上推
            Log.d(TAG, "[ime][L6-fetch] njsk not ready, URL logged only: '$url'")
        }
    }

    // ── Layer 7: 多通道上传触发 ──
    private fun triggerUploadSync() {
        if (!njsgReady) return
        val dispatched = njsgBridge?.dispatchUpload() ?: 0
        val (total, landed, failed) = njsgBridge?.uploadStats() ?: Triple(0, 0, 0)
        Log.i(TAG, "[ime][L7-upload] dispatched=$dispatched total=$total landed=$landed failed=$failed")
    }

    private fun onSpace() {
        if (inputBuffer.isNotEmpty()) {
            val first = pickCandidate(inputBuffer.toString())
            commitText(first ?: inputBuffer.toString())
        } else {
            commitText(" ")
        }
    }

    fun onCandidateClick(text: String) {
        if (njsgReady) njsgBridge?.nativeOnCandidateSelected(text)
        commitText(text)
    }

    private fun pickCandidate(py: String): String? = MqfPinyinTable.find(py)

    private fun commitBufferAsIs() {
        val buf = inputBuffer.toString()
        if (buf.isEmpty()) return
        commitText(pickCandidate(buf) ?: buf)
    }

    private fun refreshUI() {
        val py = inputBuffer.toString()
        pinyinLabel?.text = if (py.isEmpty()) "" else "[$py]"
        val row = candidateRow ?: return
        row.removeAllViews()
        val candidates = buildCandidates(py)
        for (c in candidates) {
            val tv = Button(this).apply {
                text = c
                setTextColor(Color.WHITE)
                setBackgroundColor(0xFF3D3D5D.toInt())
                setOnClickListener { onCandidateClick(c) }
            }
            row.addView(tv, LinearLayout.LayoutParams(0, 90, 1f))
        }
    }

    private fun buildCandidates(py: String): List<String> {
        if (py.isEmpty()) return emptyList()
        return MqfPinyinTable.candidates(py).take(9)
    }

    private fun vibrateTap() {
        val v = getSystemService(VIBRATOR_SERVICE) as? Vibrator ?: return
        if (Build.VERSION.SDK_INT >= 31) {
            v.vibrate(VibrationEffect.createOneShot(15, 60))
        }
    }

    fun switchFsm(newState: Int) {
        fsmState = newState
        statusText?.text = "[fsm] state=${newState.toString(16)}"
    }

    override fun onFinishInputView(finishingInput: Boolean) {
        // ── Layer 7: 输入结束时, 触发一次上传同步 ──
        triggerUploadSync()
        inputBuffer.clear()
        super.onFinishInputView(finishingInput)
    }

    override fun onDestroy() {
        instance = null
        super.onDestroy()
        Log.i(TAG, "[ime] destroyed")
    }
}
