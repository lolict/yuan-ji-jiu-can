// test_membrane_valve.c — 膜妻阀单元测试
#include "membrane_valve.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int tests_passed = 0;
static int tests_total  = 0;

#define TEST(name) do { \
    tests_total++; \
    printf("[测试] %s ... ", #name); \
    if (name()) { tests_passed++; printf("✓\n"); } \
    else { printf("✗\n"); } \
} while(0)

// 测试: 创建膜妻阀
static int test_create(void) {
    membrane_valve_t* mv = mv_create("满全法");
    if (!mv) return 0;
    if (strcmp(mv->name, "满全法") != 0) return 0;
    if (mv->phonetic_count == 0) return 0;  // 谐音表应已初始化
    if (mv->is_active) return 0;
    mv_destroy(mv);
    return 1;
}

// 测试: 激活
static int test_activate(void) {
    membrane_valve_t* mv = mv_create("满全法");
    if (!mv_activate(mv)) return 0;
    if (!mv->is_active) return 0;
    mv_destroy(mv);
    return 1;
}

// 测试: 谐音编码
static int test_phonetic_encode(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    const char* hz = mv_phonetic_encode(mv, "fuqi");
    if (!hz) return 0;
    if (strcmp(hz, "缚妻膜蛹") != 0) return 0;

    hz = mv_phonetic_encode(mv, "manquanfa");
    if (!hz) return 0;
    if (strcmp(hz, "满全法") != 0) return 0;

    // 不存在的拼音
    hz = mv_phonetic_encode(mv, "notexist");
    if (hz) return 0;

    mv_destroy(mv);
    return 1;
}

// 测试: 吉凶归属解释
static int test_interpret_fortune(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    fortune_type_t f = mv_interpret_fortune(mv, "缚妻膜蛹");
    if (f != FORTUNE_AUSPICIOUS) return 0;

    f = mv_interpret_fortune(mv, "吉");
    if (f != FORTUNE_AUSPICIOUS) return 0;

    f = mv_interpret_fortune(mv, "凶");
    if (f != FORTUNE_INAUSPICIOUS) return 0;

    f = mv_interpret_fortune(mv, "unknown");
    if (f != FORTUNE_NEUTRAL) return 0;

    mv_destroy(mv);
    return 1;
}

// 测试: 注册变体
static int test_register_variant(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    mv_register_variant(mv, MV_VARIANT_FU_QI, "缚妻");
    mv_register_variant(mv, MV_VARIANT_FU_QUAN, "缚圈");
    mv_register_variant(mv, MV_VARIANT_FU_QING, "缚情");

    if (mv->variant_count != 3) return 0;
    if (mv->active_variants[0] != MV_VARIANT_FU_QI) return 0;

    mv_destroy(mv);
    return 1;
}

// 测试: 创建意识包
static int test_create_packet(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    const char* payload = "意识传输测试";
    mv_packet_header_t* pkt = mv_create_packet(mv, MV_VARIANT_FU_QI, 100,
        payload, strlen(payload) + 1);
    if (!pkt) return 0;
    if (pkt->magic != 0x404040) return 0;
    if (pkt->source_id != 2) return 0;  // 工藤新一=2 (S_KNDO)
    if (pkt->target_id != 100) return 0;
    if (pkt->variant != MV_VARIANT_FU_QI) return 0;

    free(pkt);
    mv_destroy(mv);
    return 1;
}

// 测试: 意识加密
static int test_encrypt(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    const char* payload = "秘密意识内容";
    mv_packet_header_t* pkt = mv_create_packet(mv, MV_VARIANT_FU_QING, 200,
        payload, strlen(payload) + 1);

    // 加密前载荷应为明文
    uint8_t* data = (uint8_t*)pkt + sizeof(mv_packet_header_t);
    if (memcmp(data, payload, strlen(payload) + 1) != 0) {
        free(pkt); mv_destroy(mv); return 0;
    }

    // 加密
    mv_encrypt_packet(mv, pkt);
    if (!pkt->encrypted) { free(pkt); mv_destroy(mv); return 0; }

    // 加密后载荷应不同于明文
    if (memcmp(data, payload, strlen(payload) + 1) == 0) {
        free(pkt); mv_destroy(mv); return 0;
    }

    free(pkt);
    mv_destroy(mv);
    return 1;
}

// 测试: 发送意识包
static int test_send(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    mv_packet_header_t* pkt = mv_create_packet(mv, MV_VARIANT_FU_QUAN, 300, "test", 5);
    if (!mv_send(mv, pkt)) { free(pkt); mv_destroy(mv); return 0; }
    if (mv->packets_sent != 1) { free(pkt); mv_destroy(mv); return 0; }
    if (!pkt->encrypted) { free(pkt); mv_destroy(mv); return 0; }  // 自动加密

    free(pkt);
    mv_destroy(mv);
    return 1;
}

// 测试: 结界阻挡
static int test_block(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    // 外部ID应被阻挡
    if (!mv_check_block(mv, 999)) return 0;
    if (mv->packets_blocked != 1) return 0;

    // 源ID=1 (工藤新一)不应被阻挡
    if (mv_check_block(mv, 1)) return 0;

    mv_destroy(mv);
    return 1;
}

// 测试: 变体名称映射
static int test_variant_names(void) {
    membrane_valve_t* mv = mv_create("满全法");
    mv_activate(mv);

    // 注册多个缚X变体
    mv_register_variant(mv, MV_VARIANT_FU_QIU, "缚躯");
    mv_register_variant(mv, MV_VARIANT_FU_QI, "缚妻");
    mv_register_variant(mv, MV_VARIANT_FU_QIU2, "缚囚");
    mv_register_variant(mv, MV_VARIANT_FU_QUAN, "缚圈");
    mv_register_variant(mv, MV_VARIANT_FU_KE, "缚壳");
    mv_register_variant(mv, MV_VARIANT_FU_QIAO, "缚鞘");
    mv_register_variant(mv, MV_VARIANT_FU_QUN, "缚裙");

    if (mv->variant_count != 7) return 0;

    mv_destroy(mv);
    return 1;
}

int main(void) {
    printf("\n═══ 膜妻阀 (Membrane Valve) 单元测试 ═══\n\n");

    TEST(test_create);
    TEST(test_activate);
    TEST(test_phonetic_encode);
    TEST(test_interpret_fortune);
    TEST(test_register_variant);
    TEST(test_create_packet);
    TEST(test_encrypt);
    TEST(test_send);
    TEST(test_block);
    TEST(test_variant_names);

    printf("\n结果: %d/%d 通过\n", tests_passed, tests_total);
    return (tests_passed == tests_total) ? 0 : 1;
}
