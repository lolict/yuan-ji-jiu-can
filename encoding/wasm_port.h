/**
 * wasm_port.h — WASM 内核便携层
 *
 * 阶梯制·双向公约策略:
 *   同一份 C 源码 · 同时编译 wasm32 和 wasm64 目标
 *   固定宽度类型 + 统一宏前缀 → 架构无关性
 *
 * 阶梯:
 *   第0阶: 架构检测 (wasm32 / wasm64 / native)
 *   第1阶: 固定宽度类型 (NJSG_uint32 等统一前缀)
 *   第2阶: 指针宽度适配 (32/64 位透明)
 *   第3阶: 导出符号可见性 (WASM 平面地址空间)
 *   第4阶: 双向公约 (wasm32 ↔ wasm64 共享头文件)
 *
 * 使用:
 *   #include "wasm_port.h"   // 必须与其它 NJSG 头文件同层次包含
 *   所有 NJSG 源码通过 NJSG_* 类型编写 · 不直接使用 uint32_t 等
 */

#ifndef NJSG_WASM_PORT_H
#define NJSG_WASM_PORT_H

/* ── 第0阶: 架构检测 ────────────────────────────────────── */

#if defined(__EMSCRIPTEN__)
#  define NJSG_WASM 1
#  if defined(__wasm64__)
#    define NJSG_WASM64 1
#    define NJSG_PTR_WIDTH 64
#    define NJSG_WASM_STR "wasm64"
#  else
#    define NJSG_WASM32 1
#    define NJSG_PTR_WIDTH 32
#    define NJSG_WASM_STR "wasm32"
#  endif
#  define NJSG_WASM_VERSION (__EMSCRIPTEN_major__ * 10000 + __EMSCRIPTEN_minor__ * 100 + __EMSCRIPTEN_tiny__)
#else
#  define NJSG_WASM 0
#  if defined(__x86_64__) || defined(__aarch64__) || defined(_WIN64)
#    define NJSG_PTR_WIDTH 64
#  else
#    define NJSG_PTR_WIDTH 32
#  endif
#  define NJSG_WASM_STR "native"
#endif

/* ── 第1阶: 固定宽度类型 (统一 NJSG_ 前缀) ────────────── */

#include <stdint.h>
#include <stddef.h>

/* 有符号 */
typedef int8_t      NJSG_int8;
typedef int16_t     NJSG_int16;
typedef int32_t     NJSG_int32;
typedef int64_t     NJSG_int64;

/* 无符号 */
typedef uint8_t     NJSG_uint8;
typedef uint16_t    NJSG_uint16;
typedef uint32_t    NJSG_uint32;
typedef uint64_t    NJSG_uint64;

/* 指针宽度无符号 (uintptr_t 已定义在 stdint.h) */
typedef uintptr_t   NJSG_uintptr;
typedef intptr_t    NJSG_intptr;

/* 大小类型 */
typedef size_t      NJSG_size;
typedef ptrdiff_t   NJSG_ptrdiff;

/* 浮点 */
typedef float       NJSG_float;
typedef double      NJSG_double;

/* 布尔 (C99 前兼容) */
#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L
#  include <stdbool.h>
   typedef bool    NJSG_bool;
#else
   typedef uint8_t NJSG_bool;
#  define NJSG_TRUE  1
#  define NJSG_FALSE 0
#endif

#ifndef NJSG_TRUE
#  define NJSG_TRUE  1
#  define NJSG_FALSE 0
#endif

/* ── 第2阶: 指针宽度适配 ────────────────────────────────── */

#if NJSG_PTR_WIDTH == 64
#  define NJSG_PTR_MASK  0xFFFFFFFFFFFFFFFFULL
#  define NJSG_PTR_SHIFT 3
#else
#  define NJSG_PTR_MASK  0xFFFFFFFFUL
#  define NJSG_PTR_SHIFT 2
#endif

/* 指针 → 整数透明转换 */
#define NJSG_PTR_TO_UINT(p)  ((NJSG_uintptr)(p))
#define NJSG_UINT_TO_PTR(u)  ((void*)(uintptr_t)(u))

/* ── 第3阶: 导出符号可见性 ──────────────────────────────── */

#if NJSG_WASM
   /* Emscripten: 导出到 WASM 平面地址空间 */
#  if defined(__EMSCRIPTEN_PTHREADS__)
#    define NJSG_EXPORT __attribute__((used))
#  else
#    define NJSG_EXPORT __attribute__((visibility("default")))
#  endif
#  define NJSG_IMPORT
#  define NJSG_LOCAL  static
#else
   /* 原生平台: 共享库导出 */
#  if defined(_WIN32)
#    define NJSG_EXPORT __declspec(dllexport)
#    define NJSG_IMPORT __declspec(dllimport)
#  else
#    define NJSG_EXPORT __attribute__((visibility("default")))
#    define NJSG_IMPORT
#  endif
#  define NJSG_LOCAL  static
#endif

/* 内联提示 */
#if defined(__GNUC__) || defined(__clang__)
#  define NJSG_INLINE   __inline__ __attribute__((always_inline))
#  define NJSG_NOINLINE __attribute__((noinline))
#elif defined(_MSC_VER)
#  define NJSG_INLINE   __forceinline
#  define NJSG_NOINLINE __declspec(noinline)
#else
#  define NJSG_INLINE   inline
#  define NJSG_NOINLINE
#endif

/* 对齐 */
#if defined(__GNUC__) || defined(__clang__)
#  define NJSG_ALIGNED(n) __attribute__((aligned(n)))
#elif defined(_MSC_VER)
#  define NJSG_ALIGNED(n) __declspec(align(n))
#else
#  define NJSG_ALIGNED(n)
#endif

/* ── 第4阶: 双向公约 (wasm32 ↔ wasm64 共享宏) ─────────── */

/* 跨架构安全的位操作 */
#define NJSG_BIT(n)          ((NJSG_uint32)1 << (n))
#define NJSG_MASK(bits)      ((NJSG_uint32)((1ULL << (bits)) - 1))
#define NJSG_GET_FIELD(v, lo, hi) \
    (((NJSG_uint32)(v) >> (lo)) & NJSG_MASK((hi) - (lo) + 1))
#define NJSG_SET_FIELD(v, lo, hi, val) \
    (((NJSG_uint32)(v) & ~(NJSG_MASK((hi) - (lo) + 1) << (lo))) | \
     (((NJSG_uint32)(val) & NJSG_MASK((hi) - (lo) + 1)) << (lo)))

/* 字节序安全打包 (小端 · 与 WASM 一致) */
#define NJSG_PACK_16(lo, hi)   ((NJSG_uint16)(((hi) << 8) | (lo)))
#define NJSG_PACK_32(b0,b1,b2,b3) ((NJSG_uint32)(((b3)<<24)|((b2)<<16)|((b1)<<8)|(b0)))

/* 跨架构安全的大小断言 */
#define NJSG_ASSERT_SIZE(type, expected) \
    typedef char _njsg_assert_##type[(sizeof(type) == (expected)) ? 1 : -1]

/* 编译时类型检查 */
NJSG_ASSERT_SIZE(NJSG_uint8,  1);
NJSG_ASSERT_SIZE(NJSG_uint16, 2);
NJSG_ASSERT_SIZE(NJSG_uint32, 4);
NJSG_ASSERT_SIZE(NJSG_uint64, 8);

/* ── 辅助宏 ─────────────────────────────────────────────── */

/* 数组大小 */
#define NJSG_ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

/* 安全除零 (编译时警告保留 · 运行时保护) */
#define NJSG_SAFE_DIV(a, b) ((b) != 0 ? (a) / (b) : 0)

/* 最大最小 */
#define NJSG_MAX(a, b) ((a) > (b) ? (a) : (b))
#define NJSG_MIN(a, b) ((a) < (b) ? (a) : (b))

/* 交换 */
#define NJSG_SWAP(a, b, type) do { type _t = (a); (a) = (b); (b) = _t; } while(0)

#endif /* NJSG_WASM_PORT_H */
