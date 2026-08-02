package com.mqf.parasite

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

/**
 * 开机启动接收器 — 蚕蛹触发点之一
 */
class MqfBootReceiver : BroadcastReceiver() {

    companion object {
        const val TAG = "mqf_boot"
    }

    override fun onReceive(context: Context?, intent: Intent?) {
        Log.i(TAG, "[chrysalis] boot completed — launching CoreService")
        context ?: return
        context.startForegroundService(Intent(context, MqfCoreService::class.java))
    }
}
