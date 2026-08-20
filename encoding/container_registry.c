/**
 * container_registry.c — 容器注册局实现
 *
 * 注册 · 授权 · 跳转 · 编译 · 拒绝外部
 */

#include "container_registry.h"
#include <string.h>
#include "fullquan_math.h"

/* ═══════════════════════════════════════════════════════════════
 * 内部: 名称复制 (安全 · 无溢出)
 * ═══════════════════════════════════════════════════════════════ */

static void safe_copy_name(char *dst, const char *src) {
    int i;
    for (i = 0; i < TIER_NAME_LEN - 1 && src[i]; i++) {
        dst[i] = src[i];
    }
    dst[i] = '\0';
}

/* ═══════════════════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════════════════ */

void registry_init(container_registry_t *reg, container_group_t *group) {
    memset(reg, 0, sizeof(*reg));
    reg->magic    = CONTAINER_REGISTRY_MAGIC;
    reg->group    = group;
}

/* ═══════════════════════════════════════════════════════════════
 * 注册层级
 *
 * 注册一次 · 后续自动生效
 * 标识符即进制: radix = tier_id
 * ═══════════════════════════════════════════════════════════════ */

uint16_t registry_register_tier(container_registry_t *reg,
                                const char *name,
                                uint8_t wall_sym,
                                uint8_t rank,
                                uint8_t bit_width,
                                uint16_t capacity,
                                uint16_t chars_per) {
    if (!reg || reg->magic != CONTAINER_REGISTRY_MAGIC) return 0;
    if (reg->tier_count >= MAX_TIERS) return 0;

    uint16_t idx = reg->tier_count;
    tier_registry_entry_t *t = &reg->tiers[idx];

    /* 填充 */
    t->tier_id           = idx + 1;  /* id从1开始 */
    safe_copy_name(t->name, name);
    t->wall_symbol       = wall_sym;
    t->rank              = rank;
    t->bit_width         = bit_width ? bit_width : 1;
    t->radix             = t->tier_id;  /* 标识符即进制 */
    t->container_capacity = capacity;
    t->chars_per_container = chars_per ? chars_per : 2;
    t->registered        = 1;
    t->authorized        = 0;  /* 需要授权 */
    t->compiled          = 0;

    reg->tier_count++;
    return t->tier_id;
}

/* ═══════════════════════════════════════════════════════════════
 * 授权层级
 *
 * 只有授权的层级才能使用
 * ═══════════════════════════════════════════════════════════════ */

int registry_authorize_tier(container_registry_t *reg, uint16_t tier_id) {
    if (!reg || reg->magic != CONTAINER_REGISTRY_MAGIC) return -1;
    if (tier_id == 0 || tier_id > reg->tier_count) return -1;

    tier_registry_entry_t *t = &reg->tiers[tier_id - 1];
    if (!t->registered) return -1;

    t->authorized = 1;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 自动跳转配置
 *
 * 当 from 容器的字符用完 · 自动跳转到 to 容器
 * ═══════════════════════════════════════════════════════════════ */

int registry_set_jump(container_registry_t *reg,
                      uint16_t from_tier, uint16_t from_container,
                      uint16_t to_tier, uint16_t to_container) {
    if (!reg || reg->magic != CONTAINER_REGISTRY_MAGIC) return -1;
    if (reg->jump_count >= MAX_JUMPS) return -1;

    jump_entry_t *j = &reg->jumps[reg->jump_count];
    j->from_tier       = from_tier;
    j->from_container  = from_container;
    j->to_tier         = to_tier;
    j->to_container    = to_container;
    j->used            = 0;

    reg->jump_count++;
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 查询跳转
 * ═══════════════════════════════════════════════════════════════ */

uint16_t registry_find_jump(container_registry_t *reg,
                            uint16_t tier_id, uint16_t container_idx) {
    if (!reg || reg->magic != CONTAINER_REGISTRY_MAGIC) return 0;

    for (uint16_t i = 0; i < reg->jump_count; i++) {
        if (reg->jumps[i].from_tier == tier_id &&
            reg->jumps[i].from_container == container_idx) {
            return reg->jumps[i].to_tier;
        }
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 拒绝外部编码
 *
 * 检查是否是外部魔数 (如 ELF 或 PE)
 * 返回: 1=拒绝 · 0=接受(我们的)
 * ═══════════════════════════════════════════════════════════════ */

uint8_t registry_reject_external(uint32_t magic1, uint32_t magic2) {
    /* 我们的魔数 · 接受 */
    if (magic1 == REGISTRY_MAGIC_1 && magic2 == REGISTRY_MAGIC_2) return 0;
    if (magic1 == 1836019524) return 0;  /* "mlil" 的十进制 */
    if (magic1 == 1735616623) return 0;  /* "gtxo" 的十进制 */

    /* ELF */
    if (magic1 == 1179403649) return 1;  /* ELF 魔数十进制 */

    /* PE (MZ) */
    if ((magic1 & FQ_FULL_16) == 23117) return 1;  /* "MZ" 的十进制 */

    /* 未知 · 拒绝 */
    return 1;
}

/* ═══════════════════════════════════════════════════════════════
 * 查找层级
 * ═══════════════════════════════════════════════════════════════ */

tier_registry_entry_t *registry_find_tier(container_registry_t *reg, const char *name) {
    if (!reg || !name) return 0;

    for (uint16_t i = 0; i < reg->tier_count; i++) {
        int match = 1;
        for (int j = 0; j < TIER_NAME_LEN; j++) {
            if (reg->tiers[i].name[j] != name[j]) { match = 0; break; }
            if (name[j] == '\0') break;
        }
        if (match) return &reg->tiers[i];
    }
    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 编译到自有文件
 *
 * 输出格式:
 *   [header: 48字节] + [tier_entries: 32字节*N] + [jump_entries: 8字节*M]
 *
 * 只有我们的解释器能读 · 外部打开=乱码
 * 编译完成后 · 自身立即可用 · 不需要外部工具
 * ═══════════════════════════════════════════════════════════════ */

int registry_compile_to_buffer(container_registry_t *reg,
                               uint8_t *buffer, uint32_t buf_size) {
    if (!reg || !buffer) return -1;

    /* 计算所需大小 */
    uint32_t header_size = sizeof(registry_file_header_t);
    uint32_t tier_size   = (uint32_t)reg->tier_count * sizeof(tier_registry_entry_t);
    uint32_t jump_size   = (uint32_t)reg->jump_count * sizeof(jump_entry_t);
    uint32_t total_size  = header_size + tier_size + jump_size;

    if (total_size > buf_size) return -1;

    uint32_t pos = 0;

    /* 1. 文件头 */
    registry_file_header_t *fh = (registry_file_header_t *)buffer;
    fh->magic1         = REGISTRY_MAGIC_1;
    fh->magic2         = REGISTRY_MAGIC_2;
    fh->version        = REGISTRY_VERSION;
    fh->timestamp      = 0;  /* 简化: 不填实际时间 */
    fh->tier_count     = reg->tier_count;
    fh->container_count = (uint16_t)(reg->group ? reg->group->total_count : 0);
    fh->total_chars    = reg->group ? reg->group->total_chars : 0;
    fh->compiled_by_us = 1;
    fh->reject_external = 1;
    fh->author_id      = 1;  /* 我(lolict) */
    fh->reserved       = 0;

    pos = header_size;

    /* 2. 层级项 */
    memcpy(buffer + pos, reg->tiers, tier_size);
    pos += tier_size;

    /* 3. 跳转表 */
    memcpy(buffer + pos, reg->jumps, jump_size);
    pos += jump_size;

    /* 标记已编译 */
    for (uint16_t i = 0; i < reg->tier_count; i++) {
        reg->tiers[i].compiled = 1;
    }
    reg->compile_count++;

    return (int)pos;
}
