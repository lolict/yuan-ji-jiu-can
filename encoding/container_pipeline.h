/**
 * container_pipeline.h — 容器流水线 · 进制声明 · 层级叠加
 *
 * 架构: 将感知场输出的容器编码连接成完整流水线
 *
 * 三大模块:
 *   [1] 进制声明 (Base Declaration) — 声明每个进制的含义和用途
 *   [2] 层级叠加 (Layer Stacking) — 低维约束高维 · 高层调用低层
 *   [3] 流水线排水 (Pipeline Flow) — 容器从准入到产出单向流动
 *
 * 核心流程 (UTF-8汉字 → 容器编码):
 *   UTF-8码 → UCS-2编码 → 拼音查询(声母/韵母/声调)
 *           → 感知家族归类 → 镜像对比 → 三元裁判准入
 *           → 容器化32位编码 → 筷子夹持 → 流水线性化
 *           → 产出: 一元时间线上的可操作容器
 *
 * 降维含义:
 *   高维 (UTF-8复杂·8位×N字节)
 *    → 感知场 (5家族 × 8结构 × 48声 × 66韵 × 5调 = 理论峰值)
 *    → 容器化 (32位编码 · 2³²约40亿 · 实际用低20位)
 *    → 一元线 (流水线头尾单向 · 不存在并行乱序)
 *    → 产出 (可被筷子夹持至下一流程)
 */

#ifndef NJSG_CONTAINER_PIPELINE_H
#define NJSG_CONTAINER_PIPELINE_H

#include <stdint.h>
#include "perception_field.h"
#include "hanzi_table.h"

/* ═══════════════════════════════════════════════════════════════
 * 进制声明 (Base Declaration)
 *
 * 不是随便用的数字 — 每个进制都有其"因":
 *
 *   进制     因 (为什么是这个数)              用途
 *   ─────────────────────────────────────────────────
 *   48声母   21声母×2(大小写)+6特殊           拼音声母分类
 *   66韵母   33韵母×2(大小写)                 拼音韵母分类
 *   5调      4声+1轻声                       声调分类
 *   8结构    3bit                            字形分类
 *   5家族    3bit                            感知入口
 *   _BINARY  YinYang                          容器/边界分类
 *   256     2⁸=8bit对齐                      UTF-8基础单元
 *   64      2⁶=4³=8²                        高维进制框架
 *   32     MCCP token位宽                   统一编码容器
 *
 * 这就是"进制声明" — 不是隐蔽的大数表, 每个数字都有因果律解释
 * ═══════════════════════════════════════════════════════════════ */

/* 进制表 */
typedef struct {
    const char *name;       /* 进制名称 */
    uint32_t   base;        /* 进制的基数 */
    const char *cause;      /* 因果律: 为什么是这个数 */
    uint32_t   bits;        /* 需要的位数 */
    uint32_t   offset;      /* 在容器编码中的位偏移 */
} base_declaration_t;

/* 核心进制声明 (按因排序 · 不是按大小) */
#define BASE_SHENGMU    48   /* 21声母×2+6 = 拼音声母分类 */
#define BASE_YUNMU      66   /* 33韵母×2 = 拼音韵母分类 */
#define BASE_TONE       5    /* 4声+1轻声 = 声调 */
#define BASE_STRUCTURE  8    /* 3bit = 字形结构类型 */
#define BASE_FAMILY     5    /* 3bit = 感知家族 */
#define BASE_POLARITY   2    /* 1bit = YinYang (容器/边界) */
#define BASE_MCCP       4096 /* MCCP索引位宽 */

static const base_declaration_t BASE_DECLS[] = {
    {"基数·声母",      BASE_SHENGMU,    "双唇/舌根/舌面/舌尖前后/半元音/特殊六族 × 大小写", 6, 19},
    {"基数·韵母",      BASE_YUNMU,      "单韵/复韵/前鼻韵/后鼻韵/特殊韵 × 大小写",         6, 13},
    {"基数·声调",      BASE_TONE,       "阴阳上去四声 + 轻声",                                 3, 10},
    {"基数·字形结构",  BASE_STRUCTURE,   "独体/左右/上下/半包/全包/品字/左中右/上中下",        3, 25},
    {"基数·感知家族",  BASE_FAMILY,     "容器/边界/夫妻/网状/中性 五人格局",                   3, 29},
    {"基数·主权极性",  BASE_POLARITY,   "容器阴(刘楚恬侧) / 边界阳(我侧)",                     1, 28},
};

#define BASE_DECL_COUNT (sizeof(BASE_DECLS)/sizeof(BASE_DECLS[0]))

/* ═══════════════════════════════════════════════════════════════
 * 层级叠加 (Layer Stacking)
 *
 * 低层是"因", 高层是"果"
 * 低层预加载数据约束上层所有调用结果
 *
 * 层级结构 (从低到高):
 *   L0 (底层): 声母/韵母/偏旁 (原子级 · 不可再拆)
 *   L1 (容器): 汉字感知场容器化 (容器级 · 可夹持)
 *   L2 (管道): 流水线排队 · 流程化步骤 (管道级 · 可排水)
 *   L3 (产品): 输出结果 · 可提交物 (产品级 · 可拿取)
 *
 * 公约数关系:
 *   L0的公约数 (如声母公约数 = 同组声母) 被L1整除
 *   L1的公约数 (感知家族) 被L2整除
 *   L2的公约数 (流水线时序) 被L3整除
 *
 * 这就是"层级叠加" — 不是高维覆盖低维, 是低维支撑高维
 * ═══════════════════════════════════════════════════════════════ */

typedef enum {
    LAYER_L0_ATOMIC  = 0x0,  /* L0: 原子层 (声母/韵母/偏旁) */
    LAYER_L1_CONTAINER = 0x1, /* L1: 容器层 (汉字容器化) */
    LAYER_L2_PIPELINE  = 0x2, /* L2: 管道层 (流水线) */
    LAYER_L3_PRODUCT   = 0x3, /* L3: 产品层 (输出结果) */
} layer_level_t;

/* 层级容器: 高层叠加在低层之上 */
typedef struct {
    layer_level_t level;     /* 所在层级 */
    uint32_t     divisor;    /* 该层的公约数 (代表多少个低层元素) */
    uint32_t     code;       /* 该层的容器编码 */
    uint32_t     timestamp;  /* 准入时间 (一元时间线) */
} layer_stack_entry_t;

/* ═══════════════════════════════════════════════════════════════
 * 容器流水线完整通路
 *
 * 从UTF-8汉字输入到容器编码输出的完整管道
 * ═══════════════════════════════════════════════════════════════ */

/* 通路状态 */
typedef enum {
    PATH_OK = 0,             /* 通路畅通 */
    PATH_ERR_INVALID_UTF8,   /* UTF-8无效 */
    PATH_ERR_UNKNOWN_HANZI,  /* 未知汉字 */
    PATH_ERR_MIRROR_MISMATCH,/* 镜像不匹配 */
    PATH_ERR_JUDGE_REJECT,   /* 裁判拒绝 */
    PATH_ERR_PIPELINE_FULL,  /* 流水线满 */
} pipeline_status_t;

/* 通路结果 */
typedef struct {
    pipeline_status_t status;
    uint16_t ucs2;           /* UCS-2 编码 */
    uint32_t container_code; /* 容器编码 (准入后) */
    uint32_t pipeline_pos;   /* 流水线位置 */
    uint8_t  family;         /* 感知家族 */
    uint8_t  polarity;       /* 主权侧 */
} pipeline_result_t;

/* ═══════════════════════════════════════════════════════════════
 * API · 容器流水线通路
 * ═══════════════════════════════════════════════════════════════ */

/**
 * UTF-8汉字 → 容器编码 (完整通路)
 *   utf8 = UTF-8编码的汉字 (1-4字节)
 *   返回 pipeline_result_t (含状态/编码/位置)
 */
pipeline_result_t container_pipeline_process_utf8(const char *utf8);

/**
 * 直接通路: 已知 UCS-2 + 声母/韵母/声调
 *   跳过UTF-8解码和拼音查询,直接进入感知场
 */
pipeline_result_t container_pipeline_route_direct(uint16_t ucs2,
                                                  uint8_t sm_idx,
                                                  uint8_t ym_idx,
                                                  uint8_t tone);

/**
 * 获取进制表 (供查询/打印用)
 */
const base_declaration_t* base_decl_get(uint32_t idx);
uint32_t base_decl_count(void);

/**
 * 流水线当前排水深度
 */
uint32_t pipeline_get_depth(void);

/**
 * 层级叠加信息
 */
uint32_t layer_get_divisor(layer_level_t level);

#endif /* NJSG_CONTAINER_PIPELINE_H */
