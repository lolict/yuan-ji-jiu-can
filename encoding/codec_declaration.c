/**
 * codec_declaration.c — 声明式编码系统实现
 *
 * 核心: 声明即执行 · 改表头即改系统行为
 */

#include "codec_declaration.h"
#include <string.h>

/* ── 全局状态 ────────────────────────────────────────────── */

static codec_banner_t  g_banner;                          /* 声明表头 */
static codec_slot_t    g_slots[CODEC_MAX_CONTAINERS];      /* 容器注册表 */
static codec_protocol_t g_protocols[CODEC_MAX_PROTOCOLS];  /* 协议表 */

/* ═══════════════════════════════════════════════════════════════
 * 内部工具
 * ═══════════════════════════════════════════════════════════════ */

static int find_slot_by_id(uint32_t id) {
    for (uint32_t i = 0; i < g_banner.container_count; i++) {
        if (g_slots[i].id == id) return (int)i;
    }
    return -1;
}

static void auto_balance(void) {
    if (!g_banner.auto_balance) return;

    /* 按优先级排序 (冒泡 · 通常注册不多 · 不需要快排) */
    for (uint32_t i = 0; i < g_banner.container_count; i++) {
        for (uint32_t j = i + 1; j < g_banner.container_count; j++) {
            if (g_slots[j].priority < g_slots[i].priority) {
                codec_slot_t tmp = g_slots[i];
                g_slots[i] = g_slots[j];
                g_slots[j] = tmp;
            }
        }
    }

    /* 数 active */
    uint32_t active = 0;
    for (uint32_t i = 0; i < g_banner.container_count; i++) {
        if (g_slots[i].active) active++;
    }
    g_banner.active_count = active;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码
 * ═══════════════════════════════════════════════════════════════ */

codec_active_set_t codec_get_active_set(void) {
    codec_active_set_t set;
    memset(&set, 0, sizeof(set));
    set.mode = g_banner.mode;
    set.max_active = CODEC_MAX_CONTAINERS;

    switch (g_banner.mode) {
    case CODEC_MODE_FULL:
        /* 全量模式 · 所有注册容器都激活 */
        for (uint32_t i = 0; i < g_banner.container_count; i++) {
            set.ids[set.count++] = g_slots[i].id;
        }
        break;

    case CODEC_MODE_SELECT:
        /* 选择模式 · 只包含 active=1 的 · 按优先级排序 */
        for (uint32_t i = 0; i < g_banner.container_count; i++) {
            if (g_slots[i].active) {
                set.ids[set.count++] = g_slots[i].id;
            }
        }
        break;

    case CODEC_MODE_INERTIA:
        /* 最佳态 · 先放 immutable=1 的 · 再补优先级高的 */
        /* 第一轮: 必不可少 */
        for (uint32_t i = 0; i < g_banner.container_count; i++) {
            if (g_slots[i].immutable) {
                set.ids[set.count++] = g_slots[i].id;
            }
        }
        /* 第二轮: 补 */
        for (uint32_t i = 0; i < g_banner.container_count; i++) {
            if (!g_slots[i].immutable && g_slots[i].active) {
                set.ids[set.count++] = g_slots[i].id;
            }
        }
        break;
    }

    return set;
}

/* ═══════════════════════════════════════════════════════════════
 * 编码
 * ═══════════════════════════════════════════════════════════════ */

codec_result_t codec_encode(uint32_t input, uint32_t input_type) {
    codec_result_t result;
    memset(&result, 0, sizeof(result));
    result.error = CODEC_OK;

    /* 从活跃集找到最匹配的容器 */
    codec_active_set_t set = codec_get_active_set();
    if (set.count == 0) {
        result.error = CODEC_ERR_INACTIVE;
        return result;
    }

    /* 找优先级最高 + 类型匹配 */
    for (uint32_t i = 0; i < set.count; i++) {
        int idx = find_slot_by_id(set.ids[i]);
        if (idx < 0) continue;
        if (g_slots[idx].type == input_type) {
            result.container_id = set.ids[i];
            result.encoded_value = input;  /* 实际编码逻辑可在此加 */
            result.protocol_used = (uint8_t)g_banner.protocol_id;
            return result;
        }
    }

    /* 没类型匹配 · 就回最高的 */
    int idx = find_slot_by_id(set.ids[0]);
    if (idx >= 0) {
        result.container_id = set.ids[0];
        result.encoded_value = input;
        result.protocol_used = (uint8_t)g_banner.protocol_id;
    } else {
        result.error = CODEC_ERR_NOTFOUND;
    }

    return result;
}

codec_result_t codec_decode(uint32_t container_id, uint32_t encoded_value) {
    codec_result_t result;
    memset(&result, 0, sizeof(result));
    result.error = CODEC_OK;

    int idx = find_slot_by_id(container_id);
    if (idx < 0) {
        result.error = CODEC_ERR_NOTFOUND;
        return result;
    }

    result.container_id = container_id;
    result.encoded_value = encoded_value;  /* 实际解码逻辑可在此加 */
    result.protocol_used = (uint8_t)g_banner.protocol_id;
    return result;
}

codec_result_t codec_translate(uint32_t input, uint32_t from_type, uint32_t to_type) {
    codec_result_t result;
    memset(&result, 0, sizeof(result));

    /* encode: 从 from_type 容器编码 */
    result = codec_encode(input, from_type);
    if (result.error != CODEC_OK) return result;

    /* 在同一个容器中找到 to_type 的规则 · 做转译 */
    /* 简化: 如果类型一致 · 直接返回 */
    if (from_type == to_type) return result;

    /* 否则 · 查找转译协议 */
    for (int i = 0; i < CODEC_MAX_PROTOCOLS; i++) {
        if (g_protocols[i].id == 0) continue;
        if (g_protocols[i].source_type == from_type &&
            g_protocols[i].target_type == to_type) {
            result.protocol_used = (uint8_t)i;
            break;
        }
    }

    return result;
}

/* ═══════════════════════════════════════════════════════════════
 * API · 实现
 * ═══════════════════════════════════════════════════════════════ */

codec_error_t codec_init(codec_banner_t *banner) {
    if (!banner) return CODEC_ERR_MAGIC;
    memset(&g_banner, 0, sizeof(g_banner));
    memset(g_slots, 0, sizeof(g_slots));
    memcpy(&g_banner, banner, sizeof(codec_banner_t));
    g_banner.container_count = 0;
    g_banner.active_count = 0;
    return CODEC_OK;
}

codec_error_t codec_load_slots(const codec_slot_t *slots, uint32_t count) {
    if (!slots || count > CODEC_MAX_CONTAINERS) return CODEC_ERR_FULL;
    memcpy(g_slots, slots, count * sizeof(codec_slot_t));
    g_banner.container_count = count;

    /* 数 active */
    uint32_t active = 0;
    for (uint32_t i = 0; i < count; i++) {
        if (g_slots[i].active) active++;
    }
    g_banner.active_count = active;
    return CODEC_OK;
}

codec_error_t codec_register(const codec_slot_t *slot) {
    if (!slot) return CODEC_ERR_NOTFOUND;
    if (g_banner.container_count >= CODEC_MAX_CONTAINERS) return CODEC_ERR_FULL;
    memcpy(&g_slots[g_banner.container_count], slot, sizeof(codec_slot_t));
    g_banner.container_count++;
    if (slot->active) g_banner.active_count++;
    auto_balance();
    return CODEC_OK;
}

codec_error_t codec_unregister(uint32_t container_id) {
    int idx = find_slot_by_id(container_id);
    if (idx < 0) return CODEC_ERR_NOTFOUND;
    /* 删除 */
    for (uint32_t i = (uint32_t)idx; i < g_banner.container_count - 1; i++) {
        g_slots[i] = g_slots[i + 1];
    }
    g_banner.container_count--;
    auto_balance();
    return CODEC_OK;
}

codec_error_t codec_set_active(uint32_t container_id, int active) {
    int idx = find_slot_by_id(container_id);
    if (idx < 0) return CODEC_ERR_NOTFOUND;
    g_slots[idx].active = active ? 1 : 0;
    auto_balance();
    return CODEC_OK;
}

codec_error_t codec_set_priority(uint32_t container_id, uint8_t priority) {
    int idx = find_slot_by_id(container_id);
    if (idx < 0) return CODEC_ERR_NOTFOUND;
    g_slots[idx].priority = priority;
    auto_balance();
    return CODEC_OK;
}

codec_error_t codec_set_mode(codec_mode_t mode) {
    g_banner.mode = mode;
    /* 切换模式后 · 活跃集自动重算 */
    return CODEC_OK;
}

codec_error_t codec_register_protocol(const codec_protocol_t *protocol) {
    if (!protocol) return CODEC_ERR_NOTFOUND;
    if (protocol->id >= CODEC_MAX_PROTOCOLS) return CODEC_ERR_FULL;
    memcpy(&g_protocols[protocol->id], protocol, sizeof(codec_protocol_t));
    return CODEC_OK;
}

codec_error_t codec_switch_protocol(uint32_t protocol_id) {
    if (protocol_id >= CODEC_MAX_PROTOCOLS) return CODEC_ERR_NOTFOUND;
    g_banner.protocol_id = protocol_id;
    return CODEC_OK;
}
