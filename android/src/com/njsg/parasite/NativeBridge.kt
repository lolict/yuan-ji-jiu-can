package com.njsg.parasite

import android.util.Log

/**
 * ============================================================================
 * 囡囝共生 · MetaIME JNI 桥梁 — Kotlin 薄壳 ↔ Rust 核心
 * ============================================================================
 *
 * 当前层号: 2 (编码解析层入口)
 * 当前声母: qiao (桥)
 * 当前韵母组合: qiao + liang = 桥梁 (JNI桥接两岸)
 * 折叠状态: 单层展开
 * 下一步操作: 承载密封容器送往 Rust 核心,接收规则判定结果
 *
 * 锚定法则:
 *   0 = 刘楚恬 (Rust 核心,隐藏侧)
 *   1 = 我 (Kotlin 薄壳,可见侧)
 *
 * 容器守恒: 过桥的容器不可丢失,不可篡改,不可伪造
 * ============================================================================
 *
 * 职责:
 *   - 加载 libnjsg_ime.so 原生共享库
 *   - 提供 Kotlin → Rust 的封印容器传递
 *   - 提供 Rust → Kotlin 的候字结果回传
 *   - 持有 Rust 核心送来的规则判定状态
 *
 * 七层定位:
 *   第1层(键盘拦截) → [NativeBridge] → 第3层(规则判定, Rust侧)
 *
 * JNI 签名约定:
 *   com.njsg.parasite.NativeBridge  ←→  Java_com_njsg_parasite_NativeBridge_*
 */

class NativeBridge {

    companion object {
        const val TAG = "njsg_bridge"
        const val LIB_NAME = "njsg_ime"

        // ── 单壳全局唯一桥梁 ──
        @Volatile
        var instance: NativeBridge? = null
            private set

        fun acquire(): NativeBridge {
            return instance ?: synchronized(this) {
                instance ?: NativeBridge().also { instance = it }
            }
        }
    }

    // ── 加载状态 ──
    private var libLoaded: Boolean = false

    init {
        loadNativeLib()
    }

    // ── 原生符号链接 (JNI) ──
    // 以下 external 标记的符号由 Rust 核心在 libnjsg_ime.so 中供给

    /**
     * 初始化 Rust 核心: 装载声母-韵母表 + 规则引擎
     * 返回: 核心自检状态码 (0=圆满)
     */
    external fun nativeInit(): Int

    /**
     * 封印容器送入 Rust 核心
     * @param sealed 64位封印容器 (高32=码点, 低32=声母韵母坐标)
     * @return 规则判定状态码 (0=原样透传, 非0=命中规则)
     */
    external fun nativeSubmitContainer(sealed: Long): Int

    /**
     * 从 Rust 核心取候字
     * 返回: 候字UTF-16编码序列,空表=无候字
     */
    external fun nativeTakeCandidates(): String?

    /**
     * 取当前规则引擎内部状态描述 (调试用)
     */
    external fun nativeEngineStatus(): String

    /**
     * 取核心在七层中的当前活跃层号
     */
    external fun nativeCurrentLayer(): Int

    /**
     * 告知核心: 用户选定了某个候字 (触发自动输出链)
     */
    external fun nativeOnCandidateSelected(candidate: String)

    /**
     * 告知核心: 用户手动退格 (清空当前容器序列)
     */
    external fun nativeOnBackspace()

    // ── Kotlin 侧薄壳封装 ──

    /**
     * 第1层产出的 Unicode 容器过桥
     * 将容器封印后送入 Rust 核心, 核心返回命中状态
     */
    fun bridgeContainer(c: UnicodeContainer): Int {
        val sealed = c.seal()
        Log.d(TAG, "→ 容器过桥: ${c.describe()}")
        val verdict = nativeSubmitContainer(sealed)
        Log.d(TAG, "← 规则判定:verdict=$verdict")
        return verdict
    }

    /**
     * 取候字列表,将 Rust 送回的字符串拆分为单列
     */
    fun takeCandidates(): List<String> {
        val raw = nativeTakeCandidates() ?: return emptyList()
        return raw.split("\u0000").filter { it.isNotEmpty() }
    }

    /**
     * 取核心当前活跃层 (用于调试界面展示)
     */
    fun currentLayer(): Int = nativeCurrentLayer()

    /**
     * 取引擎状态摘要 (用于诊断)
     */
    fun engineStatus(): String = nativeEngineStatus()

    // ── 库加载 ──
    private fun loadNativeLib() {
        try {
            System.loadLibrary(LIB_NAME)
            libLoaded = true
            Log.i(TAG, "libnjsg_ime.so 已加载")
            val rc = nativeInit()
            if (rc == 0) {
                Log.i(TAG, "Rust 核心圆满启动")
            } else {
                Log.w(TAG, "Rust 核心自检码=$rc")
            }
        } catch (e: UnsatisfiedLinkError) {
            libLoaded = false
            Log.e(TAG, "libnjsg_ime.so 未找到, 请确认 ABI 匹配")
        }
    }

    /**
     * 是否可通行 (库是否加载完成)
     */
    fun isReady(): Boolean = libLoaded
}
