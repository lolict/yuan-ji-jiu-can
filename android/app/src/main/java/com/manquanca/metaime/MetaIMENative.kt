package com.manquanca.metaime

object MetaIMEMative {
    init {
        System.loadLibrary("metaime_jni")
    }

    // ── 基础接口 ──
    external fun init(): Long
    external fun free(handle: Long)
    external fun query(handle: Long, shengmu: Int, yunmu: Int, tone: Int): String?
    external fun getVersion(): String
    external fun getShengmuCount(): Int
    external fun getYunmuCount(): Int
    external fun getMaxTone(): Int

    // ── 流式输入接口 ──
    external fun flowInit(handle: Long)
    external fun flowInput(handle: Long, ch: Byte): String
    external fun flowBackspace(handle: Long): String
    external fun flowReset(handle: Long)
}
