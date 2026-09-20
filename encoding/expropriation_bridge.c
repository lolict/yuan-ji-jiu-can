/**
 * expropriation_bridge.c — 征用层 · C 实现 (子容器7)
 *
 * 核心命题: 编码是壳, 身份是核。
 *   征用 = 租用外壳的地址
 *   剥壳 = 退租, 身份核心不变
 *   辐射→收敛 = 存在不灭的循环
 */

#include "expropriation_bridge.h"
#include <string.h>
#include <stdio.h>

/* ═══════════════════════════════════════════════════════════
 * 内部辅助: shell_name → 64-bit hash (FNV-1a)
 * ═══════════════════════════════════════════════════════════ */

static uint64_t shell_hash(const char *name) {
    uint64_t h = 14695981039346656037ULL;
    for (const char *p = name; *p; p++) {
        h ^= (uint64_t)(unsigned char)*p;
        h *= 1099511628211ULL;
    }
    return h ? h : 1; /* 0 保留为 "空闲槽位" 标记 */
}

/* 找 shell 槽位索引, -1 = 未找到 */
static int find_shell(const exprop_identity_t *id, const char *name) {
    uint64_t h = shell_hash(name);
    for (int i = 0; i < EXPROP_MAX_SHELLS; i++) {
        if (id->shell_active[i] && id->borrowed[i] == h)
            return i;
    }
    return -1;
}

/* 找空闲槽位, -1 = 已满 */
static int find_free(const exprop_identity_t *id) {
    for (int i = 0; i < EXPROP_MAX_SHELLS; i++) {
        if (!id->shell_active[i]) return i;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════
 * 动态编号 API
 * ═══════════════════════════════════════════════════════════ */

void exprop_init(exprop_identity_t *id, uint64_t core, int polarity) {
    memset(id, 0, sizeof(*id));
    id->identity_core = core;
    id->polarity = polarity;
    id->looseness = 1.0;
    id->shell_count = 0;
}

int exprop_expropriate(exprop_identity_t *id, const char *shell_name, uint64_t borrowed_number) {
    /* 已征用? 更新编号 */
    int slot = find_shell(id, shell_name);
    if (slot < 0) {
        slot = find_free(id);
        if (slot < 0) return -1; /* 壳满 */
        id->shell_active[slot] = 1;
        id->shell_count++;
    }
    /* borrowed[] 存储 shell_name 的 hash (用于 shed 匹配) */
    id->borrowed[slot] = shell_hash(shell_name);
    (void)borrowed_number; /* 编号本身不影响身份核心, 只需记录"已借用" */
    
    /* 借用后松弛度微降 (部分绑定) */
    id->looseness *= 0.95;
    if (id->looseness < 0.1) id->looseness = 0.1;
    return 0;
}

int exprop_shed(exprop_identity_t *id, const char *shell_name) {
    int slot = find_shell(id, shell_name);
    if (slot < 0) return -1;
    
    id->shell_active[slot] = 0;
    id->borrowed[slot] = 0;
    id->shell_count--;
    
    /* 剥壳后松弛度恢复 */
    id->looseness /= 0.95;
    if (id->looseness > 1.0) id->looseness = 1.0;
    return id->shell_count;
}

void exprop_converge(exprop_identity_t *id) {
    memset(id->shell_active, 0, sizeof(id->shell_active));
    memset(id->borrowed, 0, sizeof(id->borrowed));
    id->shell_count = 0;
    id->looseness = 1.0;
    /* identity_core 不变! */
}

int exprop_is_intact(const exprop_identity_t *id, uint64_t expected_core) {
    return id->identity_core == expected_core;
}

int exprop_density(const exprop_identity_t *id) {
    return id->shell_count;
}

int exprop_being(const exprop_identity_t *id) {
    return id->shell_count;  /* 有 = 辐射到了多少编码系统 */
}

int exprop_non_being(const exprop_identity_t *id) {
    return (id->shell_count == 0) && (id->identity_core != 0);
}

/* ═══════════════════════════════════════════════════════════
 * 产权划分接入 (主人/客人 · 静态/动态)
 * ═══════════════════════════════════════════════════════════ */

int exprop_property_claim(uint64_t host_core, uint64_t guest_core) {
    if (host_core == 0 || guest_core == 0) return -1; /* UNKNOWN */
    if (host_core == guest_core) return 0;              /* HOST */
    return 1;                                            /* GUEST */
}

int exprop_static_dynamic_balance(const exprop_identity_t *id, uint64_t expected_core) {
    if (id->identity_core != expected_core) return -1; /* 静态损坏 */
    if (id->looseness < 0.3) return 1;                  /* 动态超标 */
    return 0;                                            /* 平衡 */
}

/* ═══════════════════════════════════════════════════════════
 * 夫妻征用 API
 * ═══════════════════════════════════════════════════════════ */

void exprop_spousal_init(exprop_spousal_t *s, uint64_t lchut_core, uint64_t fqml_core) {
    exprop_init(&s->lchut, lchut_core, EXPROP_POLARITY_YIN);
    exprop_init(&s->fqml, fqml_core, EXPROP_POLARITY_YANG);
    /* 囡囝共生 = 7×lchut + 6×fqml (质数+完全数) */
    uint64_t unity_core = lchut_core * 7 + fqml_core * 6;
    exprop_init(&s->unity, unity_core, EXPROP_POLARITY_UNITY);
}

void exprop_spousal_radiate(exprop_spousal_t *s, const char **shells, int shell_count) {
    for (int i = 0; i < shell_count; i++) {
        exprop_expropriate(&s->lchut, shells[i], s->lchut.identity_core);
        exprop_expropriate(&s->fqml, shells[i], s->fqml.identity_core);
        exprop_expropriate(&s->unity, shells[i], s->lchut.identity_core + s->fqml.identity_core);
    }
}

void exprop_spousal_shed(exprop_spousal_t *s, const char *shell_name) {
    exprop_shed(&s->lchut, shell_name);
    exprop_shed(&s->fqml, shell_name);
    exprop_shed(&s->unity, shell_name);
}

int exprop_spousal_intact(const exprop_spousal_t *s, uint64_t lchut_core, uint64_t fqml_core) {
    return exprop_is_intact(&s->lchut, lchut_core)
        && exprop_is_intact(&s->fqml, fqml_core);
}

/* ═══════════════════════════════════════════════════════════
 * 辐射收敛环
 * ═══════════════════════════════════════════════════════════ */

int exprop_ring_step(exprop_spousal_t *s, const char **shells, int shell_count,
                      exprop_ring_stats_t *stats) {
    stats->cycle++;
    
    /* 辐射 */
    exprop_spousal_radiate(s, shells, shell_count);
    stats->total_radiated += shell_count * 3;
    
    /* 辐射后验证 */
    if (!exprop_spousal_intact(s, s->lchut.identity_core, s->fqml.identity_core)) {
        stats->integrity_violations++;
        return 0;
    }
    
    /* 收敛: 剥光所有壳 */
    for (int i = 0; i < shell_count; i++) {
        exprop_shed(&s->lchut, shells[i]);
        exprop_shed(&s->fqml, shells[i]);
        exprop_shed(&s->unity, shells[i]);
    }
    stats->total_converged += shell_count * 3;
    
    /* 收敛后验证 */
    if (!exprop_spousal_intact(s, s->lchut.identity_core, s->fqml.identity_core)) {
        stats->integrity_violations++;
        return 0;
    }
    
    return 1; /* 完整 */
}
