package com.mqf.metaime

import android.content.Context
import android.graphics.Color
import android.view.Gravity
import android.view.View
import android.widget.HorizontalScrollView
import android.widget.LinearLayout
import android.widget.TextView
import com.mqf.metaime.jni.MetaIMEJNI

/**
 * CandidateView · MetaIME 候选条
 * 渲染 L6 意图候选列表
 *
 * 视觉编码:
 *   普通候选: 常规色
 *   命令候选: 高亮 (mlgy 天庭色 #FFD700)
 *   当前模式指示: 左角标 (mlll/njsg/mltt)
 */
class CandidateView(context: Context, private val jni: MetaIMEJNI) {

    val rootView: LinearLayout
    private val slotLayout: LinearLayout
    private val modeLabel: TextView

    init {
        rootView = LinearLayout(context).apply {
            orientation = LinearLayout.HORIZONTAL
            setBackgroundColor(0xFFF5F5F5.toInt())
            val pad = (12 * context.resources.displayMetrics.density).toInt()
            setPadding(pad, pad / 2, pad, pad / 2)
        }

        modeLabel = TextView(context).apply {
            text = "mlll"
            setTextColor(0xFF888888.toInt())
            textSize = 12f
            gravity = Gravity.CENTER_VERTICAL
        }
        rootView.addView(modeLabel)

        slotLayout = LinearLayout(context).apply {
            orientation = LinearLayout.HORIZONTAL
            val lp = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            layoutParams = lp
        }
        rootView.addView(slotLayout)
    }

    fun updateCandidates(candidates: List<String>) {
        slotLayout.removeAllViews()
        candidates.forEachIndexed { i, hz ->
            val tv = TextView(rootView.context).apply {
                text = "${i + 1}.$hz"
                setTextColor(0xFF333333.toInt())
                textSize = 18f
                val pad = (8 * rootView.context.resources.displayMetrics.density).toInt()
                setPadding(pad, pad / 2, pad, pad / 2)
            }
            slotLayout.addView(tv)
        }
    }

    fun setMode(mode: String) {
        modeLabel.text = when (mode) {
            "root" -> "mlll"
            "mutual" -> "njsg"
            "name" -> "mltt"
            else -> "●"
        }
    }
}
