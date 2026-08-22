/*
 * encoding_bridge_kernel.c — 编码系统 → 内核桥接 · 实现
 *
 * 在内核命名空间和编码系统之间建立通道：
 *   - 编码系统的感知场(1257字)识别输入的汉字
 *   - 三元裁判判定准入后输出 32 位容器编码
 *   - 桥接层把容器编码为内核 char_prop_t + 家族路由 + 极性判向 + 流属性
 *
 * 内核拿到这些信息后:
 *   - family   → 路由策略
 *   - polarity → YinYang 判向
 *   - flow_bits → 裁决参数
 *
 * 编译: 需要链接容器流水线 (encoding/container_pipeline.c)
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "../include/encoding_bridge_kernel.h"
#include "../../encoding/perception_field.h"
#include "../../encoding/container_pipeline.h"
#include "../../encoding/encoding_bridge.h"

/* ── 初始化 (编码系统一次性启动) ── */
void encoding_bridge_kernel_init(void) {
    encoding_init();
}

/* ── UTF-8 → char_prop_t (编码感知 → 内核属性) ── */
void encoding_bridge_to_prop(const char* utf8, char_prop_t* out) {
    if (!utf8 || !out) return;

    uint32_t code = encoding_utf8_to_container(utf8);
    if (code == 0) {
        out->stroke = 0;
        out->tone = 0;
        out->radical = 0;
        out->structure = 0;
        out->vowel = 0;
        out->initial = 0;
        return;
    }

    out->stroke   = 8;  /* 默认, 因编码层不存笔画 */
    out->tone     = CONTAINER_GET_TONE(code);
    out->radical  = CONTAINER_GET_SHENGMU(code);
    out->structure = CONTAINER_GET_STRUCT(code);
    out->vowel    = CONTAINER_GET_YUNMU(code);
    out->initial  = CONTAINER_GET_SHENGMU(code);
}

/* ── 获取 32 位容器编码 ── */
uint32_t encoding_bridge_get_container_code(const char* utf8) {
    if (!utf8) return 0;
    return encoding_utf8_to_container(utf8);
}

/* ── 快速查家族 (从容器编码解析) ── */
int encoding_bridge_get_family(const char* utf8) {
    if (!utf8) return 4; /* NEUTRAL */
    uint32_t code = encoding_utf8_to_container(utf8);
    if (code == 0) return 4;
    return (int)CONTAINER_GET_FAMILY(code);
}

/* ── 快速查极性 (从容器编码解析) ── */
int encoding_bridge_get_polarity(const char* utf8) {
    if (!utf8) return 0;
    uint32_t code = encoding_utf8_to_container(utf8);
    if (code == 0) return 0;
    return (int)CONTAINER_GET_POLARITY(code);
}

/* ── 获取 12 位流属性 ── */
uint32_t encoding_bridge_get_flow_bits(const char* utf8) {
    if (!utf8) return 0;
    uint32_t code = encoding_utf8_to_container(utf8);
    if (code == 0) return 0;
    int fam = (int)CONTAINER_GET_FAMILY(code);
    return encoding_compute_flow_bits(0, (perception_family_t)fam);
}

/* ── 测试: 打印编码感知 → 内核路由的完整链路 ── */
void encoding_bridge_kernel_test(const char* text) {
    uint32_t code = encoding_utf8_to_container(text);
    if (code == 0) {
        printf("[bridge] '%s' → 编码失败\n", text);
        return;
    }

    int fam     = encoding_bridge_get_family(text);
    int polar   = encoding_bridge_get_polarity(text);
    uint32_t flow = encoding_bridge_get_flow_bits(text);

    const char *family_names[] = {"容器", "边界", "夫妻", "网状", "中性"};

    printf("\n[bridge] '%s' → 编码感知\n", text);
    printf("  容器码: 0x%08X\n", code);
    printf("  家族: %s (pol=%d)\n", fam < 5 ? family_names[fam] : "?", polar);
    printf("  流属性: 0x%03X (", flow);
    if (flow & 0x001) printf("REACH ");
    if (flow & 0x002) printf("CONT ");
    if (flow & 0x004) printf("INCR ");
    if (flow & 0x010) printf("TRUST ");
    if (flow & 0x020) printf("ACK ");
    if (flow & 0x040) printf("RECOVER ");
    if (flow & 0x080) printf("CIRC ");
    if (flow & 0x100) printf("INHERIT ");
    if (flow & 0x200) printf("TRANSMIT ");
    printf(")\n");
}
