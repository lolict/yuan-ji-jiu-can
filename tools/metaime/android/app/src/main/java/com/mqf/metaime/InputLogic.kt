package com.mqf.metaime

/**
 * MetaIME 七层编码状态机
 *
 * 每按键驱动层间转移:
 *   L1 声母 → L2 韵母 → L3 声调 → L4 部首 → L5 语义 → L6 意图 → L7 输出
 *
 * 意图类别:
 *   INPUT       — 上屏字符
 *   COMMAND     — 执行 mlll/njsg/mltt 根命令
 *   TRANSFORM   — 转换输入模式
 *   CLEAR       — 清空缓冲
 */
class InputLogic(private val jni: com.mqf.metaime.jni.MetaIMEJNI) {

    enum class Layer {
        L1_SYMMABLE, L2_TONE, L3_RADICAL, L4_SEMANTIC, L5_INTENT, L6_OUTPUT
    }

    enum class Intent {
        INPUT, COMMAND, TRANSFORM, CLEAR
    }

    enum class Mode {
        PINYIN,   // 拼音模式
        COMMAND,  // 命令模式 (mlll/njsg/mltt)
        NAME      // 命名模式
    }

    var currentLayer = Layer.L1_SYMMABLE
        private set
    var currentMode = Mode.PINYIN
        private set

    private val buffer = StringBuilder()

    fun pushKey(keyCode: Int): KeyResult {
        // 委托给 native (Rust) 七层引擎
        val raw = jni.nativePushKey(keyCode)
        return parseResult(raw)
    }

    private fun parseResult(raw: String): KeyResult {
        // 简化 JSON 解析 (真实实现可用 kotlinx.serialization)
        val intent = extractField(raw, "intent") ?: "INPUT"
        val payload = extractField(raw, "payload") ?: ""
        val candidates = extractArray(raw, "candidates")
        return KeyResult(intent, payload, candidates)
    }

    private fun extractField(json: String, key: String): String? {
        val search = "\"$key\":\""
        val i = json.indexOf(search)
        if (i < 0) return null
        val start = i + search.length
        val end = json.indexOf("\"", start)
        if (end < 0) return null
        return json.substring(start, end)
    }

    private fun extractArray(json: String, key: String): List<String> {
        val search = "\"$key\":["
        val i = json.indexOf(search)
        if (i < 0) return emptyList()
        val start = i + search.length
        val end = json.indexOf("]", start)
        if (end < 0) return emptyList()
        val arr = json.substring(start, end)
        return arr.split(",").map { it.trim('"', ' ') }.filter { it.isNotBlank() }
    }

    fun reset() {
        currentLayer = Layer.L1_SYMMABLE
        buffer.clear()
        jni.nativeReset()
    }

    fun switchMode(mode: String) {
        currentMode = when (mode) {
            "root" -> Mode.COMMAND
            "mutual" -> Mode.PINYIN
            "name" -> Mode.NAME
            else -> Mode.PINYIN
        }
        jni.nativeSwitchMode(mode)
    }
}
