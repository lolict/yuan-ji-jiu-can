/**
 * njsg_crypto.h — 囡囝共生加密层
 *
 * 提供:
 *   - X25519 ECDH (via libsodium 或 libcrypto EVP)
 *   - AES-256-GCM AEAD (via libcrypto EVP)
 *   - HKDF-SHA256 会话密钥派生 (via libcrypto EVP)
 *   - NonceTracker 防重放 (纯 C 实现)
 *
 * 不依赖 Python cryptography, 不依赖 MIT/Apache 许可代码。
 * 链接: clang ... -lsodium -lcrypto
 */
#ifndef NJSG_CRYPTO_H
#define NJSG_CRYPTO_H

#include <stdint.h>
#include <stddef.h>
#include "../encoding/mccp.h"

/* ── 错误码 ───────────────────────────────────────────── */
#define NJSG_CRYPTO_OK       0
#define NJSG_CRYPTO_ERR     -1
#define NJSG_CRYPTO_NOMEM   -2
#define NJSG_CRYPTO_AUTH    -3   /* GCM tag 验证失败 */
#define NJSG_CRYPTO_REPLAY  -4   /* 重放检测 */

/* ── 关键大小常量 ─────────────────────────────────────── */
#define NJSG_PUBKEY_LEN     32
#define NJSG_PRIVKEY_LEN    32
#define NJSG_SHARED_LEN     32
#define NJSG_SESSION_LEN    32
#define NJSG_GCM_NONCE_LEN  12
#define NJSG_GCM_TAG_LEN    16

/* ── X25519 密钥对 ─────────────────────────────────────── */
typedef struct {
    uint8_t pub[NJSG_PUBKEY_LEN];
    uint8_t priv[NJSG_PRIVKEY_LEN];
} MqfKeypair;

/* ── Nonce tracker (防重放) ───────────────────────────── */
#define NJSG_NONCE_WINDOWS  300   /* 300 秒滑动窗口 */
#define NJSG_NONCE_HIST     512   /* 历史记录数 */
typedef struct {
    uint64_t history[NJSG_NONCE_HIST];
    uint32_t head;
    uint32_t count;
    uint64_t window_us;          /* 窗口宽度 (微秒) */
} MqfNonceTracker;

/* ── API: X25519 ──────────────────────────────────────── */
int njsg_kp_generate(MqfKeypair *kp);
int njsg_kp_from_seed(MqfKeypair *kp, const uint8_t seed[NJSG_PRIVKEY_LEN]);
int njsg_derive_shared(const MqfKeypair *local, const uint8_t peer_pub[NJSG_PUBKEY_LEN],
                      uint8_t out[NJSG_SHARED_LEN]);

/* ── API: AES-256-GCM ─────────────────────────────────── */
int njsg_gcm_encrypt(const uint8_t key[NJSG_SESSION_LEN],
                    const uint8_t nonce[NJSG_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *pt, size_t pt_len,
                    uint8_t *ct, uint8_t tag[NJSG_GCM_TAG_LEN]);

int njsg_gcm_decrypt(const uint8_t key[NJSG_SESSION_LEN],
                    const uint8_t nonce[NJSG_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *ct, size_t ct_len,
                    const uint8_t tag[NJSG_GCM_TAG_LEN],
                    uint8_t *pt);

/* ── API: HKDF-SHA256 ──────────────────────────────────── */
int njsg_hkdf(const uint8_t *ikm, size_t ikm_len,
             const uint8_t *salt, size_t salt_len,
             const uint8_t *info, size_t info_len,
             uint8_t *okm, size_t okm_len);

/* ── API: NonceTracker ────────────────────────────────── */
void njsg_nonce_init(MqfNonceTracker *nt);
int njsg_nonce_check_and_mark(MqfNonceTracker *nt, uint64_t nonce, uint64_t now_us);

/* ── 一次会话握手 (唤魂/招魂协议) ────────────────────── */
/* 简化封装: 直接在 C 中完成 唤魂→招魂→session_key 全流程     */
typedef struct {
    MqfKeypair local_kp;
    uint8_t    session_key[NJSG_SESSION_LEN];
    uint8_t    peer_pub[NJSG_PUBKEY_LEN];
    uint64_t   nonce_ctr;
    uint8_t    has_peer;
} MqfSession;

int njsg_session_huanhun(MqfSession *s);                                 /* 唤魂 */
int njsg_session_zhaohun(MqfSession *s, const uint8_t peer_pub[NJSG_PUBKEY_LEN]); /* 招魂 */

/* 加密/解密一条 MCCP 消息 */
int njsg_session_encrypt(MqfSession *s, const uint32_t *mccp, uint16_t len,
                        uint8_t *frame_buf, size_t *frame_len);
int njsg_session_decrypt(MqfSession *s, const uint8_t *frame, size_t frame_len,
                        uint32_t *mccp_out, uint16_t *len_out);

#endif /* NJSG_CRYPTO_H */
