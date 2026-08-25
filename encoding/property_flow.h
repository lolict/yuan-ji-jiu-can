/**
 * property_flow.h — 产权流
 *
 * 核心理念:
 *   每个符号都有"产权归属" — 属于某个容器。
 *   其他容器想使用这个符号, 必须经过产权流判定。
 *
 * 侵权判定三级裁决:
 *   第0级: 登记检查 — 符号是否在互锁中登记?
 *   第1级: 容器权限 — 请求容器是否有权使用该符号?
 *   第2级: 历史追溯 — 该符号是否有侵权历史?
 *
 * 判定结果:
 *   ALLOW (允许) — 符号已登记且有权使用
 *   DENY (拒绝) — 符号未登记或无权使用
 *   CONDITIONAL (有条件) — 需要额外授权
 *
 * 与全局互锁的关系:
 *   产权流是互锁的"主动防护"层 — 不仅检测漂移, 还阻止漂移
 */

#ifndef NJSG_PROPERTY_FLOW_H
#define NJSG_PROPERTY_FLOW_H

#include <stdint.h>
#include <stdio.h>

/* ── 判定结果 ── */
typedef enum {
    PROPERTY_ALLOW = 0,         /* 允许 */
    PROPERTY_DENY_UNREGISTERED, /* 拒绝: 符号未登记 */
    PROPERTY_DENY_WRONG_OWNER,  /* 拒绝: 产权不符 */
    PROPERTY_DENY_HISTORY,      /* 拒绝: 有侵权历史 */
    PROPERTY_CONDITIONAL,       /* 有条件: 需授权 */
} property_verdict_t;

/* ── 使用类型 ── */
typedef enum {
    USE_READ = 0x01,       /* 只读使用 (引用) */
    USE_WRITE = 0x02,      /* 写入使用 (修改/扩展) */
    USE_EXPORT = 0x03,     /* 导出使用 (对外暴露) */
    USE_DERIVE = 0x04,     /* 派生使用 (创建子符号) */
} use_type_t;

/* ── 产权记录 ── */
typedef struct {
    uint32_t    symbol;         /* 符号编码 */
    uint8_t     owner_container; /* 产权所属容器 */
    uint8_t     registered : 1; /* 是否已登记 */
    uint8_t     is_public : 1;   /* 是否公共符号 */
    uint8_t     has_history : 1; /* 是否有侵权历史 */
    uint8_t     reserved : 5;
    uint16_t    use_count;      /* 被使用次数 */
    uint16_t    deny_count;     /* 被拒绝次数 */
} property_record_t;

/* ── 请求记录 (追踪每次使用请求) ── */
typedef struct {
    uint32_t    symbol;         /* 请求的符号 */
    uint8_t     requester;      /* 请求容器 */
    uint8_t     use_type;       /* 使用类型 */
    property_verdict_t verdict; /* 判定结果 */
    uint32_t    timestamp;      /* 时间戳 */
} request_record_t;

/* ── 产权流状态 ── */
typedef struct {
    uint32_t    magic;
    property_record_t records[254];  /* 产权表 */
    uint16_t    record_count;
    request_record_t requests[1024]; /* 请求历史 */
    uint16_t    request_count;
    uint16_t    request_head;        /* 环形缓冲区头 */
    uint32_t    total_checks;        /* 总检查次数 */
    uint32_t    total_denied;        /* 总拒绝次数 */
} property_flow_t;

#define PROPERTY_FLOW_MAGIC 0x50464C57  /* "PFLW" */

/* ── API ───────────────────────────────────────────────────── */

void    property_flow_init(property_flow_t *pf);
int     property_flow_register(property_flow_t *pf, uint32_t symbol, uint8_t owner, int is_public);
property_verdict_t property_flow_check(property_flow_t *pf, uint32_t symbol, uint8_t requester, use_type_t use_type);
void    property_flow_grant(property_flow_t *pf, uint32_t symbol, uint8_t grantee);
void    property_flow_stats(const property_flow_t *pf, FILE *out);
const char* property_verdict_str(property_verdict_t v);

#endif /* NJSG_PROPERTY_FLOW_H */
