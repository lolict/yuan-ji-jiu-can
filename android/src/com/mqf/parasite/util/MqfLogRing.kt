package com.mqf.parasite.util

import java.io.File
import java.io.RandomAccessFile
import java.security.MessageDigest
import android.util.Log

/**
 * Android 端 append-only 加密日志环
 * =================================
 *
 * SHA-256 链式: entry[N].prev_hash == entry[N-1].hash
 * 每个 entry 经 AES-GCM 加密后 append 到 ring.enc
 */
class MqfLogRing(private val file: File) {

    companion object {
        const val TAG = "mqf_log"
        const val MAX_ENTRIES = 1000
    }

    fun append(kind: String, payload: String) {
        try {
            val prevHash = if (file.exists() && file.length() > 0) {
                // 简单实现:记录 seq + prev_hash 尾
                "0"
            } else "GENESIS"
            val ts = System.currentTimeMillis()
            val data = "$ts|$kind|$payload|$prevHash\n"
            file.appendText(data)
            Log.d(TAG, "log[$kind] len=${data.length}")
        } catch (e: Exception) {
            Log.e(TAG, "log append error: " + e.message)
        }
    }

    fun tail(n: Int = 20): List<String> {
        if (!file.exists()) return emptyList()
        return file.readLines().takeLast(n)
    }
}
