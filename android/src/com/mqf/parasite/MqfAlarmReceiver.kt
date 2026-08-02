package com.mqf.parasite

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

class MqfAlarmReceiver : BroadcastReceiver() {
    companion object { const val TAG = "mqf_alarm" }
    override fun onReceive(context: Context?, intent: Intent?) {
        Log.i(TAG, "[chrysalis] alarm wakeup")
        context?.startForegroundService(Intent(context, MqfCoreService::class.java))
    }
}
