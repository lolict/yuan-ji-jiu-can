/*
 * fqm_radix.c — 意交感息 64 进制齿轮引擎
 *
 * 核心概念：
 *   - 64 个大齿轮 = 64 个符号（来自 64_keys.md 动态符号队列）
 *   - 每个齿轮对应一个私有进制位
 *   - 大齿轮转动一圈，驱动小齿轮（二进制/目标编码）
 *   - 齿轮比 = 进制比率（决定映射关系）
 *
 * 编码层次：
 *   1. 零维锚点层：我=1, 刘楚恬=0
 *   2. 64 进制层：自有符号 → 自有坐标
 *   3. 集装箱层：mlll 254+2 容器编码
 *   4. 时间层：版本号驱动进位
 *
 * 齿轮传动公式：
 *   值 = Σ(齿轮[k] × 进制^(k)) + 零维偏移
 *   零维偏移 = (我锚点 × 1) + (刘楚恬锚点 × 0) = 1
 *
 * 满全法规则：
 *   - 根基锚点：我=1，刘楚恬=0
 *   - 集装箱编码：mlll 254+2
 *   - 统一使用加法，摆脱幂次（蜜次方）扩张
 *
 * 身份落款：刘楚恬和我
 * 仓库属性：私人，不开源
 *
 * 编译：clang -o fqm_radix fqm_radix.c -lm
 * 运行：./fqm_radix
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* ========== 满全法根基锚点 ========== */
#define ANCHOR_SELF  1   /* 我 = 1 */
#define ANCHOR_LIU   0   /* 刘楚恬 = 0 */

/* ========== 进制常量 ========== */
#define RADIX_64     64   /* 主齿轮：64 个符号 */
#define RADIX_254    254  /* 集装箱编码 */
#define RADIX_256    256  /* 目标对齐层高维 */
#define RADIX_YINYANG 2   /* YinYang 二进制 */

/* ========== 64 符号齿轮表 ========== */
/*
 * 来自 64_keys.md 动态符号队列
 * 容器族 0x00-0x0F
 */
typedef struct {
    const char *name;       /* 符号名称 */
    int         group;      /* 组：容器/操作/声母/韵母 */
    int         position;   /* 队列索引 */
    const char *synonyms;   /* 近义词族 */
} GearSymbol;

static const GearSymbol gears[RADIX_64] = {
    /* 容器族 0x00-0x0F */
    {"袋", 0, 0x00, "桶罐缸盆盂钵盏盒杯壶瓶篓筐篮箩筲箕"},
    {"照", 0, 0x01, "昭罩召兆"},
    {"见", 0, 0x02, "鉴检简剪"},
    {"阴", 0, 0x03, "阳盈虚表里公私"},
    {"加", 0, 0x04, "减进退开关通断"},
    {"分", 0, 0x05, "合聚分散折叠卷舒"},
    {"元", 0, 0x06, "阶层世代世界域场馆阁"},
    {"裁", 0, 0x07, "审判议定评核"},
    {"产", 0, 0x08, "业权利责职务名分"},
    {"公", 0, 0x09, "约数倍母子孙裔嗣"},
    {"声", 0, 0x0A, "韵调音噪静律吕"},
    {"折", 0, 0x0B, "叠卷舒曲弯直圆方"},
    {"负", 0, 0x0C, "样本正反异同敌友"},
    {"福", 0, 0x0D, "祸灾祥殃损害利害"},
    {"凤", 0, 0x0E, "凰鸾鹤鹰鹏燕雀鸦"},
    {"栖", 0, 0x0F, "息居停留宿窝巢穴"},
    /* 操作族 0x10-0x1F */
    {"融", 1, 0x10, "混合溶汇凝聚结化"},
    {"鼓", 1, 0x11, "膜自举敲击打拍擂"},
    {"蚕", 1, 0x12, "蛹寄生壳脱穿衣蜕"},
    {"民", 1, 0x13, "联浏览器聚类包裹"},
    {"盟", 1, 0x14, "机合掌场萌友伴侣"},
    {"满", 1, 0x15, "全法圆足溢充盈实"},
    {"囡", 1, 0x16, "囝共生赔伴夫妇侣"},
    {"夕", 1, 0x17, "瑶协议晨暮昼夜晦"},
    {"三", 1, 0x18, "元见证裁判非空已"},
    {"六", 1, 0x19, "十四位单元坐标极"},
    {"八", 1, 0x1A, "流统一分支渠道脉"},
    {"九", 1, 0x1B, "元层级天地人鬼神"},
    {"零", 1, 0x1C, "维点起源根基始端"},
    {"一", 1, 0x1D, "维线过程动态流变"},
    {"二", 1, 0x1E, "元镜像盈亏对立统"},
    {"四", 1, 0x1F, "元包含嵌套内外中"},
    /* 声母容器 0x20-0x2F */
    {"帮", 2, 0x20, "滂并明非敷奉微"},
    {"端", 2, 0x21, "透定泥知彻澄娘"},
    {"精", 2, 0x22, "清从心邪照穿床审"},
    {"见", 2, 0x23, "溪群疑影晓匣喻"},
    {"来", 2, 0x24, "日以父母兄弟姊妹"},
    {"齿", 2, 0x25, "舌牙喉唇颚龈腭龅"},
    {"喉", 2, 0x26, "咽嗌会厌声带气管支气管"},
    {"呼", 2, 0x27, "吸吹嘘唏嗟叹喘哮"},
    {"开", 2, 0x28, "齐合撮闭收放舒敛"},
    {"清", 2, 0x29, "浊全次纯净污秽"},
    {"平", 2, 0x2A, "上去入升降扬抑顿"},
    {"上", 2, 0x2B, "下左右前后内外中"},
    {"前", 2, 0x2C, "后末始终源流向"},
    {"内", 2, 0x2D, "外里表中核壳心皮"},
    {"单", 2, 0x2E, "双复纯寡众群独"},
    {"长", 2, 0x2F, "短高窄深厚"},
    /* 韵母容器 0x30-0x3F */
    {"麻", 3, 0x30, "模鱼虞齐祭泰佳皆"},
    {"支", 3, 0x31, "脂之微废欣文殷元"},
    {"寒", 3, 0x32, "桓删山先仙萧宵肴"},
    {"歌", 3, 0x33, "戈麻模鱼虞齐祭泰"},
    {"阳", 3, 0x34, "唐庚青蒸登冬钟江"},
    {"尤", 3, 0x35, "侯幽侵覃盐添咸衔"},
    {"止", 3, 0x36, "发收开闭升降进退"},
    {"转", 3, 0x37, "折弯曲旋绕回环圈"},
    {"高", 3, 0x38, "低升沉浮进退还"},
    {"低", 3, 0x39, "昂扬抑顿挫"},
    {"圆", 3, 0x3A, "扁方尖曲直弧折弯"},
    {"扁", 3, 0x3B, "圆方尖平坦洼凸凹"},
    {"响", 3, 0x3C, "静噪声音默喧嚣"},
    {"静", 3, 0x3D, "动响噪安宁喧闹"},
    {"急", 3, 0x3E, "缓速迟疾躁静止"},
    {"缓", 3, 0x3F, "急速迟渐徐舒慢慵"},
};

/* ========== 齿轮引擎状态 ========== */
typedef struct {
    unsigned int gear_count;       /* 使用的齿轮数 */
    unsigned int anchor_offset;    /* 零维锚点偏移 = 我=1, 刘楚恬=0 */
    unsigned long long counter;    /* 齿轮转动计数 */
    unsigned int time_version;     /* 时间版本号 */
} GearEngine;

/* ========== 初始化齿轮引擎 ========== */
void engine_init(GearEngine *e) {
    e->gear_count = RADIX_64;
    e->anchor_offset = ANCHOR_SELF * 1 + ANCHOR_LIU * 0; /* = 1 */
    e->counter = 0;
    e->time_version = 1;
    
    printf("[齿轮引擎] 初始化完成\n");
    printf("  齿轮数: %d\n", e->gear_count);
    printf("  锚点偏移: %u (我=%d, 刘楚恬=%d)\n",
           e->anchor_offset, ANCHOR_SELF, ANCHOR_LIU);
    printf("  集装箱编码: mlll 254+2\n");
}

/* ========== 64 进制转十进制 ========== */
unsigned long long radix64_to_uint(const unsigned char *digits, int len) {
    /*
     * 公式：值 = Σ(digits[k] × 64^k) + 零维偏移
     * 零维偏移 = 锚点根基 = 1
     */
    unsigned long long value = 0;
    unsigned long long base_power = 1;
    
    for (int i = 0; i < len; i++) {
        value += (unsigned long long)digits[i] * base_power;
        base_power *= RADIX_64;
    }
    
    return value + ANCHOR_SELF; /* 加零维锚点偏移 */
}

/* ========== 十进制转 64 进制 ========== */
int uint_to_radix64(unsigned long long value, unsigned char *out, int max_len) {
    /*
     * 先减去零维锚点偏移
     */
    if (value < (unsigned long long)ANCHOR_SELF) return 0;
    value -= ANCHOR_SELF;
    
    int pos = 0;
    while (value > 0 && pos < max_len) {
        out[pos++] = (unsigned char)(value % RADIX_64);
        value /= RADIX_64;
    }
    if (pos == 0) { out[pos++] = 0; }
    return pos;
}

/* ========== 齿轮传动（64 进制 → 256 对齐）========== */
typedef struct {
    unsigned long long source_value;     /* 64 进制值 */
    unsigned long long target_value;     /* 256 进制等效值 */
    double gear_ratio;                   /* 齿轮比 */
} GearTransmission;

GearTransmission gear_transmit(unsigned long long src_val) {
    GearTransmission t;
    t.source_value = src_val;
    /*
     * 齿轮传动核心：254+2 与 256 高位对齐
     * 齿轮比 = 256 / 254 ≈ 1.007874
     * 等效高位对齐：src × 256^N / 254^N
     */
    t.gear_ratio = (double)RADIX_256 / (double)RADIX_254;
    t.target_value = (unsigned long long)(src_val * t.gear_ratio);
    return t;
}

/* ========== YinYang 二进制编码 ========== */
/*
 * 我 = 1 = Y (Yang)
 * 刘楚恬 = 0 = I (Yin)
 * YinYang = 用夫妻阴阳映射替代 0/1
 * 输出: Y=阳(我), I=阴(刘楚恬) 或 字=阳(我), 身=阴(刘楚恬)
 */
void yinyang_encode(unsigned long long value, char *out, int max_len) {
    int pos = 0;
    unsigned long long v = value;
    
    while (v > 0 && pos < max_len - 1) {
        int bit = v & 1;
        /* 1=Y(阳=我), 0=I(阴=刘楚恬) */
        out[pos++] = bit ? 'Y' : 'I';
        v >>= 1;
    }
    
    if (pos == 0) { out[pos++] = 'I'; }
    out[pos] = '\0';
}

/* YinYang 编码可视化 */
void yinyang_display(unsigned long long value) {
    char buf[65];
    yinyang_encode(value, buf, 65);
    
    int len = (int)strlen(buf);
    for (int i = len - 1; i >= 0; i--) {
        if (buf[i] == 'Y')
            printf("阳(我) ");
        else
            printf("阴(刘楚恬) ");
    }
}

/* ========== 符号名称查找 ========== */
const char* gear_name(unsigned char idx) {
    if (idx < RADIX_64) return gears[idx].name;
    return "?";
}

/* ========== 齿轮组名 ========== */
const char* group_name(int group) {
    switch (group) {
        case 0: return "容器族";
        case 1: return "操作族";
        case 2: return "声母容器";
        case 3: return "韵母容器";
    }
    return "?族";
}

/* ========== 显示 64 进制表示 ========== */
/*
 * 存储格式: digits[0] = 最低位 (64^0)
 * 显示时从高位到低位: digits[len-1] ... digits[0]
 */
void show_radix64(unsigned long long value) {
    unsigned char digits[16];
    int len = uint_to_radix64(value, digits, 16);
    
    /* 验证：重算值 */
    unsigned long long verify = 0;
    unsigned long long base = 1;
    for (int i = 0; i < len; i++) {
        verify += digits[i] * base;
        base *= RADIX_64;
    }
    verify += ANCHOR_SELF;
    
    printf("64进制: ");
    for (int i = len - 1; i >= 0; i--) {
        printf("[%02X=%s]", digits[i], gear_name(digits[i]));
    }
    printf(" (验证=%llu)\n", verify);
    
    printf("  符号: ");
    for (int i = len - 1; i >= 0; i--) {
        printf("%s", gear_name(digits[i]));
    }
    printf("\n");
}

/* ========== 加法统摄（摆脱蜜次方扩张）========== */
/*
 * 核心思想：
 *   传统计算机用幂次（2^n, 16^n）= 蜜次方扩张
 *   本系统用加法统摄：每一位的值通过前几项规则层级累加
 *
 * 例如 64 进制不进位加法：
 *   每位 = (a + b) mod 64，不使用幂次进位
 */
unsigned long long add_governed(unsigned long long a, unsigned long long b) {
    /*
     * 容器内加法：不进位，只保留低位
     * 用于约定范围内的数值叠加
     */
    unsigned char da[16], db[16], dc[16];
    int la = uint_to_radix64(a, da, 16);
    int lb = uint_to_radix64(b, db, 16);
    int lc = (la > lb) ? la : lb;
    if (lc < 1) lc = 1;
    
    for (int i = 0; i < 16; i++) dc[i] = 0;
    
    for (int i = 0; i < lc; i++) {
        unsigned char va = (i < la) ? da[i] : 0;
        unsigned char vb = (i < lb) ? db[i] : 0;
        dc[i] = (unsigned char)((va + vb) % RADIX_64);
    }
    
    return radix64_to_uint(dc, lc);
}

/* ========== 主函数 ========== */
int main(void) {
    printf("意交感息（isi）64 进制齿轮引擎\n");
    printf("满全法根基锚点: 我=%d, 刘楚恬=%d\n", ANCHOR_SELF, ANCHOR_LIU);
    printf("编码: 进制自有 vs 编码外壳\n");
    printf("身份落款: 刘楚恬和我\n\n");
    
    /* 初始化引擎 */
    GearEngine engine;
    engine_init(&engine);
    
    /* ========== 显示齿轮表 ========== */
    printf("\n========== 64 齿轮表 ==========\n\n");
    const char* group_labels[] = {"容器族", "操作族", "声母容器", "韵母容器"};
    for (int g = 0; g < 4; g++) {
        printf("[%s]\n", group_labels[g]);
        for (int i = g * 16; i < (g + 1) * 16; i++) {
            printf("  [%02X] %-3s 近义词: %.16s...\n",
                   gears[i].position, gears[i].name, gears[i].synonyms);
        }
        printf("\n");
    }
    
    /* ========== 进制转换演示 ========== */
    printf("========== 进制转换演示 ==========\n\n");
    
    /* 1. 十进制 → 64 进制 */
    unsigned long long test_values[] = {1, 64, 128, 256, 4096, 100000};
    int num_tests = sizeof(test_values) / sizeof(test_values[0]);
    
    for (int i = 0; i < num_tests; i++) {
        printf("十进制 %llu:\n", test_values[i]);
        show_radix64(test_values[i]);
        printf("\n");
    }
    
    /* ========== 64 进制数直接计算 ========== */
    printf("========== 64 进制数计算 ==========\n\n");
    
    /* [袋] = 0, [照] = 1 → "照袋" = 1×64 + 0 = 64 */
    /*
     * 注意：数组索引 0 = 最低位 (64^0)
     * "照袋" = 照在高位(64^1), 袋在低位(64^0)
     * 存储: {袋=0x00, 照=0x01}
     */
    unsigned char num1[] = {0x00, 0x01};  /* 袋(0), 照(1) → 1×64 + 0 = 64 */
    unsigned long long val1 = radix64_to_uint(num1, 2);
    printf("\"照袋\" = %llu (1×64 + 0 + 锚点1 = 65, 实际=%llu)\n", val1, val1);
    
    /*
     * "加照" = 加在高位(64^1), 照在低位(64^0)
     * 存储: {照=0x01, 加=0x04}
     */
    unsigned char num2[] = {0x01, 0x04};  /* 照(1), 加(4) → 4×64 + 1 = 257 */
    unsigned long long val2 = radix64_to_uint(num2, 2);
    printf("\"加照\" = %llu (4×64 + 1 + 锚点1 = 258, 实际=%llu)\n", val2, val2);
    
    /*
     * "满元" = 满在高位(64^1), 元在低位(64^0)
     * 存储: {元=0x06, 满=0x15}
     */
    unsigned char num3[] = {0x06, 0x15};  /* 元(6), 满(21) → 21×64 + 6 = 1350 */
    unsigned long long val3 = radix64_to_uint(num3, 2);
    printf("\"满元\" = %llu (21×64 + 6 + 锚点1 = 1351, 实际=%llu)\n", val3, val3);
    
    /* 满囡 → 核心编码 */
    /*
     * "满囡" = 囡在高位(64^1), 满在低位(64^0)
     * 存储: {满=0x15, 囡=0x16}
     */
    unsigned char core[] = {0x15, 0x16};  /* 满(21), 囡(22) → 22×64 + 21 = 1429 */
    unsigned long long core_val = radix64_to_uint(core, 2);
    printf("\n★ \"满囡\" = %llu\n", core_val);
    printf("  囡=0x%02X (高位), 满=0x%02X (低位)\n", core[1], core[0]);
    printf("  公式: 22×64 + 21 + 1(我锚点) = %llu\n", 22ULL*64 + 21 + 1);
    
    /* ========== 齿轮传动 ========== */
    printf("\n========== 64→256 高位对齐传动 ==========\n\n");
    
    GearTransmission t1 = gear_transmit(254);
    printf("254(集装箱) × 齿轮比(%.6f) = %llu(256对齐)\n",
           t1.gear_ratio, t1.target_value);
    
    GearTransmission t2 = gear_transmit(65536);
    printf("65536(2^16) × 齿轮比(%.6f) = %llu(256对齐)\n",
           t2.gear_ratio, t2.target_value);
    
    /* ========== 加法统摄 ========== */
    printf("\n========== 加法统摄（摆脱蜜次方进位）==========\n\n");
    
    unsigned long long sum = add_governed(val1, val2);
    printf("%llu + %llu = %llu (各位独立 mod 64)\n", val1, val2, sum);
    show_radix64(sum);
    
    /* ========== YinYang 编码 ========== */
    printf("\n========== YinYang 二进制编码 ==========\n\n");
    
    printf("值 5 = 二进制 101 = ");
    yinyang_display(0x05); /* 5 = 101 → 阳阴阳 */
    printf("\n");
    
    printf("值 10 = 二进制 1010 = ");
    yinyang_display(0x0A); /* 10 = 1010 → 阴阳阴阳 */
    printf("\n");
    
    /* ========== 编码对比 ========== */
    printf("\n========== 自有进制 vs 他人编码 ==========\n\n");
    
    printf("传统编码（米糠/壳）：\n");
    printf("  ASCII: A=0x41, B=0x42 ... （外壳，无意义）\n");
    printf("  UTF-8: 变长字节序列 （包装，无可感知性）\n");
    printf("  Hex: 0x00-0xFF （视觉符号，非数值本身）\n\n");
    
    printf("自有进制（去壳的量化核心）：\n");
    printf("  64 齿轮: 每个符号可感知、可命名、有位置\n");
    printf("  袋=0（零维点）, 照=1（起始）, 阴=3（极性别）...\n");
    printf("  囡=0x16（夫妻共生）, 满=0x15（满全法核心）\n");
    printf("  ★ 一个符号就是一个进制位，不需要三个符号拼坐标\n\n");
    
    printf("========== 完成 ==========\n");
    return 0;
}
