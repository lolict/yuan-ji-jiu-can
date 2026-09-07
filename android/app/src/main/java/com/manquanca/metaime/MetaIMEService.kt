package com.manquanca.metaime

import android.inputmethodservice.InputMethodService
import android.view.View
import android.view.inputmethod.InputConnection
import android.widget.TextView
import android.widget.LinearLayout
import android.view.Gravity
import android.util.TypedValue
import android.graphics.Color
import android.view.ViewGroup

/**
 * 元输入法 Android 薄壳 (InputMethodService)
 * 核心逻辑在 Rust libmetaime_jni.so,本层仅负责 UI 框架 + IME 协议
 */
class MetaIMEService : InputMethodService() {

    private var nativeHandle: Long = 0L
    private val candidateViews = mutableListOf<TextView>()
    private var candidateBar: LinearLayout? = null

    override fun onCreate() {
        super.onCreate()
        nativeHandle = MetaIMEMative.init()
    }

    override fun onDestroy() {
        if (nativeHandle != 0L) {
            MetaIMEMative.free(nativeHandle)
            nativeHandle = 0L
        }
        super.onDestroy()
    }

    override fun onCreateInputView(): View {
        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setBackgroundColor(Color.parseColor("#1A1A2E"))
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }

        // 候选词栏
        candidateBar = LinearLayout(this).apply {
            orientation = LinearLayout.HORIZONTAL
            gravity = Gravity.START
            setPadding(8, 8, 8, 8)
        }

        // 拼音输入区
        val pinyinView = TextView(this).apply {
            text = "满全法 · 元输入法"
            setTextColor(Color.WHITE)
            setTextSize(TypedValue.COMPLEX_UNIT_SP, 18f)
            setPadding(16, 16, 16, 16)
        }

        layout.addView(candidateBar)
        layout.addView(pinyinView)

        // 演示: 查询 "ni3" 候选词
        if (nativeHandle != 0L) {
            // n = 声母 7, i = 韵母 4, 3声 = tone 3
            val result = MetaIMEMative.query(nativeHandle, 7, 4, 3)
            if (result != null) {
                showCandidates(result)
            }
        }

        return layout
    }

    private fun showCandidates(csv: String) {
        candidateBar?.removeAllViews()
        candidateViews.clear()

        val chars = csv.split(",").filter { it.isNotBlank() }
        for (ch in chars.take(6)) {
            val tv = TextView(this).apply {
                text = ch
                setTextColor(Color.parseColor("#E0E0E0"))
                setTextSize(TypedValue.COMPLEX_UNIT_SP, 22f)
                setPadding(24, 12, 24, 12)
                setOnClickListener {
                    currentInputConnection?.commitText(ch, 1)
                }
            }
            candidateViews.add(tv)
            candidateBar?.addView(tv)
        }
    }

    override fun onStartInputView(info: android.view.inputmethod.EditorInfo?, restarting: Boolean) {
        super.onStartInputView(info, restarting)
    }

    override fun onKeyDown(keyCode: Int, event: android.view.KeyEvent?): Boolean {
        return super.onKeyDown(keyCode, event)
    }
}
