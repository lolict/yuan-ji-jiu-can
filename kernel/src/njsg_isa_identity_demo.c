/**
 * njsg_isa_identity_demo.c — ISA 指纹 + 离线认证 桥接演示
 * 编译: cc njsg_isa_identity_demo.c -o njsg_isa_identity_demo -lm
 */
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "njsg_isa_identity.c"

int main(void) {
    printf("=============================================\n");
    printf("  囡囝共生 · ISA 指纹驱动的离线身份 v1.0\n");
    printf("  桥接模块: ISA语义表 ↔ 离线认证引擎\n");
    printf("=============================================\n\n");

    /* Step 1: 设备能力报告 */
    isa_identity_report("sovereign:lolict");

    /* Step 2: 设备注册 — 身份锚点 + ISA 指纹 → 离线能力 */
    offline_cap_t cap = {0};
    char fp[64] = {0};
    isa_capability_t cap_info;

    isa_identity_bind("sovereign:lolict", &cap, fp, sizeof(fp), &cap_info);
    offline_status(&cap);

    printf("\n  📌 注册完成 — 设备指纹已绑定至离线能力\n");
    printf("  📌 此设备后续离线认证均依赖此 ISA 指纹\n");

    /* Step 3: 模拟设备重启 — 指纹不变 */
    printf("\n=== 模拟设备重启 — 指纹不变 ===\n");
    int device_ok = isa_identity_verify_device(fp, &cap);
    printf("[验证] 设备指纹一致性: %s\n", device_ok ? "✓ 设备未变" : "✗ 设备变更!");

    /* Step 4: TOTP 续期 — 必须先通过 ISA 验证 */
    printf("\n=== 离线续期: 双重验证 ===\n");
    char totp[7];
    offline_totp_now(&cap, totp);
    printf("[用户] 出示 TOTP: %s\n", totp);

    /* 方式 A: 双重验证 (ISA 指纹 + TOTP) */
    int renewed = isa_identity_renew_offline(fp, &cap, totp);
    printf("[结果] 双重验证续期: %s\n", renewed ? "✓ 通过" : "✗ 失败");

    /* 方式 B: 模拟设备被替换 (错误指纹) */
    printf("\n=== 模拟攻击: 设备被替换 ===\n");
    char fake_fp[] = "DEAD-BEEF1234-5678";
    int attack = isa_identity_renew_offline(fake_fp, &cap, totp);
    printf("[结果] 伪造指纹续期: %s (应失败)\n", attack ? "✓ 通过(异常!)" : "✗ 拒绝(正确)");

    /* 方式 C: 单纯 TOTP 验证 (不验 ISA 指纹) */
    printf("\n=== 对比: 仅 TOTP 验证 (跳过 ISA 指纹) ===\n");
    int only_totp = offline_totp_verify(&cap, totp, 1);
    printf("[结果] 纯 TOTP 续期: %s (无设备绑定)\n", only_totp ? "✓ 通过" : "✗ 失败");

    printf("\n=== 总结 ===\n");
    printf("  1. ISA 指纹 = 设备不可伪造的硬件身份\n");
    printf("  2. 双重验证 = ISA 设备未变 + TOTP 未过期\n");
    printf("  3. 离线场景下, 即使无网络/服务器也能完成身份核验\n");
    printf("  4. 指纹变化(硬件替换/能力降级) → 自动拒绝\n");
    printf("=============================================\n");
    return 0;
}
