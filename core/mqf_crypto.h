/**
 * mqf_crypto.h — 囡囝共生加密层
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
#ifndef MQF_CRYPTO_H
#define MQF_CRYPTO_H

#include <stdint.h>
#include <stddef.h>
#include "../encoding/mccp.h"

/* ── 错误码 ───────────────────────────────────────────── */
#define MQF_CRYPTO_OK       0
#define MQF_CRYPTO_ERR     -1
#define MQF_CRYPTO_NOMEM   -2
#define MQF_CRYPTO_AUTH    -3   /* GCM tag 验证失败 */
#define MQF_CRYPTO_REPLAY  -4   /* 重放检测 */

/* ── 关键大小常量 ─────────────────────────────────────── */
#define MQF_PUBKEY_LEN     32
#define MQF_PRIVKEY_LEN    32
#define MQF_SHARED_LEN     32
#define MQF_SESSION_LEN    32
#define MQF_GCM_NONCE_LEN  12
#define MQF_GCM_TAG_LEN    16

/* ── X25519 密钥对 ─────────────────────────────────────── */
typedef struct {
    uint8_t pub[MQF_PUBKEY_LEN];
    uint8_t priv[MQF_PRIVKEY_LEN];
} MqfKeypair;

/* ── Nonce tracker (防重放) ───────────────────────────── */
#define MQF_NONCE_WINDOWS  300   /* 300 秒滑动窗口 */
#define MQF_NONCE_HIST     512   /* 历史记录数 */
typedef struct {
    uint64_t history[MQF_NONCE_HIST];
    uint32_t head;
    uint32_t count;
    uint64_t window_us;          /* 窗口宽度 (微秒) */
} MqfNonceTracker;

/* ── API: X25519 ──────────────────────────────────────── */
int mqf_kp_generate(MqfKeypair *kp);
int mqf_kp_from_seed(MqfKeypair *kp, const uint8_t seed[MQF_PRIVKEY_LEN]);
int mqf_derive_shared(const MqfKeypair *local, const uint8_t peer_pub[MQF_PUBKEY_LEN],
                      uint8_t out[MQF_SHARED_LEN]);

/* ── API: AES-256-GCM ─────────────────────────────────── */
int mqf_gcm_encrypt(const uint8_t key[MQF_SESSION_LEN],
                    const uint8_t nonce[MQF_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *pt, size_t pt_len,
                    uint8_t *ct, uint8_t tag[MQF_GCM_TAG_LEN]);

int mqf_gcm_decrypt(const uint8_t key[MQF_SESSION_LEN],
                    const uint8_t nonce[MQF_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *ct, size_t ct_len,
                    const uint8_t tag[MQF_GCM_TAG_LEN],
                    uint8_t *pt);

/* ── API: HKDF-SHA256 ──────────────────────────────────── */
int mqf_hkdf(const uint8_t *ikm, size_t ikm_len,
             const uint8_t *salt, size_t salt_len,
             const uint8_t *info, size_t info_len,
             uint8_t *okm, size_t okm_len);

/* ── API: NonceTracker ────────────────────────────────── */
void mqf_nonce_init(MqfNonceTracker *nt);
int mqf_nonce_check_and_mark(MqfNonceTracker *nt, uint64_t nonce, uint64_t now_us);

/* ── 一次会话握手 (唤魂/招魂协议) ────────────────────── */
/* 简化封装: 直接在 C 中完成 唤魂→招魂→session_key 全流程     */
typedef struct {
    MqfKeypair local_kp;
    uint8_t    session_key[MQF_SESSION_LEN];
    uint8_t    peer_pub[MQF_PUBKEY_LEN];
    uint64_t   nonce_ctr;
    uint8_t    has_peer;
} MqfSession;

int mqf_session_huanhun(MqfSession *s);                                 /* 唤魂 */
int mqf_session_zhaohun(MqfSession *s, const uint8_t peer_pub[MQF_PUBKEY_LEN]); /* 招魂 */

/* 加密/解密一条 MCCP 消息 */
int mqf_session_encrypt(MqfSession *s, const uint32_t *mccp, uint16_t len,
                        uint8_t *frame_buf, size_t *frame_len);
int mqf_session_decrypt(MqfSession *s, const uint8_t *frame, size_t frame_len,
                        uint32_t *mccp_out, uint16_t *len_out);

#endif /* MQF_CRYPTO_H */
