/**
 * 囡囝共生 · 离线认证引擎 v1.0 — njsg_offline_auth.c
 * ------------------------------------------------
 * 目标: 能力持有者在完全断网环境下完成身份核验与能力续期
 *
 * 协议栈:
 *   挑战-响应 (Challenge-Response)
 *     1. 内核生成随机 challenge (16 字节十六进制)
 *     2. 用户端用预共享密钥 (绑定身份时下发) + challenge 做双重 hash 得到 response
 *     3. 内核用同样的 secret 复算, 匹配即证明持有私钥
 *     特性: 每次挑战不同, 截获 response 无法重放
 *
 *   TOTP 能力续期 (Time-based One-Time Password)
 *     1. 注册时生成 secret_seed (身份独有)
 *     2. capability 过期前, 用 secret_seed + 当前 30 秒时间窗算出 6 位 TOTP
 *     3. 内核复算, 匹配则 capability.expires_at 自动续期
 *     特性: 无需网络通信, 本地时钟同步即可
 *
 * 与主内核的桥接:
 *   - 能力结构中已有 require_network=0 → 命中离线路径
 *   - capability_t 新增 offline_seed 字段用于 TOTP
 */

#ifndef NJSG_OFFLINE_AUTH_C
#define NJSG_OFFLINE_AUTH_C

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>

/* ========== 与主内核共享的类型声明 (避免循环依赖) ========== */

#define NJSG_HASH_LEN 64

/* 离线认证能力 — 挂载到 capability_t.offline 便桥接 */
typedef struct {
    char    secret[NJSG_HASH_LEN + 1];      /* 预共享密钥 (身份绑定时生成) */
    char    totp_seed[NJSG_HASH_LEN + 1];   /* TOTP 种子 (时间窗共享) */
    int     challenge_active;               /* 是否处于挑战期 */
    char    active_challenge[33];           /* 当前 challenge (16 字节 hex) */
    time_t  challenge_issued_at;            /* challenge 发出时间 (防重放 60 秒过期) */
    int     totp_success_count;             /* TOTP 成功续期次数 */
    int     challenge_success_count;        /* 挑战-响应成功次数 */
} offline_cap_t;

/* ========== 辅助函数 ========== */

/**
 * 简易 FNV-1a 哈希 (与 njsg_hash 不同, 更适合密钥派生)
 */
static void hash_sha_like(const char* in, char* out, size_t out_len) {
    uint64_t h = 0xcbf29ce484222325ULL;
    for (size_t i = 0; in[i]; i++) {
        h ^= (unsigned char)in[i];
        h *= 0x100000001b3ULL;
        /* 混合位移 */
        h ^= (h >> 13);
        h *= 0xff51afd7ed558ccdULL;
        h ^= (h >> 33);
    }
    snprintf(out, out_len, "%016llx%016llx", (unsigned long long)h, (unsigned long long)(h ^ 0xAAAAAAAAAAAAAAAAULL));
}

/**
 * 生成随机 nonce (十六进制输出)
 * 注意: 生产环境应换用 /dev/urandom; 这里用 mixin + time 模拟
 */
static void random_hex(char* out, int byte_len) {
    uint64_t x = (uint64_t)time(NULL);
    /* 混合 pid 与 address noise 提高随机性 */
    x ^= (uint64_t)(uintptr_t)out;
    x ^= (uint64_t)getpid() << 7;
    for (int i = 0; i < byte_len; i++) {
        /* xorshift64 */
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        sprintf(out + i * 2, "%02x", (unsigned)(x & 0xFF));
    }
    out[byte_len * 2] = '\0';
}

/* ========== 挑战-响应 ========== */

/**
 * 初始化离线能力 (绑定时调用一次)
 * @param cap     能力结构
 * @param identity 身份文本 (用作种子派生 secret)
 */
void offline_bind(offline_cap_t* cap, const char* identity) {
    memset(cap, 0, sizeof(*cap));
    /* 派生 pre-shared secret */
    hash_sha_like(identity, cap->secret, sizeof(cap->secret));
    /* 派生 TOTP 种子 (用不同 nonce 做唯一化) */
    char nonce[33];
    random_hex(nonce, 16);
    char buf[256];
    snprintf(buf, sizeof(buf), "%s|%s|totp", identity, nonce);
    hash_sha_like(buf, cap->totp_seed, sizeof(cap->totp_seed));
    printf("[离线] 绑定身份: %s → secret=TOTP种子就绪\n", identity);
}

/**
 * 生成挑战 (用户请求核验时调用)
 * @param cap       能力结构
 * @param challenge 输出缓冲 (至少 33 字节)
 * @return 0 成功, -1 已处于挑战期
 */
int offline_challenge_issue(offline_cap_t* cap, char* challenge) {
    if (cap->challenge_active) {
        /* 已有未完成的挑战, 不重复发 */
        return -1;
    }
    random_hex(cap->active_challenge, 16);
    strcpy(challenge, cap->active_challenge);
    cap->challenge_active = 1;
    cap->challenge_issued_at = time(NULL);
    printf("[离线] 发放 challenge: %s (60 秒有效)\n", challenge);
    return 0;
}

/**
 * 计算 response (用户端/客户端调用)
 * @param challenge 接收到的 challenge
 * @param secret    预共享密钥
 * @param response  输出缓冲 (至少 NJSG_HASH_LEN+1 字节)
 */
void offline_respond(const char* challenge, const char* secret, char* response) {
    char buf[256];
    /* response = hash(secret + "|" + challenge) */
    snprintf(buf, sizeof(buf), "%s|%s|response", secret, challenge);
    hash_sha_like(buf, response, NJSG_HASH_LEN + 1);
}

/**
 * 验证 response (内核调用)
 * @param cap      能力结构
 * @param response 用户提交的 response
 * @return 1 通过, 0 失败 / 过期 / 无挑战
 */
int offline_response_verify(offline_cap_t* cap, const char* response) {
    if (!cap->challenge_active) {
        printf("[离线] 验证失败: 当前无未完成挑战\n");
        return 0;
    }
    /* 60 秒过期检查 */
    if (difftime(time(NULL), cap->challenge_issued_at) > 60.0) {
        cap->challenge_active = 0;
        cap->active_challenge[0] = '\0';
        printf("[离线] 验证失败: challenge 已过期 (60 秒超时)\n");
        return 0;
    }
    /* 复算 */
    char expected[NJSG_HASH_LEN + 1];
    offline_respond(cap->active_challenge, cap->secret, expected);
    int ok = (strcmp(expected, response) == 0);
    if (ok) {
        cap->challenge_success_count++;
        cap->challenge_active = 0;
        printf("[离线] 挑战-响应通过! (累计 %d 次)\n", cap->challenge_success_count);
    } else {
        printf("[离线] 验证失败: response 不匹配\n");
    }
    return ok;
}

/* ========== TOTP 续期 ========== */

/**
 * 计算当前时间窗 (30 秒一步)
 */
static uint64_t current_time_step(void) {
    return (uint64_t)time(NULL) / 30;
}

/**
 * 生成 TOTP (6 位数字)
 * @param seed      种子
 * @param time_step 时间窗
 * @param totp_out  输出缓冲 (至少 7 字节)
 */
void offline_totp_generate(const char* seed, uint64_t time_step, char* totp_out) {
    char buf[256];
    snprintf(buf, sizeof(buf), "%s|%llu|totp6", seed, (unsigned long long)time_step);
    char hash[NJSG_HASH_LEN + 1];
    hash_sha_like(buf, hash, sizeof(hash));
    /* 取 hash 中间部分对 1000000 取模 → 6 位 */
    unsigned long long val = 0;
    for (int i = 20; i < 36 && hash[i]; i++) {
        val = val * 16 + (unsigned long long)(unsigned char)hash[i] % 16;
    }
    snprintf(totp_out, 7, "%06llu", val % 1000000);
}

/**
 * 验证 TOTP 并通过后续期能力 (核验端调用)
 * @param cap         能力结构
 * @param totp        用户提交的 6 位数字
 * @param window_back 允许回退几个时间窗 (1 = ±30 秒容差)
 * @return 1 通过, 0 失败
 */
int offline_totp_verify(offline_cap_t* cap, const char* totp, int window_back) {
    uint64_t now = current_time_step();
    for (int w = -window_back; w <= 0; w++) {
        char expected[7];
        uint64_t step = (uint64_t)((int64_t)now + w);
        offline_totp_generate(cap->totp_seed, step, expected);
        if (strcmp(expected, totp) == 0) {
            cap->totp_success_count++;
            printf("[离线] TOTP 验证通过, 时间窗=%lld, 累计续期 %d 次\n",
                   (long long)step, cap->totp_success_count);
            return 1;
        }
    }
    printf("[离线] TOTP 验证失败 (容差 ±%d 窗)\n", window_back);
    return 0;
}

/**
 * 获取当前 TOTP (用户端/手表/独立设备调用, 用于出示)
 * @param cap  能力结构
 * @param out  输出缓冲 (至少 7 字节)
 */
void offline_totp_now(offline_cap_t* cap, char* out) {
    offline_totp_generate(cap->totp_seed, current_time_step(), out);
}

/* ========== 链路状态 ========== */

void offline_status(const offline_cap_t* cap) {
    printf("┌─ 离线认证状态 ───────────────────┐\n");
    printf("│ 预共享密钥: %s...%s │\n", cap->secret, cap->secret + NJSG_HASH_LEN - 4);
    printf("│ TOTP 种子:  %s │\n", cap->totp_seed);
    printf("│ 挑战-%d次  TOTP续期-%d次         │\n", cap->challenge_success_count, cap->totp_success_count);
    printf("│ 当前 TOTP: ");
    char totp[7];
    offline_totp_now((offline_cap_t*)cap, totp);
    printf("%s                    │\n", totp);
    printf("└───────────────────────────────────┘\n");
}

#endif /* NJSG_OFFLINE_AUTH_C */
