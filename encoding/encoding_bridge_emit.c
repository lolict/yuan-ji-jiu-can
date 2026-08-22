/*
 * encoding_bridge_emit.c — 强制导出编码桥接函数符号
 *
 * 许多编码桥接函数在 encoding_bridge.h 中定义为 static inline。
 * 静态内联函数在编译时通常不会生成外部符号。
 * 此文件显式调用每个函数，强制编译器生成全局定义，
 * 使 Rust 端 FFI 可以通过 dlsym/链接找到这些符号。
 *
 * 链接器: 将此文件加入编译单元即可导出全部编码桥接 API。
 */

#include "encoding_bridge.h"

/* 强制导出每个 inline 函数 */
void __attribute__((noinline)) encoding_init_emit(void) {
    encoding_init();
}

uint32_t __attribute__((noinline)) encoding_utf8_to_container_emit(const char* utf8) {
    return encoding_utf8_to_container(utf8);
}

uint32_t __attribute__((noinline)) encoding_ucs2_to_container_emit(uint16_t ucs2, uint8_t tone) {
    return encoding_ucs2_to_container(ucs2, tone);
}

perception_family_t __attribute__((noinline)) encoding_classify_family_emit(uint16_t ucs2) {
    return encoding_classify_family(ucs2);
}

uint32_t __attribute__((noinline)) encoding_compute_flow_bits_emit(uint16_t ucs2, perception_family_t family) {
    return encoding_compute_flow_bits(ucs2, family);
}

uint8_t __attribute__((noinline)) encoding_get_polarity_emit(uint16_t ucs2) {
    return encoding_get_polarity(ucs2);
}

int __attribute__((noinline)) encoding_has_hanzi_emit(uint16_t ucs2) {
    return encoding_has_hanzi(ucs2);
}

int __attribute__((noinline)) encoding_hanzi_count_emit(void) {
    return encoding_hanzi_count();
}
