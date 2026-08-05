/**
 * mqf_crypto.c — 囡囝共生加密层 (C 实现)
 *
 * 编译: clang -O2 -std=c17 core/mqf_crypto.c core/test_crypto.c \
 *            -o core/mqf_crypto_test.elf -lsodium -lcrypto
 *
 * 链接 libsodium (X25519 base+scalarmult) + libcrypto.so.3 (AES-GCM + HKDF-SHA256)。
 * 由于沙箱无 C 头文件, 手工 extern 声明全部符号, 类型用 stub struct + 指针。
 */
#include "mqf_crypto.h"

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <dlfcn.h>

/* ═══════════════════════════════════════════════════════════
 * 类型 stub
 * ═══════════════════════════════════════════════════════════ */
typedef struct { void *p; }          MqfEvpCtx;
typedef struct { void *p; }          MqfEvpCipher;
typedef struct { void *p; }          MqfEvpPkeyCtx;
typedef struct { void *p; }          MqfEvpPkey;

/* ═══════════════════════════════════════════════════════════
 * libsodium — extern 声明
 * ═══════════════════════════════════════════════════════════ */
extern int crypto_scalarmult_curve25519(unsigned char *q,
                                        const unsigned char *n,
                                        const unsigned char *p);
extern int crypto_scalarmult_curve25519_base(unsigned char *q,
                                              const unsigned char *n);
extern void randombytes_buf(void * const buf, const size_t size);
extern void sodium_memzero(void * const pnt, const size_t len);

/* ═══════════════════════════════════════════════════════════
 * libcrypto (OpenSSL 3) — extern 声明
 * ═══════════════════════════════════════════════════════════ */
extern MqfEvpCtx        *EVP_CIPHER_CTX_new(void);
extern void              EVP_CIPHER_CTX_free(MqfEvpCtx *c);
extern int               EVP_EncryptInit_ex2(MqfEvpCtx *ctx, const MqfEvpCipher *type,
                                              const unsigned char *key,
                                              const unsigned char *iv, void *params);
extern int               EVP_DecryptInit_ex2(MqfEvpCtx *ctx, const MqfEvpCipher *type,
                                              const unsigned char *key,
                                              const unsigned char *iv, void *params);
extern int               EVP_EncryptUpdate(MqfEvpCtx *ctx, unsigned char *out,
                                            int *outl, const unsigned char *in, int inl);
extern int               EVP_DecryptUpdate(MqfEvpCtx *ctx, unsigned char *out,
                                            int *outl, const unsigned char *in, int inl);
extern int               EVP_EncryptFinal_ex(MqfEvpCtx *ctx, unsigned char *out, int *outl);
extern int               EVP_DecryptFinal_ex(MqfEvpCtx *ctx, unsigned char *out, int *outl);
extern int               EVP_CIPHER_CTX_ctrl(MqfEvpCtx *ctx, int type, int arg, void *ptr);
extern const MqfEvpCipher *EVP_aes_256_gcm(void);
extern const MqfEvpCipher *EVP_sha256(void);

/* HKDF (OpenSSL 3) */
extern MqfEvpPkeyCtx *EVP_PKEY_CTX_new_id(int type, void *e);
extern int  EVP_PKEY_derive_init(MqfEvpPkeyCtx *ctx);
extern int  EVP_PKEY_derive(MqfEvpPkeyCtx *ctx, unsigned char *key, size_t *keylen);
extern int  EVP_PKEY_CTX_set_hkdf_mode(MqfEvpPkeyCtx *ctx, int mode);
extern int  EVP_PKEY_CTX_set_hkdf_md(MqfEvpPkeyCtx *ctx, const MqfEvpCipher *md);
extern int  EVP_PKEY_CTX_set1_hkdf_key(MqfEvpPkeyCtx *ctx,
                                        const unsigned char *key, size_t keylen);
extern int  EVP_PKEY_CTX_set1_hkdf_salt(MqfEvpPkeyCtx *ctx,
                                         const unsigned char *salt, size_t saltlen);
extern int  EVP_PKEY_CTX_add1_hkdf_info(MqfEvpPkeyCtx *ctx,
                                         const unsigned char *info, size_t infolen);
extern void EVP_PKEY_CTX_free(MqfEvpPkeyCtx *ctx);

#define MQF_EVP_HKDF                         1036
#define MQF_EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND  0
#define MQF_EVP_CTRL_GCM_SET_IVLEN                0x09
#define MQF_EVP_CTRL_GCM_GET_TAG                  0x10
#define MQF_EVP_CTRL_GCM_SET_TAG                  0x11

/* ═══════════════════════════════════════════════════════════
 * X25519
 * ═══════════════════════════════════════════════════════════ */
int mqf_kp_generate(MqfKeypair *kp) {
    if (!kp) return MQF_CRYPTO_ERR;
    randombytes_buf(kp->priv, MQF_PRIVKEY_LEN);
    kp->priv[0]  &= 248;
    kp->priv[31] &= 127;
    kp->priv[31] |= 64;
    if (crypto_scalarmult_curve25519_base(kp->pub, kp->priv) != 0) {
        sodium_memzero(kp->priv, MQF_PRIVKEY_LEN);
        return MQF_CRYPTO_ERR;
    }
    return MQF_CRYPTO_OK;
}

int mqf_kp_from_seed(MqfKeypair *kp, const uint8_t seed[MQF_PRIVKEY_LEN]) {
    if (!kp || !seed) return MQF_CRYPTO_ERR;
    memcpy(kp->priv, seed, MQF_PRIVKEY_LEN);
    kp->priv[0]  &= 248;
    kp->priv[31] &= 127;
    kp->priv[31] |= 64;
    if (crypto_scalarmult_curve25519_base(kp->pub, kp->priv) != 0) {
        return MQF_CRYPTO_ERR;
    }
    return MQF_CRYPTO_OK;
}

int mqf_derive_shared(const MqfKeypair *local, const uint8_t peer_pub[MQF_PUBKEY_LEN],
                      uint8_t out[MQF_SHARED_LEN]) {
    if (!local || !peer_pub || !out) return MQF_CRYPTO_ERR;
    if (crypto_scalarmult_curve25519(out, local->priv, peer_pub) != 0) {
        return MQF_CRYPTO_ERR;
    }
    return MQF_CRYPTO_OK;
}

/* ═══════════════════════════════════════════════════════════
 * AES-256-GCM
 * ═══════════════════════════════════════════════════════════ */
int mqf_gcm_encrypt(const uint8_t key[MQF_SESSION_LEN],
                    const uint8_t nonce[MQF_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *pt, size_t pt_len,
                    uint8_t *ct, uint8_t tag[MQF_GCM_TAG_LEN]) {
    if (!key || !nonce || !ct || !tag || (!pt && pt_len > 0)) return MQF_CRYPTO_ERR;
    MqfEvpCtx *ctx = EVP_CIPHER_CTX_new();
    if (!ctx) return MQF_CRYPTO_NOMEM;
    int ok = 0;

    if (!EVP_EncryptInit_ex2(ctx, EVP_aes_256_gcm(), key, nonce, NULL)) goto enc_cleanup;
    if (!EVP_CIPHER_CTX_ctrl(ctx, MQF_EVP_CTRL_GCM_SET_IVLEN,
                              MQF_GCM_NONCE_LEN, NULL)) goto enc_cleanup;

    int aad_l = 0;
    if (aad && aad_len > 0) {
        if (!EVP_EncryptUpdate(ctx, NULL, &aad_l, aad, (int)aad_len)) goto enc_cleanup;
    }
    int ct_l = 0;
    if (pt && pt_len > 0) {
        if (!EVP_EncryptUpdate(ctx, ct, &ct_l, pt, (int)pt_len)) goto enc_cleanup;
    }
    int fin_l = 0;
    uint8_t fin[16];
    if (!EVP_EncryptFinal_ex(ctx, fin, &fin_l)) goto enc_cleanup;
    if (!EVP_CIPHER_CTX_ctrl(ctx, MQF_EVP_CTRL_GCM_GET_TAG,
                              MQF_GCM_TAG_LEN, tag)) goto enc_cleanup;

    ok = 1;
enc_cleanup:
    EVP_CIPHER_CTX_free(ctx);
    return ok ? MQF_CRYPTO_OK : MQF_CRYPTO_ERR;
}

int mqf_gcm_decrypt(const uint8_t key[MQF_SESSION_LEN],
                    const uint8_t nonce[MQF_GCM_NONCE_LEN],
                    const uint8_t *aad, size_t aad_len,
                    const uint8_t *ct, size_t ct_len,
                    const uint8_t tag[MQF_GCM_TAG_LEN],
                    uint8_t *pt) {
    if (!key || !nonce || !ct || !tag || !pt) return MQF_CRYPTO_ERR;
    MqfEvpCtx *ctx = EVP_CIPHER_CTX_new();
    if (!ctx) return MQF_CRYPTO_NOMEM;
    int ok = 0;

    if (!EVP_DecryptInit_ex2(ctx, EVP_aes_256_gcm(), key, nonce, NULL)) goto dec_cleanup;
    if (!EVP_CIPHER_CTX_ctrl(ctx, MQF_EVP_CTRL_GCM_SET_IVLEN,
                              MQF_GCM_NONCE_LEN, NULL)) goto dec_cleanup;

    int aad_l = 0;
    if (aad && aad_len > 0) {
        if (!EVP_DecryptUpdate(ctx, NULL, &aad_l, aad, (int)aad_len)) goto dec_cleanup;
    }
    int pt_l = 0;
    if (ct_len > 0) {
        if (!EVP_DecryptUpdate(ctx, pt, &pt_l, ct, (int)ct_len)) goto dec_cleanup;
    }
    if (!EVP_CIPHER_CTX_ctrl(ctx, MQF_EVP_CTRL_GCM_SET_TAG,
                              MQF_GCM_TAG_LEN, (void*)tag)) goto dec_cleanup;
    int fin_l = 0;
    uint8_t fin[16];
    if (!EVP_DecryptFinal_ex(ctx, fin, &fin_l)) goto dec_cleanup;

    ok = 1;
dec_cleanup:
    EVP_CIPHER_CTX_free(ctx);
    if (!ok) return MQF_CRYPTO_AUTH;
    return MQF_CRYPTO_OK;
}

/* ═══════════════════════════════════════════════════════════
 * HKDF-SHA256 (OpenSSL 3)
 * ═══════════════════════════════════════════════════════════ */
int mqf_hkdf(const uint8_t *ikm, size_t ikm_len,
             const uint8_t *salt, size_t salt_len,
             const uint8_t *info, size_t info_len,
             uint8_t *okm, size_t okm_len) {
    if (!ikm || !okm) return MQF_CRYPTO_ERR;

    MqfEvpPkeyCtx *pctx = EVP_PKEY_CTX_new_id(MQF_EVP_HKDF, NULL);
    if (!pctx) return MQF_CRYPTO_NOMEM;

    int rc = MQF_CRYPTO_OK;
    size_t outlen = okm_len;

    if (EVP_PKEY_derive_init(pctx) != 1) { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    if (EVP_PKEY_CTX_set_hkdf_mode(pctx, MQF_EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND) != 1)
        { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    if (EVP_PKEY_CTX_set_hkdf_md(pctx, EVP_sha256()) != 1)
        { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    if (EVP_PKEY_CTX_set1_hkdf_key(pctx, ikm, ikm_len) != 1)
        { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    if (salt && salt_len > 0) {
        if (EVP_PKEY_CTX_set1_hkdf_salt(pctx, salt, salt_len) != 1)
            { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    }
    if (info && info_len > 0) {
        if (EVP_PKEY_CTX_add1_hkdf_info(pctx, info, info_len) != 1)
            { rc = MQF_CRYPTO_ERR; goto hkdf_cleanup; }
    }
    if (EVP_PKEY_derive(pctx, okm, &outlen) != 1 || outlen != okm_len)
        rc = MQF_CRYPTO_ERR;

hkdf_cleanup:
    EVP_PKEY_CTX_free(pctx);
    return rc;
}

/* ═══════════════════════════════════════════════════════════
 * NonceTracker
 * ═══════════════════════════════════════════════════════════ */
void mqf_nonce_init(MqfNonceTracker *nt) {
    if (!nt) return;
    memset(nt, 0, sizeof(*nt));
    nt->window_us = (uint64_t)MQF_NONCE_WINDOWS * 1000000ULL;
}

static int nonce_in_history(const MqfNonceTracker *nt, uint64_t nonce) {
    for (uint32_t i = 0; i < nt->count; i++)
        if (nt->history[i] == nonce) return 1;
    return 0;
}

int mqf_nonce_check_and_mark(MqfNonceTracker *nt, uint64_t nonce, uint64_t now_us) {
    if (!nt) return MQF_CRYPTO_ERR;
    if (nonce_in_history(nt, nonce)) return MQF_CRYPTO_REPLAY;
    uint32_t pos = (nt->head + nt->count) % MQF_NONCE_HIST;
    nt->history[pos] = nonce;
    if (nt->count == MQF_NONCE_HIST) {
        nt->head = (nt->head + 1) % MQF_NONCE_HIST;
    } else {
        nt->count++;
    }
    (void)now_us;
    return MQF_CRYPTO_OK;
}

/* ═══════════════════════════════════════════════════════════
 * 会话握手 (唤魂/招魂)
 * ═══════════════════════════════════════════════════════════ */
int mqf_session_huanhun(MqfSession *s) {
    if (!s) return MQF_CRYPTO_ERR;
    memset(s, 0, sizeof(*s));
    s->nonce_ctr = 1;
    return mqf_kp_generate(&s->local_kp);
}

int mqf_session_zhaohun(MqfSession *s, const uint8_t peer_pub[MQF_PUBKEY_LEN]) {
    if (!s || !peer_pub) return MQF_CRYPTO_ERR;
    memcpy(s->peer_pub, peer_pub, MQF_PUBKEY_LEN);
    s->has_peer = 1;

    uint8_t shared[MQF_SHARED_LEN];
    int rc = mqf_derive_shared(&s->local_kp, peer_pub, shared);
    if (rc != 0) return rc;

    const uint8_t info[] = "mqf-session-v1";
    rc = mqf_hkdf(shared, MQF_SHARED_LEN,
                  NULL, 0,
                  info, sizeof(info),
                  s->session_key, MQF_SESSION_LEN);
    sodium_memzero(shared, MQF_SHARED_LEN);
    return rc;
}

/* ═══════════════════════════════════════════════════════════
 * 加密 / 解密 MCCP 消息
 * ═══════════════════════════════════════════════════════════ */
int mqf_session_encrypt(MqfSession *s, const uint32_t *mccp, uint16_t len,
                        uint8_t *frame_buf, size_t *frame_len) {
    if (!s || !mccp || !frame_buf || !frame_len || !s->has_peer) return MQF_CRYPTO_ERR;

    size_t pt_len = (size_t)len * sizeof(uint32_t);
    size_t need  = MQF_GCM_NONCE_LEN + pt_len + MQF_GCM_TAG_LEN;
    if (*frame_len < need) { *frame_len = need; return MQF_CRYPTO_NOMEM; }

    uint8_t nonce[MQF_GCM_NONCE_LEN];
    randombytes_buf(nonce, MQF_GCM_NONCE_LEN);
    nonce[0] = (uint8_t)(s->nonce_ctr >> 24);
    nonce[1] = (uint8_t)(s->nonce_ctr >> 16);
    nonce[2] = (uint8_t)(s->nonce_ctr >> 8);
    nonce[3] = (uint8_t)(s->nonce_ctr);

    uint8_t tag[MQF_GCM_TAG_LEN];
    int rc = mqf_gcm_encrypt(s->session_key, nonce, NULL, 0,
                             (const uint8_t*)mccp, pt_len,
                             frame_buf + MQF_GCM_NONCE_LEN, tag);
    if (rc != 0) return rc;

    memcpy(frame_buf, nonce, MQF_GCM_NONCE_LEN);
    memcpy(frame_buf + MQF_GCM_NONCE_LEN + pt_len, tag, MQF_GCM_TAG_LEN);
    *frame_len = MQF_GCM_NONCE_LEN + pt_len + MQF_GCM_TAG_LEN;
    s->nonce_ctr++;
    return MQF_CRYPTO_OK;
}

int mqf_session_decrypt(MqfSession *s, const uint8_t *frame, size_t frame_len,
                        uint32_t *mccp_out, uint16_t *len_out) {
    if (!s || !frame || !mccp_out || !len_out) return MQF_CRYPTO_ERR;
    if (frame_len < MQF_GCM_NONCE_LEN + MQF_GCM_TAG_LEN) return MQF_CRYPTO_AUTH;

    size_t pt_len = frame_len - MQF_GCM_NONCE_LEN - MQF_GCM_TAG_LEN;
    if (pt_len % sizeof(uint32_t) != 0) return MQF_CRYPTO_AUTH;
    uint16_t mccp_len = (uint16_t)(pt_len / sizeof(uint32_t));

    const uint8_t *nonce = frame;
    const uint8_t *ct   = frame + MQF_GCM_NONCE_LEN;
    const uint8_t *tag  = frame + MQF_GCM_NONCE_LEN + pt_len;

    int rc = mqf_gcm_decrypt(s->session_key, nonce, NULL, 0,
                             ct, pt_len, tag, (uint8_t*)mccp_out);
    if (rc != 0) return rc;
    *len_out = mccp_len;
    return MQF_CRYPTO_OK;
}
