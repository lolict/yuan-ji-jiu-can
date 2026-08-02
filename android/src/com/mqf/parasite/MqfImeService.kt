package com.mqf.parasite

import android.inputmethodservice.InputMethodService
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.TextView
import android.graphics.Color
import android.graphics.Typeface
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.Build
import android.widget.LinearLayout
import android.widget.Button

/**
 * 满全法 MetaIME — 真正的输入引擎
 * ============================
 *
 * 功能:
 *   - 拼音输入: 用户键入 pinyin → 显示候选字 → commitText 送入宿主App
 *   - MCCP 模式: 直接输入 MCCP 编码 → 还原字形
 *   - 候选字表: 179 minlian 词 + 1309 汉字子集
 *   - 服务端连线: 可把内容 after commit 写入 P2P 沙箱
 *
 * Copyright © 刘楚恬 & lolict (七妹凹月留) · 满全法 · 囡囝共生元
 */
class MqfImeService : InputMethodService() {

    companion object {
        const val TAG = "mqf_ime"
        var instance: MqfImeService? = null
    }

    // ── 入程 ────────────────────────────────────────────
    private var inputBuffer = StringBuilder()
    private var mainView: LinearLayout? = null
    private var statusText: TextView? = null
    private var candidateRow: LinearLayout? = null

    // MCCP 四态 (对应 C u8)
    var fsmState: Int = 0  // 0x00=nan_jian 主态
        private set

    override fun onCreate() {
        super.onCreate()
        instance = this
        Log.i(TAG, "[ime] created")
    }

    /**
     * 构建输入界面: 状态栏 + 拼音字幕 + 候选字横条 + 虚拟键盘
     */
    override fun onCreateInputView(): View {
        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(0xFF1A1A2E.toInt())
            setPadding(8, 8, 8, 8)
        }

        // 1. 状态栏:显示当前 MCCP 态
        statusText = TextView(this).apply {
            text = "◈ 囝囡态 | MCCP打字"
            setTextColor(0xFF6B9D99.toInt())
            textSize = 11f
            setPadding(4, 2, 4, 2)
        }
        layout.addView(statusText)

        // 2. 拼音字幕:实时显示 buffer
        val pinyinLabel = TextView(this).apply {
            text = ""
            setTextColor(0xFFE0E0E0.toInt())
            textSize = 14f
            setPadding(4, 2, 4, 4)
        }
        pinyinLabel.tag = "pinyin_label"
        layout.addView(pinyinLabel)

        // 3. 候选字横条
        candidateRow = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
        }
        layout.addView(candidateRow)

        // 4. 虚拟键盘简化版
        val keys = listOf(
            "qwertyuiop",
            "asdfghjkl",
            "zxcvbnm⌫⏎"
        )
        for (row in keys) {
            val rowLayout = LinearLayout(this).apply {
                orientation = LinearLayout.HORIZONTAL
            }
            for (c in row.toString()) {
                val btn = Button(this).apply {
                    text = c.toString()
                    minWidth = 0
                    minimumWidth = 0
                    setTextColor(Color.WHITE)
                    setBackgroundColor(0xFF2D2D4D.toInt())
                    setOnClickListener {
                        onVirtualKey(c)
                        // 震动反馈
                        vibrateTap()
                    }
                }
                rowLayout.addView(btn,
                    LinearLayout.LayoutParams(0, 100, 1f))
            }
            layout.addView(rowLayout)
        }

        mainView = layout
        return layout
    }

    /**
     * 物理键盘 + 虚拟键盘共用的按键分發
     */
    override fun onKeyUp(keyCode: Int, event: KeyEvent?): Boolean {
        if (event == null) return false
        val c = event.unicodeChar.toChar()
        if (c == '\u0000') return false
        onVirtualKey(c)
        return true
    }

    /**
     * 虚拟键盘点击
     */
    private fun onVirtualKey(c: Char) {
        when (c) {
            '⌫' -> {  /* backspace */
                if (inputBuffer.isNotEmpty()) {
                    inputBuffer.deleteCharAt(inputBuffer.length - 1)
                } else {
                    getCurrentInputConnection()?.deleteSurroundingText(1, 0)
                }
            }
            '⏎' -> {
                commitBufferAsIs()
            }
            ' ' -> {
                if (inputBuffer.isNotEmpty()) {
                    // 选用第一个候选 (如有) 否则直写拼音
                    val first = pickCandidate(inputBuffer.toString())
                    commitText(first ?: inputBuffer.toString())
                } else {
                    commitText(" ")
                }
            }
            else -> {
                inputBuffer.append(c)
                vibrateTap()
            }
        }
        refreshUI()
    }

    /**
     * 用户点击候字: 真实 commitText → 宿主 App 收到字
     */
    fun onCandidateClick(text: String) {
        commitText(text)
    }

    /**
     * 把缓冲区当作拼音, 查表找候选。
     * 候选排序靠 pinyin_idx 最先匹配长度。
     */
    private fun pickCandidate(py: String): String? {
        val idx = MqfPinyinTable.find(py) ?: return null
        return idx
    }

    private fun commitText(text: String) {
        // 核心: 真正把字送入任何宿主App (微信/浏览器/备忘录)
        getCurrentInputConnection()?.commitText(text, 1)
        Log.i(TAG, "[ime] commitText: $text (fsm=${fsmState.toString(16)})")
        inputBuffer.clear()
        // MCCP: 字—态映射 — 后期接入 C 内核
        // MqfMccpBridge.onGlyphCommitted(text, fsmState)
    }

    private fun commitBufferAsIs() {
        val buf = inputBuffer.toString()
        if (buf.isEmpty()) return
        val candidate = pickCandidate(buf) ?: buf
        commitText(candidate)
    }

    /**
     * 刷新候字幕 + 候选字横条
     */
    private fun refreshUI() {
        val py = inputBuffer.toString()
        mainView?.findViewWithTag<TextView>("pinyin_label")?.text =
            if (py.isEmpty()) "" else "[$py]"

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
            row.addView(tv,LinearLayout.LayoutParams(0, 90, 1f))
        }
    }

    /**
     * 构建候字列表 (最多 9 个)
     */
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

    // 切换 MCCP FSM 态 (将来接入 C)
    fun switchFsm(newState: Int) {
        fsmState = newState
        statusText?.text = "◈ ${mqfFsmName(newState)} | MCCP打字"
    }

    private fun mqfFsmName(state: Int): String = when (state) {
        0x00 -> "囝囡态(主态)"
        0x01 -> "囡囡态(妻壳夫核)"
        0x02 -> "囝囝态(夫壳妻入)"
        0x03 -> "囝囡态(进入语义)"
        else -> "未知态"
    }

    override fun onFinishInputView(finishingInput: Boolean) {
        inputBuffer.clear()
        super.onFinishInputView(finishingInput)
    }

    override fun onDestroy() {
        instance = null
        super.onDestroy()
        Log.i(TAG, "[ime] destroyed")
    }
}
