/*
 * mlll_pinyin_demo.c — 汉字拼音知识库 + 夫妻对偶查询 演示
 *
 * 用法: make mlll-pinyin
 */
#include <stdio.h>
#include <string.h>
#include "mlll_symbols.h"
#include "mlll_pinyin.h"

int main(void) {
    mlll_init(0.1);

    printf("════════════════════════════════════════════\n");
    printf("  MLLL 汉字拼音知识库 + 夫妻对偶查询 (v2.0)\n");
    printf("  九维 (904 symbols) + 知识库 (%d entries)\n", mlll_pinyin_count());
    printf("════════════════════════════════════════════\n");

    printf("\n[I] 知识库基础信息\n");
    printf("  总条目: %d\n", mlll_pinyin_count());
    printf("  九维符号: %u\n", REG.count);
    printf("  MLLL 音节巢: 24声母×39韵母×8声调 = %d\n", 24*39*8);

    printf("\n[II] 按汉字查找拼音/九维属性\n");
    {
        const char* test_chars[] = {"囡","囝","共","生","刘","楚","恬","秋","梦"};
        int n = sizeof(test_chars) / sizeof(test_chars[0]);
        for (int i = 0; i < n; i++) {
            const pinyin_entry_t* e = mlll_pinyin_lookup_by_char(test_chars[i]);
            if (e)
                printf("  '%s' → pinyin=%-6s stroke=%d tone=%d rad=%d struct=%d vowel=%d initial=%d\n",
                       test_chars[i], e->pinyin, e->prop.stroke, e->prop.tone,
                       e->prop.radical, e->prop.structure, e->prop.vowel, e->prop.initial);
            else
                printf("  '%s' → (未收录)\n", test_chars[i]);
        }
    }

    printf("\n[III] 按拼音反查汉字\n");
    {
        /* 找所有声母=6(n) + 韵母=15(an) 的字 */
        int out[32];
        int cnt = mlll_pinyin_find_by_pinyin(6, 15, -1, out, 32);
        printf("  声母=n(6) + 韵母=an(15) → %d 字:", cnt);
        for (int i = 0; i < cnt; i++) {
            const pinyin_entry_t* e = mlll_pinyin_get(out[i]);
            printf(" %s", e->utf8);
        }
        printf("\n");

        /* 找所有声母=11(j) 的字 */
        cnt = mlll_pinyin_find_by_pinyin(11, -1, -1, out, 32);
        printf("  声母=j(11) → %d 字:", cnt);
        for (int i = 0; i < cnt && i < 10; i++) {
            const pinyin_entry_t* e = mlll_pinyin_get(out[i]);
            printf(" %s", e->utf8);
        }
        printf("%s\n", cnt > 10 ? " ..." : "");
    }

    printf("\n[IV] 夫妻对偶拼音查询（互补度最高）\n");
    {
        const char* test_names[] = {"刘","楚","恬","秋"};
        int n = sizeof(test_names) / sizeof(test_names[0]);
        for (int i = 0; i < n; i++) {
            int spouses[5];
            int cnt = mlll_find_spouse(test_names[i], spouses, 5);
            const pinyin_entry_t* me = mlll_pinyin_lookup_by_char(test_names[i]);
            if (!me) continue;
            printf("  '%s'(initial=%d,vowel=%d,tone=%d) 夫妻对偶:",
                   test_names[i], me->prop.initial, me->prop.vowel, me->prop.tone);
            for (int j = 0; j < cnt; j++) {
                const pinyin_entry_t* s = mlll_pinyin_get(spouses[j]);
                printf(" %s(initial=%d,v=%d,t=%d)",
                       s->utf8, s->prop.initial, s->prop.vowel, s->prop.tone);
            }
            printf("\n");
        }
    }

    printf("\n[V] 九维坐标自动映射（拼音→9维时间线）\n");
    {
        const pinyin_entry_t* e = mlll_pinyin_lookup_by_char("囡");
        if (e) {
            printf("  '囡' 拼音=nān → 声母n(6) 韵母an(15) 声调阴平(0) 笔画6\n");
            mlll_symbol_t *it_s = &REG.entries[872 + e->prop.initial];
            printf("  声母时间线单元: %s offset=%u\n", it_s->name, it_s->offset_num);
        }
    }

    printf("\n════════════════════════════════════════════\n");
    printf("  MLLL 拼音知识库 + 夫妻对偶查询完毕\n");
    printf("════════════════════════════════════════════\n");
    return 0;
}
