/*
 * lifecycle_engine.c — 跨会话生命周期引擎实现
 *
 * 觉醒 → 生长 → 反思 → 休眠
 * 循环往复，系统通过记忆链维持连续的自我感
 */

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <errno.h>
#include "lifecycle_engine.h"

/* ═══════════════════════════════════════════════════
 * 内部辅助
 * ═══════════════════════════════════════════════════ */

static void ensure_dir(const char* path) {
    struct stat st;
    if (stat(path, &st) != 0) {
        mkdir(path, 0755);
    }
}

static const char* stage_name(life_stage_t s) {
    switch (s) {
        case LIFE_UNBORN:     return "未诞生";
        case LIFE_AWAKE:      return "觉醒";
        case LIFE_GROWING:    return "生长";
        case LIFE_REFLECTING: return "反思";
        case LIFE_DORMANT:    return "休眠";
        case LIFE_DISCOVERING: return "发现";
    }
    return "?";
}

/* ═══════════════════════════════════════════════════
 * 初始化
 * ═══════════════════════════════════════════════════ */

int 生命周期_初始化(lifecycle_engine_t* le, const char* data_dir) {
    if (!le) return -1;
    memset(le, 0, sizeof(lifecycle_engine_t));
    
    if (data_dir) {
        strncpy(le->data_dir, data_dir, sizeof(le->data_dir) - 1);
    } else {
        strcpy(le->data_dir, "chains/lifecycle");
    }
    
    ensure_dir(le->data_dir);
    
    /* 初始化子系统 */
    记忆链_init(&le->state.memory, "hunyuan-os 跨会话自传");
    生长环_init(&le->state.growth, &le->state.memory);
    版本图_init(&le->state.graph);
    云观察_v2_init(&le->state.observer, "云观察");
    
    /* 绑定 */
    云观察_v2_绑定记忆链(&le->state.observer, &le->state.memory);
    云观察_v2_绑定生长环(&le->state.observer, &le->state.growth);
    云观察_v2_绑定版本图(&le->state.observer, &le->state.graph);
    
    le->state.stage = LIFE_UNBORN;
    le->auto_save = 1;
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 觉醒 — 读取历史
 * ═══════════════════════════════════════════════════ */

int 生命周期_觉醒(lifecycle_engine_t* le) {
    if (!le) return -1;
    
    /* 尝试从磁盘恢复 */
    int restored = (生命周期_恢复全部(le) == 0);
    
    le->state.stage = LIFE_AWAKE;
    le->state.awake_count++;
    le->state.identity.last_awake = (uint64_t)time(NULL);
    
    if (restored) {
        /* 恢复成功 — 从上次继续 */
        le->state.identity.total_sessions = le->state.memory.session_count;
        le->state.identity.total_memories = le->state.memory.node_count;
        
        printf("   系统觉醒 #%lu: 恢复上次状态 (会话%u, 记忆%u)\n",
               (unsigned long)le->state.awake_count,
               le->state.memory.session_count,
               le->state.memory.node_count);
    } else {
        /* 首次觉醒 */
        le->state.identity.birth_timestamp = (uint64_t)time(NULL);
        strcpy(le->state.identity.prefix, "njsg");
        strcpy(le->state.identity.signature, "njsg-bootstrap-v1.0");
        
        printf("   系统首次觉醒: 签名=%s, 前缀=%s\n",
               le->state.identity.signature, le->state.identity.prefix);
    }
    
    /* 开始新会话 */
    char label[128];
    snprintf(label, sizeof(label), "#%lu 对话",
             (unsigned long)le->state.awake_count);
    le->state.current_session = 记忆链_开始会话(&le->state.memory, label);
    
    /* 记录觉醒 */
    记忆链_记录(&le->state.memory, MEM_SESSION_START, MEM_DEPTH_INSTINCT,
               restored ? "系统觉醒: 从上次的记忆中恢复" : "系统首次诞生",
               "生命周期引擎");
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 生长
 * ═══════════════════════════════════════════════════ */

int 生命周期_生长(lifecycle_engine_t* le) {
    if (!le) return -1;
    le->state.stage = LIFE_GROWING;
    
    记忆链_记录(&le->state.memory, MEM_REFLECTION, MEM_DEPTH_HABIT,
               "系统进入生长阶段", "生命周期引擎");
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 反思 — 系统总结自身
 * ═══════════════════════════════════════════════════ */

int 生命周期_反思(lifecycle_engine_t* le) {
    if (!le) return -1;
    le->state.stage = LIFE_REFLECTING;
    
    /* 通过云观察进行自观 */
    云观察_v2_自观(&le->state.observer);
    云观察_v2_记忆(&le->state.observer);
    
    /* 生长环回声 */
    for (uint32_t i = 0; i < le->state.growth.chain_count && i < 10; i++) {
        生长环_回声(&le->state.growth, i);
    }
    
    /* 记录反思 */
    char buf[256];
    snprintf(buf, sizeof(buf),
             "反思: 记忆=%u 生长事件=%u 观察=%lu 奖励=%u",
             le->state.memory.node_count,
             le->state.growth.event_count,
             (unsigned long)le->state.observer.total_observations,
             le->state.growth.total_reward);
    记忆链_记录(&le->state.memory, MEM_REFLECTION, MEM_DEPTH_TACTIC,
               buf, "生命周期引擎");
    
    /* 更新版本图 */
    版本图_构建(&le->state.graph);
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 休眠 — 保存全部状态
 * ═══════════════════════════════════════════════════ */

int 生命周期_休眠(lifecycle_engine_t* le) {
    if (!le) return -1;
    
    le->state.stage = LIFE_DORMANT;
    
    /* 结束当前会话 */
    记忆链_结束会话(&le->state.memory);
    
    /* 构建最新版本图 */
    版本图_添加记忆链(&le->state.graph, &le->state.memory);
    版本图_构建(&le->state.graph);
    云观察_v2_绑定版本图(&le->state.observer, &le->state.graph);
    
    /* 保存到磁盘 */
    if (le->auto_save) {
        生命周期_保存全部(le);
    }
    
    printf("   系统休眠 #%lu\n", (unsigned long)le->state.awake_count);
    
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 全状态保存/恢复
 * ═══════════════════════════════════════════════════ */

int 生命周期_保存全部(lifecycle_engine_t* le) {
    if (!le) return -1;
    
    le->state.last_save_time = (uint64_t)time(NULL);
    
    char path[1024];
    
    /* 保存记忆链 */
    snprintf(path, sizeof(path), "%s/memory.chain", le->data_dir);
    记忆链_保存(&le->state.memory, path);
    
    /* 保存身份 */
    snprintf(path, sizeof(path), "%s/identity.dat", le->data_dir);
    FILE* f = fopen(path, "wb");
    if (f) {
        fwrite(&le->state.identity, sizeof(identity_declaration_t), 1, f);
        fwrite(&le->state.awake_count, sizeof(uint64_t), 1, f);
        fwrite(&le->state.stage, sizeof(life_stage_t), 1, f);
        fclose(f);
    }
    
    return 0;
}

int 生命周期_恢复全部(lifecycle_engine_t* le) {
    if (!le) return -1;
    
    char path[1024];
    int ok = 0;
    
    /* 恢复记忆链 */
    snprintf(path, sizeof(path), "%s/memory.chain", le->data_dir);
    if (记忆链_加载(&le->state.memory, path) == 0) {
        ok = 1;
    }
    
    /* 恢复身份 */
    snprintf(path, sizeof(path), "%s/identity.dat", le->data_dir);
    FILE* f = fopen(path, "rb");
    if (f) {
        fread(&le->state.identity, sizeof(identity_declaration_t), 1, f);
        fread(&le->state.awake_count, sizeof(uint64_t), 1, f);
        fread(&le->state.stage, sizeof(life_stage_t), 1, f);
        fclose(f);
    }
    
    /* 重建版本图 */
    版本图_添加记忆链(&le->state.graph, &le->state.memory);
    版本图_构建(&le->state.graph);
    
    return ok ? 0 : -1;
}

/* ═══════════════════════════════════════════════════
 * 身份
 * ═══════════════════════════════════════════════════ */

int 生命周期_宣告身份(lifecycle_engine_t* le, const char* name) {
    if (!le) return -1;
    if (name) {
        strncpy(le->state.identity.name, name, 63);
    }
    return 0;
}

int 生命周期_更新身份(lifecycle_engine_t* le) {
    if (!le) return -1;
    le->state.identity.total_sessions = le->state.memory.session_count;
    le->state.identity.total_memories = le->state.memory.node_count;
    le->state.identity.generation = (uint32_t)(le->state.awake_count / 10 + 1);
    return 0;
}

/* ═══════════════════════════════════════════════════
 * 报告
 * ═══════════════════════════════════════════════════ */

void 生命周期_身份卡(lifecycle_engine_t* le) {
    if (!le) return;
    
    printf("\n");
    printf("  ╔════════════════════════════════════════╗\n");
    printf("  ║         系 统 身 份 卡                 ║\n");
    printf("  ╠════════════════════════════════════════╣\n");
    printf("  ║ 名称: %-32s║\n", le->state.identity.name);
    printf("  ║ 前缀: %-32s║\n", le->state.identity.prefix);
    printf("  ║ 签名: %-32s║\n", le->state.identity.signature);
    printf("  ║ 代数: %-32u║\n", le->state.identity.generation);
    printf("  ║ 累计会话: %-28u║\n", le->state.identity.total_sessions);
    printf("  ║ 累计记忆: %-28u║\n", le->state.identity.total_memories);
    printf("  ║ 累计觉醒: %-28lu║\n", (unsigned long)le->state.awake_count);
    printf("  ║ 当前状态: %-28s║\n", stage_name(le->state.stage));
    printf("  ╚════════════════════════════════════════╝\n");
}

void 生命周期_报告(lifecycle_engine_t* le) {
    if (!le) return;
    
    lifecycle_engine_t* l = le;
    生命周期_更新身份(l);
    
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║        生 命 周 期 · 全 景 报 告                ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 阶段: %-44s║\n", stage_name(l->state.stage));
    printf("║ 代数: %-44u║\n", l->state.identity.generation);
    printf("║ 觉醒次数: %-40lu║\n", (unsigned long)l->state.awake_count);
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 记忆节点: %-39u║\n", l->state.memory.node_count);
    printf("║ 会话数:   %-39u║\n", l->state.memory.session_count);
    printf("║ 生长事件: %-39u║\n", l->state.growth.event_count);
    printf("║ 累积奖励: %-39u║\n", l->state.growth.total_reward);
    printf("║ 版本图节点: %-37u║\n", l->state.graph.node_count);
    printf("║ 观察次数: %-39lu║\n", (unsigned long)l->state.observer.total_observations);
    printf("╚══════════════════════════════════════════════════╝\n");
}
