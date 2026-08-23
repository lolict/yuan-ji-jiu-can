/**
 * diffraction_engine.h — 衍射引擎
 *
 * 原理: 输入一个节点, 衍射出它的多层属性
 *
 * 衍射维度 (每个节点都衍射出):
 *   ┌────────────────────────────────────────────────────────────────┐
 *   │ 1. 类型衍射    = 它是什么类型 (BASE/TONE/COMPOUND/SYMBOL/EXTRA)│
 *   │ 2. 锚衍射      = 它属于哪个公约数                              │
 *   │ 3. 审美衍射    = 它和候选配对的结构平衡分数 (0.0~1.0)         │
 *   │ 4. 目标衍射    = 它服务于哪个最终目标                         │
 *   │ 5. 合作衍射    = 能和谁合作                                 │
 *   │ 6. 力衍射      = 它在关系网中的压/被压力量                    │
 *   └────────────────────────────────────────────────────────────────┘
 *
 * 核心: 不是抓芝麻, 而是衍射出整个大海的洋流方向
 */

#ifndef NJSG_DIFFRACTION_ENGINE_H
#define NJSG_DIFFRACTION_ENGINE_H

#include <stdint.h>
#include "type_discipline.h"

/* ═══════════════════════════════════════════════════════════════
 * 衍射结果
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 单个节点的衍射输出
 */
typedef struct {
    const char    *name;           /* 节点名称 */
    symbol_type_t  type_diffract;  /* 类型衍射 */
    uint8_t        anchor_diffract;/* 锚衍射 (公约数) */
    
    /* 审美衍射 */
    float          aesthetic_score; /* 美学分数 0.0~1.0 */
    uint8_t        balanced;        /* 是否达到力平衡 */
    
    /* 目标衍射 */
    uint8_t        goal_code;      /* 服务于哪个目标 */
    
    /* 合作衍射 */
    uint8_t        can_cooperate;  /* 是否可以和其他节点合作 */
    uint8_t        partner_count;  /* 兼容的伙伴数量 */
    
    /* 力衍射 (编织平衡) */
    float          pressure_given;  /* 它施加给别人的力 */
    float          pressure_received; /* 它承受的力 */
    float          equilibrium_delta; /* 力差 (越接近0越平衡) */
} diffraction_result_t;

/**
 * 配对之间的衍射输出
 */
typedef struct {
    const char    *name_a;
    const char    *name_b;
    symbol_type_t  type_a;
    symbol_type_t  type_b;
    
    /* 类型衍射: 是否同类型/同锚 */
    uint8_t        type_match;     /* 类型是否一致 */
    uint8_t        anchor_match;   /* 锚是否一致 */
    
    /* 审美衍射 */
    float          symmetry;       /* 对称分数 */
    float          proportion;     /* 比例协调 */
    float          density_balance;/* 密度平衡 */
    float          overall_aesthetic; /* 综合审美 */
    
    /* 编织平衡 */
    float          weaving_tension;    /* 编织张力 */
    float          weaving_balance;    /* 编织平衡度 */
    uint8_t        is_weaving_ok;      /* 编织是否合格 */
} pair_diffraction_t;

/* ═══════════════════════════════════════════════════════════════
 * 法庭日志
 * ═══════════════════════════════════════════════════════════════ */

#define MAX_CRIMES 512
#define CRIME_NAME_LEN 32

/**
 * 犯罪记录
 */
typedef struct {
    uint32_t       timestamp;      /* 时间戳 (滴答计数) */
    const char    *perpetrator;    /* 罪犯 (AI输出的内容) */
    uint8_t        crime_type;     /* 犯罪类型 */
    uint8_t        severity;       /* 严重度 1~10 */
    const char    *victim;         /* 受害者 (谁被这个错误影响了) */
    const char    *evidence;       /* 证据 (具体的错误特征) */
    uint8_t        resolved;       /* 是否已解决 */
} crime_record_t;

/**
 * 犯罪类型枚举
 */
typedef enum {
    CRIME_TYPE_MISMATCH    = 0x01, /* 类型不匹配 (a=á) */
    CRIME_TIER_JUMP        = 0x02, /* 层级跳跃 (跳过中间层级) */
    CRIME_AESTHETIC_FAIL   = 0x03, /* 审美失败 (瘦子配胖子) */
    CRIME_BALANCE_BREAK    = 0x04, /* 编织失衡 (压/被压不均) */
    CRIME_UNKNOWN_INPUT    = 0x05, /* 恶意输入 (骂人的话当代码) */
    CRIME_ERROR_PROPAGATE  = 0x06, /* 错误传播 (错字继续编码) */
    CRIME_GOAL_DEVIATE     = 0x07, /* 目标偏移 (偏离公约数目标) */
    CRIME_EXTRA_UNCATEG    = 0x08, /* 溢出未归类 */
} crime_type_t;

/**
 * 法庭 — 全局单例
 */
typedef struct {
    uint32_t       magic;          /* DIFFRACTION_MAGIC */
    uint32_t       tick;           /* 滴答计数 */
    
    /* 衍射引擎状态 */
    type_discipline_t *type_police; /* 关联的类型警察 */
    
    /* 法庭日志 */
    uint32_t       crime_count;    /* 犯罪记录数 */
    uint32_t       total_prosecuted; /* 总共起诉数 */
    uint32_t       total_acquitted;  /* 总共无罪数 */
    crime_record_t crimes[MAX_CRIMES];
    
    /* 模式涌现: 犯罪统计 */
    uint32_t       crime_stats[16]; /* 每种犯罪的计数 */
    
    /* 已涌现的规则 */
    uint32_t       rules_emerged;   /* 涌现出的规则数 */
    
} diffraction_court_t;

#define DIFFRACTION_MAGIC 0x44494652  /* "DIFR" */

/* ═══════════════════════════════════════════════════════════════
 * API
 * ═══════════════════════════════════════════════════════════════ */

/**
 * 初始化衍射引擎/法庭
 */
void diffraction_court_init(diffraction_court_t *court, type_discipline_t *police);

/**
 * 衍射: 输入一个节点, 输出它的全部衍射特征
 */
void diffraction_court_diffract(diffraction_court_t *court,
                                const char *name,
                                diffraction_result_t *result);

/**
 * 衍射一对: 输入两个节点, 输出它们之间的全部衍射特征
 */
void diffraction_court_diffract_pair(diffraction_court_t *court,
                                     const char *name_a,
                                     const char *name_b,
                                     pair_diffraction_t *result);

/**
 * 法庭审判: 判断是否允许
 * @return 0=允许, >0=犯罪类型(拒绝)
 */
int diffraction_court_trial(diffraction_court_t *court,
                            const char *name_a,
                            const char *name_b);

/**
 * 记录犯罪
 */
void diffraction_court_record(diffraction_court_t *court,
                              const char *perpetrator,
                              crime_type_t crime_type,
                              uint8_t severity,
                              const char *victim,
                              const char *evidence);

/**
 * 统计报告: 哪种犯罪最多
 */
void diffraction_court_stats(diffraction_court_t *court);

/**
 * 涌现: 从犯罪记录中产生新规则
 */
uint32_t diffraction_court_emerge_rules(diffraction_court_t *court);

#endif /* NJSG_DIFFRACTION_ENGINE_H */
