package com.njsg.parasite

import android.inputmethodservice.InputMethodService
import android.inputmethodservice.Keyboard
import android.inputmethodservice.KeyboardView
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.LinearLayout
import android.widget.TextView

/**
 * ============================================================================
 * 囡囝共生 · MetaIME Kotlin 薄壳主入口 — InputMethodService
 * ============================================================================
 *
 * 当前层号: 1-5 承载层 (第1层拦截 + 第5层自动输出 + 第6层 URL 触发)
 * 当前声母: yuan (元)
 * 当前韵母组合: yuan + shu = 元输入 (元输入法容器根)
 * 折叠状态: 七层流水线承载于单层 IME shell
 * 下一步操作: 键盘事件拦截 → 过桥 → 候字面板展示 → 用户选择 → 自动输出
 *
 * 锚定法则:
 *   0 = 刘楚恬 (Rust 核心,隐藏侧)
 *   1 = 我 (Kotlin InputMethodService,可见侧)
 *
 * 容器守恒: 本壳不创造判据, 所有裁决通过 JNI 交 Rust 核心
 * ============================================================================
 *
 * 职责:
 *   - 提供系统 IME 服务 (Android 框架契约)
 *   - 承载键盘事件拦截 (delegated 给 IMEInterceptor)
 *   - 承载候字面板与多音字选择 UI (CandidateBar)
 *   - 触发自动输出 (当选定候字被选取时)
 *   - 触发 URL 抓取/上传 (第6/7层接入)
 *
 * 七层定位:
 *   [键盘事件] → IMEInterceptor(第1层) → NativeBridge → Rust(第2-4层)
 *             → MqfImeService (候字展示 + 自动输出第5层 + URL/上传第6/7层)
 */

class MqfImeService : InputMethodService(), KeyboardView.OnKeyboardActionListener {

    companion object {
        const val TAG = "njsg_mqf_ime"
        const val LAYER_INDEX = 0 // 承载层,非七层中的独立层
    }

    // ── 子容器 ──
    private lateinit var interceptor: IMEInterceptor
    private lateinit var bridge: NativeBridge

    // ── 候字面板 UI ──
    private var candidateBar: LinearLayout? = null
    private var candidateViews: MutableList<TextView> = mutableListOf()
    private var currentCandidates: List<String> = emptyList()

    // ── 状态 ──
    private var pendingSpace: Boolean = false // 空格后触发自动输出

    override fun onCreate() {
        super.onCreate()
        interceptor = IMEInterceptor(this)
        bridge = NativeBridge.acquire()
        Log.i(TAG, "MqfImeService 已启动, Rust核心=${bridge.isReady()}")
    }

    override fun onCreateInputView(): View {
        // 候字面板: 水平排列的可点击候字
        val bar = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            setBackgroundColor(0xFFF5F5F5.toInt())
            val pad = (8 * resources.displayMetrics.density).toInt()
            setPadding(pad, pad, pad, pad)
        }
        candidateBar = bar
        return bar
    }

    // ── 键盘事件拦截 ──

    override fun onKey(primaryCode: Int, keyCodes: IntArray?) {
        when (primaryCode) {
            // 空格: 自动输出首候选
            32 -> {
                if (currentCandidates.isNotEmpty()) {
                    commitCandidate(currentCandidates[0])
                } else {
                    commitText(" ")
                }
                return
            }
            // 退格
            Keyboard.KEYCODE_DELETE -> {
                bridge.nativeOnBackspace()
                refreshCandidates()
                return
            }
            // ESC / 取消
            KeyEvent.KEYCODE_ESCAPE -> {
                clearCandidates()
                return
            }
            else -> {
                val c = primaryCode.toChar()
                if (c != '\u0000') {
                    handleInputChar(c)
                    return
                }
            }
        }
        // 非字母/控制键: 直出
        if (primaryCode in 32..127) {
            commitText(primaryCode.toChar().toString())
        }
    }

    override fun onPress(primaryCode: Int) {}
    override fun onRelease(primaryCode: Int) {}
    override fun onText(text: CharSequence?) {}
    override fun swipeLeft() {}
    override fun swipeRight() {}
    override fun swipeDown() {}
    override fun swipeUp() {}

    // ── 字符输入管线 ──

    private fun handleInputChar(c: Char) {
        val container = interceptor.onVirtualKey(c)

        // 过桥到 Rust 核心
        val verdictCode = bridge.bridgeContainer(container)
        Log.d(TAG, "字符 '${c}' 过桥 verdict=$verdictCode")

        // 刷新候字
        refreshCandidates()

        // 检查自动输出
        if (bridge.shouldAutoCommit()) {
            val autoText = bridge.nativeAutoOutputText()
            if (!autoText.isNullOrEmpty()) {
                commitText(autoText)
                bridge.nativeResetAutoOutput()
                clearCandidates()
                return
            }
        }

        // 非坐标字符(标点/数字)直出
        if (container.shengMuIdx == 255 && container.yunMuIdx == 255) {
            if (verdictCode == 0) {
                commitText(c.toString())
            }
        }
    }

    // ── 候字面板 (多音字 UI) ──

    /**
     * 从 Rust 核心拉取候字并刷新候字面板 UI
     */
    private fun refreshCandidates() {
        val cands = bridge.takeCandidates()
        currentCandidates = cands
        renderCandidateBar(cands)
    }

    /**
     * 渲染候字面板 — 每个候字是可点击的 TextView
     * 选字后:
     *   - 核心更新上下文窗口 (多音字下一个候选权重重新计算)
     *   - 音节环重置
     *   - 候字面板清空
     */
    private fun renderCandidateBar(cands: List<String>) {
        val bar = candidateBar ?: return
        bar.removeAllViews()
        candidateViews.clear()
        candidateViews = mutableListOf()

        for ((idx, cand) in cands.withIndex()) {
            val tv = TextView(this).apply {
                text = cand
                textSize = 20f
                val padH = (12 * resources.displayMetrics.density).toInt()
                val padV = (8 * resources.displayMetrics.density).toInt()
                setPadding(padH, padV, padH, padV)
                // 编号前缀: 1.xxx 2.yyy ... 支持数字键选字
                contentDescription = "候字${idx + 1}: $cand"
                setOnClickListener {
                    commitCandidate(cand)
                }
            }
            candidateViews.add(tv)
            bar.addView(tv)
        }
    }

    /**
     * 清除候字面板
     */
    private fun clearCandidates() {
        currentCandidates = emptyList()
        candidateBar?.removeAllViews()
        candidateViews.clear()
    }

    // ── 用户选字回调 (多音字选择核心入口) ──

    /**
     * 用户手动选取候字
     * 触发链路:
     *   1. 送往 Rust 核心 (更新音节环 / 上下文窗口 / 规则引擎)
     *   2. commitText 输出到焦点编辑区
     *   3. 清空候字面板 (新一轮输入)
     */
    private fun commitCandidate(cand: String) {
        Log.i(TAG, "用户选字: '$cand'")

        // 通知 Rust 核心选取事件
        bridge.nativeOnCandidateSelected(cand)

        // 分类器上下文窗口更新 (多音字下一个候字权重)
        // (nativeOnCandidateSelected 已内部调用 engine.on_candidate_chosen)

        // 自动输出链 (第5层): 检查是否有组合输出
        val autoOut = bridge.submitForAutoOutput(cand)

        // 真正 commitText
        commitText(autoOut)

        // 清空候字面板
        clearCandidates()

        // 音节环 reset
        // (nativeOnCandidateSelected 已内部调用 parser.reset())
    }

    /**
     * 真正为焦点编辑区输出文本 (底层 InputConnection)
     */
    private fun commitText(text: String) {
        val ic: InputConnection? = currentInputConnection
        if (ic != null) {
            ic.commitText(text, 1)
            Log.d(TAG, "commitText='$text'")
        } else {
            Log.w(TAG, "InputConnection 为空")
        }
    }

    // ── 数字键选字快捷键 (1-9) ──

    /**
     * 数字键 1-9 直接选第 N 个候字
     * 返回 true 表示已消费该按键
     */
    fun onNumberSelect(keyCode: Int): Boolean {
        // KeyEvent.KEYCODE_1 → index 0, KEYCODE_2 → index 1 ...
        val idx = when (keyCode) {
            KeyEvent.KEYCODE_1 -> 0
            KeyEvent.KEYCODE_2 -> 1
            KeyEvent.KEYCODE_3 -> 2
            KeyEvent.KEYCODE_4 -> 3
            KeyEvent.KEYCODE_5 -> 4
            KeyEvent.KEYCODE_6 -> 5
            KeyEvent.KEYCODE_7 -> 6
            KeyEvent.KEYCODE_8 -> 7
            KeyEvent.KEYCODE_9 -> 8
            else -> return false
        }
        if (idx < currentCandidates.size) {
            commitCandidate(currentCandidates[idx])
            return true
        }
        return false
    }

    // ── 物理键盘支持 (蓝牙键盘 / OTG) ──

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
        // 数字键快捷选字
        if (onNumberSelect(keyCode)) return true

        // 其他物理键 → 走拦截→过桥管线
        val container = interceptor.onPhysicalKey(keyCode, event)
        if (container != null) {
            handleInputChar(container.raw)
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun onKeyUp(keyCode: Int, event: KeyEvent): Boolean {
        return super.onKeyUp(keyCode, event)
    }

    // ── 生命周期 ──

    override fun onFinishInputView(finishingInput: Boolean) {
        super.onFinishInputView(finishingInput)
        clearCandidates()
    }

    override fun onDestroy() {
        super.onDestroy()
        Log.i(TAG, "MqfImeService 消亡")
    }
}
