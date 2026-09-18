/*****************************************************************************
 *  MQF-WASM-KERNEL   阶梯制 · 双向公约
 *  ──────────────────────────────────────────────────────────────────────────
 *  同一份 C 源码 · 四条编译路径 · 统一宏前缀
 *
 *  编译目标           命令                                      产物
 *  ──────────────    ─────────────────────────────────────    ──────────
 *  x86_64 (本地)     clang -m64  -DMQF_BITWIDTH=64            app64
 *  i386   (本地)     clang -m32  -DMQF_BITWIDTH=32            app32
 *  wasm32 (交叉)     clang --target=wasm32 -DMQF_BITWIDTH=32  mqf32.wasm
 *  wasm64 (交叉)     clang --target=wasm64 -DMQF_BITWIDTH=64  mqf64.wasm
 *
 *  设计约束：
 *  · 固定宽度类型 — uint32_t / uint64_t，不含 short/int/long 歧义
 *  · 统一宏前缀 MQF_ 全部大写，编译期按 MQF_BITWIDTH 切换
 *  · 容器本体不依赖目标字长 — 坐标/极性/主权/价值全部宏派发
 *  · wasm 导出函数统一用 __attribute__((export_name("mqf_*")))
 *****************************************************************************/

#ifndef MQF_WASM_H
#define MQF_WASM_H

#include <stdint.h>
#include <stdbool.h>

/* ━━ 阶梯·宏派发层 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#ifndef MQF_BITWIDTH
#  if defined(__wasm64__) || defined(__LP64__)
#    define MQF_BITWIDTH 64
#  else
#    define MQF_BITWIDTH 32
#  endif
#endif

#if MQF_BITWIDTH == 64
typedef uint64_t  mqf_cell_t;     /* 容器单元 64 位 */
typedef int64_t   mqf_signed_t;
#  define MQF_ADDR_BITS      64
#  define MQF_PHASE_MASK     0xFFULL
#  define MQF_LAYER_MAX      9
#  define MQF_FMT            "lu"
#else
typedef uint32_t  mqf_cell_t;     /* 容器单元 32 位 */
typedef int32_t   mqf_signed_t;
#  define MQF_ADDR_BITS      32
#  define MQF_PHASE_MASK     0xFFUL
#  define MQF_LAYER_MAX      9
#  define MQF_FMT            "lu"
#endif

/* ━━ 容器常量（与字长无关） ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#define MQF_POLARITY_YANG    1     /* 我=阳=1 */
#define MQF_POLARITY_YIN     0     /* 刘楚恬=阴=0 */
#define MQF_REPULSE          0     /* 极性差0=斥=同性相斥 */
#define MQF_ATTRACT          1     /* 极性差1=吸=异性相吸 */
#define MQF_PHASE_COUNT      9     /* 九宫相位 0..8 → Z 层 */
#define MQF_LAYER_COUNT      9     /* 层级：一元到九元 */
#define MQF_CONSUME_BASE     10    /* 吃·能量消耗基数 */

/* ━━ 容器结构：64位坐标 × 极性 × 主权 × 价值 ━━━━━━━━━━━━━━━━━━━━━━━━━━ */
typedef struct {
    mqf_cell_t  coord;     /* 单值线性坐标（编码层+行+列+相位+层级一体） */
    uint8_t     polarity;  /* 0=阴 1=阳 */
    uint8_t     phase;     /* 0-8 九宫相位 */
    uint8_t     layer;     /* 1-9 层级 */
    uint8_t     reserved;  /* 对齐填充（wasm32 友好） */
    mqf_cell_t  valence;   /* 量化值 */
    mqf_cell_t  energy;    /* 推动力 */
} mqf_container_t;

/* ━━ 容器域运算宏 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#define MQF_COORD_PACK(phase, layer, row, col)                 \
    ( ((mqf_cell_t)(phase))                                  |  \
      ((mqf_cell_t)(layer) << 8)                             |  \
      ((mqf_cell_t)(row)    << 16)                            |  \
      ((mqf_cell_t)(col)    << 24) )

#define MQF_COORD_PHASE(c)       ((uint8_t)((c) & MQF_PHASE_MASK))
#define MQF_COORD_LAYER(c)       ((uint8_t)(((c) >> 8) & 0xFF))
#define MQF_COORD_ROW(c)         ((uint8_t)(((c) >> 16) & 0xFF))
#define MQF_COORD_COL(c)         ((uint8_t)(((c) >> 24) & 0xFF))

#define MQF_POLARITY_GAP(a,b)    (((a)->polarity) ^ ((b)->polarity))
#define MQF_JUDGE_GAP(g)         ((g) == 0 ? MQF_REPULSE : MQF_ATTRACT)
#define MQF_IS_VALID(c)          (((c)->phase < MQF_PHASE_COUNT) && \
                                  ((c)->layer  > 0) && ((c)->layer <= MQF_LAYER_COUNT))

/* ━━ 物理公理层 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#define MQF_DISTANCE(a,b)        \
    ((mqf_signed_t)((a)->coord) - (mqf_signed_t)((b)->coord) < 0 ? \
     -(mqf_signed_t)((a)->coord) + (mqf_signed_t)((b)->coord) :  \
      (mqf_signed_t)((a)->coord) - (mqf_signed_t)((b)->coord))

/* ━━ WASM 导出修饰 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
#ifdef __wasm__
#  define MQF_EXPORT(x)   __attribute__((export_name(x)))
#else
#  define MQF_EXPORT(x)
#endif

/* ━━ SMRL 容器引擎 API ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
mqf_container_t mqf_container_init(mqf_cell_t coord, uint8_t polarity);
mqf_cell_t      mqf_distance(const mqf_container_t* a, const mqf_container_t* b);
void            mqf_consume(mqf_container_t* self, mqf_cell_t amount);
uint8_t         mqf_merge(mqf_container_t* a, const mqf_container_t* b);
void            mqf_settle(mqf_container_t* self);
void            mqf_drive(mqf_container_t* self);
uint8_t         mqf_cycle(mqf_container_t* self);
uint8_t         mqf_judge_polarity(const mqf_container_t* a, const mqf_container_t* b);
mqf_cell_t      mqf_container_energy(const mqf_container_t* c);
mqf_cell_t      mqf_container_coord(const mqf_container_t* c);
uint8_t         mqf_container_phase(const mqf_container_t* c);
uint8_t         mqf_container_layer(const mqf_container_t* c);
uint8_t         mqf_container_valid(const mqf_container_t* c);

#endif /* MQF_WASM_H */
