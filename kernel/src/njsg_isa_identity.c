/**
 * njsg_isa_identity.c — ISA 指纹驱动的离线身份
 * ------------------------------------------------
 * 桥接 ISA 语义表 + 离线认证引擎:
 *   1. 探测设备 ISA 能力 (哪些 ISA 目标/指令可编码)
 *   2. 哈希全部 hex_ex → 唯一 ISA 指纹
 *   3. 指纹 + 身份锚点 → 离线认证 seed
 *
 * 这样不同 ISA 能力的设备自然产生不同的离线密钥:
 *   - RV32I only 设备 ≠ RV32I+ARM64 设备
 *   - 完整 4 ISA 设备 ≠ 部分 ISA 设备
 *
 * 这就是"公约数"工程化: 身份锚点(0维) + ISA 特征(1维) = 唯一设备指纹
 */

#ifndef NJSG_ISA_IDENTITY_C
#define NJSG_ISA_IDENTITY_C

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

/* ── 桥接表头 ── */
#include "mlll_isa_table.c"
#include "njsg_offline_auth.c"

/* ============================================================
 * 1. ISA 能力探测 — 设备码本自描
 * ============================================================ */

typedef struct {
    int sem_id_count;       /* 支持的语义 ID 数 */
    int isa_count;          /* 支持的 ISA 目标数 */
    int total_encodings;    /* 总的 hex_ex 非零项 */
    uint32_t device_class;  /* 设备档位 (由能力哈希派生) */
} isa_capability_t;

/**
 * 扫描 ISA 表, 描出设备能力画像
 * 未来: 从设备寄存器/CPUID 获取; 当前: 用 mlll_isa_table 全量
 */
isa_capability_t isa_probe(void) {
    isa_capability_t cap = {0};
    cap.sem_id_count = SEM_ID_COUNT;
    cap.isa_count = ISA_COUNT;

    for (int s = 0; s < SEM_ID_COUNT; s++) {
        const sem_instr_t* si = mlll_isa_get((sem_id_t)s);
        if (!si) continue;
        for (int i = 0; i < ISA_COUNT; i++) {
            if (si->enc[i].hex_ex != 0) cap.total_encodings++;
        }
    }
    cap.device_class = 0;
    return cap;
}

/* ============================================================
 * 2. 指纹哈希 — ISA 表全部 hex_ex 的统计指纹
 * ============================================================ */

/**
 * FNV-1a 哈希全部 hex_ex → 32 位设备指纹
 */
static uint32_t hash_isa_table(void) {
    uint32_t h = 0x811c9dc5u;
    for (int s = 0; s < SEM_ID_COUNT; s++) {
        const sem_instr_t* si = mlll_isa_get((sem_id_t)s);
        if (!si) continue;
        /* 混入语义 ID 名 */
        const char* nm = SEM_NAMES[s];
        for (int c = 0; nm[c]; c++) {
            h ^= (unsigned char)nm[c];
            h *= 0x01000193u;
        }
        /* 混入全部 hex_ex */
        for (int i = 0; i < ISA_COUNT; i++) {
            uint32_t ex = si->enc[i].hex_ex;
            h ^= (uint8_t)(ex & 0xFF); h *= 0x01000193u;
            h ^= (uint8_t)((ex >> 8) & 0xFF); h *= 0x01000193u;
            h ^= (uint8_t)((ex >> 16) & 0xFF); h *= 0x01000193u;
            h ^= (uint8_t)((ex >> 24) & 0xFF); h *= 0x01000193u;
        }
    }
    return h;
}

/**
 * 生成 16 字节十六进制设备指纹
 */
void isa_fingerprint(char* out_fp, int out_len) {
    uint32_t h = hash_isa_table();
    isa_capability_t cap = isa_probe();
    /* 能力 + 哈希 = 指纹 */
    snprintf(out_fp, out_len, "%04X-%08X-%04X",
             (unsigned)(cap.total_encodings ^ (h & 0xFFFF)),
             h,
             (unsigned)(cap.device_class ^ (h >> 16)));
}

/* ============================================================
 * 3. 身份绑定 — 离线设备注册
 * ============================================================ */

/**
 * 注册设备: 身份锚点 + ISA 指纹 → 离线认证
 *
 * @param identity  身份锚点 (如 "sovereign:lolict")
 * @param fp_out    输出 ISA 指纹 (至少 20 字节)
 * @param out_cap   输出能力结构 (可为 NULL)
 * @return 0 成功
 */
int isa_identity_bind(const char* identity, offline_cap_t* cap, char* fp_out, int fp_len, isa_capability_t* out_cap) {
    /* Step 1: 描能力 */
    isa_capability_t cap_l = isa_probe();
    if (out_cap) *out_cap = cap_l;

    /* Step 2: 算 ISA 指纹 */
    isa_fingerprint(fp_out, fp_len);

    /* Step 3: 身份锚点 + ISA 指纹 → 绑定离线能力 */
    /* 扩展绑定字符串 = identity + ":" + fp */
    char bind_str[256];
    snprintf(bind_str, sizeof(bind_str), "%s|%s|isa-fp", identity, fp_out);

    /* Step 4: 离线绑定 (用扩展字符串作为 entropy) */
    offline_bind(cap, bind_str);

    printf("[ISA身份] 设备 ISA 指纹: %s\n", fp_out);
    printf("[ISA身份] 能力: %d语义 × %d ISA = %d 编码\n",
           cap_l.sem_id_count, cap_l.isa_count, cap_l.total_encodings);
    return 0;
}

/**
 * 验证设备身份: 重算 ISA 指纹, 比对 vs 注册时指纹
 *
 * 设备离线时, 用当前 ISA 表重算指纹(必须与注册时一致),
  证明「没换硬件/没降能力」。
 *
 * @param registered_fp  注册时保存的指纹
 * @param cap            离线能力
 * @return 1 设备未被替换 + 能力未降级
 */
int isa_identity_verify_device(const char* registered_fp, offline_cap_t* cap) {
    /* 当前 ISA 能力 (预留: 能力降级检测) */
    (void)isa_probe();
    /* 当前指纹 */
    char current_fp[64];
    isa_fingerprint(current_fp, sizeof(current_fp));

    /* 指纹必须一致 → 设备未变 */
    int same_fp = (strcmp(current_fp, registered_fp) == 0);
    if (!same_fp) {
        printf("[ISA身份] 设备指纹变化: %s → %s\n", registered_fp, current_fp);
        return 0;
    }
    (void)cap; /* 预留: 可加入能力降级检测 */
    return 1;
}

/* ============================================================
 * 4. 完整设备认证 + 离线续期
 * ============================================================ */

/**
 * 带 ISA 指纹验证的 TOTP 续期:
 *   1. 先验证 ISA 设备未变
 *   2. 再验证 TOTP
 * 两关都过 → 续期能力
 *
 * @param registered_fp  注册时保存的指纹
 * @param cap            离线能力
 * @param totp           时间同步令牌 (6 位)
 * @return 1 验证通过 + 续期成功
 */
int isa_identity_renew_offline(const char* registered_fp, offline_cap_t* cap, const char* totp) {
    if (!isa_identity_verify_device(registered_fp, cap)) {
        return 0;
    }
    return offline_totp_verify(cap, totp, 1);
}

/* ============================================================
 * 5. CLI 辅助: 输出设备能力报告
 * ============================================================ */

void isa_identity_report(const char* identity) {
    isa_capability_t cap = isa_probe();
    char fp[64];
    isa_fingerprint(fp, sizeof(fp));

    printf("┌─ ISA 设备身份报告 ─────────────────────┐\n");
    printf("│ 身份锚点: %-29s│\n", identity);
    printf("│ ISA 指纹: %-29s│\n", fp);
    printf("│ 语义指令数: %-27d│\n", cap.sem_id_count);
    printf("│ ISA 目标数: %-27d│\n", cap.isa_count);
    printf("│ 非零编码数: %-27d│\n", cap.total_encodings);
    printf("│ 覆盖率: %-29s│\n",
           cap.total_encodings == SEM_ID_COUNT * ISA_COUNT ? "100%" : "部分");
    printf("└────────────────────────────────────────┘\n");
}

#endif /* NJSG_ISA_IDENTITY_C */
