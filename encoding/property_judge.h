/**
 * property_judge.h — 产权流侵权自动判定树
 *
 * 核心公式: vA + vB == 2·v_core
 *
 * 法理:
 *   v_core = 产权容器的核心价值 (公约数锚点)
 *   vA     = A方声索/登记的价值 (公倍数A)
 *   vB     = B方声索/登记的价值 (公倍数B)
 *
 * 侵权判定:
 *   vA + vB == 2·v_core → 平衡 (无侵权)
 *   vA + vB >  2·v_core → 超标 (双方或一方虚报)
 *   vA + vB <  2·v_core → 亏欠 (一方被侵吞)
 *
 * 判定树从粗到细:
 *   第一层: 总量是否平衡?
 *   第二层: 哪一方偏离?
 *   第三层: 偏离程度 (轻微/中度/严重)
 *   第四层: 侵权类型 (侵吞/虚报/混淆/篡改)
 *   第五层: 法条适用
 */

#ifndef NJSG_PROPERTY_JUDGE_H
#define NJSG_PROPERTY_JUDGE_H

#include <stdint.h>

/* ═══ 侵权类型 ═══ */
typedef enum {
    INFRING_NONE        = 0,    /* 无侵权 */
    INFRING_OVERCLAIM   = 1,    /* 虚报 (超过核心) */
    INFRING_UNDERCLAIM  = 2,    /* 侵吞 (低于核心) */
    INFRING_CONFUSION   = 3,    /* 混淆 (所有权不清) */
    INFRING_TAMPER      = 4,    /* 篡改 (伪造编码) */
    INFRING_MUTUAL      = 5,    /* 双方虚报 (共谋) */
    INFRING_BYPASS      = 6,    /* 绕过 (未经裁定) */
} infringement_t;

/* ═══ 偏离程度 ═══ */
typedef enum {
    DEVIATION_NONE      = 0,    /* 无偏离 */
    DEVIATION_MILD      = 1,    /* 轻微 (±5%以内) */
    DEVIATION_MODERATE  = 2,    /* 中度 (±5~20%) */
    DEVIATION_SEVERE    = 3,    /* 严重 (±20%以上) */
} deviation_t;

/* ═══ 法条 ═══ */
typedef enum {
    CLAUSE_BALANCED     = 0,    /* 第0条: 平衡态, 无侵权 */
    CLAUSE_A_OVERCLAIM  = 1,    /* 第1条: A方虚报侵权 */
    CLAUSE_B_OVERCLAIM  = 2,    /* 第2条: B方虚报侵权 */
    CLAUSE_MUTUAL_OVER  = 3,    /* 第3条: 双方虚报 */
    CLAUSE_A_UNDERCLAIM = 4,    /* 第4条: A方被侵吞 */
    CLAUSE_B_UNDERCLAIM = 5,    /* 第5条: B方被侵吞 */
    CLAUSE_CONFUSION    = 6,    /* 第6条: 所有权混淆 */
    CLAUSE_TAMPER       = 7,    /* 第7条: 编码篡改 */
    CLAUSE_BYPASS       = 8,    /* 第8条: 绕过裁定 */
} clause_t;

/* ═══ 裁定结果 ═══ */
typedef struct {
    uint32_t        v_core;          /* 核心价值 */
    uint32_t        v_a;             /* A方声索 */
    uint32_t        v_b;             /* B方声索 */
    int32_t         delta;           /* (vA+vB) - 2·v_core */
    uint8_t         balanced;        /* 是否平衡 */
    uint8_t         deviation_a;     /* A方偏离度 */
    uint8_t         deviation_b;     /* B方偏离度 */
    infringement_t  infringement;    /* 侵权类型 */
    clause_t        clause;          /* 适用法条 */
    const char     *verdict_text;    /* 裁定文本 */
} property_verdict_t;

/* ═══ API ═══ */

/**
 * 核心判定: vA + vB == 2·v_core
 *   返回裁定结果
 */
property_verdict_t property_judge(uint32_t v_core, uint32_t v_a, uint32_t v_b);

/**
 * 带容差的判定 (生产用)
 */
property_verdict_t property_judge_tolerant(uint32_t v_core, uint32_t v_a,
                                            uint32_t v_b, uint8_t tolerance_pct);

/**
 * 裁定报告输出
 */
void property_verdict_report(const property_verdict_t *v, char *buf, int bufsize);

/**
 * 批量判定演示
 */
void property_judge_demo(void);

#endif /* NJSG_PROPERTY_JUDGE_H */
