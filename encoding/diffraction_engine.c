/**
 * diffraction_engine.c — 衍射引擎实现
 *
 * 核心: 不是抓芝麻, 而是衍射出洋流方向
 * 每个节点输入 → 衍射出类型/审美/目标/合作/力
 * 每对节点输入 → 衍射出对称/比例/密度/编织平衡
 */

#include "diffraction_engine.h"
#include <string.h>
#include <math.h>

#ifndef M_PI
#define M_PI 3.14159265
#endif

/* ═══════════════════════════════════════════════════════════════
 * 内部工具
 * ═══════════════════════════════════════════════════════════════ */

static uint32_t tick_counter = 0;

static uint32_t next_tick(void) {
    return ++tick_counter;
}

/* 计算两个UTF-8字符的视觉"重量" (简化: 用字节长度代理) */
static float visual_weight(const char *s) {
    if (!s) return 0.5f;
    int len = strlen(s);
    /* 单字节=轻(ASCII), 双字节=中(拉丁扩展), 三字节=重(CJK/符号) */
    if (len <= 1) return 0.3f;
    if (len == 2) return 0.6f;
    return 0.9f;
}

/* 计算视觉"高度" (简化: 用字符结构代理) */
static float visual_height(const char *s) {
    if (!s) return 0.5f;
    /* 有上延的字符(b,d,f,h,k,l,t) = 高 */
    /* 有下延的字符(g,j,p,q,y) = 低 */
    /* 基础字符(a,c,e,m,n,o,r,s,u,v,w,x,z) = 中 */
    /* 简化: 用字节数代理 */
    int len = strlen(s);
    if (len >= 3) return 0.8f; /* 复杂符号视觉高度大 */
    if (len == 2) return 0.6f;
    return 0.4f;
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void diffraction_court_init(diffraction_court_t *court, type_discipline_t *police) {
    memset(court, 0, sizeof(*court));
    court->magic = DIFFRACTION_MAGIC;
    court->type_police = police;
    court->tick = next_tick();
}

/* ═══════════════════════════════════════════════════════════════
 * 单节点衍射
 * ═══════════════════════════════════════════════════════════════ */

void diffraction_court_diffract(diffraction_court_t *court,
                                const char *name,
                                diffraction_result_t *result) {
    if (!court || court->magic != DIFFRACTION_MAGIC || !name || !result) return;
    
    memset(result, 0, sizeof(*result));
    result->name = name;
    
    /* 类型衍射 */
    result->type_diffract = type_discipline_get_type(court->type_police, name);
    
    /* 锚衍射: 从类型警察中找锚 */
    result->anchor_diffract = 0xFF;
    if (court->type_police) {
        for (uint16_t i = 0; i < court->type_police->count; i++) {
            if (strcmp(court->type_police->entries[i].name, name) == 0) {
                result->anchor_diffract = court->type_police->entries[i].anchor;
                break;
            }
        }
    }
    
    /* 审美衍射: 基于视觉重量和高度 */
    float w = visual_weight(name);
    float h = visual_height(name);
    result->aesthetic_score = 1.0f - fabsf(w - h); /* 越接近越美 */
    
    /* 目标衍射: 基于类型 */
    switch (result->type_diffract) {
        case TYPE_BASE:     result->goal_code = 1; break; /* 基韵 → 锚定 */
        case TYPE_TONE:     result->goal_code = 2; break; /* 声调 → 时钟 */
        case TYPE_COMPOUND: result->goal_code = 3; break; /* 复韵 → 配对 */
        case TYPE_SYMBOL:   result->goal_code = 3; break; /* 符号 → 配对 */
        case TYPE_EXTRA:    result->goal_code = 0; break; /* 溢出 → 待定 */
        default:            result->goal_code = 0; break;
    }
    
    /* 合作衍射 */
    result->can_cooperate = (result->type_diffract != TYPE_UNKNOWN && 
                             result->type_diffract != TYPE_EXTRA);
    result->partner_count = result->can_cooperate ? 1 : 0;
    
    /* 力衍射: 初始为0, 在配对时计算 */
    result->pressure_given = w;
    result->pressure_received = 0.0f;
    result->equilibrium_delta = w;
    result->balanced = 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 配对衍射
 * ═══════════════════════════════════════════════════════════════ */

void diffraction_court_diffract_pair(diffraction_court_t *court,
                                     const char *name_a,
                                     const char *name_b,
                                     pair_diffraction_t *result) {
    if (!court || !name_a || !name_b || !result) return;
    
    memset(result, 0, sizeof(*result));
    result->name_a = name_a;
    result->name_b = name_b;
    
    /* 类型衍射 */
    symbol_type_t ta = type_discipline_get_type(court->type_police, name_a);
    symbol_type_t tb = type_discipline_get_type(court->type_police, name_b);
    result->type_a = ta;
    result->type_b = tb;
    result->type_match = (ta == tb) ? 1 : 0;
    
    /* 锚衍射 */
    uint8_t anchor_a = 0xFF, anchor_b = 0xFF;
    if (court->type_police) {
        for (uint16_t i = 0; i < court->type_police->count; i++) {
            if (strcmp(court->type_police->entries[i].name, name_a) == 0)
                anchor_a = court->type_police->entries[i].anchor;
            if (strcmp(court->type_police->entries[i].name, name_b) == 0)
                anchor_b = court->type_police->entries[i].anchor;
        }
    }
    result->anchor_match = (anchor_a == anchor_b && anchor_a != 0xFF) ? 1 : 0;
    
    /* 审美衍射 */
    float wa = visual_weight(name_a);
    float wb = visual_weight(name_b);
    float ha = visual_height(name_a);
    float hb = visual_height(name_b);
    
    /* 对称: 重量越接近越对称 */
    result->symmetry = 1.0f - fabsf(wa - wb);
    /* 比例: 高度越接近越协调 */
    result->proportion = 1.0f - fabsf(ha - hb);
    /* 密度平衡: 综合 */
    result->density_balance = 1.0f - (fabsf(wa - wb) + fabsf(ha - hb)) / 2.0f;
    /* 综合审美 */
    result->overall_aesthetic = (result->symmetry + result->proportion + result->density_balance) / 3.0f;
    
    /* 编织平衡 */
    result->weaving_tension = fabsf(wa - wb) + fabsf(ha - hb);
    result->weaving_balance = 1.0f - result->weaving_tension / 2.0f;
    if (result->weaving_balance < 0) result->weaving_balance = 0;
    /* 编织合格: 张力<0.5 且 类型匹配 */
    result->is_weaving_ok = (result->weaving_tension < 0.5f && result->type_match) ? 1 : 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 法庭审判
 * ═══════════════════════════════════════════════════════════════ */

int diffraction_court_trial(diffraction_court_t *court,
                            const char *name_a,
                            const char *name_b) {
    if (!court || court->magic != DIFFRACTION_MAGIC) return -1;
    
    pair_diffraction_t diff;
    diffraction_court_diffract_pair(court, name_a, name_b, &diff);
    
    court->tick = next_tick();
    
    /* 规则1: 类型不匹配 = 犯罪 */
    if (!diff.type_match) {
        diffraction_court_record(court, name_a, CRIME_TYPE_MISMATCH, 8,
                                name_b, "type mismatch");
        return CRIME_TYPE_MISMATCH;
    }
    
    /* 规则2: 同类型但不同锚 = 犯罪 (a vs o) */
    if (diff.type_match && !diff.anchor_match) {
        diffraction_court_record(court, name_a, CRIME_TYPE_MISMATCH, 6,
                                name_b, "anchor mismatch");
        return CRIME_TYPE_MISMATCH;
    }
    
    /* 规则3: 审美失败 = 犯罪 */
    if (diff.overall_aesthetic < 0.3f) {
        diffraction_court_record(court, name_a, CRIME_AESTHETIC_FAIL, 5,
                                name_b, "aesthetic fail");
        return CRIME_AESTHETIC_FAIL;
    }
    
    /* 规则4: 编织失衡 = 犯罪 */
    if (!diff.is_weaving_ok) {
        diffraction_court_record(court, name_a, CRIME_BALANCE_BREAK, 4,
                                name_b, "weaving imbalance");
        return CRIME_BALANCE_BREAK;
    }
    
    /* 通过 */
    court->total_acquitted++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 记录犯罪
 * ═══════════════════════════════════════════════════════════════ */

void diffraction_court_record(diffraction_court_t *court,
                              const char *perpetrator,
                              crime_type_t crime_type,
                              uint8_t severity,
                              const char *victim,
                              const char *evidence) {
    if (!court || court->magic != DIFFRACTION_MAGIC) return;
    if (court->crime_count >= MAX_CRIMES) return;
    
    crime_record_t *cr = &court->crimes[court->crime_count];
    cr->timestamp = court->tick;
    cr->perpetrator = perpetrator;
    cr->crime_type = crime_type;
    cr->severity = severity;
    cr->victim = victim;
    cr->evidence = evidence;
    cr->resolved = 0;
    
    court->crime_count++;
    court->total_prosecuted++;
    
    /* 统计 */
    if (crime_type < 16) court->crime_stats[crime_type]++;
}

/* ═══════════════════════════════════════════════════════════════
 * 统计报告
 * ═══════════════════════════════════════════════════════════════ */

void diffraction_court_stats(diffraction_court_t *court) {
    if (!court || court->magic != DIFFRACTION_MAGIC) return;
    
    /* 这里只是记录, 具体输出由上层实现 */
    /* 统计已记录在 crime_stats[] 中 */
}

/* ═══════════════════════════════════════════════════════════════
 * 涌现: 从犯罪记录中产生新规则
 * ═══════════════════════════════════════════════════════════════ */

uint32_t diffraction_court_emerge_rules(diffraction_court_t *court) {
    if (!court || court->magic != DIFFRACTION_MAGIC) return 0;
    
    uint32_t new_rules = 0;
    
    /* 规则涌现: 如果某种犯罪超过阈值, 产生新规则 */
    for (int i = 1; i < 16; i++) {
        if (court->crime_stats[i] >= 5 && court->crime_stats[i] < 100) {
            /* 这种犯罪频繁出现 → 产生一条新规则来专门拦截它 */
            new_rules++;
            /* 标记为已处理, 避免重复涌现 */
            court->crime_stats[i] = 100;
        }
    }
    
    court->rules_emerged += new_rules;
    return new_rules;
}
