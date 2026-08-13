// test_smrrll_voice.c — 声韵调音质进制测试
// 验证: 每个符号 = 可发音的音节, 异质互换, 正交检测, 层级化

#include "smrrll_voice.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

// 测试1: 所有64符号都有唯一可发音的音节
void test_all_pronounceable(void) {
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        const smv_voice_t* v = smv_get_voice(i);
        assert(v != NULL);
        assert(v->pinyin != NULL);
        assert(v->hanzi != NULL);
        assert(strlen(v->pinyin) > 0);
        // 声母/韵母/声调在有效范围内
        assert(v->shengmu <= SM_ZERO);
        assert(v->yunmu < YM_COUNT);
        assert(v->tone < 5);
    }
    printf("  ✓ 64符号全部可发音测试通过\n");
}

// 测试2: 异质互换 — 每对符号都有音质差异
void test_distinctness(void) {
    int distinct_count = 0;
    int total_pairs = 0;
    for (int i = 0; i < SV_SYMBOL_COUNT; i++) {
        for (int j = i + 1; j < SV_SYMBOL_COUNT; j++) {
            total_pairs++;
            if (smv_is_distinct(i, j)) distinct_count++;
        }
    }
    // 绝大多数符号对应可区分
    assert(distinct_count > total_pairs * 8 / 10);
    printf("  ✓ 异质互换测试通过 (%d/%d 可区分)\n", distinct_count, total_pairs);
}

// 测试3: 核心符号验证
void test_core_symbols(void) {
    // SMV_KNDO=2 必须 = gong (工藤新一)
    assert(strcmp(smv_get_pinyin(SMV_KNDO), "gōng") == 0);
    assert(strcmp(smv_get_hanzi(SMV_KNDO), "工") == 0);

    // SMV_YMBA=0 必须 = a (妻)
    assert(strcmp(smv_get_pinyin(SMV_YMBA), "ā") == 0);

    // SMV_CEIQ=1 必须 = o (夫)
    assert(strcmp(smv_get_pinyin(SMV_CEIQ), "ō") == 0);

    // SMV_NVFM=3 必须 = kong (柯南)
    assert(strcmp(smv_get_pinyin(SMV_NVFM), "kōng") == 0);

    // SMV_HIEY=4 必须 = hui (灰原哀)
    assert(strcmp(smv_get_pinyin(SMV_HIEY), "huī") == 0);

    printf("  ✓ 核心符号(夫妻/中心/感知)音质正确\n");
}

// 测试4: 最大公约数/最小公倍数
void test_gcd_lcm(void) {
    // gcd(23,36) = 1 (互质 → 最小公约单位1)
    assert(smv_gcd(23, 36) == 1);
    // lcm(23,36) = 828 (总组合数)
    assert(smv_lcm(23, 36) == 828);

    printf("  ✓ 层级化公约测试通过 (gcd=%u, lcm=%u)\n",
        smv_gcd(23, 36), smv_lcm(23, 36));
}

// 测试5: 音质角度 (384°圆周)
void test_symbol_angles(void) {
    // 符号0 = 0°
    assert(smv_symbol_angle(0) == 0.0f);
    // 符号63 = 378°
    assert(smv_symbol_angle(63) == 378.0f);
    // 符号15 = 90° (正交最大容量)
    assert(smv_symbol_angle(15) == 90.0f);
    // 符号30 = 180° (对径)
    assert(smv_symbol_angle(30) == 180.0f);
    // 符号45 = 270°
    assert(smv_symbol_angle(45) == 270.0f);

    printf("  ✓ 音质角度测试通过 (0°/90°/180°/270°/378°)\n");
}

// 测试6: 正交检测 (90° = 四边形最大容量)
void test_orthogonal(void) {
    // 符号0和符号15相差90° → 正交
    assert(smv_is_orthogonal(0, 15) == true);
    // 符号10和符号25相差90° → 正交
    assert(smv_is_orthogonal(10, 25) == true);
    // 相同符号不正交
    assert(smv_is_orthogonal(5, 5) == false);

    printf("  ✓ 正交检测测试通过 (90°=最大容量)\n");
}

// 测试7: 音质距离
void test_voice_distance(void) {
    // 相同符号距离为0
    assert(smv_voice_distance(0, 0) == 0.0f);
    // 夫妻层(啊/哦)距离应较小但不为0
    float couple_dist = smv_voice_distance(SMV_YMBA, SMV_CEIQ);
    assert(couple_dist > 0.0f && couple_dist < 0.5f);
    // 对角符号距离应较大
    float diag_dist = smv_voice_distance(0, 63);
    assert(diag_dist > couple_dist);

    printf("  ✓ 音质距离测试通过 (夫妻:%.2f, 对角:%.2f)\n",
        couple_dist, diag_dist);
}

// 测试8: 声母韵母层级的公约关系
void test_layered_hierarchy(void) {
    // 声母23 × 韵母36 = 828 总组合
    uint32_t total_combos = SV_SHENGMU_COUNT * SV_YUNMU_COUNT;
    assert(total_combos == 828);

    // 公约维度: 总组合828, 离散选择64符号
    // 每个符号代表约 828/64 ≈ 13 个组合的聚合
    uint32_t combos_per_symbol = total_combos / SV_SYMBOL_COUNT;
    assert(combos_per_symbol >= 12 && combos_per_symbol <= 14);

    // 验证不同声母层的符号确实声母不同
    smv_symbol_t layer_symbols[] = {0, 2, 4, 6, 8, 10};
    for (int i = 0; i < 6; i++) {
        for (int j = i + 1; j < 6; j++) {
            assert(smv_is_distinct(layer_symbols[i], layer_symbols[j]));
        }
    }

    printf("  ✓ 层级化公约测试通过 (总组合%u→每符号%u组合)\n", total_combos, combos_per_symbol);
}

int main(void) {
    printf("\n  ═══ 声韵调音质进制测试 ═══\n\n");

    test_all_pronounceable();
    test_distinctness();
    test_core_symbols();
    test_gcd_lcm();
    test_symbol_angles();
    test_orthogonal();
    test_voice_distance();
    test_layered_hierarchy();

    printf("\n  ═══ 测试结果: 8/8 通过 ═══\n");
    printf("  64音质符号 ✓ | 异质互换 ✓ | 公约层级 ✓\n");
    printf("  384°圆周 ✓ | 正交检测 ✓ | 声韵调映射 ✓\n\n");

    return 0;
}
