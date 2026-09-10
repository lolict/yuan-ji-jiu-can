package com.mqf.metaime.jni

/**
 * MetaIME JNI 接口类
 * 加载 libmetaime_core.so (Rust 七层编码引擎)
 *
 * 设计约束:
 *   - 不依赖云端大模型
 *   - 所有智能来自编译时常量 (嵌入 .rodata)
 *   - 通信路径: Kotlin → JNI → Rust → JNI → Kotlin
 */
class MetaIMEJNI {

    companion object {
        init {
            System.loadLibrary("metaime_core")
        }
    }

    /**
     * 初始化 Rust 引擎
     * 加载 54 容器 · 16 符号 · 128 链接 · 64 近义词
     */
    external fun nativeInit(): Long

    /**
     * 推入一个按键
     * @param keyCode Android KeyEvent.keyCode
     * @return JSON 格式: {"intent":"INPUT","payload":"巴","candidates":["巴","爸"]}
     */
    external fun nativePushKey(keyCode: Int): String

    /**
     * 重置七层状态机
     */
    external fun nativeReset(): Boolean

    /**
     * 切换输入模式 (COMMAND 意图下)
     */
    external fun nativeSwitchMode(mode: String): Boolean

    /**
     * 销毁引擎句柄
     */
    external fun nativeDestroy(): Unit

    /**
     * 获取容器 (调试用)
     */
    external fun nativeGetStats(): String
}
