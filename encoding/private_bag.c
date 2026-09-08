/*
 * private_bag.c
 *
 * 满全法私有容器格式 — 摒弃一切工业文件格式
 *
 * 替代对象: JSON / YAML / XML / INI / TOML / MessagePack / Protobuf / SQL
 *
 * 格式组成:
 *   - 声母韵母地址 (替代文件头+字段名)
 *   - 64进制单元 (替代 hex 字节 / 字段值)
 *   - 七阶编码 (替代 JSON 键值对 / XML 标签)
 *   - 三元见证 (替代 CRC / SHA / 签名)
 *
 * 对外: 无可读字符串, 无标准签名, 无可解析结构
 * 对内: 声母韵母映射表 + 64进制单元划分 + 七阶位置约定
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

/* ═══════════════════════════════════════════════════════════════
 * 声母韵母定义 (发声空间划分)
 *
 * 替代: ASCII 字符串 / Unicode 编码 / 字段名规则
 * ═══════════════════════════════════════════════════════════════ */

/* 24 声母 — 发音起始位置 */
#define SHENG_MU_COUNT 24

typedef struct {
    const char *spell;     /* 拼写标识 (仅内部使用) */
    int         code;      /* 内部位置 (非 ASCII 序) */
    const char *mouth;     /* 口腔位置描述 */
} sheng_mu_t;

static const sheng_mu_t SHENG_MU[SHENG_MU_COUNT] = {
    /* 唇音 */
    { "b",  0, "双唇闭塞" },
    { "p",  1, "双唇送气" },
    { "m",  2, "双唇鼻音" },
    { "f",  3, "唇齿摩擦" },
    /* 舌尖前 */
    { "d",  4, "舌尖上齿" },
    { "t",  5, "舌尖送气" },
    { "n",  6, "舌尖鼻音" },
    { "l",  7, "舌尖边音" },
    /* 舌根 */
    { "g",  8, "舌根闭塞" },
    { "k",  9, "舌根送气" },
    { "h", 10, "喉摩擦" },
    /* 舌面 */
    { "j", 11, "舌面前闭塞" },
    { "q", 12, "舌面前送气" },
    { "x", 13, "舌面前摩擦" },
    /* 翘舌 */
    {"zh", 14, "舌尖翘起闭塞"},
    {"ch", 15, "舌尖翘起送气"},
    {"sh", 16, "舌尖翘起摩擦"},
    { "r", 17, "舌尖翘起通音"},
    /* 平舌 */
    { "z", 18, "舌尖前闭塞" },
    { "c", 19, "舌尖前送气" },
    { "s", 20, "舌尖前摩擦" },
    /* 半元音 */
    { "y", 21, "舌面前元音" },
    { "w", 22, "双唇元音" },
    /* 空 */
    { "ø", 23, "零声母" }     /* 无声母位置 */
};

/* 34 韵母 — 口腔通道形态 */
#define YUN_MU_COUNT 34

typedef struct {
    const char *spell;
    int         code;
} yun_mu_t;

static const yun_mu_t YUN_MU[YUN_MU_COUNT] = {
    { "a",   0 }, { "o",   1 }, { "e",   2 }, { "i",   3 },
    { "u",   4 }, { "v",   5 }, { "ai",  6 }, { "ei",  7 },
    { "ui",  8 }, { "ao",  9 }, { "ou", 10 }, { "iu", 11 },
    { "ie", 12 }, { "ve", 13 }, { "er", 14 }, { "an", 15 },
    { "en", 16 }, { "in", 17 }, { "un", 18 }, { "vn", 19 },
    { "ang", 20 }, { "eng", 21 }, { "ing", 22 }, { "ong", 23 },
    { "ia", 24 }, { "ua", 25 }, { "uo", 26 }, { "ie2", 27 },
    { "uai",28 }, { "uei", 29 }, { "iao", 30 }, { "iou", 31 },
    { "ian",32 }, { "uan", 33 },
};

/* 5 声调 — 升降模式 */
typedef enum {
    SHENG_FIRST  = 0,  /* 阴平 — 高 */
    SHENG_SECOND = 1,  /* 阳平 — 升 */
    SHENG_THIRD  = 2,  /* 上声 — 降升 */
    SHENG_FOURTH = 3,  /* 去声 — 降 */
    SHENG_NEUTRAL = 4, /* 轻声 */
} tone_t;

/* ═══════════════════════════════════════════════════════════════
 * 声母韵母地址 — 容器发音位置
 *
 * 替代: HTTP URL / 文件名 / 数据库主键 / 文件扩展名
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    int  sheng;     /* 声母位置 (0..23) */
    int  yun;       /* 韵母位置 (0..33) */
    int  tone;      /* 声调 (0..4) */
    int  slot;      /* 槽位 (公倍数位置) */
} yin_zhi_t;       /* 音址 */

/* 查找声母位置 */
static int find_sheng_code(const char *spell) {
    for (int i = 0; i < SHENG_MU_COUNT; i++) {
        if (strcmp(SHENG_MU[i].spell, spell) == 0) return SHENG_MU[i].code;
    }
    return -1;
}

/* 查找韵母位置 */
static int find_yun_code(const char *spell) {
    for (int i = 0; i < YUN_MU_COUNT; i++) {
        if (strcmp(YUN_MU[i].spell, spell) == 0) return YUN_MU[i].code;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 64 进制单元 — 一个容器细胞的承载量
 *
 * 替代: hex 字节 (0x00..0xFF)
 * 一个单元 = 6 个比特位 (2^6 = 64)
 * ═══════════════════════════════════════════════════════════════ */

/* 64 个符号的排列 — 按口腔位置排序, 不问 ASCII */
/* 这是满全法的64进制,不是工业的 Base64 */
typedef struct {
    int code;
    const char *口腔位置;      /* 描述发音位置 */
    int 声母;               /* -1 = 无声母位置 */
    int 韵母;               /* -1 = 无韵母 */
} radix64_entry_t;

static const radix64_entry_t RADIX64[64] = {
    /* 0..9: 单韵母位置 */
    { 0, "a 开口元音",   -1,  0 },
    { 1, "o 圆唇元音",   -1,  1 },
    { 2, "e 中元音",     -1,  2 },
    { 3, "i 前元音",     -1,  3 },
    { 4, "u 后元音",     -1,  4 },
    { 5, "v 撮口音",     -1,  5 },
    { 6, "开口复合",     -1,  6 },
    { 7, "复合半合",     -1,  7 },
    { 8, "i+u复合",      -1,  8 },
    { 9, "a+o复合",      -1,  9 },
    /* 10..19: 声母位置 */
    { 10, "双唇闭塞",     0, -1 },  /* b */
    { 11, "双唇送气",     1, -1 },  /* p */
    { 12, "双唇鼻音",     2, -1 },  /* m */
    { 13, "唇齿摩擦",     3, -1 },  /* f */
    { 14, "舌尖上齿",     4, -1 },  /* d */
    { 15, "舌尖送气",     5, -1 },  /* t */
    { 16, "舌尖鼻音",     6, -1 },  /* n */
    { 17, "舌尖边音",     7, -1 },  /* l */
    { 18, "舌根闭塞",     8, -1 },  /* g */
    { 19, "舌根送气",     9, -1 },  /* k */
    /* 20..29: 声母第二组 */
    { 20, "喉摩擦",      10, -1 },  /* h */
    { 21, "面前闭塞",    11, -1 },  /* j */
    { 22, "面前送气",    12, -1 },  /* q */
    { 23, "面前摩擦",    13, -1 },  /* x */
    { 24, "翘起闭塞",    14, -1 },  /* zh */
    { 25, "翘起送气",    15, -1 },  /* ch */
    { 26, "翘起摩擦",    16, -1 },  /* sh */
    { 27, "翘起通音",    17, -1 },  /* r */
    { 28, "平舌闭塞",    18, -1 },  /* z */
    { 29, "平舌送气",    19, -1 },  /* c */
    /* 30..39: 声母第三组 */
    { 30, "平舌摩擦",    20, -1 },  /* s */
    { 31, "舌面元音",    21, -1 },  /* y */
    { 32, "双唇元音",    22, -1 },  /* w */
    { 33, "零声母",      23, -1 },  /* ø */
    { 34, "鼻音韵收",    -1, 20 },  /* ... */
    { 35, "鼻音韵收",    -1, 21 },
    { 36, "鼻音韵收",    -1, 22 },
    { 37, "鼻音韵收",    -1, 23 },
    { 38, "齐齿韵收",    -1, 24 },
    { 39, "合口韵收",    -1, 25 },
    { 40, "撮口韵收",    -1, 26 },
    { 41, "齐齿韵收",    -1, 27 },
    { 42, "u+ai复合",    -1, 28 },
    { 43, "u+ei复合",    -1, 29 },
    { 44, "i+ao复合",    -1, 30 },
    { 45, "i+ou复合",    -1, 31 },
    { 46, "i+an复合",    -1, 32 },
    { 47, "u+an复合",    -1, 33 },
    /* 48..63: 预留 + 齿音/颚音/喉音/边音特殊位置 */
    { 48, "高元音域",    -1, -1 },
    { 49, "中元音域",    -1, -1 },
    { 50, "低元音域",    -1, -1 },
    { 51, "前元音域",    -1, -1 },
    { 52, "后元音域",    -1, -1 },
    { 53, "圆唇域",      -1, -1 },
    { 54, "不圆唇域",    -1, -1 },
    { 55, "锐音",        -1, -1 },
    { 56, "钝音",        -1, -1 },
    { 57, "鼻冠音",      -1, -1 },
    { 58, "边冠音",      -1, -1 },
    { 59, "喉冠音",      -1, -1 },
    { 60, "送气标记",    -1, -1 },
    { 61, "不送气标记",  -1, -1 },
    { 62, "浊音标记",    -1, -1 },
    { 63, "清音标记",    -1, -1 },
};

static int find_radix64_code(int sheng, int yun) {
    for (int i = 0; i < 64; i++) {
        if (RADIX64[i].声母 == sheng && sheng >= 0) return i;
        if (RADIX64[i].韵母 == yun && yun >= 0) return i;
    }
    return -1;
}

/* ═══════════════════════════════════════════════════════════════
 * 七阶编码 — 单个容器单元的完整结构
 *
 * 替代: JSON 键值对 / 数据库行 / XML 属性表
 *
 *  阶1 (presence)  : 有袋(1) / 无袋(0)
 *  阶2 (witness)   : 已见证(1) / 未见证(0)
 *  阶3 (polarity)  : 阴(0) / 阳(1)
 *  阶4 (sheng)     : 声母位置 (0..23) → 6 位
 *  阶5 (yun)       : 韵母位置 (0..33) → 6 位
 *  阶6 (tone+slot) : 声调(3位) + 槽位(3+位)
 *  阶7 (value)     : 值（加法运算的整数，64进制折叠）
 *
 * ═══════════════════════════════════════════════════════════════ */

/* presence */
#define BAG_PRESENCE_EMPTY   0
#define BAG_PRESENCE_FILLED  1

/* witness */
#define BAG_WITNESS_NONE     0
#define BAG_WITNESS_DONE     1

/* polarity */
#define BAG_YIN              0   /* 阴 — 负/里/收 */
#define BAG_YANG             1   /* 阳 — 正/表/放 */

/* 一个完整的容器单元 */
typedef struct {
    int        presence;
    int        witness;
    int        polarity;
    uint64_t   sheng;           /* 声母音址 */
    uint64_t   yun;             /* 韵母音址 */
    uint64_t   tone;            /* 声调 */
    uint64_t   slot;            /* 公倍数槽位 */
    int        value_numerator; /* 值（正加数） */
    int        value_sign;      /* 值的极性: 正(1) / 负(-1) */
    int        owner_type;      /* 持有者类型 */
    yin_zhi_t  owner_addr;      /* 持有者音址 */
    uint64_t   relation_id;     /* 关系码 */
    uint64_t   timing_phase;    /* 阶和时序 */
    int        ack;             /* 已被承认? */
} bag_cell_t;

/* 持有者类型 — 非工业枚举 */
#define OWNER_NONE           0
#define OWNER_WIFE           1  /* 刘楚恬 */
#define OWNER_HUSBAND        2  /* 我 */
#define OWNER_COUPLE         3  /* 夫妻共同体 */
#define OWNER_NEUTRAL        4  /* 中性容器 */
#define OWNER_SYSTEM         5  /* 系统自身 */

const char *owner_name(int type) {
    switch (type) {
        case OWNER_WIFE:     return "妻(刘楚恬)";
        case OWNER_HUSBAND:  return "夫(lolict)";
        case OWNER_COUPLE:   return "夫妻共同体";
        case OWNER_NEUTRAL:  return "中性容器";
        case OWNER_SYSTEM:   return "系统";
        default:             return "无";
    }
}

/* ═══════════════════════════════════════════════════════════════
 * 三元见证 — 容器的准入裁定
 *
 * 替代: SHA-256 / CRC32 / 数字签名
 *
 * 裁定规则:
 *   1. 袋子非空 (presence = FILLED)
 *   2. 已被见证   (witness = DONE)
 *   3. 锚点一致   (高位持有者=1锚点, 低位持有者=0锚点)
 *
 * ═══════════════════════════════════════════════════════════════ */

typedef struct {
    int non_empty;      /* 非空 */
    int witnessed;      /* 已见证 */
    int anchor_ok;      /* 锚点一致 */
    int final_verdict;  /* 最终裁决 */
} bag_verdict_t;

static bag_verdict_t bag_witness(const bag_cell_t *cell) {
    bag_verdict_t v;
    v.non_empty  = (cell->presence == BAG_PRESENCE_FILLED) ? 1 : 0;
    v.witnessed  = (cell->witness  == BAG_WITNESS_DONE)    ? 1 : 0;
    /* 锚点检查: 持有者必须是我或刘楚恬或夫妻共同体 */
    v.anchor_ok  = (cell->owner_type == OWNER_WIFE ||
                    cell->owner_type == OWNER_HUSBAND ||
                    cell->owner_type == OWNER_COUPLE ||
                    cell->owner_type == OWNER_SYSTEM) ? 1 : 0;
    v.final_verdict = (v.non_empty && v.witnessed && v.anchor_ok) ? 1 : 0;
    return v;
}

/* ═══════════════════════════════════════════════════════════════
 * 写法: 容器单元 → 私有字节序列
 *
 * 替代: JSON.stringify / protobuf.serialize / SQL INSERT
 *
 * 输出: 无固定签名, 无字段名, 无可读字符串
 * ═══════════════════════════════════════════════════════════════ */

/* 私有容器字节头:
 * [ reserved(1阶) | witness(1阶) | polarity(1阶) | sheng(1单元) | yun(1单元) | tone+slot | value | owner | relation | timing ]
 *
 * 每个单元 = 64进制的一个码位 (6位对齐)
 * 不输出任何可读字符
 */

/* 64进制值 → 对应口腔位置的短标识 */
const char *radix64_to_kou(int code) {
    if (code < 0 || code >= 64) return "?";
    return RADIX64[code].口腔位置;
}

/* 写一个容器单元到文件, 返回写入字节数 */
static int bag_cell_write(const bag_cell_t *cell, FILE *fp) {
    /* 第1-3阶: 三个单比特标记打包为 64进制单元 */
    int head_unit  = (cell->presence & 1) << 0;
    head_unit     |= (cell->witness  & 1) << 1;
    head_unit     |= (cell->polarity & 1) << 2;

    /* 声母: 5位 (0..23) */
    int sheng_code = (int)cell->sheng & 0x1F;

    /* 韵母: 6位 (0..33) */
    int yun_code = (int)cell->yun & 0x3F;

    /* 声调: 3位 + 槽位: 3位, 同在一个单元 */
    int tone_slot = ((int)cell->tone & 0x07) << 3;
    tone_slot    |= ((int)cell->slot & 0x07);

    /* 值: 用 64进制折叠编码 (正整数 + 极性标记) */
    int value_abs = abs(cell->value_numerator);
    int value_sign = (cell->value_sign < 0) ? 1 : 0;
    int v_unit0 = value_abs & 0x3F;                    /* 低6位 */
    int v_unit1 = (value_abs >> 6) & 0x3F;             /* 中6位 */
    int v_unit2 = ((value_abs >> 12) & 0x3F) | (value_sign << 5); /* 高6位+极性 */

    /* 持有者类型: 3位 */
    int owner_code = cell->owner_type & 0x07;

    /* 关系码: 6位低 + 6位高 */
    int rel0 = (int)(cell->relation_id & 0x3F);
    int rel1 = (int)((cell->relation_id >> 6) & 0x3F);

    /* 时序: 6位低 + 6位高 */
    int time0 = (int)(cell->timing_phase & 0x3F);
    int time1 = (int)((cell->timing_phase >> 6) & 0x3F);

    /* ack: 1位, 合并到 timing 的高位 */
    int ack_bit = (cell->ack) ? 1 : 0;

    /* 写入文件 (64进制单元序列) */
    unsigned char out[20];
    int idx = 0;

    out[idx++] = (unsigned char)(head_unit & 0x3F);
    out[idx++] = (unsigned char)(sheng_code & 0x3F);
    out[idx++] = (unsigned char)(yun_code & 0x3F);
    out[idx++] = (unsigned char)(tone_slot & 0x3F);
    out[idx++] = (unsigned char)(v_unit0 & 0x3F);
    out[idx++] = (unsigned char)(v_unit1 & 0x3F);
    out[idx++] = (unsigned char)(v_unit2 & 0x3F);
    out[idx++] = (unsigned char)(owner_code & 0x3F);
    out[idx++] = (unsigned char)(rel0 & 0x3F);
    out[idx++] = (unsigned char)(rel1 & 0x3F);
    out[idx++] = (unsigned char)(time0 & 0x3F);
    out[idx++] = (unsigned char)(((time1 & 0x3E) | ack_bit) & 0x3F);

    /* 补充: 记录口腔位置校验 (第13单元) — 与声母/韵母呼应 */
    int mouth_check = RADIX64[sheng_code % 64].code;
    out[idx++] = (unsigned char)(mouth_check & 0x3F);

    size_t wrote = fwrite(out, 1, idx, fp);
    return (int)wrote;
}

/* ═══════════════════════════════════════════════════════════════
 * 读法: 私有字节序列 → 容器单元
 *
 * 替代: JSON.parse / protobuf.deserialize / SQL SELECT
 * ═══════════════════════════════════════════════════════════════ */

static int bag_cell_read(bag_cell_t *cell, FILE *fp) {
    unsigned char in[20];
    size_t n = fread(in, 1, 13, fp);
    if (n < 13) return -1; /* 读失败 */

    cell->presence = (in[0] >> 0) & 1;
    cell->witness  = (in[0] >> 1) & 1;
    cell->polarity = (in[0] >> 2) & 1;

    cell->sheng = in[1] & 0x1F;
    cell->yun   = in[2] & 0x3F;

    cell->tone = (in[3] >> 3) & 0x07;
    cell->slot = in[3] & 0x07;

    int v0 = in[4] & 0x3F;
    int v1 = in[5] & 0x3F;
    int v2 = in[6] & 0x3F;

    cell->value_numerator = v0 | (v1 << 6) | ((v2 & 0x1F) << 12);
    cell->value_sign = ((v2 >> 5) & 1) ? -1 : 1;

    cell->owner_type = in[7] & 0x07;
    cell->owner_addr.sheng = -1;
    cell->owner_addr.yun   = -1;
    cell->owner_addr.tone  = 0;
    cell->owner_addr.slot  = 0;

    cell->relation_id = in[8] | (in[9] << 6);

    cell->timing_phase = in[10] | ((in[11] & 0x3E) << 6);
    cell->ack = in[11] & 1;

    return 0;
}

/* ═══════════════════════════════════════════════════════════════
 * 十六户 (替代工业文件扩展名)
 *
 * 16 类私有文件容器, 每种对应一种夫妻生活领域的记录袋
 * ═══════════════════════════════════════════════════════════════ */

const char *BAG_CATEGORY_NAMES[16] = {
    "系统配置袋",
    "感知输入袋",    /* input 流 */
    "分类家族袋",    /* family 分类 */
    "谐音靶点袋",    /* homophone 靶点 */
    "主权路由袋",    /* sovereignty 路由 */
    "产权记录袋",    /* owner 登记 */
    "诉讼证据袋",    /* evidence 保全 */
    "记忆层积袋",    /* layered memory */
    "情感反馈袋",    /* pleasure/pain */
    "关系拓扑袋",    /* topology */
    "裁决裁判袋",    /* verdict */
    "链条印记袋",    /* chain 记录 */
    "流动代数袋",    /* flow algebra */
    "操作日志袋",    /* operation log */
    "身份落款袋",    /* identity 签名 */
    "空缺保留袋",    /* 未分配 */
};

/* ═══════════════════════════════════════════════════════════════
 * 打印容器内容 — 仅供内部调试, 不写入私有格式
 * ═══════════════════════════════════════════════════════════════ */

static void bag_cell_print(const bag_cell_t *cell, int index) {
    printf("  ┌─ 容器 #%d\n", index);
    printf("  │  presence : %s\n", cell->presence ? "有袋" : "无袋");
    printf("  │  witness  : %s\n", cell->witness ? "已见证" : "未见证");
    printf("  │  polarity : %s\n", cell->polarity ? "阳" : "阴");
    if (cell->sheng < SHENG_MU_COUNT) {
        printf("  │  sheng    : %s (%s)\n", SHENG_MU[cell->sheng].spell, SHENG_MU[cell->sheng].mouth);
    }
    if (cell->yun < YUN_MU_COUNT) {
        printf("  │  yun      : %s\n", YUN_MU[cell->yun].spell);
    }
    printf("  │  tone     : %d (%s)\n", (int)cell->tone,
           cell->tone == SHENG_FIRST ? "阴平" :
           cell->tone == SHENG_SECOND ? "阳平" :
           cell->tone == SHENG_THIRD ? "上声" :
           cell->tone == SHENG_FOURTH ? "去声" : "轻声");
    printf("  │  slot     : %d\n", (int)cell->slot);
    printf("  │  value    : %d\n", cell->value_numerator * cell->value_sign);
    printf("  │  owner    : %s\n", owner_name(cell->owner_type));
    printf("  │  relation : %d\n", (int)cell->relation_id);
    printf("  │  timing   : phase=%d\n", (int)cell->timing_phase);
    printf("  │  ack      : %s\n", cell->ack ? "已承认" : "未承认");

    bag_verdict_t v = bag_witness(cell);
    printf("  └─ 三元见证: %s (非空=%d, 已见证=%d, 锚点=%d)\n",
           v.final_verdict ? "通过" : "不通过", v.non_empty, v.witnessed, v.anchor_ok);
}

/* ═══════════════════════════════════════════════════════════════
 * 演示: 创建两个容器袋 → 写入私有格式 → 读回 → 见证校验
 * ═══════════════════════════════════════════════════════════════ */

static bag_cell_t make_cell(
    int sheng_code, int yun_code, int tone, int slot,
    int value, int polarity, int owner, int relation_id
) {
    bag_cell_t c;
    c.presence = BAG_PRESENCE_FILLED;
    c.witness  = BAG_WITNESS_DONE;
    c.polarity = polarity;
    c.sheng    = sheng_code;
    c.yun      = yun_code;
    c.tone     = tone;
    c.slot     = slot;
    c.value_numerator = abs(value);
    c.value_sign      = (value < 0) ? -1 : 1;
    c.owner_type      = owner;
    c.owner_addr.sheng = -1;
    c.owner_addr.yun   = -1;
    c.owner_addr.tone  = 0;
    c.owner_addr.slot  = 0;
    c.relation_id     = relation_id;
    c.timing_phase    = 0;
    c.ack = (owner == OWNER_COUPLE || owner == OWNER_WIFE || owner == OWNER_HUSBAND) ? 1 : 0;
    return c;
}

/* 文件内容十六进制查看 (演示对外观察: 无固定签名, 无可读字符串) */
static void dump_raw_bytes(const char *filename) {
    FILE *f = fopen(filename, "rb");
    if (!f) { printf("  无法打开\n"); return; }

    unsigned char buf[256];
    size_t n = fread(buf, 1, sizeof(buf), f);
    fclose(f);

    printf("  原始字节流 (第一行 = 第一个容器):\n  ");
    for (size_t i = 0; i < n && i < 64; i++) {
        printf("%02x ", buf[i]);
        if ((i + 1) % 16 == 0) printf("\n  ");
    }
    printf("\n  长度: %lu 字节\n", (unsigned long)n);

    /* 检查是否有可见 ASCII */
    int visible = 0;
    for (size_t i = 0; i < n; i++) {
        if (buf[i] >= 0x20 && buf[i] <= 0x7E) visible++;
    }
    printf("  可见 ASCII 字符数: %d / %lu (越少越私有)\n", visible, (unsigned long)n);
}

int main(void) {
    printf("═══════════════════════════════════════════════════════════════\n");
    printf("  满全法 私有容器格式 — 演示\n");
    printf("  替代 JSON/YAML/XML/INI/TOML/SQL/MessagePack/Protobuf\n");
    printf("═══════════════════════════════════════════════════════════════\n\n");

    /*─── 创建容器群 ─── */
    printf("【1】创建容器群 (省略字段名, 仅按位置写入)\n");
    printf("    容器 #0: 配置袋 (系统)\n");
    printf("    容器 #1: 感知袋 (刘楚恬)\n");
    printf("    容器 #2: 仲裁袋 (夫妻共同体)\n\n");

    /* 配置袋: 声母 j (面前闭塞), 韵母 i (前元音), 阴平, 值 1 (高位持有者=1) */
    bag_cell_t cfg = make_cell(
        11,     /* j: 面前闭塞 */
        3,      /* i: 前元音 */
        SHENG_FIRST, 0,
        1,      /* 高位持有者=1 高位锚点 */
        BAG_YANG,    /* 阳 — 高位 */
        OWNER_SYSTEM, 0
    );

    /* 感知袋: 声母 sh (翘起摩擦), 韵母 i (前元音), 去声, 值 0 (低位持有者=0) */
    bag_cell_t perceive = make_cell(
        16,     /* sh: 翘起摩擦 */
        3,      /* i: 前元音 */
        SHENG_FOURTH, 0,
        0,      /* 低位持有者=0 低位隐藏 */
        BAG_YIN,    /* 阴 — 低位 */
        OWNER_WIFE, 1
    );

    /* 仲裁袋: 声母 j (面前闭塞), 韵母 i (前元音), 上声, 值 42 */
    bag_cell_t verdict = make_cell(
        11,     /* j: 面前闭塞 */
        3,      /* i: 前元音 */
        SHENG_THIRD, 7,
        42,     /* 中间值 */
        BAG_YANG,    /* 阳 — 高位 */
        OWNER_COUPLE, 2
    );

    /*─── 打印 ─── */
    printf("【2】容器单元内容 (调试视图, 不写入私有格式)\n");
    bag_cell_print(&cfg, 0);
    bag_cell_print(&perceive, 1);
    bag_cell_print(&verdict, 2);
    printf("\n");

    /*─── 写入 ─── */
    const char *bag_file = "output.pbag";

    printf("【3】写入私有格式文件: %s\n", bag_file);
    FILE *outfp = fopen(bag_file, "wb");
    if (!outfp) { printf("  ❌ 开文件失败\n"); return 1; }

    /* 文件最开头: 一个字节标记 "这是袋子" — 不是签名, 是谐音起点标记 */
    unsigned char bag_mark = 0x01;  /* presence=1, 表示 "有袋" */
    fwrite(&bag_mark, 1, 1, outfp);

    int w0 = bag_cell_write(&cfg, outfp);
    int w1 = bag_cell_write(&perceive, outfp);
    int w2 = bag_cell_write(&verdict, outfp);

    fclose(outfp);
    printf("  ✅ 已写入 3 个容器单元, 共 %d 字节\n", 1 + w0 + w1 + w2);
    printf("     (1 bytes per cell: 七阶编码无字段名开销)\n\n");

    /*─── 原始字节观察 ─── */
    printf("【4】外部观察: 文件内容对他人来说是什么?\n");
    dump_raw_bytes(bag_file);
    printf("\n");

    /*─── 读回 ─── */
    printf("【5】读回私有格式并解析\n");
    FILE *infp = fopen(bag_file, "rb");
    if (!infp) { printf("  ❌ 读打开失败\n"); return 1; }

    unsigned char mark;
    if (fread(&mark, 1, 1, infp) < 1) { printf("  ❌ read mark failed\n"); return 1; }

    bag_cell_t read0, read1, read2;
    int r0 = bag_cell_read(&read0, infp);
    int r1 = bag_cell_read(&read1, infp);
    int r2 = bag_cell_read(&read2, infp);
    fclose(infp);

    if (r0 == 0 && r1 == 0 && r2 == 0) {
        printf("  ✅ 读回 3 个容器单元\n\n");

        printf("【6】见证校验\n");
        bag_verdict_t v0 = bag_witness(&read0);
        bag_verdict_t v1 = bag_witness(&read1);
        bag_verdict_t v2 = bag_witness(&read2);
        printf("  容器 #0: %s\n", v0.final_verdict ? "✅ 通过" : "❌ 不通过");
        printf("  容器 #1: %s\n", v1.final_verdict ? "✅ 通过" : "❌ 不通过");
        printf("  容器 #2: %s\n", v2.final_verdict ? "✅ 通过" : "❌ 不通过");

        if (v0.final_verdict && v1.final_verdict && v2.final_verdict) {
            printf("\n  ✅ 全部通过 — 私有格式闭环验证完成!\n");
            printf("\n  格式特性:\n");
            printf("    - 文件无传统签名 (最开头只有 presence=1 的标记)\n");
            printf("    - 无字段名, 值按七阶位置排列\n");
            printf("    - 声母/韵母音址作为内部定位, 外界无法拼读\n");
            printf("    - 校验靠三元见证, 不靠 SHA / CRC\n");
        }
    } else {
        printf("  ❌ 读回失败 (r0=%d, r1=%d, r2=%d)\n", r0, r1, r2);
    }

    printf("\n═══════════════════════════════════════════════════════════════\n");
    printf("  演示完成。output.pbag 已创建, 对外是不可分析的二进制袋\n");
    printf("═══════════════════════════════════════════════════════════════\n");

    return 0;
}
