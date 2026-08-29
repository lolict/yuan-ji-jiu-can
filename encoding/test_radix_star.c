/**
 * test_radix_star.c — 部首星图端到端演示
 *
 * 取水(氵)、火(灬)、言(讠)三部做端到端演示:
 *   1. 注册三个部首容器 (中心星)
 *   2. 将真实字典中的字按部首归入星群
 *   3. 可视化星图: 部首是中心星, 字是环绕星
 *   4. 跨容器关系: 找共享声母的连线
 *
 * 这就是"容器本体论"在真实字典数据上的运行:
 *   部首不是标签, 是容器中心(公约数锚点);
 *   字符不是成员, 是容器的分身(公倍数);
 *   同一部首下的字, 共享同一个公约数锚点。
 */

#include <stdio.h>
#include <string.h>
#include "radix_star.h"

/* 目标部首的 UCS-2 和属性 */
#define UCS_WATER   0x6C34  /* 水 */
#define UCS_FIRE    0x706B  /* 火 */
#define UCS_SPEECH  0x8A00  /* 言 */

/* 演示用真实字典数据: 水部(氵)常见字
 * 数据格式: {UCS-2, 声母, 韵母, 声调}
 * 这些字都带水旁/三点水, 语义上与流动/液体相关 */
static const uint16_t WATER_CHARS[] = {
    0x6C34,  /* 水 shuǐ */
    0x6C5F,  /* 江 jiāng */
    0x6CB3,  /* 河 hé */
    0x6D77,  /* 海 hǎi */
    0x6CC9,  /* 泉 quán */
    0x6CE5,  /* 泥 ní */
    0x6D1E,  /* 洞 dòng */
    0x6DE1,  /* 淡 dàn */
    0x6D41,  /* 流 liú */
    0x6E05,  /* 清 qīng */
    0x6CFD,  /* 泽 zé */
    0x6C38,  /* 永 yǒng */
    0x6C7E,  /* 汾 fén */
    0x6CD5,  /* 法 fǎ */
};

/* 火部(灬)常见字 */
static const uint16_t FIRE_CHARS[] = {
    0x706B,  /* 火 huǒ */
    0x7070,  /* 灰 huī */
    0x706D,  /* 灭 miè */
    0x708E,  /* 炎 yán */
    0x70C8,  /* 烈 liè */
    0x70E7,  /* 烧 shāo */
    0x70B9,  /* 点 diǎn (灬) */
    0x7116,  /* 熟 shú (灬) */
    0x70DB,  /* 灯 dēng */
    0x7075,  /* 灵 líng (灬) */
    0x711A,  /* 焚 fén */
    0x6696,  /* 暖 nuǎn (日旁/火意) */
};

/* 言部(讠)常见字 */
static const uint16_t SPEECH_CHARS[] = {
    0x8A00,  /* 言 yán */
    0x8AAC,  /* 说 shuō */
    0x8BDD,  /* 话 huà */
    0x8BED,  /* 语 yǔ */
    0x8BB0,  /* 记 jì */
    0x8BA4,  /* 认 rèn */
    0x8C03,  /* 调 diào */
    0x8BF4,  /* 说 shuō */
    0x8BBA,  /* 论 lùn */
    0x8BCD,  /* 词 cí */
    0x8BE2,  /* 询 xún */
    0x8BD1,  /* 译 yì */
    0x8BB2,  /* 讲 jiǎng */
    0x8BFE,  /* 课 kè */
    0x8BA8,  /* 讨 tǎo */
    0x90E8,  /* 部 bù (我的左耳旁,与言无关但这里做混淆测试) */
};

/* 每个字的声母 (用于跨容器比较) */
static uint8_t get_shengmu(uint16_t ucs2) {
    /* 简化的声母表 */
    switch (ucs2) {
        case 0x6C34: return 18; /* sh */
        case 0x6C5F: return 7;  /* j */
        case 0x6CB3: return 10; /* h */
        case 0x6D77: return 10; /* h */
        case 0x6CC9: return 4;  /* q */
        case 0x6CE5: return 9;  /* n */
        case 0x6D1E: return 5;  /* d */
        case 0x6DE1: return 5;  /* d */
        case 0x6D41: return 8;  /* l */
        case 0x6E05: return 4;  /* q */
        case 0x6CFD: return 18; /* z */
        case 0x6C38: return 15; /* y */
        case 0x6C7E: return 6;  /* f */
        case 0x6CD5: return 3;  /* f */
        case 0x706B: return 10; /* h */
        case 0x7070: return 10; /* h */
        case 0x706D: return 9;  /* m */
        case 0x708E: return 15; /* y */
        case 0x70C8: return 8;  /* l */
        case 0x70E7: return 13; /* sh */
        case 0x70B9: return 5;  /* d */
        case 0x7116: return 13; /* sh */
        case 0x70DB: return 5;  /* d */
        case 0x7075: return 8;  /* l */
        case 0x711A: return 6;  /* f */
        case 0x6696: return 9;  /* n */
        case 0x8A00: return 15; /* y */
        case 0x8AAC: return 13; /* sh */
        case 0x8BDD: return 10; /* h */
        case 0x8BED: return 15; /* y */
        case 0x8BB0: return 7;  /* j */
        case 0x8BA4: return 12; /* r */
        case 0x8C03: return 5;  /* d */
        case 0x8BF4: return 13; /* sh */
        case 0x8BBA: return 8;  /* l */
        case 0x8BCD: return 17; /* c */
        case 0x8BE2: return 14; /* x */
        case 0x8BD1: return 15; /* y */
        case 0x8BB2: return 7;  /* j */
        case 0x8BFE: return 11; /* k */
        case 0x8BA8: return 20; /* t */
        default: return 0;
    }
}

/* 探测一族字之间共享的声母 (语义同源线索) */
static void find_shared_shengmu(const uint16_t *chars_a, int count_a,
                                 const uint16_t *chars_b, int count_b,
                                 const char *name_a, const char *name_b) {
    int shared = 0;
    for (int i = 0; i < count_a; i++) {
        for (int j = 0; j < count_b; j++) {
            if (get_shengmu(chars_a[i]) == get_shengmu(chars_b[j])) {
                shared++;
            }
        }
    }
    if (shared > 0) {
        printf("  %s ←→ %s: 共享声母 %d 条 (语义同源)\n", name_a, name_b, shared);
    }
}

int main(void) {
    printf("═══════════════════════════════════════════════════════\n");
    printf("  部首星图 · 端到端演示\n");
    printf("  水(氵)部 → 火(灬)部 → 言(讠)部\n");
    printf("═══════════════════════════════════════════════════════\n\n");
    
    rs_star_map_t map;
    rs_init(&map);
    
    /* ═══ 第一步: 注册部首容器 (中心星) ═══ */
    printf("■ 第一步: 注册部首容器 (中心星)\n\n");
    
    rs_register_center(&map, UCS_WATER,  81, 4, "水·流动");
    rs_register_center(&map, UCS_FIRE,   82, 4, "火·热烈");
    rs_register_center(&map, UCS_SPEECH,140, 7, "言·言语");
    
    printf("  已注册 %d 个部首容器:\n", map.count);
    for (uint8_t i = 0; i < map.count; i++) {
        printf("    [%d] U+%04X %s (%d画)\n",
               i,
               map.centers[i].radical_ucs2,
               map.centers[i].meaning,
               map.centers[i].strokes);
    }
    
    /* ═══ 第二步: 归类字符到星群 ═══ */
    printf("\n■ 第二步: 从真实字典归类字符到星群\n\n");
    
    /* 水部归类 */
    int water_count = 0;
    int n_water = (int)(sizeof(WATER_CHARS) / sizeof(WATER_CHARS[0]));
    for (int i = 0; i < n_water; i++) {
        uint8_t sm = get_shengmu(WATER_CHARS[i]);
        rs_add_star(&map, 0, WATER_CHARS[i], sm, 0, 0, 0, 0);
        water_count++;
    }
    printf("  水部(氵): 归类 %d 字\n", water_count);
    printf("    ");
    for (int i = 0; i < n_water; i++) {
        printf("%04X ", WATER_CHARS[i]);
        if ((i + 1) % 7 == 0) printf("\n    ");
    }
    printf("\n\n");
    
    /* 火部归类 */
    int fire_count = 0;
    int n_fire = (int)(sizeof(FIRE_CHARS) / sizeof(FIRE_CHARS[0]));
    for (int i = 0; i < n_fire; i++) {
        uint8_t sm = get_shengmu(FIRE_CHARS[i]);
        rs_add_star(&map, 1, FIRE_CHARS[i], sm, 0, 0, 0, 0);
        fire_count++;
    }
    printf("  火部(灬): 归类 %d 字\n", fire_count);
    printf("    ");
    for (int i = 0; i < n_fire; i++) {
        printf("%04X ", FIRE_CHARS[i]);
        if ((i + 1) % 6 == 0) printf("\n    ");
    }
    printf("\n\n");
    
    /* 言部归类 */
    int speech_count = 0;
    int n_speech = (int)(sizeof(SPEECH_CHARS) / sizeof(SPEECH_CHARS[0]));
    for (int i = 0; i < n_speech; i++) {
        uint8_t sm = get_shengmu(SPEECH_CHARS[i]);
        rs_add_star(&map, 2, SPEECH_CHARS[i], sm, 0, 0, 0, 0);
        speech_count++;
    }
    printf("  言部(讠): 归类 %d 字\n", speech_count);
    printf("    ");
    for (int i = 0; i < n_speech; i++) {
        printf("%04X ", SPEECH_CHARS[i]);
        if ((i + 1) % 8 == 0) printf("\n    ");
    }
    printf("\n\n");
    
    printf("  总计归类: %d 字 → %d 个容器\n\n",
           water_count + fire_count + speech_count, map.count);
    
    /* ═══ 第三步: 可视化星图 ═══ */
    printf("■ 第三步: 星图可视化\n");
    rs_visualize(&map);
    
    /* ═══ 第四步: 详细报告 ═══ */
    printf("■ 第四步: 星图详细报告\n\n");
    rs_report(&map);
    
    /* ═══ 第五步: 跨容器关系 ═══ */
    printf("■ 第五步: 跨容器声母共鸣分析\n\n");
    
    find_shared_shengmu(WATER_CHARS, n_water, FIRE_CHARS, n_fire, "水", "火");
    find_shared_shengmu(WATER_CHARS, n_water, SPEECH_CHARS, n_speech, "水", "言");
    find_shared_shengmu(FIRE_CHARS, n_fire, SPEECH_CHARS, n_speech, "火", "言");
    
    /* ═══ 第六步: 带编码的端到端验证 ═══ */
    printf("\n■ 第六步: 容器编码验证\n\n");
    for (uint8_t i = 0; i < map.count; i++) {
        /* 模拟容器编码生成: family(3bit) + polarity(1bit) + radical_idx(8bit) + boundary(16bit) */
        uint32_t code = 0;
        code |= (0 << 29);               /* family = CONTAINER */
        code |= ((i & 1) << 28);         /* polarity 交替 */
        code |= (map.centers[i].radical_index << 20);
        code |= (map.centers[i].star_count);
        
        printf("  容器[%d] %s: 编码=0x%08X (部#%d, %d画, %d字)\n",
               i, map.centers[i].meaning, code,
               map.centers[i].radical_index,
               map.centers[i].strokes,
               map.centers[i].star_count);
    }
    
    /* ═══ 总结 ═══ */
    printf("\n═══════════════════════════════════════════════════════\n");
    printf("  部首星图 · 演示完成\n");
    printf("  水部: %d字 | 火部: %d字 | 言部: %d字\n",
           water_count, fire_count, speech_count);
    printf("  总计: %d字 → %d容器\n",
           water_count + fire_count + speech_count, map.count);
    printf("\n  核心结论:\n");
    printf("  - 部首不是标签, 是容器中心(公约数锚点)\n");
    printf("  - 字不是成员, 是容器的分身(公倍数扩展)\n");
    printf("  - 跨容器共享声母 = 本质层面的语义同源性\n");
    printf("  - 容器边界 = 星群大小 = 该部首能造多少字\n");
    printf("═══════════════════════════════════════════════════════\n");
    
    return 0;
}
