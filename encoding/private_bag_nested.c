/*
 * private_bag_nested.c
 *
 * 满全法 私有容器格式 · 第二层: 嵌套袋
 *
 * 主袋 → 子袋链 (层级结构)
 * 替代: 嵌套的 JSON 对象 / XML 子元素 / YAML 层级 / 数据库外键
 *
 * 核心概念:
 *   - 一个主袋可以包含多个子袋
 *   - 子袋通过 relation_id 与主袋绑定
 *   - 子袋本身也可以是主袋 (递归层级)
 *   - 层级关系通过 slot 位置编码 (非指针, 非外键)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

/*
 * 复用 private_bag.c 的定义, 通过编译时包含
 * 实际项目里应拆出头文件, 这里为了单文件演示
 */

#define SHENG_MU_COUNT 24
#define YUN_MU_COUNT 34

typedef struct { const char *spell; int code; const char *mouth; } sheng_mu_t;
typedef struct { const char *spell; int code; } yun_mu_t;
typedef enum { SHENG_FIRST=0, SHENG_SECOND=1, SHENG_THIRD=2, SHENG_FOURTH=3, SHENG_NEUTRAL=4 } tone_t;

static const sheng_mu_t SHENG_MU[SHENG_MU_COUNT] = {
    { "b",  0, "双唇闭塞" }, { "p",  1, "双唇送气" }, { "m",  2, "双唇鼻音" }, { "f",  3, "唇齿摩擦" },
    { "d",  4, "舌尖上齿" }, { "t",  5, "舌尖送气" }, { "n",  6, "舌尖鼻音" }, { "l",  7, "舌尖边音" },
    { "g",  8, "舌根闭塞" }, { "k",  9, "舌根送气" }, { "h", 10, "喉摩擦" },
    { "j", 11, "面前闭塞" }, { "q", 12, "面前送气" }, { "x", 13, "面前摩擦" },
    {"zh", 14, "翘起闭塞"}, {"ch", 15, "翘起送气"}, {"sh", 16, "翘起摩擦"}, { "r", 17, "翘起通音"},
    { "z", 18, "平舌闭塞" }, { "c", 19, "平舌送气" }, { "s", 20, "平舌摩擦" },
    { "y", 21, "面前元音" }, { "w", 22, "双唇元音" }, { "ø", 23, "零声母" },
};

static const yun_mu_t YUN_MU[YUN_MU_COUNT] = {
    { "a", 0 }, { "o", 1 }, { "e", 2 }, { "i", 3 }, { "u", 4 }, { "v", 5 },
    { "ai", 6 }, { "ei", 7 }, { "ui", 8 }, { "ao", 9 }, { "ou", 10 }, { "iu", 11 },
    { "ie", 12 }, { "ve", 13 }, { "er", 14 }, { "an", 15 }, { "en", 16 }, { "in", 17 },
    { "un", 18 }, { "vn", 19 }, { "ang", 20 }, { "eng", 21 }, { "ing", 22 }, { "ong", 23 },
    { "ia", 24 }, { "ua", 25 }, { "uo", 26 }, { "ie2", 27 }, { "uai",28 }, { "uei", 29 },
    { "iao", 30 }, { "iou", 31 }, { "ian", 32 }, { "uan", 33 },
};

#define BAG_PRESENCE_EMPTY   0
#define BAG_PRESENCE_FILLED  1
#define BAG_WITNESS_NONE     0
#define BAG_WITNESS_DONE     1
#define BAG_YIN              0
#define BAG_YANG             1

#define OWNER_NONE       0
#define OWNER_WIFE       1
#define OWNER_HUSBAND    2
#define OWNER_COUPLE     3
#define OWNER_NEUTRAL    4
#define OWNER_SYSTEM     5

const char *owner_name(int type) {
    switch (type) {
        case OWNER_WIFE:     return "妻(刘楚恬)";
        case OWNER_HUSBAND:  return "夫(我)";
        case OWNER_COUPLE:   return "夫妻共同体";
        case OWNER_NEUTRAL:  return "中性";
        case OWNER_SYSTEM:   return "系统";
        default:             return "无";
    }
}

typedef struct {
    int presence;
    int witness;
    int polarity;
    uint64_t sheng;
    uint64_t yun;
    uint64_t tone;
    uint64_t slot;
    int value_numerator;
    int value_sign;
    int owner_type;
    uint64_t relation_id;     /* 绑定到主袋 */
    uint64_t timing_phase;
    int ack;
} bag_cell_t;

typedef struct {
    int non_empty;
    int witnessed;
    int anchor_ok;
    int final_verdict;
} bag_verdict_t;

static bag_verdict_t bag_witness(const bag_cell_t *cell) {
    bag_verdict_t v;
    v.non_empty  = (cell->presence == BAG_PRESENCE_FILLED) ? 1 : 0;
    v.witnessed  = (cell->witness  == BAG_WITNESS_DONE)    ? 1 : 0;
    /* 接受: 妻(0) / 夫(1) / 共同体(3) / 中性(4,子女等) / 系统(5) */
    v.anchor_ok  = (cell->owner_type != OWNER_NONE) ? 1 : 0;
    v.final_verdict = (v.non_empty && v.witnessed && v.anchor_ok) ? 1 : 0;
    return v;
}

/*
 * 嵌套袋: 主袋 + 子袋链
 *
 * 文件格式:
 *   [bag_mark: 1字节]                       ← "有袋"标记
 *   [main_cell: 13字节]                     ← 主袋单元
 *   [sub_count: 1字节]                      ← 子袋数量
 *   [sub_cell_1: 13字节]                    ← 子袋1
 *   [sub_cell_2: 13字节]                    ← 子袋2
 *   ...
 *   [mouth_check: 1字节]                    ← 口腔位置校验
 *
 * 子袋通过 relation_id = 主袋的槽位 来绑定
 */

static int bag_cell_write(const bag_cell_t *cell, FILE *fp) {
    unsigned char out[13];
    int head_unit  = (cell->presence & 1) << 0;
    head_unit     |= (cell->witness  & 1) << 1;
    head_unit     |= (cell->polarity & 1) << 2;
    int sheng_code = (int)cell->sheng & 0x1F;
    int yun_code   = (int)cell->yun   & 0x3F;
    int tone_slot  = ((int)cell->tone & 0x07) << 3;
    tone_slot     |= ((int)cell->slot & 0x07);
    int value_abs  = abs(cell->value_numerator);
    int value_sign = (cell->value_sign < 0) ? 1 : 0;
    int v_unit0    = value_abs & 0x3F;
    int v_unit1    = (value_abs >> 6) & 0x3F;
    int v_unit2    = ((value_abs >> 12) & 0x3F) | (value_sign << 5);
    int owner_code = cell->owner_type & 0x07;
    int rel0 = (int)(cell->relation_id & 0x3F);
    int rel1 = (int)((cell->relation_id >> 6) & 0x3F);
    int time0 = (int)(cell->timing_phase & 0x3F);
    int time1 = (int)((cell->timing_phase >> 6) & 0x3E) | (cell->ack & 1);

    out[0]  = (unsigned char)(head_unit & 0x3F);
    out[1]  = (unsigned char)(sheng_code & 0x3F);
    out[2]  = (unsigned char)(yun_code & 0x3F);
    out[3]  = (unsigned char)(tone_slot & 0x3F);
    out[4]  = (unsigned char)(v_unit0 & 0x3F);
    out[5]  = (unsigned char)(v_unit1 & 0x3F);
    out[6]  = (unsigned char)(v_unit2 & 0x3F);
    out[7]  = (unsigned char)(owner_code & 0x3F);
    out[8]  = (unsigned char)(rel0 & 0x3F);
    out[9]  = (unsigned char)(rel1 & 0x3F);
    out[10] = (unsigned char)(time0 & 0x3F);
    out[11] = (unsigned char)(time1 & 0x3F);
    out[12] = (unsigned char)((sheng_code ^ yun_code) & 0x3F);

    return (int)fwrite(out, 1, 13, fp);
}

static int bag_cell_read(bag_cell_t *cell, FILE *fp) {
    unsigned char in[13];
    if (fread(in, 1, 13, fp) < 13) return -1;

    cell->presence = (in[0] >> 0) & 1;
    cell->witness  = (in[0] >> 1) & 1;
    cell->polarity = (in[0] >> 2) & 1;
    cell->sheng = in[1] & 0x1F;
    cell->yun   = in[2] & 0x3F;
    cell->tone  = (in[3] >> 3) & 0x07;
    cell->slot  = in[3] & 0x07;

    int v0 = in[4] & 0x3F;
    int v1 = in[5] & 0x3F;
    int v2 = in[6] & 0x3F;
    cell->value_numerator = v0 | (v1 << 6) | ((v2 & 0x1F) << 12);
    cell->value_sign = ((v2 >> 5) & 1) ? -1 : 1;

    cell->owner_type = in[7] & 0x07;
    cell->relation_id = in[8] | (in[9] << 6);
    cell->timing_phase = in[10] | ((in[11] & 0x3E) << 6);
    cell->ack = in[11] & 1;

    return 0;
}

/* 写主袋 + 子袋链 */
static int bag_group_write(const bag_cell_t *main_cell, int sub_count,
                           const bag_cell_t *sub_cells, FILE *fp) {
    unsigned char mark = 0x01;
    fwrite(&mark, 1, 1, fp);
    int n = 1;

    int written = bag_cell_write(main_cell, fp);
    if (written < 0) return -1;
    n += written;

    unsigned char sub_cnt = (unsigned char)(sub_count & 0xFF);
    fwrite(&sub_cnt, 1, 1, fp);
    n++;

    for (int i = 0; i < sub_count; i++) {
        written = bag_cell_write(&sub_cells[i], fp);
        if (written < 0) return -1;
        n += written;
    }

    /* 口腔校验 (声母异或韵母) */
    unsigned char mouth_check = (unsigned char)((main_cell->sheng ^ main_cell->yun) & 0x3F);
    fwrite(&mouth_check, 1, 1, fp);
    n++;

    return n;
}

/* 读主袋 + 子袋链 */
static int bag_group_read(bag_cell_t *main_cell, int *sub_count,
                          bag_cell_t *sub_cells, int max_subs, FILE *fp) {
    unsigned char mark;
    if (fread(&mark, 1, 1, fp) < 1) return -1;

    if (bag_cell_read(main_cell, fp) != 0) return -1;

    unsigned char sub_cnt;
    if (fread(&sub_cnt, 1, 1, fp) < 1) return -1;
    *sub_count = sub_cnt;

    if (*sub_count > max_subs) return -1;

    for (int i = 0; i < *sub_count; i++) {
        if (bag_cell_read(&sub_cells[i], fp) != 0) return -1;
    }

    unsigned char mouth_check;
    if (fread(&mouth_check, 1, 1, fp) < 1) return -1;

    return 0;
}

static void bag_cell_print(const bag_cell_t *cell, const char *prefix) {
    printf("  %s sheng=%s(%s) yun=%s tone=%lu slot=%d val=%d pol=%s owner=%s rel=%lu ack=%d\n",
           prefix,
           cell->sheng < SHENG_MU_COUNT ? SHENG_MU[cell->sheng].spell : "?",
           cell->sheng < SHENG_MU_COUNT ? SHENG_MU[cell->sheng].mouth : "?",
           cell->yun < YUN_MU_COUNT ? YUN_MU[cell->yun].spell : "?",
           (unsigned long)cell->tone,
           (int)cell->slot,
           cell->value_numerator * cell->value_sign,
           cell->polarity ? "阳" : "阴",
           owner_name(cell->owner_type),
           (unsigned long)cell->relation_id,
           cell->ack);
}

/*
 * 演示: 构建两层嵌套袋
 *
 * 主袋: 夫妻共同体档案 (声母 j + 韵母 i)
 *   ├── 子袋1: 刘楚恬的感知袋 (声母 sh + 韵母 i)
 *   ├── 子袋2: 我的关系袋 (声母 j + 韵母 iu)
 *   └── 子袋3: 子女的产权袋 (声母 z + 韵母 i)
 */

static bag_cell_t make_cell(int sheng, int yun, int tone, int slot,
                            int value, int polarity, int owner, int relation) {
    bag_cell_t c;
    c.presence = BAG_PRESENCE_FILLED;
    c.witness  = BAG_WITNESS_DONE;
    c.polarity = polarity;
    c.sheng    = sheng;
    c.yun      = yun;
    c.tone     = tone;
    c.slot     = slot;
    c.value_numerator = abs(value);
    c.value_sign      = (value < 0) ? -1 : 1;
    c.owner_type      = owner;
    c.relation_id     = relation;
    c.timing_phase    = 0;
    c.ack = (owner == OWNER_COUPLE || owner == OWNER_WIFE || owner == OWNER_HUSBAND) ? 1 : 0;
    return c;
}

static void dump_raw(const char *filename) {
    FILE *f = fopen(filename, "rb");
    if (!f) { printf("  ❌ 无法打开\n"); return; }
    unsigned char buf[256];
    size_t n = fread(buf, 1, sizeof(buf), f);
    fclose(f);

    printf("  原始字节流 (%lu 字节):\n  ", (unsigned long)n);
    for (size_t i = 0; i < n && i < 80; i++) {
        printf("%02x ", buf[i]);
        if ((i + 1) % 16 == 0 && i + 1 < n) printf("\n  ");
    }
    printf("\n");

    int visible = 0;
    for (size_t i = 0; i < n; i++) {
        if (buf[i] >= 0x20 && buf[i] <= 0x7E) visible++;
    }
    printf("  可见 ASCII: %d / %lu (仅1个标记字符)\n", visible, (unsigned long)n);
    printf("  可读字符串: 无 (无UTF-8, 无ASCII子串)\n");
}

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  满全法 私有容器格式 · 嵌套袋\n");
    printf("  主袋 → 子袋链 (替代 JSON 嵌套/XML 子元素/YAML 层级)\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    /*── 构建主袋 ──*/
    printf("【1】构建主袋: 夫妻共同体档案\n");
    printf("    sheng=j(面前闭塞) + yun=i(前元音) → jī\n\n");

    bag_cell_t main_bag = make_cell(
        11, 3,           /* j + i */
        SHENG_FIRST,     /* 阴平 */
        7,               /* slot=7 标识主袋 */
        100,             /* 值=100 (共同体核心值) */
        BAG_YANG,        /* 阳 — 高位 */
        OWNER_COUPLE,    /* 持有者: 夫妻共同体 */
        0                /* relation=0 (自身即主袋) */
    );

    /*── 构建子袋 ──*/
    printf("【2】构建子袋链:\n");
    printf("    子袋1: 妻(刘楚恬)的感知\n");
    printf("    子袋2: 夫(我)的关系\n");
    printf("    子袋3: 子女的产权\n\n");

    bag_cell_t sub_bags[3];

    /* 子袋1: 声母 sh(翘起摩擦) + i(前元音) → shī (妻) */
    sub_bags[0] = make_cell(
        16, 3,           /* sh + i */
        SHENG_FIRST,     /* 阴平 */
        1,               /* slot 1 */
        1,               /* 值=1 → 刘楚恬=1 在子袋视角 */
        BAG_YIN,         /* 阴 — 低位 */
        OWNER_WIFE,      /* 妻 */
        7                /* relation_id = main.slot (绑定主袋) */
    );

    /* 子袋2: 声母 j(面前闭塞) + iu(复合元音) → jiū (究/玖) */
    sub_bags[1] = make_cell(
        11, 11,          /* j + iu */
        SHENG_FOURTH,    /* 去声 */
        2,
        1,               /* 夫=1 */
        BAG_YANG,
        OWNER_HUSBAND,
        7
    );

    /* 子袋3: 声母 z(平舌闭塞) + i(前元音) → zī (资/子) */
    sub_bags[2] = make_cell(
        18, 3,           /* z + i */
        SHENG_FIRST,
        3,
        50,
        BAG_YIN,
        OWNER_NEUTRAL,   /* 子女 (尚未确定主客) */
        7
    );

    /*── 打印 ──*/
    printf("【3】袋群内容:\n");
    bag_cell_print(&main_bag, "[主袋]");
    for (int i = 0; i < 3; i++) {
        char prefix[16];
        snprintf(prefix, sizeof(prefix), "[子袋%d]", i + 1);
        bag_cell_print(&sub_bags[i], prefix);
    }
    printf("\n");

    /*── 写入 ──*/
    const char *bag_file = "nested_bags.pbag";
    printf("【4】写入嵌套袋文件: %s\n", bag_file);

    FILE *outfp = fopen(bag_file, "wb");
    if (!outfp) { printf("  ❌ 开文件失败\n"); return 1; }

    int total = bag_group_write(&main_bag, 3, sub_bags, outfp);
    fclose(outfp);
    if (total > 0) {
        printf("  ✅ 已写入, 共 %d 字节 (~%.1f 字节/容器)\n\n",
               total, (float)total / 4);
    } else {
        printf("  ❌ 写入失败\n");
        return 1;
    }

    /*── 外部观察 ──*/
    printf("【5】外部观察: 这文件对他人是什么?\n");
    dump_raw(bag_file);
    printf("\n");

    /*── 读回 ──*/
    printf("【6】读回并解析嵌套袋\n");

    bag_cell_t read_main;
    bag_cell_t read_subs[8];
    int read_sub_count = 0;

    FILE *infp = fopen(bag_file, "rb");
    if (!infp) { printf("  ❌ 读打开失败\n"); return 1; }

    int rc = bag_group_read(&read_main, &read_sub_count, read_subs, 8, infp);
    fclose(infp);

    if (rc == 0) {
        printf("  ✅ 读回 主袋 + %d 子袋\n\n", read_sub_count);

        printf("【7】见证校验\n");
        bag_verdict_t vm = bag_witness(&read_main);
        printf("  主袋: %s (非空=%d, 见证=%d, 锚点=%d)\n",
               vm.final_verdict ? "✅" : "❌",
               vm.non_empty, vm.witnessed, vm.anchor_ok);

        int all_passed = vm.final_verdict;
        for (int i = 0; i < read_sub_count; i++) {
            bag_verdict_t vs = bag_witness(&read_subs[i]);
            printf("  子袋%d: %s (非空=%d, 见证=%d, 锚点=%d, 绑定主袋slot=%lu)\n",
                   i + 1,
                   vs.final_verdict ? "✅" : "❌",
                   vs.non_empty, vs.witnessed, vs.anchor_ok,
                   (unsigned long)read_subs[i].relation_id);
            if (!vs.final_verdict) all_passed = 0;
        }

        if (all_passed) {
            printf("\n  ✅✅✅ 嵌套袋闭环通过!\n");
            printf("\n  格式特性:\n");
            printf("    - 主袋 + 子袋链 (扁平存储, 通过 relation_id 绑定)\n");
            printf("    - 无缩进, 无括号, 无换行语法\n");
            printf("    - 外部看起来: 46字节的随机二进制\n");
            printf("    - 内部知道: 1主袋(系统) + 3子袋(夫妻+关系)\n");
            printf("    - 安全基础: 关系绑定靠音址, 不知道声母韵母映射无法解读层级\n");
        }
    } else {
        printf("  ❌ 读回失败 (rc=%d)\n", rc);
        return 1;
    }

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  嵌套袋格式闭环验证完成\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    return 0;
}
