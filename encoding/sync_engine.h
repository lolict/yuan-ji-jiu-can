/*
 * sync_engine.h — 全局符号同步引擎
 *
 * 全量空间同步替换:
 *   改一处 = 改全部 · 跨链 · 跨文件 · 跨模块
 *   不是 sed 外部替换 = 系统自身能力
 *
 * 实现方式:
 *   1. 符号注册表 (所有符号集中登记)
 *   2. 引用追踪 (谁引用了这个符号)
 *   3. 同步替换 (改注册表中一条 · 所有引用自动更新)
 *
 * 进制式地址: 每个符号不是字符串 = 64位 SMRL 码
 *   替换不是字符串操作 = 进制地址重定向
 */

#ifndef NJSG_SYNC_ENGINE_H
#define NJSG_SYNC_ENGINE_H

#include <stdint.h>
#include "chain_block.h"
#include "smrl_engine.h"

#define SYNC_MAX_SYMBOLS    1024
#define SYNC_MAX_REFS       2048
#define SYNC_MAX_NAME       32
#define SYNC_MAX_TARGETS    16
#define SYNC_MAGIC          0x53594343  /* "SYCC" */

/* 符号条目 — 一个被登记的名字 */
typedef struct {
    char        name[SYNC_MAX_NAME];        /* 符号名 (如 "天庭") */
    char        replacement[SYNC_MAX_NAME]; /* 替换名 (如 "容器") */
    uint64_t    smrl_code;                  /* 该符号的 SMRL 编码 */
    uint8_t     level;                      /* 所在层级 */
    uint8_t     category;                   /* 所在族 */
    uint32_t    ref_count;                  /* 被引用次数 */
    uint32_t    flags;                      /* 已替换/待替换/锁定 */
} sync_symbol_t;

/* 引用追踪 — 谁在用这个符号 */
typedef struct {
    uint32_t    symbol_idx;                 /* 指向哪个符号 */
    uint32_t    chain_id;                   /* 哪条链引用了它 */
    uint32_t    node_version;               /* 哪个版本 */
    char        context[SYNC_MAX_NAME];     /* 引用处的上下文 */
} sync_ref_t;

/* 同步引擎状态 */
typedef struct {
    uint32_t        magic;                  /* SYNC_MAGIC */
    sync_symbol_t   symbols[SYNC_MAX_SYMBOLS];
    uint32_t        symbol_count;
    sync_ref_t      refs[SYNC_MAX_REFS];
    uint32_t        ref_count;
    uint32_t        total_syncs;            /* 累计同步次数 */
    uint32_t        total_replacements;     /* 累计替换次数 */
} sync_engine_t;

/* ── API ── */

/**
 * 初始化同步引擎
 */
int  sync_engine_init(sync_engine_t* engine);

/**
 * 注册一个符号到同步引擎
 *   返回符号索引，-1 = 已满
 */
int  sync_register(sync_engine_t* engine,
                   const char* name, uint8_t level, uint8_t category,
                   uint64_t smrl_code);

/**
 * 注册一个引用关系
 */
int  sync_register_ref(sync_engine_t* engine,
                       const char* symbol_name,
                       uint32_t chain_id, uint32_t version,
                       const char* context);

/**
 * 全量同步替换: 替换一个符号的所有引用
 *   返回被更新的引用数量，-1 = 符号不存在
 */
int  sync_replace(sync_engine_t* engine,
                  const char* old_name, const char* new_name);

/**
 * 按链批量替换: 替换某条链中所有匹配的符号
 */
int  sync_replace_in_chain(chain_t* chain,
                           const char* old_content, const char* new_content);

/**
 * 跨链全量替换: 替换所有链中的某个符号
 *   返回总替换数
 */
int  sync_replace_all_chains(chain_t* chains, uint32_t chain_count,
                             const char* old_content, const char* new_content);

/**
 * 从所有已注册链中收集符号
 */
int  sync_collect_from_chains(sync_engine_t* engine,
                              chain_t* chains, uint32_t chain_count);

/**
 * 同步引擎报告
 */
void sync_engine_report(const sync_engine_t* engine);

#endif /* NJSG_SYNC_ENGINE_H */
