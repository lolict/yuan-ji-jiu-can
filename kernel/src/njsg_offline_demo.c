/**
 * 离线认证独立演示 — njsg_offline_demo.c
 * 编译: gcc njsg_offline_demo.c -o njsg_offline_demo -lm
 */

#include <stdio.h>
#include <string.h>
#include <unistd.h>

/* 直接引入离线认证引擎 */
#define NJSG_OFFLINE_AUTH_TEST
#include "njsg_offline_auth.c"

int main(int argc, char** argv) {
    (void)argc; (void)argv;
    printf("============================================\n");
    printf("  囡囝共生 · 离线认证引擎 v1.0 Demo\n");
    printf("  场景: 完全断网下主权人身份核验与续期\n");
    printf("============================================\n\n");

    /* ---- 模拟身份绑定 (首次注册时, 在线完成一次) ---- */
    offline_cap_t cap;
    offline_bind(&cap, "sovereign:lolict");
    offline_status(&cap);

    /* ---- 场景 1: 挑战-响应 (身份核验) ---- */
    printf("\n=== 场景 1: 挑战-响应身份核验 ===\n");
    printf("[用户] 请求核验身份...\n");

    char challenge[33] = {0};
    if (offline_challenge_issue(&cap, challenge) != 0) {
        printf("[错误] 已有未完成挑战\n");
        return 1;
    }
    printf("[系统] 请用预共享密钥对 challenge 签名并回传 response\n");

    /* 客户端用 secret + challenge 算 response (预共享密钥已有) */
    char response[NJSG_HASH_LEN + 1];
    offline_respond(challenge, cap.secret, response);
    printf("[用户] 提交 response: %s\n", response);

    /* 内核验 response */
    int ok = offline_response_verify(&cap, response);
    printf("[结果] 挑战-响应 %s\n", ok ? "通过✓" : "失败✗");

    /* ---- 场景 2: TOTP 续期能力 (过期后不联网续命) ---- */
    printf("\n=== 场景 2: TOTP 离线续期 ===\n");

    /* 获取当前 TOTP (用户从手表/独立设备读 6 位数) */
    char totp[7];
    offline_totp_now(&cap, totp);
    printf("[用户] 当前 TOTP (6位): %s\n", totp);

    /* 模拟能力过期后, 提交 TOTP 续期 */
    printf("[用户] 提交 TOTP 请求续期...\n");
    ok = offline_totp_verify(&cap, totp, 1);
    printf("[结果] TOTP 续期 %s\n", ok ? "通过✓" : "失败✗");

    /* 故意提交错误 TOTP */
    printf("\n[用户] 提交错误 TOTP: 000000\n");
    ok = offline_totp_verify(&cap, "000000", 1);
    printf("[结果] 错误 TOTP %s (应失败)\n", ok ? "通过✓ (异常!)" : "拒绝✓");

    /* ---- 最终状态 ---- */
    printf("\n");
    offline_status(&cap);

    printf("\n[总结] 在不连接任何外部服务器的情况下:\n");
    printf("  1. 挑战-响应验证了用户持有预共享密钥\n");
    printf("  2. TOTP 实现了基于时间的本地能力续期\n");
    printf("  3. 所有核验完全依赖本地密钥与时钟, 零网络\n");
    printf("============================================\n");
    return 0;
}
