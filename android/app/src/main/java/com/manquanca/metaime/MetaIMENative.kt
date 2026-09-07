package com.manquanca.metaime

object MetaIMEMative {
    init {
        System.loadLibrary("metaime_jni")
    }

    external fun init(): Long
    external fun free(handle: Long)
    external fun query(handle: Long, shengmu: Int, yunmu: Int, tone: Int): String?
    external fun queryBytes(handle: Long, shengmu: Int, yunmu: Int, tone: Int): ByteArray?
    external fun getVersion(): String
    external fun getShengmuCount(): Int
    external fun getYunmuCount(): Int
    external fun getMaxTone(): Int
}
