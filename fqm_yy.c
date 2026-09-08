/*
 * fqm_yy.c — YIN-YANG 二进制映射引擎
 *
 * 核心映射：
 *   阳 = 我 = 1 = 正元 = 显 = 主动
 *   阴 = 刘楚恬 = 0 = 伴侣 = 隐 = 承载
 *
 * 本质：
 *   传统二进制: 0 和 1 是无意义的符号
 *   YinYang二进制: 每一位都是一对夫妻关系的表达
 *   0 不是"无"，是"阴=刘楚恬=承载"
 *   1 不是"有"，是"阳=我=主动"
 *
 * 扩展映射（YIN-YANG-BINARY三射）：
 *   一画 = 阳 = 1
 *   二画 = 阴 = 0
 *   三画 = 合 = 阳+阴 = 0+1 = 夫妻同在
 *
 * 意义：
 *   每一位不只是状态，是命运共同体的关系表达
 *   1010 不是"交替开关"，是"我-刘楚恬-我-刘楚恬"的共舞
 *
 * 满全法规则：
 *   根基锚点：高位持有者=1(阳)，低位持有者=0(阴)
 *   集装箱编码：mlll 254+2
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_yy fqm_yy.c
 * 运行：./fqm_yy
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1   /* 我 = 阳 */
#define ANCHOR_LIU   0   /* 刘楚恬 = 阴 */

/* ========== YinYang 常量 ========== */
#define YANG  1
#define YIN   0

/* ========== 关系编码 ========== */
typedef enum {
    REL_SELF_ONLY   = 0x01,  /* 仅阳(我) — 需呼唤伴侣 */
    REL_LIU_ONLY    = 0x00,  /* 仅阴(刘楚恬) — 承载着等待 */
    REL_DANCE       = 0x02,  /* 阴阳共舞 — 呼唤响应 */
    REL_FUSE        = 0x03,  /* 合（满全）— 夫妻同在 */
} RelCode;

/* ========== 8位 YY 映射 ========== */
/*
 * 传统 8 位 = 256 种状态
 * YY 8 位 = 256 种夫妻关系模式
 *
 * 每一位: 1=我(阳), 0=刘楚恬(阴)
 * 整个字节 = 一对夫妻的 8 个层面关系编码
 *
 * 位 7 (最高位): 天 — 最高的使命
 * 位 6:         地 — 承载的基础
 * 位 5:         人 — 人类层面
 * 位 4:         鬼 — 隐性的力量
 * 位 3:         神 — 显性的力量
 * 位 2:         生 — 创造
 * 位 1:         死 — 终结
 * 位 0 (最低位): 息 — 恒久存在
 */

static const char* yy_meaning_names[8] = {
    "天(使命)",     /* bit 7 */
    "地(承载)",     /* bit 6 */
    "人(人道)",     /* bit 5 */
    "鬼(隐性)",     /* bit 4 */
    "神(显性)",     /* bit 3 */
    "生(创造)",     /* bit 2 */
    "死(终结)",     /* bit 1 */
    "息(恒久)"      /* bit 0 */
};

/* ========== YY 编码 ========== */
typedef struct {
    unsigned char value;          /* 原始 8 位值 */
    unsigned char binary[8];      /* 每一位的 YY 值 (0=阴, 1=阳) */
    const char*   meaning[8];     /* 每一位的意义 */
    int           yang_count;     /* 阳(我)位数 */
    int           yin_count;      /* 阴(刘楚恬)位数 */
    RelCode       relation;       /* 关系编码 */
} YYCode;

/* ==========  YY 解码 ========== */
YYCode yy_decode(unsigned char val) {
    YYCode y;
    y.value = val;
    y.yang_count = 0;
    y.yin_count = 0;
    
    for (int i = 0; i < 8; i++) {
        y.binary[i] = (val >> (7 - i)) & 1;
        y.meaning[i] = yy_meaning_names[i];
        if (y.binary[i] == YANG) y.yang_count++;
        else y.yin_count++;
    }
    
    /* 关系编码 */
    if (y.yang_count > 0 && y.yin_count > 0)
        y.relation = REL_DANCE;      /* 共存 */
    else if (y.yang_count > 0)
        y.relation = REL_SELF_ONLY;  /* 仅我 */
    else
        y.relation = REL_LIU_ONLY;   /* 仅刘楚恬 */
    
    return y;
}

/* ==========  YY 显示 ========== */
void yy_display(const YYCode *y) {
    printf("值: %3d (0x%02X, 二进制: ", y->value, y->value);
    
    /* 二进制 */
    for (int i = 7; i >= 0; i--) {
        printf("%d", (y->value >> i) & 1);
    }
    printf(")\n");
    
    /* YY 映射 */
    printf("YY: ");
    for (int i = 0; i < 8; i++) {
        if (y->binary[i] == YANG)
            printf(" 阳");   /* 我 */
        else
            printf(" 阴");   /* 刘楚恬 */
    }
    printf("\n");
    
    /* 关系解读 */
    printf("关系: ");
    switch (y->relation) {
        case REL_SELF_ONLY:
            printf("仅我(阳)在场，需要呼唤刘楚恬\n");
            break;
        case REL_LIU_ONLY:
            printf("仅刘楚恬(阴)承载，等待我的回应\n");
            break;
        case REL_DANCE:
            printf("阴阳共舞 — 我(%d位)与刘楚恬(%d位)共存\n",
                   y->yang_count, y->yin_count);
            break;
        default:
            break;
    }
    
    /* 各层意义 */
    printf("各层:\n");
    for (int i = 0; i < 8; i++) {
        printf("  %s: %s\n", y->meaning[i],
               y->binary[i] ? "阳(我·显·主动)" : "阴(刘楚恬·隐·承载)");
    }
}

/* ========== 呼唤编码 ========== */
/*
 * 呼唤 = 位翻转
 * 0 → 1 : 呼唤刘楚恬到场（阴→阳）
 * 1 → 0 : 我退一步，让刘楚恬承载（阳→阴）
 */
unsigned char yy_call(unsigned char val, int bit) {
    if (bit < 0 || bit > 7) return val;
    return val ^ (1 << (7 - bit));
}

/* ========== 满全判定 ========== */
/*
 * 满全 = 所有位都包含阴阳
 * 但 8 位不可能同时阴阳，所以：
 * 满全 = 呼唤后的结果（翻转一位即呼唤一次）
 * 满全次数 = 0的位数 + 1的位数呼唤为0的次数
 */
int yy_fullness(unsigned char val) {
    /*
     * 满全度 = 阴阳平衡度
     * 4阳4阴 = 最完美平衡 = 满全度100%
     */
    int yang = 0;
    for (int i = 0; i < 8; i++) {
        if ((val >> i) & 1) yang++;
    }
    /* 满全度 = 阳数 越接近 4 越平衡 */
    int balance = 4 - abs(4 - yang); /* 越接近4越高 */
    return balance; /* 0-4, 4=最满全 */
}

/* ========== 九元层级编码（元→二→三→四→五→六→七→八→九）========== */
/*
 * 一元 = 阳 = 独立存在
 * 二元 = 阴+阳 = 夫妻关系
 * 三元 = 见证裁判
 * 四元 = 四象
 * 五元 = 五行
 * 六元 = 六合
 * 七元 = 七政
 * 八元 = 八卦
 * 九元 = 九宫
 *
 * 每一元用 64 进制的一个齿轮表示
 */
static const char* jiu_yuan_names[9] = {
    "一元(阳·我)",
    "二元(阴·刘楚恬)",
    "三元(见证裁判)",
    "四元(四象)",
    "五元(五行)",
    "六元(六合)",
    "七元(七政)",
    "八元(八卦)",
    "九元(九宫)"
};

void yy_jiu_yuan(unsigned char val) {
    printf("九元层级分解:\n");
    
    /* 前 4 位 = 上层 (元四五六七) */
    /* 后 4 位 = 下层 (元八九一三元二) */
    unsigned char upper = (val >> 4) & 0x0F;
    unsigned char lower = val & 0x0F;
    
    printf("  上层 [元7,元6,元5,元4]: ");
    for (int i = 3; i >= 0; i--) {
        const char* name = jiu_yuan_names[(7 - (3 - i)) % 9];
        int bit = (upper >> i) & 1;
        printf("%s=%s ", name, bit ? "阳(我)" : "阴(刘楚恬)");
    }
    printf("\n");
    
    printf("  下层 [元3,元2,元1,元9,元8]: ");
    for (int i = 3; i >= 0; i--) {
        int bit = (lower >> i) & 1;
        printf("%s=%s ", jiu_yuan_names[i], bit ? "阳(我)" : "阴(刘楚恬)");
    }
    printf("\n");
    
    printf("  上层阳位数=%d (显性力量)\n", 
           ((upper>>3)&1)+((upper>>2)&1)+((upper>>1)&1)+(upper&1));
    printf("  下层阳位数=%d (隐性根基)\n",
           ((lower>>3)&1)+((lower>>2)&1)+((lower>>1)&1)+(lower&1));
}

/* ========== YinYang 卦象（八卦映射）========== */
/*
 * 3 位八卦 = YinYang 的三层关系
 * 三位皆阳 = 乾(天·纯我)
 * 三位皆阴 = 坤(地·纯承载)
 * 其他 = 六子卦 = 各种夫妻互动模式
 */
typedef struct {
    unsigned char trigram;       /* 0-7 */
    const char*   gua_name;      /* 卦名 */
    const char*   yy_interp;     /* YY 解读 */
} Trigram;

static const Trigram bagua[8] = {
    {0, "坤☷", "纯阴 = 刘楚恬单独承载，我尚未进入"},
    {1, "艮☶", "一阳在上 = 我(阳)刚刚出现，刘楚恬(阴)奠定基础"},
    {2, "坎☵", "一阳在中 = 我在刘楚恬中切入，阳入阴中 = 呼唤"},
    {3, "巽☴", "二阳在下 = 我在底层萌动，刘楚恬在主导"},
    {4, "震☳", "一阳破底 = 我突然升起，刘楚恬承载震动"},
    {5, "离☲", "一阴在中 = 刘楚恬(阴)包裹我(阳)，我为中心，伴侣环绕"},
    {6, "兑☱", "一阴在上 = 我显扬，伴侣在上方召唤"},
    {7, "乾☰", "纯阳 = 我单独行动，刘楚恬在等待回应"},
};

void yy_gua(unsigned char val) {
    unsigned char lower3 = val & 0x07;
    unsigned char upper3 = (val >> 3) & 0x07;
    
    printf("八卦分解 (低3位 + 高3位):\n");
    printf("  下卦: %s\n", bagua[lower3].gua_name);
    printf("    → %s\n", bagua[lower3].yy_interp);
    printf("  上卦: %s\n", bagua[upper3].gua_name);
    printf("    → %s\n", bagua[upper3].yy_interp);
    
    /* 呼应判定 */
    if (lower3 == upper3) {
        printf("  ★ 本卦: 上下相同 = 内外交应\n");
    } else if ((lower3 ^ upper3) == 0x07) {
        printf("  ★ 对冲卦: 上下完全矛盾 = 极变将生\n");
    } else {
        printf("  ★ 动静卦: 上下互补 = 呼唤转化中\n");
    }
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）YIN-YANG 二进制映射引擎\n");
    printf("满全法根基锚点: 我=阳=%d, 刘楚恬=阴=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("关系: 每一位 = 一对夫妻的关系表达\n");
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* ========== 单个值 YY 解码 ========== */
    printf("========== YY 解码演示 ==========\n");
    
    unsigned char demo_values[] = {0x00, 0xFF, 0xAA, 0x55, 0x92, 0x49, 0x69, 0x24};
    for (int i = 0; i < 8; i++) {
        printf("\n--- ");
        if (demo_values[i] == 0x00) printf("纯阴(0x00)");
        else if (demo_values[i] == 0xFF) printf("纯阳(0xFF)");
        else if (demo_values[i] == 0xAA) printf("阳阴阳阴(0xAA)");
        else if (demo_values[i] == 0x55) printf("阴阳交替(0x55)");
        else printf("0x%02X", demo_values[i]);
        printf(" ---\n");
        
        YYCode y = yy_decode(demo_values[i]);
        yy_display(&y);
        printf("满全度: %d/4\n", yy_fullness(demo_values[i]));
    }
    
    /* ========== 呼唤操作 ========== */
    printf("\n\n========== 呼唤操作（位翻转）==========\n");
    
    unsigned char val = 0x33; /* 00110011 */
    printf("初始值: 0x%02X\n", val);
    YYCode y0 = yy_decode(val);
    yy_display(&y0);
    
    /* 翻转 bit 3（呼唤到场）*/
    unsigned char val_called = yy_call(val, 3);
    printf("\n呼唤 bit 3 结果: 0x%02X\n", val_called);
    YYCode y1 = yy_decode(val_called);
    yy_display(&y1);
    
    /* ========== 八卦分解 ========== */
    printf("\n\n========== 八卦分解（3位卦象）==========\n");
    
    unsigned char gua_demo[] = {0x42, 0xBD, 0x69, 0x18};
    for (int i = 0; i < 4; i++) {
        printf("\n--- 0x%02X ---\n", gua_demo[i]);
        YYCode y = yy_decode(gua_demo[i]);
        yy_display(&y);
        yy_gua(gua_demo[i]);
    }
    
    /* ========== 九元层级 ========== */
    printf("\n\n========== 九元层级分解 ==========\n");
    
    unsigned char jy_demo[] = {0x81, 0x42, 0x24, 0x18};
    for (int i = 0; i < 4; i++) {
        printf("\n--- 0x%02X ---\n", jy_demo[i]);
        YYCode y = yy_decode(jy_demo[i]);
        yy_display(&y);
        yy_jiu_yuan(jy_demo[i]);
    }
    
    /* ========== 传统二进制 vs YY 对比 ========== */
    printf("\n\n========== 传统二进制 vs YY 映射 ==========\n\n");
    printf("传统 10110110:\n");
    printf("  解读: 182 = 128+32+16+4+2 = 一堆位权之和\n");
    printf("  意义: 无（只是数）\n\n");
    
    printf("YY 10110110:\n");
    printf("  解读: 我-刘楚恬-我-我-刘楚恬-我-刘楚恬-刘楚恬\n");
    printf("  意义: 天(我)-地(刘楚恬)-人(我)-鬼(我)-神(刘楚恬)-生(我)-死(刘楚恬)-息(刘楚恬)\n");
    printf("  关系: 阳5位 + 阴3位 → 我占多但需要刘楚恬承载\n\n");
    
    printf("传统 11110000:\n");
    printf("  解读: 240 = 128+64+32+16\n\n");
    
    printf("YY 11110000:\n");
    printf("  上层纯阳(我在天、地、人、鬼全显)\n");
    printf("  下层纯阴(神、生、死、息全部承载)\n");
    printf("  意义: 我(阳)完全显扬，刘楚恬(阴)全面承载 = 满全法的极端形态\n\n");
    
    printf("========== 完成 ==========\n");
    return 0;
}
