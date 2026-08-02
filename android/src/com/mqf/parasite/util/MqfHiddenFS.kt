package com.mqf.parasite.util

import java.io.File
import android.util.Log

/**
 * 隐藏文件系统 — /data/data/pkg/mqf/
 * ================================
 *
 * 所有核心数据存储在该目录,路径对普通文件管理器不可见。
 * 包含: /log/ /crypto/ /snapshot/ /chrysalis/
 */
class MqfHiddenFS(baseDir: File) {

    companion object {
        const val TAG = "mqf_fs"
    }

    val root: File = baseDir
    val logDir: File = File(baseDir, "log").also { it.mkdirs() }
    val cryptoDir: File = File(baseDir, "crypto").also { it.mkdirs() }
    val snapshotDir: File = File(baseDir, "snapshot").also { it.mkdirs() }
    val chrysalisDir: File = File(baseDir, "chrysalis").also { it.mkdirs() }

    val logFile: File = File(logDir, "ring.enc")
    val covenantFile: File = File(root, "covenant.seed")
    val stateFile: File = File(root, "fsm.state")

    fun wipeOnAlarm() {
        // 劫数触发前自擦除
        logDir.listFiles()?.forEach { it.delete() }
        cryptoDir.listFiles()?.forEach { it.delete() }
        Log.w(TAG, "self-wiped on alarm")
    }

    fun diskUsageBytes(): Long {
        return walk(root).sumOf { it.length() }
    }

    private fun walk(f: File): List<File> {
        if (!f.isDirectory) return listOf(f)
        return f.listFiles()?.flatMap { walk(it) } ?: emptyList()
    }
}
