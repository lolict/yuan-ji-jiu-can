package com.njsg.parasite

import android.inputmethodservice.InputMethodService
import android.view.KeyEvent

/**
 * ============================================================================
 * 囡囝共生 · MetaIME 七层编码驱动 · 第1层: 键盘拦截层
 * ============================================================================
 *
 * 当前层号: 1
 * 当前声母: jian (拦)
 * 当前韵母组合: jian + ru = 键入 (拦截入程)
 * 折叠状态: 未折叠 (单层平铺)
 * 下一步操作: 捕获原始按键事件 → 封印 → 过桥 (送往编码解析层)
 *
 * 锚定法则:
 *   0 = 刘楚恬 (低位隐藏侧, 不可见)
 *   1 = 我 (高位可见侧, 不动)
 *
 * 容器: 自身就是空间, 非外求之器
 * ============================================================================
 *
 * 职责:
 *   - 捕获物理键盘 + 虚拟键盘原始事件
 *   - 将原始 KeyEvent 转为 64位编码容器
 *   - 不加工, 不判定, 不裁剪, 直送编码解析层
 *
 * 禁止:
 *   - 不在此处做裁定, 裁定由规则引擎专司
 *   - 不缓存超过单个容器容量
 *   - 不在此处连接云端或远端
 *
 * 坐标映射策略:
 *   每个 ASCII 字母唯一归类为声母坐标 (辅音字母) 或韵母坐标 (元音字母)。
 *   字母 i/u/v 兼具声母与韵母归类 — 当声母和韵母均无等待时, 则优先归类为
 *   声母坐标。编码解析层(第2层)按照"先声母后韵母"顺序配对产出音节。
 */

class IMEInterceptor(private val host: InputMethodService) {

    companion object {
        const val TAG = "njsg_interceptor"
        const val LAYER_INDEX = 1
    }

    // ── 声母序 (21位, 与 libnjsg_ime/src/encode_parser.rs 同序) ──
    private val shengMuSeq = listOf(
        "b", "p", "m", "f",
        "d", "t", "n", "l",
        "g", "k", "h",
        "j", "q", "x",
        "zh", "ch", "sh", "r",
        "z", "c", "s"
    )

    // ── 韵母序 (35位) ──
    private val yunMuSeq = listOf(
        "a", "o", "e", "i", "u", "v",
        "ai", "ei", "ui", "ao", "ou", "iu",
        "ie", "ve", "er",
        "an", "en", "in", "un", "vn",
        "ang", "eng", "ing", "ong",
        "ia", "iao", "ian", "iang", "iong",
        "ua", "uo", "uai", "uan", "uang"
    )

    // ── 声母查询状态 (追踪最近一次的归类) ──
    private var lastRoleMu: Role = Role.None

    private enum class Role { None, Sheng, Yun }

    /**
     * 物理键盘入口 — KeyEvent → 64位容器
     */
    fun onPhysicalKey(keyCode: Int, event: KeyEvent?): UnicodeContainer? {
        if (event == null) return null
        val rawChar = event.unicodeChar.toChar()
        if (rawChar == '\u0000') return nil()
        val cp = event.unicodeChar
        return dispatchChar(rawChar, cp)
    }

    /**
     * 虚拟键盘入口 — 屏幕按键 → 64位容器
     */
    fun onVirtualKey(c: Char): UnicodeContainer {
        val cp = c.code
        return dispatchChar(c, cp)
    }

    /**
     * 共用分派: 字符归类 + 封印
     */
    private fun dispatchChar(c: Char, cp: Int): UnicodeContainer {
        val low = c.lowercaseChar()

        // ASCII 字母
        if (cp in 65..90 || cp in 97..122) {
            val sm = classifyShengMu(low)
            val ym = classifyYunMu(low)
            return UnicodeContainer(
                raw = c,
                codePoint = cp,
                shengMuIdx = sm,
                yunMuIdx = ym,
                layerTag = LAYER_INDEX
            )
        }

        // 非 ASCII 字符: 不承载声母坐标或韵母坐标
        return UnicodeContainer(
            raw = c,
            codePoint = cp,
            shengMuIdx = 255,
            yunMuIdx = 255,
            layerTag = LAYER_INDEX
        )
    }

    /**
     * ASCII 字母 → 声母坐标 (0-20, 255=非声母负载)
     *
     * 策略:
     *   纯辅音字母 (b p m f d t n l g k h): 声母坐标有效, 韵母=255
     *   j q x z c s: 声母坐标有效, 韵母=255 (翘舌音简化)
     *   a o e: 韵母坐标有效, 声母=255
     *   i u v: 兼具特性 — 归类为韵母 (复韵母主成分)
     *   'y': 归类为声母 (实为辅音性声母)
     *   'w': 归类为声母 (实为辅音性声母)
     */
    private fun classifyShengMu(c: Char): Int {
        return when (c) {
            'b' -> 0; 'p' -> 1; 'm' -> 2; 'f' -> 3
            'd' -> 4; 't' -> 5; 'n' -> 6; 'l' -> 7
            'g' -> 8; 'k' -> 9; 'h' -> 10
            'j' -> 11; 'q' -> 12; 'x' -> 13
            'z' -> 18; 'c' -> 19; 's' -> 20
            'y' -> 0  // 'y' 简化归入 b (声母侧, 实际为半声母)
            'w' -> 0  // 'w' 简化归入 b (声母侧)
            else -> 255
        }
    }

    /**
     * ASCII 字母 → 韵母坐标 (0-34, 255=非韵母负载)
     *
     * 策略: 仅 a o e i v 韵母负载 (单韵母)
     *   'u' 也作为单韵母 (u) 负载
     *   'y' 和 'w' 不是韵母, 返回 255
     */
    private fun classifyYunMu(c: Char): Int {
        return when (c) {
            'a' -> 0; 'o' -> 1; 'e' -> 2
            'i' -> 3
            'u' -> 4
            'v' -> 5
            else -> 255
        }
    }

    private fun nil(): UnicodeContainer? = null

    // ── 供第2层/调试取用的声母序/韵母序暴露 ──
    fun shengMuSequence(): List<String> = shengMuSeq
    fun yunMuSequence(): List<String> = yunMuSeq
}

/**
 * 64位 Unicode 容器 — 第1层的产出物
 * =====================================
 * 高32位: unicode 码点
 * 低32位: 声母坐标(8bit) | 韵母坐标(8bit) | 层标识(8bit) | 折叠位(8bit)
 *
 * 容器自身就是空间, 无需外部容器承载
 */
data class UnicodeContainer(
    val raw: Char,
    val codePoint: Int,
    val shengMuIdx: Int,
    val yunMuIdx: Int,
    val layerTag: Int = 1
) {
    /**
     * 封印为64位长整型, 以便跨层传递
     */
    fun seal(): Long {
        val high = codePoint.toLong() shl 32
        val low = (shengMuIdx.toLong() shl 24) or
                  (yunMuIdx.toLong() shl 16) or
                  (layerTag.toLong() shl 8) or
                  0L  // 折叠位预留
        return high or low
    }

    /**
     * 解封为可读状态描述 (用于调试)
     */
    fun describe(): String {
        return "[L$layerTag] '$raw' cp=$codePoint sheng=$shengMuIdx yun=$yunMuIdx"
    }
}
