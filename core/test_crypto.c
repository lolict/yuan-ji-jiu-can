/**
 * test_crypto.c — 囡囝共生 C 加密层测试
 *
 * 编译:
 *   clang -O2 -std=c17 -Wall -Wextra -Werror -I. \
 *         core/test_crypto.c core/mqf_crypto.c \
 *         -o core/mqf_crypto_test.elf -lsodium -lcrypto -ldl
 *
 * 测试实际 X25519 ECDH + AES-GCM 端到端加密。
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <inttypes.h>

#include "mqf_crypto.h"

static int g_fail = 0;

#define ASSERT(c, m) do { \
    if (c) printf("  ✓ %s\n", m); \
    else { printf("  ✗ FAIL: %s\n", m); g_fail++; } \
} while (0)

/* ── Test 1: X25519 密钥对生成 ───────────────────────── */
static void test_kp_gen(void) {
    printf("\n[1] X25519 密钥对生成\n");
    MqfKeypair kp;
    ASSERT(mqf_kp_generate(&kp) == 0, "mqf_kp_generate() 成功");
    /* pubkey 不全零 (sanity) */
    uint8_t zero[32] = {0};
    ASSERT(memcmp(kp.pub, zero, 32) != 0, "公钥非全零");
    /* priv 已 clamp (低3位 = 0, bit254 = 1, bit255 = 0) */
    ASSERT((kp.priv[0] & 0x7) == 0, "私钥 clamp bit[2:0] = 0");
    ASSERT((kp.priv[31] & 0x80) == 0, "私钥 clamp bit255 = 0");
    ASSERT((kp.priv[31] & 0x40) != 0, "私钥 clamp bit254 = 1");
}

/* ── Test 2: ECDH 共享秘密一致性 ──────────────────────── */
static void test_ecdh(void) {
    printf("\n[2] X25519 ECDH 共享秘密\n");
    MqfKeypair alice, bob;
    mqf_kp_generate(&alice);
    mqf_kp_generate(&bob);

    uint8_t sa[32], sb[32];
    mqf_derive_shared(&alice, bob.pub, sa);
    mqf_derive_shared(&bob, alice.pub, sb);

    ASSERT(memcmp(sa, sb, 32) == 0, "alice.ecdh(bob.pub) == bob.ecdh(alice.pub)");
    uint8_t zero[32] = {0};
    ASSERT(memcmp(sa, zero, 32) != 0, "共享秘密非全零");
    printf("      alice shared: "); for(int i=0;i<8;i++) printf("%02x",sa[i]); printf("...\n");
    printf("      bob   shared: "); for(int i=0;i<8;i++) printf("%02x",sb[i]); printf("...\n");
}

/* ── Test 3: AES-256-GCM 加密/解密 ────────────────────── */
static void test_aes_gcm(void) {
    printf("\n[3] AES-256-GCM 加密/解密\n");
    uint8_t key[32];
    memset(key, 0xA5, 32);
    uint8_t nonce[12];
    memset(nonce, 0, 12);

    const char *pt = "Hello, 囡囝共生元!";
    size_t pt_len = strlen(pt);

    uint8_t ct_buf[64];
    uint8_t tag[16];
    uint8_t pt_out[64];

    ASSERT(mqf_gcm_encrypt(key, nonce, NULL, 0,
                           (const uint8_t*)pt, pt_len, ct_buf, tag) == 0,
           "GCM 加密成功");
    ASSERT(memcmp(ct_buf, pt, pt_len) != 0, "密文 ≠ 明文");

    ASSERT(mqf_gcm_decrypt(key, nonce, NULL, 0,
                           ct_buf, pt_len, tag, pt_out) == 0,
           "GCM 解密成功");
    pt_out[pt_len] = '\0';
    ASSERT(strcmp((char*)pt_out, pt) == 0, "解密后恢复原始明文");
    printf("      原文: %s\n", pt);
    printf("      解密: %s\n", pt_out);
}

/* ── Test 4: GCM 篡改检测 ─────────────────────────────── */
static void test_gcm_tamper(void) {
    printf("\n[4] GCM 篡改检测\n");
    uint8_t key[32], nonce[12], ct[64], tag[16], pt[64];
    memset(key, 0x5A, 32);
    memset(nonce, 0, 12);

    uint8_t msg[] = "MQF confidential message";
    mqf_gcm_encrypt(key, nonce, NULL, 0, msg, sizeof(msg), ct, tag);

    /* 篡改一个密文字节 */
    ct[5] ^= 0xFF;
    int rc = mqf_gcm_decrypt(key, nonce, NULL, 0, ct, sizeof(msg), tag, pt);
    ASSERT(rc == MQF_CRYPTO_AUTH, "篡改密文 → 认证失败 (GQ 拒绝)");
    ct[5] ^= 0xFF;  /* 还原 */

    /* 篡改 tag */
    tag[0] ^= 0xFF;
    rc = mqf_gcm_decrypt(key, nonce, NULL, 0, ct, sizeof(msg), tag, pt);
    ASSERT(rc == MQF_CRYPTO_AUTH, "篡改 tag → 认证失败");
    tag[0] ^= 0xFF; /* 还原 */

    /* 正确 tag */
    rc = mqf_gcm_decrypt(key, nonce, NULL, 0, ct, sizeof(msg), tag, pt);
    ASSERT(rc == 0, "正确 tag → 解密成功");
}

/* ── Test 5: NonceTracker 重放检测 ────────────────────── */
static void test_nonce_tracker(void) {
    printf("\n[5] NonceTracker 重放检测\n");
    MqfNonceTracker nt;
    mqf_nonce_init(&nt);

    ASSERT(mqf_nonce_check_and_mark(&nt, 1000, 0) == 0, "nonce 1000 首次通过");
    ASSERT(mqf_nonce_check_and_mark(&nt, 1001, 0) == 0, "nonce 1001 通过");
    ASSERT(mqf_nonce_check_and_mark(&nt, 1000, 0) == MQF_CRYPTO_REPLAY,
           "nonce 1000 二次使用 → 重放检测命中");
    ASSERT(mqf_nonce_check_and_mark(&nt, 1002, 0) == 0, "nonce 1002 通过");
}

/* ── Test 6: 完整握手 + 加密消息 ──────────────────────── */
static void test_full_handshake(void) {
    printf("\n[6] 唤魂 → 招魂 → 加密消息 端到端\n");

    MqfSession alice, bob;
    ASSERT(mqf_session_huanhun(&alice) == 0, "唤魂(alice) 成功");
    ASSERT(mqf_session_huanhun(&bob) == 0, "唤魂(bob) 成功");

    /* 交换公钥 */
    ASSERT(mqf_session_zhaohun(&alice, bob.local_kp.pub) == 0,
           "alice 招魂 bob.pub → session key");
    ASSERT(mqf_session_zhaohun(&bob, alice.local_kp.pub) == 0,
           "bob 招魂 alice.pub → session key");

    /* 双方 session key 一致 */
    ASSERT(memcmp(alice.session_key, bob.session_key, 32) == 0,
           "会话密钥一致");

    /* alice 加密 → bob 解密 */
    uint32_t msg[] = {
        MCP(MCCP_HANZI, 1000, 4, MQF_STATE_NAN_JIAN),
        MCP(MCCP_HANZI, 1001, 2, MQF_STATE_NAN_JIAN),
        MCP(MCCP_HANZI, 1002, 2, MQF_STATE_NAN_JIAN),
        MCP(MCCP_HANZI, 1003, 3, MQF_STATE_NAN_JIAN),
        MCP_HUANHUN,
    };
    size_t frame_len = 256;
    uint8_t frame[256];
    ASSERT(mqf_session_encrypt(&alice, msg, 5, frame, &frame_len) == 0,
           "alice 加密 MCCP 消息");

    uint32_t decoded[32];
    uint16_t decoded_len;
    ASSERT(mqf_session_decrypt(&bob, frame, frame_len, decoded, &decoded_len) == 0,
           "bob 解密成功");
    ASSERT(decoded_len == 5, "解密后消息长度 = 5");
    ASSERT(memcmp(msg, decoded, 5 * sizeof(uint32_t)) == 0,
           "解密后 MCCP token 与原文一致");

    /* bob 回复 */
    uint32_t reply[] = { MCP_DALING, MCP_SEP };
    size_t reply_len = 128;
    uint8_t reply_frame[128];
    ASSERT(mqf_session_encrypt(&bob, reply, 2, reply_frame, &reply_len) == 0,
           "bob 加密回复");
    uint32_t reply_dec[8];
    uint16_t reply_dec_len;
    ASSERT(mqf_session_decrypt(&alice, reply_frame, reply_len,
                                reply_dec, &reply_dec_len) == 0,
           "alice 解密 bob 回复");
    ASSERT(reply_dec[0] == MCP_DALING, "回复内容 = 答灵 (DALING)");
}

int main(void) {
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║    囡囝共生 · 加密层 C 测试 (libcrypto + libsodium)║\n");
    printf("║    无 Python, 无 MIT/Apache                     ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");

    test_kp_gen();
    test_ecdh();
    test_aes_gcm();
    test_gcm_tamper();
    test_nonce_tracker();
    test_full_handshake();

    printf("\n════════════════════════════════════════════════════\n");
    if (g_fail == 0) {
        printf("全 部 加 密 测 试 通 过 ✓\n");
        return 0;
    }
    printf("失 败 %d 项 ✗\n", g_fail);
    return 1;
}
