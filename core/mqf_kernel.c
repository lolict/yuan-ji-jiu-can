/**
 * mqf_kernel.c — 囡囝共生 C 内核实现
 *
 * 直接替代 Python 版 kernel.py。
 * 编译: clang -O2 -Wall -std=c17 core/mqf_kernel.c core/main.c -o mqf.elf
 */
#include "mqf_kernel.h"

#include <string.h>
#include <stdio.h>
#include <inttypes.h>

/* ── FSM 转换表 (回译自 Python quaternity_fsm.py TRANSITIONS) ──
 *  9 条规则, 4 态 (nan_jian / nan_nan / jian_jian / jian_nan)
 *  每条: (from, event) → to
 */
static const MqfFsmRule k_fsm_table[] = {
    /* from=囡囝 (主态) */
    { MQF_STATE_NAN_JIAN,   0 /* wife_deepen */,  MQF_STATE_NAN_NAN,   0 },
    { MQF_STATE_NAN_JIAN,   2 /* husband_take */, MQF_STATE_JIAN_JIAN, 0 },
    { MQF_STATE_NAN_NAN,    1 /* release */,       MQF_STATE_NAN_JIAN,   0 },
    { MQF_STATE_JIAN_JIAN,  3 /* deepen */,       MQF_STATE_JIAN_NAN,   0 },
    { MQF_STATE_JIAN_JIAN,  0 /* wife_deepen */,  MQF_STATE_NAN_NAN,   0 },
    { MQF_STATE_JIAN_NAN,   4 /* wife_push */,   MQF_STATE_NAN_JIAN,   0 },
    { MQF_STATE_JIAN_NAN,   5 /* retreat */,      MQF_STATE_JIAN_JIAN, 0 },
};
#define MQF_FSM_RULES  (sizeof k_fsm_table / sizeof k_fsm_table[0])

static const char *k_fsm_names[] = {
    "囝囡(nan_jian)", "囡囡(nan_nan)", "囝囝(jian_jian)", "囝囡(jian_nan)"
};
static const char *k_ps_names[] = {
    "灵初相遇", "缔约中", "已立约", "合一递归", "解体"
};

/* ── Anchor 计算 (确定性 FNV-1a 64 → 32) ──────────────── */
uint32_t mqf_anchor_hash(const uint32_t *seq, uint16_t len) {
    uint64_t h = 0xcbf29ce484222325ULL;
    for (uint16_t i = 0; i < len; i++) {
        h ^= seq[i];
        h *= 0x100000001b3ULL;
    }
    h ^= (uint64_t)len << 32;
    return (uint32_t)(h ^ (h >> 32));
}

/* ── OS 初始化 ─────────────────────────────────────────── */
int mqf_os_init(MqfCoupleOS *os, uint32_t anchor) {
    memset(os, 0, sizeof(*os));
    os->couple_anchor = anchor;
    os->init = 1;
    return 0;
}

/* ── 创建进程 (灵初相遇) ──────────────────────────────── */
MqfProcess *mqf_create_process(MqfCoupleOS *os,
                               const uint32_t *seq, uint16_t len,
                               uint32_t pa, uint32_t pb) {
    if (!os || !os->init || len > MQF_MAX_SEQ_LEN || os->proc_count >= MQF_MAX_PROCESSES)
        return NULL;
    MqfProcess *p = &os->procs[os->proc_count];
    memset(p, 0, sizeof(*p));
    memcpy(p->seq, seq, len * sizeof(uint32_t));
    p->len      = len;
    p->seq_cap  = MQF_MAX_SEQ_LEN;
    p->parties[0] = pa;
    p->parties[1] = pb;
    p->ps       = MQF_PS_XIANGYU;     /* 相遇 */
    p->fsm      = MQF_STATE_NAN_JIAN; /* 默认主态 */
    p->anchor   = mqf_anchor_hash(seq, len);
    os->proc_count++;
    return p;
}

/* ── 签名 ─────────────────────────────────────────────── */
int mqf_sign(MqfProcess *p, int party_idx) {
    if (!p || party_idx < 0 || party_idx > 1) return -1;
    p->signs[party_idx] = 1;
    uint8_t s = p->signs[0] + p->signs[1];
    if (s == 1) {
        p->ps = MQF_PS_DIYUE;  /* 缔约中 */
    } else if (s == 2) {
        p->ps = MQF_PS_LIYUE;  /* 已立约 */
    }
    return 0;
}

uint8_t mqf_is_liyue(const MqfProcess *p) {
    return p && p->ps == MQF_PS_LIYUE;
}

/* ── 一元递归 (核心动作) ──────────────────────────────── */
/**
 * 双方以誓约 anchor 为锚, 在各自状态变化时重复三元合一,
 * 一直到双方差异为 0。返回 residual alignment 差异。
 */
int64_t mqf_unary_recursion(MqfProcess *p) {
    if (!p || p->ps != MQF_PS_LIYUE) return -1;
    /* 夫妻各持有誓约, 差异 = 双方认知距离。*/
    /* parties[0]/parties[1] 是两认知节点所持的 anchor 值 — 一致表示合一; 偏差需仲裁。*/
    uint32_t a = p->parties[0];
    uint32_t b = p->parties[1];
    int64_t gap = (int64_t)a - (int64_t)b;
    if (gap < 0) gap = -gap;
    /* 誓约内容自身的确定性差异 (modulated by FSM state 和 seq len) */
    uint64_t sig = ((uint64_t)p->anchor << 32) | ((uint64_t)p->fsm << 16) | (uint64_t)p->len;
    int64_t content_gap = (int64_t)(sig % 256);
    int64_t total_gap = gap + content_gap;
    if (total_gap == 0) {
        p->ps = MQF_PS_HESUI;  /* 合一递归 */
    }
    return total_gap;
}

/* ── FSM 转换 ────────────────────────────────────────── */
uint8_t mqf_fsm_transition(MqfProcess *p, uint8_t event) {
    if (!p) return 0xFF;
    for (size_t i = 0; i < MQF_FSM_RULES; i++) {
        if (k_fsm_table[i].from == p->fsm && k_fsm_table[i].event == event) {
            p->fsm = k_fsm_table[i].to;
            return p->fsm;
        }
    }
    return p->fsm;  /* 无匹配，保持 */
}

/* ── 字符串名 ──────────────────────────────────────────── */
const char *mqf_fsm_name(uint8_t state) {
    if (state < 4) return k_fsm_names[state];
    return "未知态";
}
const char *mqf_ps_name(uint8_t state) {
    if (state < 5) return k_ps_names[state];
    return "未知";
}

/* ── 调试输出 ─────────────────────────────────────────── */
void mqf_process_dump(const MqfProcess *p, char *buf, size_t cap) {
    if (!p || !buf || cap < 16) return;
    int n = snprintf(buf, cap,
        "Process{ ps=%s fsm=%s len=%u parties=[%08" PRIx32 ",%08" PRIx32 "] signed=[%u,%u] anchor=%08" PRIx32 " }",
        mqf_ps_name(p->ps),
        mqf_fsm_name(p->fsm),
        p->len,
        p->parties[0], p->parties[1],
        p->signs[0], p->signs[1],
        p->anchor);
    if (n < 0) buf[0] = '\0';
}
