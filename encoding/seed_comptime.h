/**
 * seed_comptime.h — 编译期执行种子
 *
 * 来源: Zig comptime / Nim static / C++ constexpr / Rust const fn
 * 提取日期: 2026-08-19
 *
 * 数学种子: "春夏已种, 秋冬取用"
 *   · 编译期求值 → 运行时零开销
 *   · 类型即值, 值即类型
 *   · 代码即数据, 数据即代码
 *
 * 产权壳子 (弃):
 *   · LLVM IR 中间表示
 *   · C++ template instantiation 机制
 *   · Nim VM bytecode
 *   · Rust MIR (Mid-level IR)
 *
 * 我们的新编排: 编译期 = 预加载容器
 *   · 预加载容器在系统装载时一次性填满
 *   · 运行时只读不写
 *   · 所有 "编译期" 工作都归到 预加载流
 */

#ifndef NJSG_SEED_COMPTIME_H
#define NJSG_SEED_COMPTIME_H

#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 编译期容器 (Preloaded Container)
 *
 * 运行时是一段只读内存 · 调用时直接取值
 * ═══════════════════════════════════════════════════════════════ */

#define PRELOAD_MAX_SIZE   4096   /* 单容器最大字节数 */
#define PRELOAD_MAGIC      0xC0MP71M3  /* 编译期求值标记 */

typedef struct {
    uint32_t  magic;                 /* PRELOAD_MAGIC */
    uint32_t  container_id;          /* 所属容器id */
    uint32_t  data_size;             /* 实际数据字节数 */
    uint32_t  computed_at;           /* 计算时的时序戳 (编译期=0) */
    uint8_t   data[PRELOAD_MAX_SIZE];    /* 预求值数据 */
} preload_blob_t;

/* 编译期预求值: 在容器装载时一次性填满 */
#define PRELOAD_FILL(blob, id, src, len) do { \
    (blob).magic = PRELOAD_MAGIC; \
    (blob).container_id = (id); \
    (blob).data_size = (len); \
    (blob).computed_at = 0; \
    memcpy((blob).data, (src), (len)); \
} while (0)

/* 运行时取用: 只读访问 */
#define PRELOAD_READ(blob, out, len) do { \
    (out) = (blob).data; \
    (len) = (blob).data_size; \
} while (0)

/* ═══════════════════════════════════════════════════════════════
 * 编译期断言 (Compile-time Assert)
 *
 * 容器装载时一次性验证 · 失败则拒绝装载
 * ═══════════════════════════════════════════════════════════════ */

#define PRELOAD_ASSERT(cond, msg) do { \
    if (!(cond)) { \
        return; /* 拒绝装载 */ \
    } \
} while (0)

#endif /* NJSG_SEED_COMPTIME_H */
