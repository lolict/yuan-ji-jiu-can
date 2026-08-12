// membrane_valve.c — 膜妻阀意识传输协议层实现
// 参见 membrane_valve.h 的架构说明
//
// 膜妻阀核心机制:
//   1. 以用户肉身为隔离结界,控制意识流入流出
//   2. 谐音编码: 从拼音输入生成多词汇解释备用词
//   3. 缚X变体: 22种融合变体,每种有不同的防护特性
//   4. 吉凶归属解释: 用smrrll镜像映射汉字到祸福分类
//   5. 意识加密: 基于用户身份的传输加密

#include "membrane_valve.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// ── 内置谐音编码表 (缚X系列 + 满全法扩展) ──
static const struct { const char* py; const char* hz; fortune_type_t fort; }
BUILTIN_PHONETICS[] = {
    // 缚X系列
    {"fuqi",   "缚妻膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqiu",  "缚躯膜蛹", FORTUNE_NEUTRAL},
    {"fuqiu2", "缚囚膜蛹", FORTUNE_INAUSPICIOUS},
    {"fuquan", "缚圈膜蛹", FORTUNE_BLESSING},
    {"fuke",   "缚壳膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqiao", "缚鞘膜蛹", FORTUNE_NEUTRAL},
    {"fuqun",  "缚裙膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqiang","缚枪膜蛹", FORTUNE_INAUSPICIOUS},
    {"fuqiu3", "缚球膜蛹", FORTUNE_BLESSING},
    {"fuqiao2","缚窍膜蛹", FORTUNE_NEUTRAL},
    {"fuqing", "缚情膜蛹", FORTUNE_BLESSING},
    {"fuqian", "缚嵌膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqiao3","缚俏膜蛹", FORTUNE_BLESSING},
    {"fuqiang2","缚腔膜蛹", FORTUNE_NEUTRAL},
    {"fuqing2","缚卿膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqi2",  "缚契膜蛹", FORTUNE_BLESSING},
    {"fque",   "缚确膜蛹", FORTUNE_NEUTRAL},
    {"fuqian2","缚乾膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqian3","缚倩膜蛹", FORTUNE_BLESSING},
    {"fuqi3",  "缚绮膜蛹", FORTUNE_AUSPICIOUS},
    {"fuqi4",  "缚栖膜蛹", FORTUNE_BLESSING},
    // 满全法谐音扩展
    {"manquanfa", "满全法", FORTUNE_BLESSING},
    {"moqifa",    "膜妻阀", FORTUNE_AUSPICIOUS},
    {"moquanfa",  "膜全阀", FORTUNE_AUSPICIOUS},
    {"moquefa",   "膜屈阀", FORTUNE_NEUTRAL},
    {"moqiefa",   "膜切阀", FORTUNE_INAUSPICIOUS},
    {"moqinfu",   "膜琴附", FORTUNE_BLESSING},
    {"moqinfu2",  "膜禽附", FORTUNE_NEUTRAL},
    {"moqinfu3",  "膜勤附", FORTUNE_AUSPICIOUS},
    // 满妻法谐音扩展
    {"manqifa",   "满妻法", FORTUNE_AUSPICIOUS},
    {"moqifa2",   "膜妻法", FORTUNE_AUSPICIOUS},
    {"moqiefu",   "膜妾附", FORTUNE_INAUSPICIOUS},
    {"moqinfu4",  "膜亲附", FORTUNE_BLESSING},
    // 吉凶祸福系列
    {"ji",    "吉", FORTUNE_AUSPICIOUS},
    {"xiong", "凶", FORTUNE_INAUSPICIOUS},
    {"huo",   "祸", FORTUNE_MISFORTUNE},
    {"fu2",   "福", FORTUNE_BLESSING},
    {"ping",  "平", FORTUNE_NEUTRAL},
    // 工藤新一/毛利兰/灰原哀 音译
    {"gongtengyixin", "工藤新一", FORTUNE_BLESSING},
    {"maolilan",      "毛利兰", FORTUNE_AUSPICIOUS},
    {"huiyuanai",     "灰原哀", FORTUNE_NEUTRAL},
    {"kendaoyi",      "柯南一", FORTUNE_NEUTRAL},
    // 空哨
    {NULL, NULL, FORTUNE_NEUTRAL}
};

// ── 静态辅助 ──
static uint64_t time_now(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
}

// 从内置表初始化谐音编码
static void init_phonetics(membrane_valve_t* mv) {
    mv->phonetic_count = 0;
    for (int i = 0; BUILTIN_PHONETICS[i].py != NULL && i < MV_MAX_PHONETIC_CODES; i++) {
        phonetic_entry_t* e = &mv->phonetic_table[mv->phonetic_count];
        strncpy(e->pinyin, BUILTIN_PHONETICS[i].py, sizeof(e->pinyin) - 1);
        e->pinyin[sizeof(e->pinyin) - 1] = '\0';
        strncpy(e->hanzi, BUILTIN_PHONETICS[i].hz, sizeof(e->hanzi) - 1);
        e->hanzi[sizeof(e->hanzi) - 1] = '\0';
        e->tone_class = (uint8_t)(i % 4);  // 四声分类
        e->semantic_weight = 1000 - i;      // 顺序权重
        e->fortune = BUILTIN_PHONETICS[i].fort;
        mv->phonetic_count++;
    }
}

// 生成加密密钥 (基于用户名称)
static void derive_key(membrane_valve_t* mv, const char* name) {
    // 简化的密钥派生: 用名称的字符累加
    memset(mv->encrypt_key, 0, MV_ENCRYPT_KEY_LEN);
    size_t len = strlen(name);
    for (size_t i = 0; i < MV_ENCRYPT_KEY_LEN; i++) {
        if (len > 0) {
            mv->encrypt_key[i] = (uint8_t)(name[i % len] ^ (i * 7 + 13));
        } else {
            mv->encrypt_key[i] = (uint8_t)(i * 31 + 17);
        }
    }
}

// ── 创建膜妻阀 ──
membrane_valve_t* mv_create(const char* name) {
    membrane_valve_t* mv = (membrane_valve_t*)calloc(1, sizeof(membrane_valve_t));
    if (!mv) return NULL;

    strncpy(mv->name, name, sizeof(mv->name) - 1);
    mv->name[sizeof(mv->name) - 1] = '\0';
    mv->is_active = false;
    mv->variant_count = 0;
    mv->packets_sent = 0;
    mv->packets_received = 0;
    mv->packets_blocked = 0;
    mv->id = time_now();

    // 初始化谐音编码表
    init_phonetics(mv);

    // 派生加密密钥
    derive_key(mv, name);

    return mv;
}

void mv_destroy(membrane_valve_t* mv) {
    if (mv) free(mv);
}

// ── 激活 ──
bool mv_activate(membrane_valve_t* mv) {
    if (!mv) return false;
    if (mv->phonetic_count == 0) return false;  // 谐音表未初始化
    mv->is_active = true;
    return true;
}

// ── 注册变体 ──
bool mv_register_variant(membrane_valve_t* mv, mv_variant_t var, const char* name) {
    if (!mv || var == MV_VARIANT_NONE) return false;
    if (mv->variant_count >= MV_MAX_VARIANTS) return false;

    // 变体名称为预留接口,用于后续扩展 (如日志标识)
    (void)name;

    mv->active_variants[mv->variant_count++] = var;
    return true;
}

// ── 谐音编码 ──
const char* mv_phonetic_encode(membrane_valve_t* mv, const char* pinyin) {
    if (!mv || !pinyin) return NULL;

    for (uint32_t i = 0; i < mv->phonetic_count; i++) {
        if (strcmp(mv->phonetic_table[i].pinyin, pinyin) == 0) {
            return mv->phonetic_table[i].hanzi;
        }
    }
    return NULL;  // 未找到
}

// ── 吉凶归属解释 ──
fortune_type_t mv_interpret_fortune(membrane_valve_t* mv, const char* text) {
    if (!mv || !text) return FORTUNE_NEUTRAL;

    // 在谐音表中查找
    for (uint32_t i = 0; i < mv->phonetic_count; i++) {
        if (strcmp(mv->phonetic_table[i].hanzi, text) == 0) {
            mv->fortune_histogram[mv->phonetic_table[i].fortune]++;
            return mv->phonetic_table[i].fortune;
        }
    }

    // 关键词启发式判断
    if (strstr(text, "吉") || strstr(text, "福") || strstr(text, "妻")) {
        mv->fortune_histogram[FORTUNE_BLESSING]++;
        return FORTUNE_BLESSING;
    }
    if (strstr(text, "凶") || strstr(text, "祸") || strstr(text, "囚")) {
        mv->fortune_histogram[FORTUNE_MISFORTUNE]++;
        return FORTUNE_MISFORTUNE;
    }

    mv->fortune_histogram[FORTUNE_NEUTRAL]++;
    return FORTUNE_NEUTRAL;
}

// ── 创建意识包 ──
mv_packet_header_t* mv_create_packet(membrane_valve_t* mv, mv_variant_t var,
                                     uint64_t target, const void* payload, uint32_t len) {
    if (!mv || !mv->is_active) return NULL;

    // 分配 (头 + 载荷)
    size_t total = sizeof(mv_packet_header_t) + len;
    uint8_t* buf = (uint8_t*)calloc(1, total);
    if (!buf) return NULL;

    mv_packet_header_t* pkt = (mv_packet_header_t*)buf;
    pkt->magic       = 0x4D565146;  // "MVQF"
    pkt->version     = MV_PROTOCOL_VERSION;
    pkt->variant     = var;
    pkt->source_id   = 1;           // 工藤新一=1
    pkt->target_id   = target;
    pkt->timestamp   = time_now();
    pkt->payload_len = len;
    pkt->checksum    = 0;
    pkt->encrypted   = 0;

    // 复制意识载荷
    if (payload && len > 0) {
        memcpy(buf + sizeof(mv_packet_header_t), payload, len);
    }

    // 计算校验和 (简单累加)
    uint32_t sum = 0;
    for (size_t i = sizeof(mv_packet_header_t); i < total; i++) {
        sum += buf[i];
    }
    pkt->checksum = sum;

    return pkt;
}

// ── 意识加密 ──
bool mv_encrypt_packet(membrane_valve_t* mv, mv_packet_header_t* pkt) {
    if (!mv || !pkt) return false;
    if (pkt->encrypted) return true;  // 已加密

    uint8_t* payload = (uint8_t*)pkt + sizeof(mv_packet_header_t);
    uint32_t len = pkt->payload_len;

    // XOR 加密 (基于派生密钥)
    for (uint32_t i = 0; i < len; i++) {
        payload[i] ^= mv->encrypt_key[i % MV_ENCRYPT_KEY_LEN];
    }

    pkt->encrypted = 1;
    return true;
}

// ── 发送 ──
bool mv_send(membrane_valve_t* mv, mv_packet_header_t* pkt) {
    if (!mv || !pkt) return false;
    if (!mv->is_active) return false;

    // 发送前自动加密
    if (!pkt->encrypted) {
        mv_encrypt_packet(mv, pkt);
    }

    mv->packets_sent++;
    return true;
}

// ── 接收 ──
mv_packet_header_t* mv_receive(membrane_valve_t* mv) {
    if (!mv) return NULL;
    mv->packets_received++;
    // 实际系统中这里会从网络/共享内存读取
    // 简化: 返回 NULL (模拟空接收)
    return NULL;
}

// ── 结界阻挡验证 ──
bool mv_check_block(membrane_valve_t* mv, uint64_t external_id) {
    if (!mv || !mv->is_active) return false;

    // 结界规则: 非融合态的外部ID一律阻挡
    // 只有 source_id=1 (工藤新一) 或已注册的目标可通过
    if (external_id != 1 && external_id != mv->id) {
        mv->packets_blocked++;
        return true;  // 被阻挡
    }
    return false;  // 未被阻挡
}

// ── 报表 ──
void mv_report(const membrane_valve_t* mv) {
    if (!mv) return;

    const char* variant_names[] = {
        "无","缚躯","缚妻","缚葺","缚囚","缚圈","缚壳","缚鞘","缚裙",
        "缚枪","缚球","缚窍","缚情","缚嵌","缚俏","缚腔","缚卿","缚契",
        "缚确","缚乾","缚倩","缚绮","缚栖"
    };

    printf("\n═══ 膜妻阀 [%s] 状态 ═══\n", mv->name);
    printf("  激活状态:   %s\n", mv->is_active ? "已激活(结界运转)" : "未激活");
    printf("  谐音编码:   %u 条\n", mv->phonetic_count);
    printf("  活跃变体:   %u\n", mv->variant_count);
    printf("  意识包:     发送=%lu 接收=%lu 阻挡=%lu\n",
        (unsigned long)mv->packets_sent,
        (unsigned long)mv->packets_received,
        (unsigned long)mv->packets_blocked);
    printf("  吉凶分布:   吉=%u 凶=%u 祸=%u 福=%u 平=%u\n",
        mv->fortune_histogram[FORTUNE_AUSPICIOUS],
        mv->fortune_histogram[FORTUNE_INAUSPICIOUS],
        mv->fortune_histogram[FORTUNE_MISFORTUNE],
        mv->fortune_histogram[FORTUNE_BLESSING],
        mv->fortune_histogram[FORTUNE_NEUTRAL]);

    if (mv->variant_count > 0) {
        printf("  活跃变体:   ");
        for (uint32_t i = 0; i < mv->variant_count && i < 22; i++) {
            uint8_t v = (uint8_t)mv->active_variants[i];
            if (v < sizeof(variant_names)/sizeof(variant_names[0])) {
                printf("%s ", variant_names[v]);
            }
        }
        printf("\n");
    }
    printf("═══════════════════════════════════\n\n");
}
