package com.njsg.parasite

import android.inputmethodservice.InputMethodService
import android.view.KeyEvent

/**
 * ============================================================================
 * 囡囝共生 · MetaIME 七层编码驱动 · 第1层:键盘拦截层
 * ============================================================================
 *
 * 当前层号: 1
 * 当前声母: jian (拦)
 * 当前韵母组合: jian + ru = 键入 (拦截入程)
 * 折叠状态: 未折叠 (单层平铺)
 * 下一步操作: 捕获原始按键事件,送往编码解析层
 *
 * 锚定法则:
 *   0 = 刘楚恬 (低位隐藏侧,不可见)
 *   1 = 我 (高位可见侧,不动)
 *
 * 容器: 自身就是空间,非外求之器
 * ============================================================================
 *
 * 职责:
 *   - 捕获物理键盘 + 虚拟键盘原始事件
 *   - 将原始 KeyEvent 转为 64位编码容器
 *   - 不加工,不判定,不裁剪,直送编码解析层
 *
 * 禁止:
 *   - 不在此处做输入判定,判定由规则引擎专司
 *   - 不缓存超过单个容器容量
 *   - 不在此处连接云端或远端
 */

class IMEInterceptor(private val host: InputMethodService) {

    companion object {
        const val TAG = "njsg_interceptor"
        const val LAYER_INDEX = 1  // 第1层标识
    }

    // ── 声母表 (共21个,对照满全法声母序) ──
    // b p m f d t n l g k h j q x zh ch sh r z c s
    private val shengMuSeq = listOf(
        "b", "p", "m", "f",
        "d", "t", "n", "l",
        "g", "k", "h",
        "j", "q", "x",
        "zh", "ch", "sh", "r",
        "z", "c", "s"
    )

    // ── 韵母表 (共35个,含单韵母/复合韵母) ──
    private val yunMuSeq = listOf(
        "a", "o", "e", "i", "u", "v",
        "ai", "ei", "ui", "ao", "ou", "iu",
        "ie", "ve", "er",
        "an", "en", "in", "un", "vn",
        "ang", "eng", "ing", "ong",
        "ia", "iao", "ian", "iang", "iong",
        "ua", "uo", "uai", "uan", "uang"
    )

    /**
     * 键盘事件入口 — 物理键盘 ↑
     * 原始 KeyEvent 被送至此处,尚未被任何规则触碰
     */
    fun onPhysicalKey(keyCode: Int, event: KeyEvent?): UnicodeContainer? {
        if (event == null) return null
        val rawChar = event.unicodeChar.toChar()
        if (rawChar == '\u0000') return null

        // 编码为64位容器: 高32位 = unicodeChar, 低32位 = 声母韵母坐标
        val container = UnicodeContainer(
            raw = rawChar,
            codePoint = event.unicodeChar,
            shengMuIdx = mapCharToShengMu(rawChar),
            yunMuIdx = mapCharToYunMu(rawChar),
            layerTag = LAYER_INDEX
        )
        return container
    }

    /**
     * 虚拟键盘入口 — 屏幕按键 ↓
     * 将虚拟按键字符封印入容器,同样送往编码解析层
     */
    fun onVirtualKey(c: Char): UnicodeContainer {
        return UnicodeContainer(
            raw = c,
            codePoint = c.code,
            shengMuIdx = mapCharToShengMu(c),
            yunMuIdx = mapCharToYunMu(c),
            layerTag = LAYER_INDEX
        )
    }

    // ── 声母映射: 字符 → 声母坐标 (0-20, 255=非字母) ──
    private fun mapCharToShengMu(c: Char): Int {
        val low = c.lowercaseChar()
        return when (low) {
            'b' -> 0; 'p' -> 1; 'm' -> 2; 'f' -> 3
            'd' -> 4; 't' -> 5; 'n' -> 6; 'l' -> 7
            'g' -> 8; 'k' -> 9; 'h' -> 10
            'j' -> 11; 'q' -> 12; 'x' -> 13
            'z' -> 18; 'c' -> 19; 's' -> 20
            else -> 255  // 非声母承载字符
        }
    }

    // ── 韵母映射: 字符 → 韵母坐标 (0-35, 255=非韵母承载) ──
    private fun mapCharToYunMu(c: Char): Int {
        val low = c.lowercaseChar()
        return when (low) {
            'a' -> 0; 'o' -> 1; 'e' -> 2
            'i' -> 3; 'u' -> 4; 'v' -> 5
            else -> 255  // 单字符不承载复合韵母,复合韵母由解析层组合
        }
    }

}

/**
 * 64位 Unicode 容器 — 第1层的产出物
 * =====================================
 * 高32位: unicode 码点 + 原始字符
 * 低32位: 声母坐标(8bit) + 韵母坐标(8bit) + 层标识(8bit) + 折叠位(8bit)
 *
 * 容器自身就是空间,无需外部容器承载
 */
data class UnicodeContainer(
    val raw: Char,
    val codePoint: Int,
    val shengMuIdx: Int,
    val yunMuIdx: Int,
    val layerTag: Int = 1
) {
    /**
     * 封印为64位长整型,以便跨层传递
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
     * 解封为可读状态描述 (用于调试/日志)
     */
    fun describe(): String {
        return "[L$layerTag] '$raw' cp=$codePoint sheng=$shengMuIdx yun=$yunMuIdx"
    }
}
