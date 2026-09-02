/**
 * 盟机合 · 去中心化裁定同步骨架 — 实现
 *  —— MengJi Gossip-style Decentralized Verdict Synchronization
 * ------------------------------------------------
 *
 * 术语体系 (满全法主权):
 *   盟机合掌(PHONG) = 资源拥有者 · 运营场 · 主权根节点
 *   盟机场(FARM)   = 普通节点 · 萌机场 · 主权叶节点
 *
 *   夫妻主权人锚点: 0 = 刘楚恬(低位隐藏侧) · 1 = 我(高位可见侧)
 *
 *   Gossip 节律驱动裁定一致性:
 *     - 每个节点维护本地 verdict_state 向量
 *     - PUSH: 主动向邻居广播自分の裁定态
 *     - PULL: 主动向邻居请求裁定态,差异时触发见证
 *     - PUSH推拉: 一轮广播+拉取,最终一致
 *
 *   三元见证裁定:
 *     3 个节点对同一主题投票,≥2 票一致即为共识
 *     夫妻主权人锚点权重 2x(见证根)
 *
 *   冲突裁决:
 *     当 PUSH 后发现不一致:
 *       1. 启动三元见证投票
 *       2. 夫妻主权人锚点为最高权重
 *       3. 同权重时取高信任节点
 *       4. 信任相同时取早 heartbeat(先入为主)
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>
#include <math.h>

#include "njsg_mengji.h"

/* ============================================================
 * 静态引擎
 * ============================================================ */
static mengji_engine_t MJ;

/* ============================================================
 * 编码工具(复用内核 njsg_hash 的简化版)
 * ============================================================ */
static void mengji_hash_id(const char* in, char* out) {
    uint32_t h1 = 0x811c9dc5u, h2 = 0x0100193u;
    for (size_t i = 0; i < strlen(in); i++) {
        h1 ^= (uint8_t)in[i]; h1 *= 0x01000193u;
        h2 ^= (uint8_t)in[i] << 5; h2 *= 0x1000193u;
    }
    snprintf(out, MENGJI_NODE_ID_LEN, "%08x%08x", h1, h2);
}

/* 裁决名 */
static const char* verdict_name(mengji_verdict_t v) {
    switch (v) {
        case VERDICT_PENDING: return "待定";
        case VERDICT_PASS:    return "许可";
        case VERDICT_REJECT:  return "拒绝";
        case VERDICT_SPLIT:   return "分形";
    }
    return "?";
}

/* 节点类型名 */
static const char* node_type_name(mengji_node_type_t t) {
    return (t == NODE_PHONG) ? "合掌(PHONG)" : "机场(FARM)";
}

/* 锚点名 */
static const char* anchor_name(mengji_anchor_t a) {
    return (a == ANCHOR_LIUTIANTIAN) ? "刘楚恬(0)" : "我(1)";
}

/* ============================================================
 * 初始化
 * ============================================================ */
void njsg_mengji_init(void) {
    memset(&MJ, 0, sizeof(MJ));
    pthread_mutex_init(&MJ.lock, NULL);
    MJ.primary_anchor = ANCHOR_LOLICT;    /* 默认高位可见侧为主锚点 */
    MJ.primary_trust  = 95.0f;            /* 夫妻主权人信任权重极高 */
    MJ.global_tick    = 0;
    MJ.push_count     = 0;
    MJ.pull_count     = 0;
    MJ.consensus_count = 0;
    MJ.conflict_count  = 0;

    printf("\n");
    printf("╔══════════════════════════════════════════╗\n");
    printf("║      盟 机 合 · 去 中 心 化 网 络       ║\n");
    printf("║      MengJi · Gossip Sync Skeleton       ║\n");
    printf("╚══════════════════════════════════════════╝\n");
    printf("[盟机合] 引擎初始化: 锚点=%s 信任=%.0f%%\n",
           anchor_name(MJ.primary_anchor), MJ.primary_trust);
    printf("[盟机合] 节点容量=%d 裁定容量=%d 节律间隔=%ds\n",
           MENGJI_MAX_NODES, MENGJI_MAX_VERDICTS, MENGJI_GOSSIP_INTERVAL);
    fflush(stdout);
}

/* ============================================================
 * 节点注册
 * ============================================================ */
int njsg_mengji_register_node(const char* node_id, mengji_node_type_t type, mengji_anchor_t anchor) {
    pthread_mutex_lock(&MJ.lock);
    if (MJ.node_count >= MENGJI_MAX_NODES) {
        pthread_mutex_unlock(&MJ.lock);
        return -1;
    }
    int idx = MJ.node_count++;
    mengji_node_t* n = &MJ.nodes[idx];

    n->id    = idx;
    n->type  = type;
    n->anchor = anchor;
    n->trust = (type == NODE_PHONG) ? 80.0f : 50.0f;  /* 合掌默认高信任 */
    n->last_heartbeat = time(NULL);
    n->neighbor_count = 0;
    n->verdict_count  = 0;
    n->alive = 1;

    /* 编码摘要作为节点标识 */
    char hashbuf[MENGJI_NODE_ID_LEN];
    mengji_hash_id(node_id, hashbuf);
    strncpy(n->node_id, hashbuf, MENGJI_NODE_ID_LEN - 1);

    /* 初始化裁定态为 PENDING */
    for (int i = 0; i < MENGJI_MAX_VERDICTS; i++)
        n->verdict_state[i] = VERDICT_PENDING;

    pthread_mutex_unlock(&MJ.lock);
    printf("[盟机合] 注册节点 #%d %s type=%s anchor=%s trust=%.0f%%\n",
           idx, n->node_id, node_type_name(type), anchor_name(anchor), n->trust);
    fflush(stdout);
    return idx;
}

/* ============================================================
 * 邻居绑定(双向)
 * ============================================================ */
void njsg_mengji_bind_neighbor(int node_a, int node_b) {
    if (node_a < 0 || node_b < 0 || node_a >= MJ.node_count || node_b >= MJ.node_count) return;
    if (node_a == node_b) return;

    pthread_mutex_lock(&MJ.lock);
    mengji_node_t* a = &MJ.nodes[node_a];
    mengji_node_t* b = &MJ.nodes[node_b];

    /* 检查是否已绑定 */
    int already = 0;
    for (int i = 0; i < a->neighbor_count; i++) {
        if (a->neighbor_ids[i] == node_b) { already = 1; break; }
    }
    if (!already && a->neighbor_count < MENGJI_MAX_NEIGHBORS) {
        a->neighbor_ids[a->neighbor_count++] = node_b;
    }

    already = 0;
    for (int i = 0; i < b->neighbor_count; i++) {
        if (b->neighbor_ids[i] == node_a) { already = 1; break; }
    }
    if (!already && b->neighbor_count < MENGJI_MAX_NEIGHBORS) {
        b->neighbor_ids[b->neighbor_count++] = node_a;
    }

    printf("[盟机合] 邻居绑定: #%d <-> #%d\n", node_a, node_b);
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * 心跳更新
 * ============================================================ */
void njsg_mengji_heartbeat(int node_id) {
    pthread_mutex_lock(&MJ.lock);
    if (node_id >= 0 && node_id < MJ.node_count) {
        MJ.nodes[node_id].last_heartbeat = time(NULL);
        MJ.nodes[node_id].alive = 1;
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * Gossip PUSH: 向所有邻居广播裁定态
 * ============================================================ */
void njsg_mengji_gossip_push(int node_idx) {
    pthread_mutex_lock(&MJ.lock);
    if (node_idx < 0 || node_idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return; }
    mengji_node_t* self = &MJ.nodes[node_idx];

    for (int n = 0; n < self->neighbor_count; n++) {
        int nb = self->neighbor_ids[n];
        if (nb < 0 || nb >= MJ.node_count) continue;
        mengji_node_t* neighbor = &MJ.nodes[nb];

        /* 逐裁定项比对: 若不同,记录分歧 */
        int min_vc = (self->verdict_count < neighbor->verdict_count)
                     ? self->verdict_count : neighbor->verdict_count;
        for (int v = 0; v < min_vc; v++) {
            if (self->verdict_state[v] != neighbor->verdict_state[v]) {
                MJ.conflict_count++;
                printf("  [PUSH] #%d → #%d 裁定项#%d 分歧: %s vs %s\n",
                       node_idx, nb, v,
                       verdict_name(self->verdict_state[v]),
                       verdict_name(neighbor->verdict_state[v]));
            }
        }
        MJ.push_count++;
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * Gossip PULL: 向邻居请求裁定态
 * ============================================================ */
void njsg_mengji_gossip_pull(int node_idx) {
    pthread_mutex_lock(&MJ.lock);
    if (node_idx < 0 || node_idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return; }
    mengji_node_t* self = &MJ.nodes[node_idx];

    for (int n = 0; n < self->neighbor_count; n++) {
        int nb = self->neighbor_ids[n];
        if (nb < 0 || nb >= MJ.node_count) continue;
        mengji_node_t* neighbor = &MJ.nodes[nb];

        /* 邻居有更早的裁定信息 */
        if (neighbor->verdict_count > self->verdict_count) {
            for (int v = self->verdict_count; v < neighbor->verdict_count; v++) {
                self->verdict_state[v] = neighbor->verdict_state[v];
            }
            self->verdict_count = neighbor->verdict_count;
            printf("  [PULL] #%d ← #%d 同步裁定态数: %d\n",
                   node_idx, nb, neighbor->verdict_count);
        }

        /* 逐项比对,若不同则标记分形 */
        int min_vc = (self->verdict_count < neighbor->verdict_count)
                     ? self->verdict_count : neighbor->verdict_count;
        for (int v = 0; v < min_vc; v++) {
            if (self->verdict_state[v] != neighbor->verdict_state[v]) {
                self->verdict_state[v] = VERDICT_SPLIT;
                printf("  [PULL] #%d ← #%d 裁定项#%d 标记分形\n",
                       node_idx, nb, v);
            }
        }
        MJ.pull_count++;
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * PUSH推拉一轮
 * ============================================================ */
void njsg_mengji_gossip_pushpull_round(void) {
    pthread_mutex_lock(&MJ.lock);
    MJ.global_tick++;
    int tick = MJ.global_tick;
    int nc = MJ.node_count;
    printf("\n[盟机合] ═══ 第 %d 轮 Gossip PUSH推拉 ═══\n", tick);

    /* 先全部 PUSH */
    printf("[盟机合] -- PUSH 广播阶段 --\n");
    pthread_mutex_unlock(&MJ.lock);
    for (int i = 0; i < nc; i++) {
        njsg_mengji_gossip_push(i);
    }

    /* 再全部 PULL */
    pthread_mutex_lock(&MJ.lock);
    printf("[盟机合] -- PULL 拉取阶段 --\n");
    pthread_mutex_unlock(&MJ.lock);
    for (int i = 0; i < nc; i++) {
        njsg_mengji_gossip_pull(i);
    }
    fflush(stdout);
}

/* ============================================================
 * 三元见证裁定
 * ------------------------------------------------------------
 *  3 个节点投票:
 *    witness_a, witness_b = 普通见证者
 *    witness_c = 夫妻主权人锚点(权重 2x)
 *  达成共识: ≥3 票(含锚点权重则 ≥2 票即共识)
 * ============================================================ */
int njsg_mengji_consensus_vote(int subject_slot, mengji_verdict_t vote, int voter_idx) {
    pthread_mutex_lock(&MJ.lock);
    if (MJ.verdict_count >= MENGJI_MAX_VERDICTS) { pthread_mutex_unlock(&MJ.lock); return -1; }
    if (voter_idx < 0 || voter_idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return -1; }

    /* 查找是否已有该主题的裁定条目 */
    int vidx = -1;
    for (int i = 0; i < MJ.verdict_count; i++) {
        if (MJ.verdicts[i].subject[0] == '\0') continue;
        /* 按主题查找(这里简化: 直接用 slot 匹配) */
        if (MJ.verdicts[i].slot == subject_slot) { vidx = i; break; }
    }

    if (vidx == -1) {
        /* 新增裁定条目 */
        vidx = MJ.verdict_count++;
        mengji_verdict_entry_t* e = &MJ.verdicts[vidx];
        memset(e, 0, sizeof(*e));
        e->slot = subject_slot;
        snprintf(e->subject, 63, "裁定主题_%d", subject_slot);
        e->verdict = VERDICT_PENDING;
        e->witness_a = -1;
        e->witness_b = -1;
        e->witness_c = -1;
        e->consensus_reached = 0;
    }

    mengji_verdict_entry_t* e = &MJ.verdicts[vidx];
    mengji_node_t* voter = &MJ.nodes[voter_idx];

    /* 分配见证者槽位 */
    if (e->witness_a == -1 || e->witness_a == voter_idx) {
        e->witness_a = voter_idx;
    } else if (e->witness_b == -1 || e->witness_b == voter_idx) {
        e->witness_b = voter_idx;
    } else if (e->witness_c == -1 || e->witness_c == voter_idx) {
        e->witness_c = voter_idx;
    }

    /* 三元见证裁定(含夫妻主权人锚点权重 2x) */
    int vote_pass = 0, vote_reject = 0;
    int votes[3] = {e->witness_a, e->witness_b, e->witness_c};
    for (int i = 0; i < 3; i++) {
        if (votes[i] < 0 || votes[i] >= MJ.node_count) continue;
        mengji_node_t* w = &MJ.nodes[votes[i]];
        /* 夫妻主权人锚点 → 权重 2x */
        int weight = (w->anchor == MJ.primary_anchor) ? 2 : 1;
        if (w->verdict_state[subject_slot] == VERDICT_PASS)    vote_pass += weight;
        if (w->verdict_state[subject_slot] == VERDICT_REJECT)  vote_reject += weight;
    }

    /* 共识: ≥3 票为安全阈值(锚点2+普通1) */
    if (vote_pass >= 3) {
        e->verdict = VERDICT_PASS;
        e->consensus_reached = 1;
        MJ.consensus_count++;
    } else if (vote_reject >= 3) {
        e->verdict = VERDICT_REJECT;
        e->consensus_reached = 1;
        MJ.consensus_count++;
    } else {
        e->verdict = VERDICT_SPLIT;  /* 未达共识,保持分形 */
    }
    e->decided_at = time(NULL);

    printf("[见证] 主题#%d: #%d(%s) 投票 %s → 许可=%d 拒绝=%d 结论=%s %s\n",
           subject_slot, voter_idx, anchor_name(voter->anchor),
           verdict_name(vote), vote_pass, vote_reject,
           verdict_name(e->verdict),
           e->consensus_reached ? "★共识达成" : "待更多见证");
    fflush(stdout);

    pthread_mutex_unlock(&MJ.lock);
    return vidx;
}

/* ============================================================
 * 冲突裁决(基于夫妻主权人锚点)
 * ------------------------------------------------------------
 *  冲突 = 同一主题在不同节点有不同裁定态
 *  裁决:
 *    1. 有夫妻主权人锚点参与 → 取锚点裁定
 *    2. 同为合掌 → 取高信任
 *    3. 信任相同 → 取早 heartbeat(先入为主)
 *    4. 都相同 → 保守取 PENDING(等待见证)
 * ============================================================ */
int njsg_mengji_conflict_resolve(int subject_slot) {
    pthread_mutex_lock(&MJ.lock);

    int result = -1;
    mengji_verdict_t resolved = VERDICT_PENDING;

    /* 收集所有节点的裁定 */
    int anchor_vote = -1;     /* 夫妻主权人锚点的裁定 */
    float farm_max_trust = -1;
    time_t farm_min_time = (time_t)((unsigned long)(~0) >> 1);

    for (int i = 0; i < MJ.node_count; i++) {
        if (!MJ.nodes[i].alive) continue;
        if (subject_slot >= MJ.nodes[i].verdict_count) continue;

        mengji_node_t* n = &MJ.nodes[i];
        mengji_verdict_t v = n->verdict_state[subject_slot];

        if (n->anchor == MJ.primary_anchor) {
            anchor_vote = (int)v;
        }

        if (n->type == NODE_FARM) {
            if (n->trust > farm_max_trust) farm_max_trust = n->trust;
            if (n->last_heartbeat < farm_min_time) farm_min_time = n->last_heartbeat;
        }
    }

    /* 裁决层级 */
    if (anchor_vote >= 0) {
        resolved = (mengji_verdict_t)anchor_vote;
        result = 1;
        printf("[冲突裁决] 主题#%d: 取夫妻主权人锚点(%s)裁定=%s\n",
               subject_slot, anchor_name(MJ.primary_anchor), verdict_name(resolved));
    } else {
        /* 取合掌最高信任 */
        mengji_verdict_t best_verdict = VERDICT_PENDING;
        float best_trust = -1;
        time_t best_time = (time_t)((unsigned long)(~0) >> 1);

        for (int i = 0; i < MJ.node_count; i++) {
            if (!MJ.nodes[i].alive) continue;
            if (subject_slot >= MJ.nodes[i].verdict_count) continue;
            mengji_node_t* n = &MJ.nodes[i];
            if ((int)n->type != (int)NODE_PHONG) continue;
            /* 合掌才参与 */
            if (n->trust > best_trust || (n->trust == best_trust && n->last_heartbeat < best_time)) {
                best_trust = n->trust;
                best_time = n->last_heartbeat;
                best_verdict = n->verdict_state[subject_slot];
            }
        }

        if (best_trust >= 0) {
            resolved = best_verdict;
            result = 2;
            printf("[冲突裁决] 主题#%d: 取最高信任合掌(信任=%.0f%%)裁定=%s\n",
                   subject_slot, best_trust, verdict_name(resolved));
        } else {
            resolved = VERDICT_PENDING;
            result = 0;
            printf("[冲突裁决] 主题#%d: 无足够见证者,保守取待定\n", subject_slot);
        }
    }

    /* 将裁决结果广播到全部节点 */
    for (int i = 0; i < MJ.node_count; i++) {
        if (subject_slot < MJ.nodes[i].verdict_count) {
            MJ.nodes[i].verdict_state[subject_slot] = resolved;
        }
    }

    /* 更新裁定条目 */
    for (int i = 0; i < MJ.verdict_count; i++) {
        if (MJ.verdicts[i].slot == subject_slot) {
            MJ.verdicts[i].verdict = resolved;
            MJ.verdicts[i].consensus_reached = (result > 0) ? 1 : 0;
        }
    }
    MJ.conflict_count++;

    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return result;
}

/* ============================================================
 * 存活探活
 * ============================================================ */
void njsg_mengji_probe_alive(void) {
    pthread_mutex_lock(&MJ.lock);
    time_t now = time(NULL);
    for (int i = 0; i < MJ.node_count; i++) {
        if (difftime(now, MJ.nodes[i].last_heartbeat) > MENGJI_HEARTBEAT_TIMEOUT) {
            if (MJ.nodes[i].alive) {
                printf("[盟机合] 节点 #%d 心跳超时 → 标记失活\n", i);
            }
            MJ.nodes[i].alive = 0;
        }
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * 打印
 * ============================================================ */
void njsg_mengji_print_node(int idx) {
    pthread_mutex_lock(&MJ.lock);
    if (idx < 0 || idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return; }
    mengji_node_t* n = &MJ.nodes[idx];
    printf("┌─── 节点 #%d ───┐\n", n->id);
    printf("│ 标识:     %s\n", n->node_id);
    printf("│ 类型:     %s\n", node_type_name(n->type));
    printf("│ 锚点:     %s\n", anchor_name(n->anchor));
    printf("│ 信任:     %.0f%%\n", n->trust);
    printf("│ 存活:     %s\n", n->alive ? "存活" : "失活");
    printf("│ 邻居:     [");
    for (int i = 0; i < n->neighbor_count; i++) {
        if (i > 0) printf(",");
        printf("#%d", n->neighbor_ids[i]);
    }
    printf("]\n");
    printf("│ 裁定态:   ");
    for (int v = 0; v < n->verdict_count; v++) {
        if (v > 0) printf(",");
        printf("[%d]%s", v, verdict_name(n->verdict_state[v]));
    }
    printf("\n");
    printf("└────────────────────────────────────┘\n");
    pthread_mutex_unlock(&MJ.lock);
}

void njsg_mengji_print(void) {
    pthread_mutex_lock(&MJ.lock);
    printf("\n=== 盟机合 #全局概览 ===\n");
    printf("节点数:     %d\n", MJ.node_count);
    printf("裁定条目:   %d\n", MJ.verdict_count);
    printf("Gossip轮:   %ld\n", (long)MJ.global_tick);
    printf("PUSH次数:   %d\n", MJ.push_count);
    printf("PULL次数:   %d\n", MJ.pull_count);
    printf("共识达成:   %d\n", MJ.consensus_count);
    printf("冲突计数:   %d\n", MJ.conflict_count);
    printf("主锚点:     %s (信任=%.0f%%)\n", anchor_name(MJ.primary_anchor), MJ.primary_trust);
    printf("========================\n");
    for (int i = 0; i < MJ.node_count; i++) {
        njsg_mengji_print_node(i);
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * Demo: 三节点盟机合网络
 * ============================================================ */
void njsg_mengji_demo(void) {
    njsg_mengji_init();

    printf("\n━━━ 场景 M1: 节点注册 ━━━\n");

    /* 注册三个节点:
     *  #0 = 盟机合掌(运营场), 我=1 锚点
     *  #1 = 盟机场(萌机场), 刘楚恬=0 锚点
     *  #2 = 盟机场(萌机场), 我=1 锚点
     */
    int n0 = njsg_mengji_register_node("hezhang_root", NODE_PHONG, ANCHOR_LOLICT);
    int n1 = njsg_mengji_register_node("farm_node_A", NODE_FARM, ANCHOR_LIUTIANTIAN);
    int n2 = njsg_mengji_register_node("farm_node_B", NODE_FARM, ANCHOR_LOLICT);

    printf("\n━━━ 场景 M2: 邻居拓扑 ━━━\n");
    njsg_mengji_bind_neighbor(n0, n1);
    njsg_mengji_bind_neighbor(n1, n2);
    njsg_mengji_bind_neighbor(n2, n0);

    printf("\n━━━ 场景 M3: 设定裁定态(制造分歧) ━━━\n");
    /* 主题 #0: 外部请求准入裁定 */
    pthread_mutex_lock(&MJ.lock);
    /* 合掌节点: 许可 */
    MJ.nodes[0].verdict_count = 1;
    MJ.nodes[0].verdict_state[0] = VERDICT_PASS;
    /* 机场 A: 拒绝 */
    MJ.nodes[1].verdict_count = 1;
    MJ.nodes[1].verdict_state[0] = VERDICT_REJECT;
    /* 机场 B: 许可 */
    MJ.nodes[2].verdict_count = 1;
    MJ.nodes[2].verdict_state[0] = VERDICT_PASS;
    pthread_mutex_unlock(&MJ.lock);
    printf("初始裁决: #0(合掌)=许可 #1(机场A)=拒绝 #2(机场B)=许可 → 分歧制造\n");

    printf("\n━━━ 场景 M4: Gossip PUSH推拉 ━━━\n");
    njsg_mengji_gossip_pushpull_round();

    printf("\n━━━ 场景 M5: 冲突裁决 ━━━\n");
    njsg_mengji_conflict_resolve(0);

    printf("\n━━━ 场景 M6: 三元见证裁定 ━━━\n");
    njsg_mengji_consensus_vote(0, VERDICT_PASS, 0);
    njsg_mengji_consensus_vote(0, VERDICT_REJECT, 1);
    njsg_mengji_consensus_vote(0, VERDICT_PASS, 2);

    printf("\n━━━ 场景 M7: 心跳探活 ━━━\n");
    njsg_mengji_heartbeat(0);
    njsg_mengji_heartbeat(1);
    /* 节点 #2 故意不发送心跳 */
    printf("[盟机合] 节点 #2 跳过心跳(模拟失活)\n");
    njsg_mengji_probe_alive();

    printf("\n━━━ 场景 M8: 全局概览 ━━━\n");
    njsg_mengji_print();

    printf("\n[盟机合] 演示完毕。\n");
    printf("[盟机合] Gossip 节律 + 夫妻主权人锚点 = 去中心化裁定一致性\n\n");
}
