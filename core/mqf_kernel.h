/**
 * mqf_kernel.h — 满全法内核公共类型 + FSM
 *
 * 所有数据都是 u32/u8 栈数组或手动 malloc，没有 Python 对象。
 * 这意味着可以编译为裸机固件、WebAssembly、或 Android NDK 共享库。
 */
#ifndef MQF_KERNEL_H
#define MQF_KERNEL_H

#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

#include "../encoding/mccp.h"

/* ── 配置常量 (编译期决定) ───────────────────────────── */
#define MQF_MAX_PARTIES      8
#define MQF_MAX_PROCESSES   64
#define MQF_MAX_SEQ_LEN    256
#define MQF_NAME_MAXGlyphs   16

/* ── 进程状态 ────────────────────────────────────────── */
enum MqfProcessState {
    MQF_PS_XIANGYU  = 0,   /* 灵初相遇 — 未签名   */
    MQF_PS_DIYUE    = 1,   /* 缔约中   — 一方签    */
    MQF_PS_LIYUE    = 2,   /* 已立约   — 双方签    */
    MQF_PS_HESUI    = 3,   /* 合一递归 — 双方对齐  */
    MQF_PS_JIETI    = 4,   /* 解体     — 一方退出  */
};

/* ── 四态 FSM (囡囝共生元) ──────────────────────────── */
/* 初始态: MQF_STATE_NAN_JIAN, 编码在 mcCP.h            */

/* ── 核心数据结构 (无指针开销版) ─────────────────────── */
/* 进程 (婚姻誓约)                                         */
typedef struct {
    uint32_t  seq[MQF_MAX_SEQ_LEN];  /* MCCP 定长序列 (誓约内容)      */
    uint16_t  len;                   /* 当前 glyph 数                 */
    uint16_t  seq_cap;               /* 容量 (通常 = MQF_MAX_SEQ_LEN)*/
    uint32_t  parties[2];            /* 两立约人名 anchor (uint32 hash)*/
    uint8_t   signs[2];              /* 是否已签                     */
    uint8_t   ps;                    /* ProcessState                 */
    uint8_t   fsm;                   /* QuaternityState              */
    uint32_t  anchor;                /* 誓约锚 (FNV-1a 32-bit)      */
} MqfProcess;  /* 大小: ≤ 1040 bytes (全在栈)              */

/* FSM 转换规则节点                                         */
typedef struct {
    uint8_t   from;       /* 当前态         */
    uint8_t   event;      /* 触发事件       */
    uint8_t   to;         /* 下一态         */
    uint8_t   op;         /* 转置后执行的操作 */
} MqfFsmRule;

/* 关系节点:在 Python 中叫 LingXiNode                       */
typedef struct MqfNode {
    uint32_t  name[MQF_NAME_MAXGlyphs];  /* 名字的 MCCP 编码      */
    uint8_t   pubkey[32];               /* X25519 public (32B)  */
    uint8_t   privkey[32];              /* X25519 private (32B) */
    uint8_t   sess_key[32];             /* AES-256 session key  */
    uint8_t   has_sess;                 /* 1 表示 sess_key 可用 */
    uint64_t  nonce_ctr;                /* 单调防重放计数        */
} MqfNode; /* 112 bytes / node */

/* CoupleOS:系统实例                                        */
typedef struct {
    MqfProcess procs[MQF_MAX_PROCESSES];
    uint16_t    proc_count;
    uint8_t     init;
    uint32_t    couple_anchor;  /* 夫妻联合锚 (joint_anchor) */
} MqfCoupleOS;

/* ── API 声明 ──────────────────────────────────────── */

/* 初始化 */
int         mqf_os_init(MqfCoupleOS *os, uint32_t anchor);

/* 进程创建 */
MqfProcess *mqf_create_process(MqfCoupleOS *os, const uint32_t *seq, uint16_t len,
                               uint32_t pa, uint32_t pb);

/* 签名 & 立约 */
int         mqf_sign(MqfProcess *p, int party_idx);
uint8_t     mqf_is_liyue(const MqfProcess *p);

/* 一元递归: 返回差异 (=0 表示完美对齐)                    */
int64_t     mqf_unary_recursion(MqfProcess *p);

/* FSM */
uint8_t     mqf_fsm_transition(MqfProcess *p, uint8_t event);
const char *mqf_fsm_name(uint8_t state);
const char *mqf_ps_name(uint8_t state);

/* Anchor 计算 */
uint32_t    mqf_anchor_hash(const uint32_t *seq, uint16_t len);

/* 调试打印 */
void        mqf_process_dump(const MqfProcess *p, char *buf, size_t cap);

#endif /* MQF_KERNEL_H */
