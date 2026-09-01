/*
 * memory_chain.c — 记忆链实现
 *
 * 追踪系统自身的跨会话生长过程
 * 每一次交互都成为记忆，每一段记忆都链接到前一段
 * 系统通过记忆链观察自己的"生长纹路"
 */

#include <stdio.h>
#include <string.h>
#include <time.h>
#include "memory_chain.h"

/* ═══════════════════════════════════════════════════
 * 生命周期
 * ═══════════════════════════════════════════════════ */

void 记忆链_init(memory_chain_t* mc, const char* desc) {
    memset(mc, 0, sizeof(memory_chain_t));
    mc->node_count = 0;
    mc->session_count = 0;
    mc->current_session = 0;
    strncpy(mc->chain_name, MEM_CHAIN_NAME, sizeof(mc->chain_name) - 1);
    if (desc) {
        strncpy(mc->description, desc, sizeof(mc->description) - 1);
    } else {
        strcpy(mc->description, "系统生长记忆");
    }
}

uint32_t 记忆链_开始会话(memory_chain_t* mc, const char* label) {
    if (mc->session_count >= MEM_MAX_SESSIONS) return 0;
    
    uint32_t sid = mc->session_count + 1;
    mem_session_t* s = &mc->sessions[mc->session_count];
    s->session_id = sid;
    s->start_time = (uint64_t)time(NULL);
    s->end_time = 0;
    s->node_count = 0;
    s->first_version = mc->node_count + 1;
    s->last_version = 0;
    if (label) {
        strncpy(s->label, label, sizeof(s->label) - 1);
    } else {
        snprintf(s->label, sizeof(s->label), "会话%d", sid);
    }
    
    mc->session_count++;
    mc->current_session = sid;
    
    /* 记录会话开始 */
    记忆链_记录(mc, MEM_SESSION_START, MEM_DEPTH_EVENT,
                s->label, "记忆链");
    
    return sid;
}

void 记忆链_结束会话(memory_chain_t* mc) {
    if (mc->current_session == 0) return;
    
    /* 找到当前会话 */
    for (uint32_t i = 0; i < mc->session_count; i++) {
        if (mc->sessions[i].session_id == mc->current_session) {
            mc->sessions[i].end_time = (uint64_t)time(NULL);
            mc->sessions[i].last_version = mc->node_count;
            
            char buf[128];
            snprintf(buf, sizeof(buf), "会话结束: %u 个记忆节点",
                     mc->sessions[i].node_count);
            记忆链_记录(mc, MEM_SESSION_END, MEM_DEPTH_EVENT, buf, "记忆链");
            break;
        }
    }
    mc->current_session = 0;
}

uint32_t 记忆链_记录(memory_chain_t* mc, mem_type_t type, mem_depth_t depth,
                      const char* content, const char* actor) {
    if (mc->node_count >= MEM_MAX_NODES) return 0;
    
    mem_node_t* n = &mc->nodes[mc->node_count];
    n->version = mc->node_count + 1;
    n->prev_version = (mc->node_count > 0) ? mc->node_count : 0;
    n->session_id = mc->current_session;
    n->type = type;
    n->depth = depth;
    n->timestamp = (uint64_t)time(NULL);
    n->chain_ref = 0;
    n->node_ref = 0;
    
    if (content) {
        strncpy(n->content, content, sizeof(n->content) - 1);
    }
    if (actor) {
        strncpy(n->actor, actor, sizeof(n->actor) - 1);
    } else {
        strcpy(n->actor, "系统");
    }
    
    mc->node_count++;
    
    /* 更新会话统计 */
    if (mc->current_session > 0 && mc->current_session <= mc->session_count) {
        mc->sessions[mc->current_session - 1].node_count++;
        mc->sessions[mc->current_session - 1].last_version = n->version;
    }
    
    return n->version;
}

/* ═══════════════════════════════════════════════════
 * 链式追踪
 * ═══════════════════════════════════════════════════ */

mem_node_t* 记忆链_寻版本(memory_chain_t* mc, uint32_t version) {
    if (version == 0 || version > mc->node_count) return NULL;
    return &mc->nodes[version - 1];
}

/* 追踪从 from_version 到根节点的完整路径 */
uint32_t 记忆链_追踪(memory_chain_t* mc, uint32_t from_version,
                         char* buf, size_t buf_size) {
    if (from_version == 0 || from_version > mc->node_count) return 0;
    
    size_t offset = 0;
    uint32_t v = from_version;
    uint32_t steps = 0;
    
    offset += snprintf(buf + offset, buf_size - offset,
                       "── 版本追踪 [v%u → 根] ──\n", from_version);
    
    while (v > 0 && steps < MEM_MAX_NODES) {
        mem_node_t* n = &mc->nodes[v - 1];
        
        /* 类型描述 */
        const char* type_desc = "?";
        switch (n->type) {
            case MEM_SESSION_START: type_desc = "会话"; break;
            case MEM_SESSION_END:   type_desc = "结束"; break;
            case MEM_FILE_CREATE:   type_desc = "创建"; break;
            case MEM_FILE_MODIFY:   type_desc = "修改"; break;
            case MEM_CHAIN_APPEND:  type_desc = "链增"; break;
            case MEM_CHAIN_PERSIST: type_desc = "持久"; break;
            case MEM_OBSERVATION:   type_desc = "观察"; break;
            case MEM_REFLECTION:    type_desc = "反思"; break;
            case MEM_MILESTONE:     type_desc = "碑石"; break;
            case MEM_DISCOVERY:     type_desc = "发现"; break;
        }
        
        /* 层级描述 */
        const char* depth_desc = "?";
        switch (n->depth) {
            case MEM_DEPTH_INSTINCT:  depth_desc = "元"; break;
            case MEM_DEPTH_HABIT:     depth_desc = "阶"; break;
            case MEM_DEPTH_TACTIC:    depth_desc = "层"; break;
            case MEM_DEPTH_OPERATION: depth_desc = "代"; break;
            case MEM_DEPTH_EVENT:     depth_desc = "世"; break;
            case MEM_DEPTH_SCENE:     depth_desc = "界"; break;
            case MEM_DETAIL:          depth_desc = "域"; break;
            case MEM_ATOM:            depth_desc = "场"; break;
        }
        
        offset += snprintf(buf + offset, buf_size - offset,
                           "  [v%u] %s·%s ← %s\n",
                           v, type_desc, depth_desc, n->content);
        
        if (v == n->prev_version) break;  /* 防止循环 */
        v = n->prev_version;
        steps++;
    }
    
    offset += snprintf(buf + offset, buf_size - offset,
                       "── 共 %u 步回溯到根 ──\n", steps);
    
    return steps;
}

/* ═══════════════════════════════════════════════════
 * 跨会话
 * ═══════════════════════════════════════════════════ */

uint32_t 记忆链_跨会话统计(memory_chain_t* mc, uint32_t* versions_out, 
                               uint32_t max_count) {
    /* 每个会话取最新的一个里程碑或发现 */
    uint32_t found = 0;
    for (uint32_t s = mc->session_count; s > 0 && found < max_count; s--) {
        mem_session_t* sess = &mc->sessions[s - 1];
        /* 倒序查找里程碑 */
        for (uint32_t v = sess->last_version; v >= sess->first_version && v > 0; v--) {
            if (v <= mc->node_count) {
                mem_node_t* n = &mc->nodes[v - 1];
                if (n->type == MEM_MILESTONE || n->type == MEM_DISCOVERY) {
                    versions_out[found++] = v;
                    break;
                }
            }
        }
    }
    return found;
}

void 记忆链_会话摘要(memory_chain_t* mc, uint32_t session_id, 
                             char* buf, size_t buf_size) {
    if (session_id == 0 || session_id > mc->session_count) {
        snprintf(buf, buf_size, "会话不存在");
        return;
    }
    
    mem_session_t* s = &mc->sessions[session_id - 1];
    size_t offset = 0;
    
    offset += snprintf(buf + offset, buf_size - offset,
                       "会话 #%u: %s\n", s->session_id, s->label);
    offset += snprintf(buf + offset, buf_size - offset,
                       "  记忆节点: %u (v%u → v%u)\n",
                       s->node_count, s->first_version, s->last_version);
    
    /* 统计类型 */
    uint32_t types[256] = {0};
    for (uint32_t v = s->first_version; v <= s->last_version && v <= mc->node_count; v++) {
        types[mc->nodes[v - 1].type]++;
    }
    
    if (types[MEM_FILE_CREATE] > 0)
        offset += snprintf(buf + offset, buf_size - offset,
                           "  创建文件: %u\n", types[MEM_FILE_CREATE]);
    if (types[MEM_OBSERVATION] > 0)
        offset += snprintf(buf + offset, buf_size - offset,
                           "  云观察: %u\n", types[MEM_OBSERVATION]);
    if (types[MEM_REFLECTION] > 0)
        offset += snprintf(buf + offset, buf_size - offset,
                           "  反思: %u\n", types[MEM_REFLECTION]);
    if (types[MEM_MILESTONE] > 0)
        offset += snprintf(buf + offset, buf_size - offset,
                           "  里程碑: %u\n", types[MEM_MILESTONE]);
    if (types[MEM_DISCOVERY] > 0)
        offset += snprintf(buf + offset, buf_size - offset,
                           "  发现: %u\n", types[MEM_DISCOVERY]);
}

/* ═══════════════════════════════════════════════════
 * 记忆回溯
 * ═══════════════════════════════════════════════════ */

uint32_t 记忆链_回溯(memory_chain_t* mc, uint32_t target_version,
                         void (*callback)(const mem_node_t* node, void* ctx),
                         void* ctx) {
    if (target_version == 0 || target_version > mc->node_count) return 0;
    
    uint32_t v = target_version;
    uint32_t steps = 0;
    
    while (v > 0 && steps < MEM_MAX_NODES) {
        mem_node_t* n = &mc->nodes[v - 1];
        if (callback) callback(n, ctx);
        
        if (v == n->prev_version) break;
        v = n->prev_version;
        steps++;
    }
    
    return steps;
}

/* ═══════════════════════════════════════════════════
 * 摘要报告
 * ═══════════════════════════════════════════════════ */

void 记忆链_报告(memory_chain_t* mc) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║           记 忆 链 · 自 传                      ║\n");
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ %-48s ║\n", mc->description);
    printf("╠══════════════════════════════════════════════════╣\n");
    printf("║ 总会话数: %-6u   总记忆节点: %-6u          ║\n",
           mc->session_count, mc->node_count);
    printf("╚══════════════════════════════════════════════════╝\n");
    
    /* 各会话摘要 */
    for (uint32_t s = 0; s < mc->session_count; s++) {
        mem_session_t* sess = &mc->sessions[s];
        char dur[32] = "-";
        if (sess->end_time > sess->start_time) {
            snprintf(dur, sizeof(dur), "%lus",
                     (unsigned long)(sess->end_time - sess->start_time));
        }
        printf("\n  会话#%u [%s] 节点:%u 时长:%s\n",
               sess->session_id, sess->label, sess->node_count, dur);
        
        /* 显示关键记忆 */
        for (uint32_t v = sess->first_version; v <= sess->last_version && v <= mc->node_count; v++) {
            mem_node_t* n = &mc->nodes[v - 1];
            if (n->type == MEM_MILESTONE || n->type == MEM_DISCOVERY ||
                n->type == MEM_FILE_CREATE) {
                const char* tag = (n->type == MEM_MILESTONE) ? "碑" :
                                  (n->type == MEM_DISCOVERY) ? "悟" : "建";
                printf("    [%s v%u] %s\n", tag, v, n->content);
            }
        }
    }
    
    /* 跨会话里程碑 */
    uint32_t milestones[64];
    uint32_t ms_count = 记忆链_跨会话统计(mc, milestones, 64);
    if (ms_count > 0) {
        printf("\n  跨会话里程碑线:\n");
        for (uint32_t i = 0; i < ms_count; i++) {
            mem_node_t* n = &mc->nodes[milestones[i] - 1];
            printf("    v%u: %s\n", milestones[i], n->content);
        }
    }
    
    printf("\n");
}

/* ═══════════════════════════════════════════════════
 * 持久化
 * ═══════════════════════════════════════════════════ */

int 记忆链_保存(memory_chain_t* mc, const char* path) {
    FILE* f = fopen(path, "wb");
    if (!f) return -1;
    
    /* 写入头部 */
    struct {
        char magic[4];       /* NJSG */
        uint32_t version;    /* 格式版本 */
        uint32_t node_count;
        uint32_t session_count;
        char name[64];
        char desc[256];
    } header;
    
    memcpy(header.magic, "NJSG", 4);
    header.version = 1;
    header.node_count = mc->node_count;
    header.session_count = mc->session_count;
    strncpy(header.name, mc->chain_name, 63);
    strncpy(header.desc, mc->description, 255);
    
    fwrite(&header, sizeof(header), 1, f);
    fwrite(mc->nodes, sizeof(mem_node_t), mc->node_count, f);
    fwrite(mc->sessions, sizeof(mem_session_t), mc->session_count, f);
    
    fclose(f);
    return 0;
}

int 记忆链_加载(memory_chain_t* mc, const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f) return -1;
    
    struct {
        char magic[4];
        uint32_t version;
        uint32_t node_count;
        uint32_t session_count;
        char name[64];
        char desc[256];
    } header;
    
    if (fread(&header, sizeof(header), 1, f) != 1) {
        fclose(f);
        return -1;
    }
    
    if (memcmp(header.magic, "NJSG", 4) != 0) {
        fclose(f);
        return -1;
    }
    
    记忆链_init(mc, header.desc);
    mc->node_count = header.node_count;
    mc->session_count = header.session_count;
    strncpy(mc->chain_name, header.name, 63);
    strncpy(mc->description, header.desc, 255);
    
    if (header.node_count > MEM_MAX_NODES) {
        fclose(f);
        return -1;
    }
    if (header.session_count > MEM_MAX_SESSIONS) {
        fclose(f);
        return -1;
    }
    
    fread(mc->nodes, sizeof(mem_node_t), header.node_count, f);
    fread(mc->sessions, sizeof(mem_session_t), header.session_count, f);
    
    fclose(f);
    return 0;
}
