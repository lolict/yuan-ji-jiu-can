/**
 * wasm_kernel.h — WASM 内核
 *
 * 为 WASM 目标提供最小运行时内核:
 *   - 内存平面化管理 (WASM 线性内存)
 *   - 跨架构 64位 cell 编码 (与 hunyuan-rs 对齐)
 *   - 符号主权/极性/见证三元组
 *
 * 这是满全法在 WASM 上的"灵魂内核" —
 * 不依赖 libc · 不依赖 OS · 只依赖 WASM 线性内存
 *
 * 阶梯制:
 *   第0阶: 内存分配 (线性内存 bump allocator)
 *   第1阶: Cell 编码 (64位 = 低32位 MCCP + 高32位 主权/极性/见证)
 *   第2阶: 符号路由 (谐音/感知/主权 三层)
 *   第3阶: 状态机 (囡囝四态)
 */

#ifndef NJSG_WASM_KERNEL_H
#define NJSG_WASM_KERNEL_H

#include "wasm_port.h"

/* ── 内核配置 ────────────────────────────────────────────── */

#define WK_MAGIC          0x574B4E4C  /* "WKNL" */
#define WK_VERSION        0x0003
#define WK_CELL_SIZE      8           /* 64位 cell = 8 bytes */
#define WK_SYMBOL_MAX     254         /* mlll 集装箱容量 */
#define WK_FLOW_MAX       8           /* 八流统一 */
#define WK_STATE_MAX      4           /* 囡囝四态 */

/* ── 64位 Cell 编码 (与 hunyuan-rs/src/mirror.rs 对齐) ────
 *
 *  63          32 31           0
 *  ┌────────────┬─────────────┐
 *  │ 主权/极性  │  MCCP 码点  │
 *  │  witness   │  codepoint  │
 *  └────────────┴─────────────┘
 *
 *  高32位细分:
 *    63:60  witness  (见证 · 三元裁判)
 *    59:56  polarity (极性 · YinYang)
 *    55:32  sovereignty (主权 · 容器标识)
 */

#define WK_CELL_MCCP_MASK     0xFFFFFFFFU
#define WK_CELL_HI_MASK       0xFFFFFFFF00000000ULL

#define WK_WITNESS_SHIFT      60
#define WK_WITNESS_MASK       0xF
#define WK_POLARITY_SHIFT     56
#define WK_POLARITY_MASK      0xF
#define WK_SOVEREIGNITY_SHIFT 32
#define WK_SOVEREIGNITY_MASK  0xFFFFFF

/* Cell 构造/析取 */
#define WK_CELL(mccp, sov, pol, wit) \
    (((NJSG_uint64)((wit) & 0xF) << 60) | \
     ((NJSG_uint64)((pol) & 0xF) << 56) | \
     ((NJSG_uint64)((sov) & 0xFFFFFF) << 32) | \
     ((NJSG_uint32)(mccp)))

#define WK_CELL_MCP(cell)     ((NJSG_uint32)((cell) & 0xFFFFFFFFU))
#define WK_CELL_SOVEREIGN(cell) (((cell) >> 32) & 0xFFFFFF)
#define WK_CELL_POLARITY(cell)  (((cell) >> 56) & 0xF)
#define WK_CELL_WITNESS(cell)   (((cell) >> 60) & 0xF)

/* ── 极性 (YinYang 二进制) ──────────────────────────────── */

#define WK_POLARITY_WIFE    0x0   /* 容器(女) = 刘楚恬 */
#define WK_POLARITY_HUSBAND 0x1   /* 界线(男) = lolict */
#define WK_POLARITY_SYSTEM  0x2   /* 系统中性 */

/* ── 囡囝四态 ────────────────────────────────────────────── */

typedef enum {
    WK_STATE_NN   = 0x00, /* 囡囡 = 妻PID1 容器包容器 */
    WK_STATE_NJSG = 0x01, /* 囡囝 = 妻壳夫核 (平等) */
    WK_STATE_JJ   = 0x02, /* 囝囝 = 夫PID1 界线包界线 */
    WK_STATE_JN   = 0x03, /* 囝囡 = 夫壳妻进入 */
} wk_state_t;

/* ── 八流 ────────────────────────────────────────────────── */

typedef enum {
    WK_FLOW_PRELOAD    = 0x00,
    WK_FLOW_CATEGORY   = 0x01,
    WK_FLOW_CLUSTER    = 0x02,
    WK_FLOW_DISTINGUISH = 0x03,
    WK_FLOW_PROPERTY   = 0x04,
    WK_FLOW_INTERPRET  = 0x05,
    WK_FLOW_RELATION   = 0x06,
    WK_FLOW_CONCEPT    = 0x07,
} wk_flow_t;

/* ── 内核状态 ────────────────────────────────────────────── */

typedef struct {
    NJSG_uint32 magic;           /* WK_MAGIC */
    NJSG_uint16 version;         /* WK_VERSION */
    NJSG_uint8  state;           /* wk_state_t 当前态 */
    NJSG_uint8  active : 1;      /* 内核是否激活 */
    NJSG_uint8  locked : 1;      /* 互锁是否锁定 */
    NJSG_uint8  reserved : 6;
    
    /* 线性内存 (bump allocator) */
    NJSG_uint8  *mem_base;       /* WASM 线性内存基址 */
    NJSG_uint32  mem_size;       /* 当前使用 */
    NJSG_uint32  mem_capacity;   /* 总容量 */
    
    /* 符号表 */
    NJSG_uint64  symbols[WK_SYMBOL_MAX];  /* 64位 cell 数组 */
    NJSG_uint16  symbol_count;
    
    /* 流状态 */
    NJSG_uint8   flow_state[WK_FLOW_MAX]; /* 八流各自状态 */
    
    /* 统计 */
    NJSG_uint32  total_cells;
    NJSG_uint32  total_routes;
    NJSG_uint32  total_judgments;
} wk_kernel_t;

/* ── API ────────────────────────────────────────────────── */

/**
 * 初始化内核
 * @param kernel    内核结构体指针
 * @param mem_base  线性内存基址 (WASM 中为 0)
 * @param mem_cap   线性内存总容量 (WASM 中为 __builtin_wasm_memory_size(0) * 64KB)
 */
NJSG_EXPORT void wk_init(wk_kernel_t *kernel, NJSG_uint8 *mem_base, NJSG_uint32 mem_cap);

/**
 * 注册一个 64位 cell 到符号表
 * @return 0=成功 · -1=表满
 */
NJSG_EXPORT int  wk_add_cell(wk_kernel_t *kernel, NJSG_uint64 cell);

/**
 * 通过 MCCP 码点查找 cell
 * @return cell 值 · 未找到返回 0
 */
NJSG_EXPORT NJSG_uint64 wk_find_by_mccp(const wk_kernel_t *kernel, NJSG_uint32 mccp);

/**
 * 状态切换 (囡囝四态)
 * @return 0=成功 · -1=无效状态
 */
NJSG_EXPORT int  wk_set_state(wk_kernel_t *kernel, wk_state_t new_state);

/**
 * 路由判决 (三层分别器简化版)
 * @param symbol    要路由的 cell
 * @param flow_type 使用哪一流
 * @return 0=ALLOW · 非0=DENY (位编码原因)
 */
NJSG_EXPORT int  wk_route(wk_kernel_t *kernel, NJSG_uint64 symbol, wk_flow_t flow_type);

/**
 * 获取内核统计
 */
NJSG_EXPORT void wk_stats(const wk_kernel_t *kernel, char *buf, NJSG_size buf_size);

/**
 * 持久化: 保存内核状态到文件
 * @param kernel     内核指针
 * @param filename   文件路径
 * @return 0=成功 · -1=失败
 */
NJSG_EXPORT int  wk_save(const wk_kernel_t *kernel, const char *filename);

/**
 * 持久化: 从文件加载内核状态
 * @param kernel     内核指针 (需先 wk_init)
 * @param filename   文件路径
 * @return 0=成功 · -1=失败
 */
NJSG_EXPORT int  wk_load(wk_kernel_t *kernel, const char *filename);

/**
 * 获取内核序列化大小 (用于预分配缓冲区)
 */
NJSG_EXPORT NJSG_uint32 wk_serialized_size(void);

#endif /* NJSG_WASM_KERNEL_H */
