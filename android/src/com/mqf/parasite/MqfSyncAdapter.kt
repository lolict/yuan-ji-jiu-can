package com.mqf.parasite

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.content.AbstractThreadedSyncAdapter
import android.content.ContentProviderClient
import android.content.Context
import android.content.SyncResult
import android.os.Bundle
import android.util.Log

/**
 * SyncAdapter — 系统框架级定时唤醒
 */
class MqfSyncAdapter(context: Context, autoInitialize: Boolean) :
    AbstractThreadedSyncAdapter(context, autoInitialize) {

    companion object { const val TAG = "mqf_sync" }

    override fun onPerformSync(
        account: android.accounts.Account?,
        extras: Bundle?,
        authority: String?,
        provider: ContentProviderClient?,
        syncResult: SyncResult?
    ) {
        Log.i(TAG, "[chrysalis] sync wakeup")
        val ctx = context.applicationContext
        ctx.startForegroundService(Intent(ctx, MqfCoreService::class.java))
    }
}

class MqfSyncService : Service() {
    private lateinit var adapter: MqfSyncAdapter
    override fun onCreate() { adapter = MqfSyncAdapter(applicationContext, true) }
    override fun onBind(intent: Intent?): IBinder? = adapter.syncAdapterBinder
}
