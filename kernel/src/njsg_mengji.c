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
    MJ.bag_count       = 0;
    MJ.bag_log_count   = 0;
    MJ.bag_total_weight = 0;

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
 * 里程碑二 · 分布式三元见证 —— 第三方背书
 * ------------------------------------------------------------
 *  当某盟机场(见证者)对某个袋子编号背书时调用:
 *    - 记录见证者身份
 *    - 检查摘要一致性
 *    - 推进三元见证状态流转:
 *        EMPTY → SINGLE → DOUBLE → WITNESSED / REJECTED
 *
 *  返回: 三元见证状态码(见 mengji_triune_state_t)
 * ============================================================ */
int njsg_mengji_triune_register(int farm_id,
                                 int subject_slot,
                                 const char* bag_hash) {
    pthread_mutex_lock(&MJ.lock);

    if (farm_id < 0 || farm_id >= MJ.node_count) {
        pthread_mutex_unlock(&MJ.lock);
        return -1;
    }
    if (!bag_hash || strlen(bag_hash) != 16) {
        pthread_mutex_unlock(&MJ.lock);
        return -1;
    }

    /* 查找该槽位的见证条目 */
    int tidx = -1;
    for (int i = 0; i < MJ.triune_count; i++) {
        if (MJ.triunes[i].slot == subject_slot) { tidx = i; break; }
    }
    if (tidx == -1) {
        if (MJ.triune_count >= MENGJI_MAX_VERDICTS) {
            pthread_mutex_unlock(&MJ.lock);
            return -1;
        }
        tidx = MJ.triune_count++;
        mengji_triune_entry_t* ne = &MJ.triunes[tidx];
        memset(ne, 0, sizeof(*ne));
        ne->slot = subject_slot;
        snprintf(ne->subject, 63, "袋子_%d", subject_slot);
        ne->bag_hash[0] = '\0';
        ne->triune_state = TRIUNE_EMPTY;
        ne->witness_farm_1 = -1;
        ne->witness_farm_2 = -1;
        ne->witness_3rd = -1;
    }

    mengji_triune_entry_t* e = &MJ.triunes[tidx];
    mengji_node_t* farmer = &MJ.nodes[farm_id];

    /* —— 状态机流转 —— */
    switch (e->triune_state) {
    case TRIUNE_EMPTY:
        /* 首次持有此袋子 */
        e->witness_farm_1 = farm_id;
        strncpy(e->bag_hash, bag_hash, 16);
        e->bag_hash[16] = '\0';
        e->triune_state = TRIUNE_SINGLE;
        printf("[三元见证] 槽#%d: 盟机场#%d(%s)首次持有摘要=%s → SINGLE\n",
               subject_slot, farm_id, anchor_name(farmer->anchor), bag_hash);
        break;

    case TRIUNE_SINGLE:
        /* 第二个盟机场出现 */
        if (farm_id == e->witness_farm_1) {
            /* 同一场再次出现,不计 */
            printf("[三元见证] 槽#%d: 盟机场#%d重复,状态保持\n", subject_slot, farm_id);
        } else if (strcmp(e->bag_hash, bag_hash) == 0) {
            /* 摘要一致 → DOUBLE */
            e->witness_farm_2 = farm_id;
            e->triune_state = TRIUNE_DOUBLE;
            printf("[三元见证] 槽#%d: 盟机场#%d(%s)摘要一致(%s) → DOUBLE\n",
                   subject_slot, farm_id, anchor_name(farmer->anchor), bag_hash);
        } else {
            /* 摘要冲突 → REJECTED */
            e->triune_state = TRIUNE_REJECTED;
            printf("[三元见证] 槽#%d: 盟机场#%d摘要不一致(%s vs %s) → REJECTED\n",
                   subject_slot, farm_id, e->bag_hash, bag_hash);
        }
        break;

    case TRIUNE_DOUBLE:
        /* 已双场一致,等待第三方背书 */
        if (farm_id != e->witness_farm_1 && farm_id != e->witness_3rd) {
            if (strcmp(e->bag_hash, bag_hash) == 0) {
                e->witness_3rd = farm_id;
                e->triune_state = TRIUNE_WITNESSED;
                printf("[三元见证] 槽#%d: 第三场#%d(%s)背书成功(%s) → WITNESSED\n",
                       subject_slot, farm_id, anchor_name(farmer->anchor), bag_hash);
            } else {
                e->triune_state = TRIUNE_REJECTED;
                printf("[三元见证] 槽#%d: 第三场#%d摘要不一致(%s vs %s) → REJECTED\n",
                       subject_slot, farm_id, e->bag_hash, bag_hash);
            }
        } else {
            printf("[三元见证] 槽#%d: 盟机场#%d 已参与过,忽略\n", subject_slot, farm_id);
        }
        break;

    default:
        printf("[三元见证] 槽#%d: 已处于终态(%d),不再更新\n", subject_slot, e->triune_state);
        break;
    }

    fflush(stdout);
    pthread_mutex_unlock(&MJ.lock);
    return (int)e->triune_state;
}

/* ============================================================
 * 里程碑二 · 分布式三元见证 —— 三条件验证
 * ------------------------------------------------------------
 *  验证三元见证三条件全部满足:
 *    条件一(非空): 袋子在至少两个不同盟机场出现且摘要一致
 *    条件二(已见证): 存在第三个盟机场背书
 *    条件三(锚点一致): 参与各方在信任链相容范围内、锚点无泄露
 *
 *  返回: 0 = 全部通过 非零 = 失败码
 *    1 = 条件一失败(未达双场一致)
 *    2 = 条件二失败(未获第三方背书)
 *    3 = 条件三失败(信任链相交不达标)
 * ============================================================ */
int njsg_mengji_triune_verify(int subject_slot) {
    pthread_mutex_lock(&MJ.lock);

    int tidx = -1;
    for (int i = 0; i < MJ.triune_count; i++) {
        if (MJ.triunes[i].slot == subject_slot) { tidx = i; break; }
    }
    if (tidx == -1) {
        printf("[三元验证] 槽#%d: 无见证记录 → 条件一失败\n", subject_slot);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }

    mengji_triune_entry_t* e = &MJ.triunes[tidx];
    const char* triune_state_name[] = {"空","单","双","已见证","已拒绝"};

    /* —— 条件一: 非空(双场且摘要一致) —— */
    if (e->triune_state == TRIUNE_EMPTY) {
        printf("[三元验证] 槽#%d: 状态=%s → 条件一失败(空)\n",
               subject_slot, triune_state_name[e->triune_state]);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }
    if (e->triune_state == TRIUNE_SINGLE) {
        printf("[三元验证] 槽#%d: 状态=%s → 条件一失败(仅单场)\n",
               subject_slot, triune_state_name[e->triune_state]);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }
    if (e->triune_state == TRIUNE_REJECTED) {
        printf("[三元验证] 槽#%d: 状态=%s → 条件一失败(已拒绝)\n",
               subject_slot, triune_state_name[e->triune_state]);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }
    printf("[三元验证] 槽#%d: 条件一通过 — 双场摘要一致(%s)\n", subject_slot, e->bag_hash);

    /* —— 条件二: 已见证(第三方背书) —— */
    if (e->triune_state != TRIUNE_WITNESSED) {
        printf("[三元验证] 槽#%d: 状态=%s → 条件二失败(无第三方背书)\n",
               subject_slot, triune_state_name[e->triune_state]);
        pthread_mutex_unlock(&MJ.lock);
        return 2;
    }
    if (e->witness_3rd < 0 || e->witness_3rd >= MJ.node_count) {
        printf("[三元验证] 槽#%d: 条件二失败(第三方节点无效)\n", subject_slot);
        pthread_mutex_unlock(&MJ.lock);
        return 2;
    }
    printf("[三元验证] 槽#%d: 条件二通过 — 第三场#%d已背书\n", subject_slot, e->witness_3rd);

    /* —— 条件三: 锚点一致(信任链相容) ——
     *  计算三方信任链相交(取最低信任为交集)
     *  若交集 ≥ 信任下限容差(30%) 且锚点无冲突 → 通过 */
    {
        float t1 = -1, t2 = -1, t3 = -1;
        int wid[3] = {e->witness_farm_1, e->witness_farm_2, e->witness_3rd};
        mengji_anchor_t anchors[3];
        for (int k = 0; k < 3; k++) {
            if (wid[k] >= 0 && wid[k] < MJ.node_count) {
                float trust_val = MJ.nodes[wid[k]].trust;
                anchors[k] = MJ.nodes[wid[k]].anchor;
                if (k == 0) t1 = trust_val;
                else if (k == 1) t2 = trust_val;
                else t3 = trust_val;
            }
        }
        /* 信任相交 = 取三方最低信任 */
        float trust_intersect = t1;
        if (t2 >= 0 && t2 < trust_intersect) trust_intersect = t2;
        if (t3 >= 0 && t3 < trust_intersect) trust_intersect = t3;

        /* 锚点相容: 至少有两方锚点一致(兼容双主体共存) */
        int anchor_agree = (anchors[0] == anchors[1]) ||
                           (anchors[1] == anchors[2]) ||
                           (anchors[0] == anchors[2]);

        printf("[三元验证] 槽#%d: 信任相交=%.0f%% 锚点相容=%s\n",
               subject_slot, trust_intersect, anchor_agree ? "是" : "否");

        if (trust_intersect < 30.0f) {
            printf("[三元验证] 槽#%d: 条件三失败 — 信任相交不足30%%\n", subject_slot);
            pthread_mutex_unlock(&MJ.lock);
            return 3;
        }
        if (!anchor_agree) {
            printf("[三元验证] 槽#%d: 条件三失败 — 锚点全不一致\n", subject_slot);
            pthread_mutex_unlock(&MJ.lock);
            return 3;
        }
    }

    printf("[三元验证] 槽#%d: ★★★ 三条件全部通过 ★★★\n", subject_slot);
    fflush(stdout);
    pthread_mutex_unlock(&MJ.lock);
    return 0;
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
 * 里程碑三 · 跨袋子合并拆分 (v1.6 袋子整并协议)
 * ------------------------------------------------------------
 *  仅加法(合并)与减法(拆分)，无乘除，袋子级整袋跃迁。
 * ============================================================ */

/* 按袋号查找袋子索引;未找到返回 -1 */
static int find_bag_idx(int bag_id) {
    for (int i = 0; i < MJ.bag_count; i++) {
        if (MJ.bags[i].bag_id == bag_id) return i;
    }
    return -1;
}

/* 组合见证哈希:用 FNV-1a 对两个摘要拼接结果做散列,产出 16 位十六进制 */
static void combine_witness_hash(const char* ha, const char* hb, char* out) {
    uint32_t h1 = 0x811c9dc5u, h2 = 0x0100193u;
    char buf[40];
    snprintf(buf, sizeof(buf), "%s%s", ha, hb);
    for (size_t i = 0; i < strlen(buf); i++) {
        h1 ^= (uint8_t)buf[i]; h1 *= 0x01000193u;
        h2 ^= ((uint8_t)buf[i]) << 3; h2 *= 0x1000193u;
    }
    snprintf(out, 17, "%08x%08x", h1, h2);
}

/* ---- 加法合并 ---- */
int njsg_mengji_bag_merge(int bag_a, int bag_b, const char* merge_note) {
    pthread_mutex_lock(&MJ.lock);
    (void)merge_note;  /* 标记入口,日志已在外层打印 */

    int ia = find_bag_idx(bag_a);
    int ib = find_bag_idx(bag_b);
    if (ia < 0 || ib < 0) {
        printf("[袋子合并] 袋号未找到: a=%d(idx=%d) b=%d(idx=%d)\n", bag_a, ia, bag_b, ib);
        pthread_mutex_unlock(&MJ.lock);
        return -1;
    }
    if (MJ.bag_count >= MENGJI_MAX_BAGS) {
        printf("[袋子合并] 袋子容量已满 %d\n", MENGJI_MAX_BAGS);
        pthread_mutex_unlock(&MJ.lock);
        return -2;
    }

    int new_weight = MJ.bags[ia].weight + MJ.bags[ib].weight;
    if (new_weight < MENGJI_BAG_WEIGHT_MIN || new_weight > MENGJI_BAG_WEIGHT_MAX) {
        printf("[袋子合并] 权值 %d 超限[%d,%d]\n", new_weight, MENGJI_BAG_WEIGHT_MIN, MENGJI_BAG_WEIGHT_MAX);
        pthread_mutex_unlock(&MJ.lock);
        return -3;
    }

    /* 新增合并袋子 */
    int ic = MJ.bag_count++;
    mengji_bag_entry_t* c = &MJ.bags[ic];
    memset(c, 0, sizeof(*c));
    /* 袋号 = 两个源袋号拼接哈希摘要(简单递增编号) */
    c->bag_id = (MJ.bags[ia].bag_id * 1000 + MJ.bags[ib].bag_id);
    c->weight = new_weight;
    combine_witness_hash(MJ.bags[ia].witness_hash, MJ.bags[ib].witness_hash, c->witness_hash);
    c->parent_id = -1;
    c->source_a  = bag_a;
    c->source_b  = bag_b;
    c->triune_slot = -1;
    c->created_at  = time(NULL);
    c->anchor      = MJ.primary_anchor;

    /* 加法合并: 源袋子权值归入新袋子(源袋权值归零=消耗) */
    MJ.bags[ia].weight = 0;
    MJ.bags[ib].weight = 0;
    /* 总权值守恒: 新袋权值 = 原两袋之和, 不新增 */
    /* bag_total_weight 不变(权值从源袋转移到了新袋) */

    printf("[袋子合并] ★ 袋#%d + 袋#%d → 袋#%d(w=%d,见证=%s) 总权值=%d(守恒)\n",
           bag_a, bag_b, c->bag_id, new_weight, c->witness_hash, MJ.bag_total_weight);

    /* 产出增量日志 */
    njsg_mengji_bag_incremental_log(MJ_BAG_MERGE, bag_a, bag_b, c->bag_id,
                                     new_weight, c->witness_hash, -1);

    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return c->bag_id;
}

/* ---- 减法拆分 ---- */
int njsg_mengji_bag_split(int parent_bag, int sub_weight, const char* split_note) {
    pthread_mutex_lock(&MJ.lock);
    (void)split_note;  /* 标记入口,日志已在外层打印 */

    int ip = find_bag_idx(parent_bag);
    if (ip < 0) {
        printf("[袋子拆分] 父袋号未找到: %d\n", parent_bag);
        pthread_mutex_unlock(&MJ.lock);
        return -1;
    }
    if (MJ.bag_count >= MENGJI_MAX_BAGS) {
        printf("[袋子拆分] 袋子容量已满 %d\n", MENGJI_MAX_BAGS);
        pthread_mutex_unlock(&MJ.lock);
        return -2;
    }
    if (sub_weight < MENGJI_BAG_WEIGHT_MIN || sub_weight > MENGJI_BAG_WEIGHT_MAX) {
        printf("[袋子拆分] 请求权值 %d 超限[%d,%d]\n", sub_weight, MENGJI_BAG_WEIGHT_MIN, MENGJI_BAG_WEIGHT_MAX);
        pthread_mutex_unlock(&MJ.lock);
        return -3;
    }
    if (MJ.bags[ip].weight < sub_weight) {
        printf("[袋子拆分] 父袋#%d 权值不足(%d<%d)\n", parent_bag, MJ.bags[ip].weight, sub_weight);
        pthread_mutex_unlock(&MJ.lock);
        return -4;
    }

    /* 创建子袋 */
    int ic = MJ.bag_count++;
    mengji_bag_entry_t* c = &MJ.bags[ic];
    memset(c, 0, sizeof(*c));
    c->bag_id     = parent_bag * 100 + 1;  /* 子袋号 = 父袋号 × 100 + 1 */
    c->weight     = sub_weight;
    /* 子袋承接父袋的见证摘要 */
    strncpy(c->witness_hash, MJ.bags[ip].witness_hash, 16);
    c->witness_hash[16] = '\0';
    c->parent_id  = parent_bag;
    c->source_a   = -1;
    c->source_b   = -1;
    c->triune_slot = -1;
    c->created_at  = time(NULL);
    c->anchor      = MJ.bags[ip].anchor;   /* 继承父袋锚点 */

    /* 父袋权值减少(减法守恒) */
    MJ.bags[ip].weight -= sub_weight;

    /* 总权值不变(守恒) */

    printf("[袋子拆分] ■ 父袋#%d(w 余%d) - 子袋#%d(w=%d,继承见证=%s) 总权值=%d(守恒)\n",
           parent_bag, MJ.bags[ip].weight, c->bag_id, sub_weight,
           c->witness_hash, MJ.bag_total_weight);

    /* 产出增量日志 */
    njsg_mengji_bag_incremental_log(MJ_BAG_SPLIT, parent_bag, -1, c->bag_id,
                                     -sub_weight, c->witness_hash, -1);

    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return c->bag_id;
}

/* ---- 增量日志条目 ----
 *  注意: 此函数假设调用者已持有 MJ.lock(从 merge/split 调用),
 *        内部不再重复加锁, 避免非递归 mutex 死锁.
 * ============================================================ */
int njsg_mengji_bag_incremental_log(mengji_bag_op_t op,
                                      int source, int auxiliary, int result,
                                      int weight_delta, const char* result_hash,
                                      int witness_node) {
    static const char* op_names[] = {"合并","拆分","日志","验证"};
    const char* op_str = (op <= MJ_BAG_VALIDATE) ? op_names[op] : "?";

    if (MJ.bag_log_count >= MENGJI_MAX_BAG_LOGS) {
        fprintf(stderr, "[增量日志] 日志已满 %d\n", MENGJI_MAX_BAG_LOGS);
        return -1;
    }

    int il = MJ.bag_log_count++;
    mengji_bag_log_entry_t* l = &MJ.bag_logs[il];
    memset(l, 0, sizeof(*l));
    l->log_id        = il;
    l->op            = op;
    l->source_bag    = source;
    l->auxiliary_bag = auxiliary;
    l->result_bag    = result;
    l->weight_delta  = weight_delta;
    strncpy(l->result_hash, result_hash ? result_hash : "0000000000000000", 16);
    l->result_hash[16] = '\0';
    l->witness_node  = witness_node;
    l->timestamp     = time(NULL);

    printf("[增量日志#%d] 操作=%s 源袋#%d 辅助袋#%d 结果袋#%d 权值变化=%+d 见证=%s\n",
           il, op_str, source, auxiliary, result, weight_delta, l->result_hash);

    return il;
}

/* ---- 验证单个袋子合规 ----
 *  返回: 0=合规  1=袋号无效  2=权值超限  3=见证摘要格式错误
 *        4=父袋链接断裂
 * ============================================================ */
int njsg_mengji_bag_validate(int bag_id) {
    pthread_mutex_lock(&MJ.lock);

    int idx = find_bag_idx(bag_id);
    if (idx < 0) {
        printf("[袋子验证] 袋#%d: 袋号无效\n", bag_id);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }

    mengji_bag_entry_t* b = &MJ.bags[idx];

    /* 袋号合规 */
    if (b->bag_id < 0) {
        printf("[袋子验证] 袋#%d: 袋号为负\n", bag_id);
        pthread_mutex_unlock(&MJ.lock);
        return 1;
    }

    /* 权值守恒(上下限内) */
    if (b->weight < MENGJI_BAG_WEIGHT_MIN || b->weight > MENGJI_BAG_WEIGHT_MAX) {
        printf("[袋子验证] 袋#%d: 权值 %d 超限[%d,%d]\n",
               bag_id, b->weight, MENGJI_BAG_WEIGHT_MIN, MENGJI_BAG_WEIGHT_MAX);
        pthread_mutex_unlock(&MJ.lock);
        return 2;
    }

    /* 见证摘要格式: 必须为非空 16 位十六进制 */
    if (strlen(b->witness_hash) != 16) {
        printf("[袋子验证] 袋#%d: 见证摘要长度异常(%zu)\n", bag_id, strlen(b->witness_hash));
        pthread_mutex_unlock(&MJ.lock);
        return 3;
    }
    for (int i = 0; i < 16; i++) {
        char ch = b->witness_hash[i];
        if (!((ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F'))) {
            printf("[袋子验证] 袋#%d: 见证摘要含非法字符 '%c'\n", bag_id, ch);
            pthread_mutex_unlock(&MJ.lock);
            return 3;
        }
    }

    /* 见证链链接: 若有父袋,父袋必须存在 */
    if (b->parent_id >= 0) {
        int pidx = find_bag_idx(b->parent_id);
        if (pidx < 0) {
            printf("[袋子验证] 袋#%d: 父袋#%d 不存在,链断裂\n", bag_id, b->parent_id);
            pthread_mutex_unlock(&MJ.lock);
            return 4;
        }
        printf("[袋子验证] 袋#%d: 父袋#%d 存在,见证链完整\n", bag_id, b->parent_id);
    }

    printf("[袋子验证] 袋#%d: ★★★ 合规(w=%d,见证=%s) ★★★\n",
           bag_id, b->weight, b->witness_hash);
    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return 0;
}

/* ---- 验证袋子全集守恒 ----
 *  检查: Σ(所有袋子权值) == bag_total_weight
 *  返回: 0=守恒  1=总权值不匹配  2=子袋权值和超过父袋原始值
 * ============================================================ */
int njsg_mengji_bag_validate_all(void) {
    pthread_mutex_lock(&MJ.lock);

    int sum = 0;
    for (int i = 0; i < MJ.bag_count; i++) {
        sum += MJ.bags[i].weight;
    }

    printf("[全集验证] 袋子总数=%d 各袋权值累加=%d 记录总权值=%d\n",
           MJ.bag_count, sum, MJ.bag_total_weight);

    if (sum != MJ.bag_total_weight) {
        printf("[全集验证] ★ 权值不匹配(累加%d ≠ 记录%d) ★\n", sum, MJ.bag_total_weight);
        pthread_mutex_unlock(&MJ.lock);
        fflush(stdout);
        return 1;
    }

    printf("[全集验证] ★★★ 权值守恒通过 ★★★\n");
    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return 0;
}

/* ---- 打印单个袋子 ---- */
void njsg_mengji_print_bag(int bag_id) {
    pthread_mutex_lock(&MJ.lock);
    int idx = find_bag_idx(bag_id);
    if (idx < 0) {
        printf("袋#%d 不存在\n", bag_id);
        pthread_mutex_unlock(&MJ.lock);
        return;
    }
    mengji_bag_entry_t* b = &MJ.bags[idx];
    printf("┌─── 袋#%d ───┐\n", b->bag_id);
    printf("│ 权值:     %d\n", b->weight);
    printf("│ 见证摘要: %s\n", b->witness_hash);
    printf("│ 父袋:     %s\n", (b->parent_id >= 0) ? "#" : "无(根袋)");
    if (b->parent_id >= 0) printf("│           #%d\n", b->parent_id);
    if (b->source_a >= 0)
        printf("│ 合并源:   #%d + #%d\n", b->source_a, b->source_b);
    printf("│ 锚点:     %s\n", anchor_name(b->anchor));
    printf("└─────────────────────────┘\n");
    pthread_mutex_unlock(&MJ.lock);
}

/* ---- 打印增量日志条目 ---- */
void njsg_mengji_print_bag_log(int log_id) {
    pthread_mutex_lock(&MJ.lock);
    if (log_id < 0 || log_id >= MJ.bag_log_count) {
        printf("日志#%d 不存在\n", log_id);
        pthread_mutex_unlock(&MJ.lock);
        return;
    }
    mengji_bag_log_entry_t* l = &MJ.bag_logs[log_id];
    static const char* op_names[] = {"合并","拆分","日志","验证"};
    const char* op_str = (l->op <= MJ_BAG_VALIDATE) ? op_names[l->op] : "?";
    printf("┌─── 增量日志#%d ───┐\n", l->log_id);
    printf("│ 操作:     %s\n", op_str);
    printf("│ 源袋:     #%d\n", l->source_bag);
    printf("│ 辅助袋:   #%d\n", l->auxiliary_bag);
    printf("│ 结果袋:   #%d\n", l->result_bag);
    printf("│ 权值变化: %+d\n", l->weight_delta);
    printf("│ 结果见证: %s\n", l->result_hash);
    printf("└──────────────────────────────┘\n");
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * 打印
 * ============================================================ */
/* 三元见证状态名 */
static const char* triune_state_name(mengji_triune_state_t s) {
    switch (s) {
        case TRIUNE_EMPTY:     return "空";
        case TRIUNE_SINGLE:    return "单";
        case TRIUNE_DOUBLE:    return "双";
        case TRIUNE_WITNESSED: return "已见证";
        case TRIUNE_REJECTED:  return "已拒绝";
    }
    return "?";
}

/* 内部版:调用者已持锁,不再重复上锁 */
static void mengji_print_node_unlocked(int idx) {
    if (idx < 0 || idx >= MJ.node_count) return;
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
}

void njsg_mengji_print_node(int idx) {
    pthread_mutex_lock(&MJ.lock);
    mengji_print_node_unlocked(idx);
    pthread_mutex_unlock(&MJ.lock);
}

void njsg_mengji_print(void) {
    pthread_mutex_lock(&MJ.lock);
    printf("\n=== 盟机合 #全局概览 ===\n");
    printf("节点数:     %d\n", MJ.node_count);
    printf("裁定条目:   %d\n", MJ.verdict_count);
    printf("三元见证:   %d\n", MJ.triune_count);
    printf("Gossip轮:   %ld\n", (long)MJ.global_tick);
    printf("PUSH次数:   %d\n", MJ.push_count);
    printf("PULL次数:   %d\n", MJ.pull_count);
    printf("共识达成:   %d\n", MJ.consensus_count);
    printf("冲突计数:   %d\n", MJ.conflict_count);
    printf("袋子数:     %d (总权值=%d)\n", MJ.bag_count, MJ.bag_total_weight);
    printf("增量日志:   %d\n", MJ.bag_log_count);
    printf("主锚点:     %s (信任=%.0f%%)\n", anchor_name(MJ.primary_anchor), MJ.primary_trust);

    /* 打印三元见证详情 */
    if (MJ.triune_count > 0) {
        printf("\n--- 三元见证条目 ---\n");
        for (int i = 0; i < MJ.triune_count; i++) {
            mengji_triune_entry_t* t = &MJ.triunes[i];
            printf("  [槽#%d] 袋子摘要=%s 状态=%s 场1=#%d 场2=#%d 见证=#%d\n",
                   t->slot, t->bag_hash, triune_state_name(t->triune_state),
                   t->witness_farm_1, t->witness_farm_2, t->witness_3rd);
        }
    }

    /* 打印袋子详情 */
    if (MJ.bag_count > 0) {
        printf("\n--- 袋子集合 ---\n");
        for (int i = 0; i < MJ.bag_count; i++) {
            mengji_bag_entry_t* b = &MJ.bags[i];
            printf("  [袋#%d] w=%d 见证=%s 父=%s",
                   b->bag_id, b->weight, b->witness_hash,
                   (b->parent_id >= 0) ? "#" : "根");
            if (b->parent_id >= 0) printf("%d", b->parent_id);
            if (b->source_a >= 0) printf(" 合并(%d+%d)", b->source_a, b->source_b);
            printf("\n");
        }
    }

    /* 打印增量日志 */
    if (MJ.bag_log_count > 0) {
        printf("\n--- 增量日志 ---\n");
        static const char* op_names[] = {"合并","拆分","日志","验证"};
        for (int i = 0; i < MJ.bag_log_count; i++) {
            mengji_bag_log_entry_t* l = &MJ.bag_logs[i];
            const char* op_str = (l->op <= MJ_BAG_VALIDATE) ? op_names[l->op] : "?";
            printf("  [日志#%d] %s 源#%d 辅#%d 结果#%d Δ=%+d %s\n",
                   i, op_str, l->source_bag, l->auxiliary_bag,
                   l->result_bag, l->weight_delta, l->result_hash);
        }
    }

    printf("========================\n");
    for (int i = 0; i < MJ.node_count; i++) {
        mengji_print_node_unlocked(i);
    }
    pthread_mutex_unlock(&MJ.lock);
}

/* ============================================================
 * 里程碑一 · 种子场握手自动发现邻居
 * ------------------------------------------------------------
 *  新节点(node_idx)向种子场(seed_idx)发起握手,
 *  种子应答其邻居列表 → 新节点自动绑定到未知邻居.
 *
 *  返回: 新发现的邻居数(0=无新发现 -1=参数错误)
 * ============================================================ */
int njsg_mengji_discover_from_seed(int node_idx, int seed_idx) {
    pthread_mutex_lock(&MJ.lock);

    if (node_idx < 0 || node_idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return -1; }
    if (seed_idx < 0 || seed_idx >= MJ.node_count) { pthread_mutex_unlock(&MJ.lock); return -1; }
    if (node_idx == seed_idx) { pthread_mutex_unlock(&MJ.lock); return -1; }

    mengji_node_t* self = &MJ.nodes[node_idx];
    mengji_node_t* seed = &MJ.nodes[seed_idx];
    int discovered = 0;

    printf("\n[种子发现] 节点#%d 向种子场#%d 发起 handshake\n", node_idx, seed_idx);
    printf("           种子场应答邻居: [");
    for (int i = 0; i < seed->neighbor_count; i++) {
        if (i > 0) printf(",");
        printf("#%d", seed->neighbor_ids[i]);
    }
    printf("]\n");

    /* Seed 应答其邻居列表 → 新节点绑定到未知邻居(双向) */
    for (int i = 0; i < seed->neighbor_count; i++) {
        int peer = seed->neighbor_ids[i];
        if (peer == node_idx || peer < 0 || peer >= MJ.node_count) continue;

        /* 已经是邻居? */
        int already = 0;
        for (int k = 0; k < self->neighbor_count; k++) {
            if (self->neighbor_ids[k] == peer) { already = 1; break; }
        }

        if (!already && self->neighbor_count < MENGJI_MAX_NEIGHBORS) {
            self->neighbor_ids[self->neighbor_count++] = peer;
            mengji_node_t* peer_node = &MJ.nodes[peer];
            /* 反向绑定 */
            int peer_has = 0;
            for (int k = 0; k < peer_node->neighbor_count; k++) {
                if (peer_node->neighbor_ids[k] == node_idx) { peer_has = 1; break; }
            }
            if (!peer_has && peer_node->neighbor_count < MENGJI_MAX_NEIGHBORS) {
                peer_node->neighbor_ids[peer_node->neighbor_count++] = node_idx;
            }
            discovered++;
            printf("           发现邻居#%d,双向绑定完成\n", peer);
        }
    }

    printf("[种子发现] 共发现 %d 个新邻居\n", discovered);
    pthread_mutex_unlock(&MJ.lock);
    fflush(stdout);
    return discovered;
}

/* ============================================================
 * 里程碑一 · 离线恢复完整测试 (场景 M15)
 * ------------------------------------------------------------
 *  独立验证标准 (设计文档):
 *  部署至少三处节点,模拟一处离线后重启,
 *  确认其能通过 Pull 阶段补全离线期间错失的全部裁决袋,
 *  且无重复存储、无漏项.
 * ============================================================ */
void njsg_mengji_scenario_M15_offline_recovery(void) {
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  里程碑一 · 邻居发现 + 离线 Pull 恢复测试       ║\n");
    printf("║  独立验证: 三节点 / 一处离线 / Pull 补全无遗漏  ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    /* ── 阶段 1: 注册三节点 ── */
    printf("\n━━━ [阶段 1] 注册三节点 ━━━\n");
    int a = njsg_mengji_register_node("M15_node_A_PHONG", NODE_PHONG, ANCHOR_LOLICT);
    int b = njsg_mengji_register_node("M15_node_B_FARM",  NODE_FARM, ANCHOR_LIUTIANTIAN);
    int c = njsg_mengji_register_node("M15_node_C_FARM",  NODE_FARM, ANCHOR_LOLICT);

    /* 拓扑: A <-> B <-> C 链式(A 为种子场) */
    njsg_mengji_bind_neighbor(a, b);
    njsg_mengji_bind_neighbor(b, c);
    /* C 只知 B,未知 A — 需要通过种子场发现 A */

    /* ── 阶段 2: 种子场发现 — C 通过 B 发现 A ── */
    printf("\n━━━ [阶段 2] C 通过种子场 B 发现邻居 A ━━━\n");
    int discovered = njsg_mengji_discover_from_seed(c, b);
    /* 验证: C 应该发现 A */
    int c_knows_a = 0;
    pthread_mutex_lock(&MJ.lock);
    for (int i = 0; i < MJ.nodes[c].neighbor_count; i++) {
        if (MJ.nodes[c].neighbor_ids[i] == a) { c_knows_a = 1; break; }
    }
    pthread_mutex_unlock(&MJ.lock);
    printf("[阶段 2] C 已知 A: %s\n", c_knows_a ? "是" : "否");

    /* ── 阶段 3: A 和 B 产出裁定(C 尚未参与) ── */
    printf("\n━━━ [阶段 3] A 和 B 产出裁定(C 尚未参与) ━━━\n");
    pthread_mutex_lock(&MJ.lock);
    /* A 产出裁定 0(许可) 和 1(拒绝) */
    MJ.nodes[a].verdict_count = 2;
    MJ.nodes[a].verdict_state[0] = VERDICT_PASS;
    MJ.nodes[a].verdict_state[1] = VERDICT_REJECT;
    /* B 同步 A 的裁定(PushPull) */
    MJ.nodes[b].verdict_count = 2;
    MJ.nodes[b].verdict_state[0] = VERDICT_PASS;
    MJ.nodes[b].verdict_state[1] = VERDICT_REJECT;
    /* C 有 0 裁定(尚未参与) */
    MJ.nodes[c].verdict_count = 0;
    pthread_mutex_unlock(&MJ.lock);
    printf("[阶段 3] A 裁定态: [0]许可 [1]拒绝\n");
    printf("[阶段 3] B 裁定态: [0]许可 [1]拒绝 (已同步)\n");
    printf("[阶段 3] C 裁定态: (空,未参与)\n");

    /* ── 阶段 4: 模拟 C 离线 ── */
    printf("\n━━━ [阶段 4] C 进入离线 30 秒(心跳超时) ━━━\n");
    pthread_mutex_lock(&MJ.lock);
    MJ.nodes[c].alive = 0;
    MJ.nodes[c].last_heartbeat = time(NULL) - MENGJI_HEARTBEAT_TIMEOUT - 1;
    pthread_mutex_unlock(&MJ.lock);
    njsg_mengji_probe_alive();
    printf("[阶段 4] C 离线期间,A 继续产出新裁定...\n");
    /* A 产出新裁定 #2(许可) */
    pthread_mutex_lock(&MJ.lock);
    MJ.nodes[a].verdict_count = 3;
    MJ.nodes[a].verdict_state[2] = VERDICT_PASS;
    /* B 同步新裁定 */
    MJ.nodes[b].verdict_count = 3;
    MJ.nodes[b].verdict_state[2] = VERDICT_PASS;
    /* C 仍只有 0 裁定(错失裁定 #2) */
    pthread_mutex_unlock(&MJ.lock);

    /* ── 阶段 5: C 恢复上线 → Pull 补全 ── */
    printf("\n━━━ [阶段 5] C 恢复上线 — Pull 补全离线期间错失的裁定 ━━━\n");
    pthread_mutex_lock(&MJ.lock);
    MJ.nodes[c].alive = 1;
    MJ.nodes[c].last_heartbeat = time(NULL);
    pthread_mutex_unlock(&MJ.lock);
    njsg_mengji_gossip_pull(c);  /* C 向邻居拉取 */

    /* ── 阶段 6: 验证 ── */
    printf("\n━━━ [阶段 6] 验证: C 是否通过 Pull 补全全部裁定 ━━━\n");
    int pass = 1;
    pthread_mutex_lock(&MJ.lock);
    printf("[验证] C 裁定数: %d (期望 3)\n", MJ.nodes[c].verdict_count);
    if (MJ.nodes[c].verdict_count != 3) {
        pass = 0;
        printf("[验证] ★ 裁定数不匹配! ★\n");
    } else {
        /* 逐项比对 */
        int v0_ok = (MJ.nodes[c].verdict_state[0] == VERDICT_PASS);
        int v1_ok = (MJ.nodes[c].verdict_state[1] == VERDICT_REJECT);
        int v2_ok = (MJ.nodes[c].verdict_state[2] == VERDICT_PASS);
        printf("[验证] C 裁定态: [0]%s(%s)  [1]%s(%s)  [2]%s(%s)\n",
               verdict_name(MJ.nodes[c].verdict_state[0]), v0_ok ? "OK" : "FAIL",
               verdict_name(MJ.nodes[c].verdict_state[1]), v1_ok ? "OK" : "FAIL",
               verdict_name(MJ.nodes[c].verdict_state[2]), v2_ok ? "OK" : "FAIL");
        if (!v0_ok || !v1_ok || !v2_ok) pass = 0;
    }
    /* 无重复存储: 只检查 verdict_count==3,无多余 */
    if (MJ.nodes[c].verdict_count > 3) pass = 0;
    /* 无漏项: verdict_count==3 保证了无遗漏 */
    pthread_mutex_unlock(&MJ.lock);

    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║  里程碑一 验证结果: %s                ║\n",
           pass ? "★ ★ ★ PASS ★ ★ ★" : "✗ ✗ ✗ FAIL ✗ ✗ ✗");
    printf("╚══════════════════════════════════════════════════╝\n");
    fflush(stdout);

    /* 打印恢复后的 C 节点 */
    njsg_mengji_print_node(c);
}

/* ============================================================
 * Demo: 三节点盟机合网络
 * ============================================================ */
void njsg_mengji_demo(void) {
    njsg_mengji_init();

    printf("\n━━━ 场景 M1: 节点注册 ━━━\n");

    /* 注册三个节点:
     *  #0 = 盟机合掌(运营场), 高位持有者=1 锚点
     *  #1 = 盟机场(萌机场), 低位持有者=0 锚点
     *  #2 = 盟机场(萌机场), 高位持有者=1 锚点
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

    /* ── 里程碑二 · 分布式三元见证演示 ── */
    printf("\n━━━ 场景 M9: 三元见证 · 三场背书 ━━━\n");

    /* 第四节点:盟机场 C(第三方) */
    int n3 = njsg_mengji_register_node("farm_node_C", NODE_FARM, ANCHOR_LOLICT);
    njsg_mengji_bind_neighbor(n0, n3);

    /* 三方对槽位#5(袋子"deadbeefcafebabe")发起背书 */
    const char* bag_hash = "deadbeefcafebabe";
    printf("\n[里程碑二] 三方对同一袋子 #%d 摘要=%s 发起背书:\n", 5, bag_hash);
    njsg_mengji_triune_register(n0, 5, bag_hash);   /* 场0 首次 */
    njsg_mengji_triune_register(n1, 5, bag_hash);   /* 场1 摘要一致 → DOUBLE */
    njsg_mengji_triune_register(n3, 5, bag_hash);   /* 场3 第三方背书 → WITNESSED */

    /* 验证三条件 */
    printf("\n[里程碑二] 验证三条件:\n");
    int verify_result = njsg_mengji_triune_verify(5);
    printf("[里程碑二] 验证结果: %s (返回码=%d)\n",
           verify_result == 0 ? "★★★ 全部通过 ★★★" : "部分失败", verify_result);

    /* 演示冲突袋子被拒绝场景 */
    printf("\n━━━ 场景 M10: 三元见证 · 冲突拒绝 ━━━\n");
    njsg_mengji_triune_register(n0, 6, "aaaaaaaaaaaaaaaa");   /* 场0 首次 */
    njsg_mengji_triune_register(n1, 6, "bbbbbbbbbbbbbbbb");   /* 场1 摘要冲突 → REJECTED */
    verify_result = njsg_mengji_triune_verify(6);
    printf("[里程碑二] 冲突验证: %s (返回码=%d)\n",
           verify_result == 0 ? "全部通过" : "预期拒绝", verify_result);

    printf("\n━━━ 场景 M11: 里程碑二全局概览 ━━━\n");
    njsg_mengji_print();

    /* ── 里程碑三 · 跨袋子合并拆分演示 ── */
    printf("\n━━━ 场景 M12: 袋子注册 + 加法合并 ━━━\n");
    /* 直接向引擎注入两个根袋子(模拟裁定份额) */
    pthread_mutex_lock(&MJ.lock);
    int ba = MJ.bag_count++;
    mengji_bag_entry_t* bag_a = &MJ.bags[ba];
    memset(bag_a, 0, sizeof(*bag_a));
    bag_a->bag_id = 101;
    bag_a->weight = 5;
    strncpy(bag_a->witness_hash, "aaaaaaaaaaaaaaaa", 16);
    bag_a->witness_hash[16] = '\0';
    bag_a->parent_id = -1;
    bag_a->source_a = -1;
    bag_a->source_b = -1;
    bag_a->anchor = ANCHOR_LIUTIANTIAN;
    bag_a->created_at = time(NULL);

    int bb = MJ.bag_count++;
    mengji_bag_entry_t* bag_b = &MJ.bags[bb];
    memset(bag_b, 0, sizeof(*bag_b));
    bag_b->bag_id = 102;
    bag_b->weight = 3;
    strncpy(bag_b->witness_hash, "bbbbbbbbbbbbbbbb", 16);
    bag_b->witness_hash[16] = '\0';
    bag_b->parent_id = -1;
    bag_b->source_a = -1;
    bag_b->source_b = -1;
    bag_b->anchor = ANCHOR_LOLICT;
    bag_b->created_at = time(NULL);
    MJ.bag_total_weight = 8;
    pthread_mutex_unlock(&MJ.lock);

    printf("[里程碑三] 注入袋#101(w=5,见证=aa..aa) + 袋#102(w=3,见证=bb..bb)\n");
    njsg_mengji_bag_validate(101);
    njsg_mengji_bag_validate(102);

    /* 加法合并 */
    printf("\n[里程碑三] 执行加法合并 袋#101 + 袋#102:\n");
    int merged = njsg_mengji_bag_merge(101, 102, "合并演示M12");
    printf("[里程碑三] 合并结果: 新袋号=%d\n", merged);
    njsg_mengji_bag_validate(merged);
    njsg_mengji_bag_validate_all();

    printf("\n━━━ 场景 M13: 袋子减法拆分 + 子袋见证链继承 ━━━\n");
    /* 拆分合并袋:减出 w=4 的子袋 */
    printf("[里程碑三] 从合并袋#%d 减出子袋 w=4:\n", merged);
    int child = njsg_mengji_bag_split(merged, 4, "拆分演示M13");
    printf("[里程碑三] 子袋号=%d\n", child);

    /* 验证子袋与父袋 */
    njsg_mengji_bag_validate(merged);
    njsg_mengji_bag_validate(child);

    /* 全体验证 */
    int all_ok = njsg_mengji_bag_validate_all();
    printf("[里程碑三] 全集守恒验证: %s\n", (all_ok == 0) ? "★★★ 通过 ★★★" : "失败");

    printf("\n━━━ 场景 M14: 里程碑三全局概览 ━━━\n");
    njsg_mengji_print();

    /* ── 里程碑一 · 独立离线恢复测试 ──
     * M15 需要独立的引擎上下文(从零开始注册节点),
     * 但 M15 内部通过 njsg_mengji_register_node 使用 node_count 编号,
     * 因此直接将其安排在 demo 最末尾,承接已有编号,不影响已有场景.
     * 上下文数据隔离:
     *   - M15 的三节点排在已有节点之后(编号独立)
     *   - M15 内部的裁定/袋子为增量追加(与 M1-M14 的 slot 编号独立)
     */
    njsg_mengji_scenario_M15_offline_recovery();

    printf("\n[盟机合] 演示完毕。\n");
    printf("[盟机合] 里程碑一: 邻居发现 + 离线恢复(M15)\n");
    printf("[盟机合] 里程碑二: 分布式三元见证 + 冲突裁决\n");
    printf("[盟机合] 里程碑三: 跨袋子合并拆分 + 增量日志 + 验证协议 = 合伙人协议 v1.4-v1.6 完整落地\n\n");
}
